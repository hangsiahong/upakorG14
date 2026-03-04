use std::path::{Path, PathBuf};
use std::fs;
use crate::utils::Result;
use crate::models::{Temperature, FanSpeeds, PowerDraw};

/// Trait for system hardware monitoring via sysfs
#[async_trait::async_trait]
pub trait SysMonitorTrait: Send + Sync {
    /// Read current temperatures from sysfs
    async fn get_temperatures(&self) -> Result<Temperature>;

    /// Read fan speeds from sysfs
    async fn get_fan_speeds(&self) -> Result<FanSpeeds>;

    /// Read power draw (may not be available on all systems)
    async fn get_power_draw(&self) -> Result<PowerDraw>;
}

/// Real implementation that reads from Linux sysfs
pub struct RealSysMonitor {
    thermal_base: PathBuf,
    hwmon_base: PathBuf,
    power_supply_base: PathBuf,
}

impl RealSysMonitor {
    /// Create a new sysfs monitor with default paths
    pub fn new() -> Self {
        Self {
            thermal_base: PathBuf::from("/sys/class/thermal"),
            hwmon_base: PathBuf::from("/sys/class/hwmon"),
            power_supply_base: PathBuf::from("/sys/class/power_supply"),
        }
    }

    /// Create a sysfs monitor with custom base paths (useful for testing)
    pub fn with_paths(thermal_base: PathBuf, hwmon_base: PathBuf, power_supply_base: PathBuf) -> Self {
        Self {
            thermal_base,
            hwmon_base,
            power_supply_base,
        }
    }

    /// Find the CPU thermal zone
    fn find_cpu_thermal_zone(&self) -> Option<PathBuf> {
        if !self.thermal_base.exists() {
            tracing::debug!("Thermal base path does not exist: {:?}", self.thermal_base);
            return None;
        }

        // Try common thermal zone names for CPU
        let zone_names = ["thermal_zone0", "thermal_zone1", "cpu-thermal", "x86_pkg_temp"];

        for zone_name in zone_names {
            let zone_path = self.thermal_base.join(zone_name);
            if zone_path.exists() {
                tracing::debug!("Found CPU thermal zone: {:?}", zone_path);
                return Some(zone_path);
            }
        }

        // If no specific zone found, try the first available thermal zone
        if let Ok(entries) = fs::read_dir(&self.thermal_base) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.join("temp").exists() {
                    tracing::debug!("Using thermal zone: {:?}", path);
                    return Some(path);
                }
            }
        }

        None
    }

    /// Read temperature from a thermal zone path
    fn read_temp_from_zone(&self, zone_path: &Path) -> Option<f64> {
        let temp_file = zone_path.join("temp");
        match fs::read_to_string(&temp_file) {
            Ok(content) => {
                // Temperature is in millidecelsius
                let temp_millidecelsius: i64 = content.trim().parse().ok()?;
                Some(temp_millidecelsius as f64 / 1000.0)
            }
            Err(e) => {
                tracing::debug!("Failed to read temp file {:?}: {}", temp_file, e);
                None
            }
        }
    }

    /// Find hwmon devices and read fan speeds
    fn read_fan_speeds(&self) -> Vec<(String, u32)> {
        let mut speeds = Vec::new();

        if !self.hwmon_base.exists() {
            tracing::debug!("hwmon base path does not exist: {:?}", self.hwmon_base);
            return speeds;
        }

        if let Ok(entries) = fs::read_dir(&self.hwmon_base) {
            for entry in entries.flatten() {
                let hwmon_path = entry.path();
                let hwmon_name = hwmon_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                // Try to read fan inputs (fan1_input, fan2_input, etc.)
                for fan_num in 1..=5 {
                    let fan_file = hwmon_path.join(format!("fan{}_input", fan_num));
                    if let Ok(content) = fs::read_to_string(&fan_file) {
                        if let Ok(rpm) = content.trim().parse::<u32>() {
                            if rpm > 0 {
                                speeds.push((format!("{}-fan{}", hwmon_name, fan_num), rpm));
                            }
                        }
                    }
                }
            }
        }

        speeds
    }

    /// Read temperature from hwmon devices
    fn read_hwmon_temperatures(&self) -> Vec<(String, f64)> {
        let mut temps = Vec::new();

        if !self.hwmon_base.exists() {
            return temps;
        }

        if let Ok(entries) = fs::read_dir(&self.hwmon_base) {
            for entry in entries.flatten() {
                let hwmon_path = entry.path();

                // Try to read the name to identify the device
                let name_file = hwmon_path.join("name");
                let device_name = if name_file.exists() {
                    fs::read_to_string(&name_file)
                        .ok()
                        .map(|s| s.trim().to_string())
                } else {
                    None
                };

                // Read temperature inputs (temp1_input, temp2_input, etc.)
                for temp_num in 1..=10 {
                    let temp_file = hwmon_path.join(format!("temp{}_input", temp_num));
                    if let Ok(content) = fs::read_to_string(&temp_file) {
                        if let Ok(millidecelsius) = content.trim().parse::<i64>() {
                            let temp_celsius = millidecelsius as f64 / 1000.0;
                            let label = device_name.clone()
                                .unwrap_or_else(|| format!("unknown-temp{}", temp_num));
                            temps.push((label, temp_celsius));
                        }
                    }
                }
            }
        }

        temps
    }

    /// Read battery capacity (percentage)
    fn read_battery_capacity(&self) -> Option<u8> {
        let battery_path = self.power_supply_base.join("BAT0");
        if !battery_path.exists() {
            // Try BAT1 as fallback
            let battery_path = self.power_supply_base.join("BAT1");
            if !battery_path.exists() {
                return None;
            }
        }

        let capacity_file = battery_path.join("capacity");
        fs::read_to_string(&capacity_file)
            .ok()?
            .trim()
            .parse::<u8>()
            .ok()
    }

    /// Read battery status (Charging/Discharging)
    fn read_battery_status(&self) -> Option<String> {
        let battery_base = if self.power_supply_base.join("BAT0").exists() {
            self.power_supply_base.join("BAT0")
        } else if self.power_supply_base.join("BAT1").exists() {
            self.power_supply_base.join("BAT1")
        } else {
            return None;
        };

        let status_file = battery_base.join("status");
        fs::read_to_string(&status_file)
            .ok()
            .map(|s| s.trim().to_string())
    }
}

impl Default for RealSysMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SysMonitorTrait for RealSysMonitor {
    async fn get_temperatures(&self) -> Result<Temperature> {
        // Try thermal zones first
        if let Some(zone_path) = self.find_cpu_thermal_zone() {
            if let Some(cpu_temp) = self.read_temp_from_zone(&zone_path) {
                return Ok(Temperature {
                    cpu: cpu_temp as f32,
                    gpu: None, // GPU temp would need specific hwmon detection
                });
            }
        }

        // Fallback to hwmon
        let hwmon_temps = self.read_hwmon_temperatures();
        if let Some((_, cpu_temp)) = hwmon_temps.first() {
            return Ok(Temperature {
                cpu: *cpu_temp as f32,
                gpu: None,
            });
        }

        tracing::warn!("Could not read temperature from sysfs");
        Ok(Temperature {
            cpu: 0.0,
            gpu: None,
        })
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        let fan_readings = self.read_fan_speeds();

        if fan_readings.is_empty() {
            tracing::debug!("No fan speeds found in sysfs");
            return Ok(FanSpeeds {
                cpu_rpm: 0,
                gpu_rpm: 0,
                cpu_percentage: 0,
                gpu_percentage: 0,
            });
        }

        // Heuristic: first fan is CPU, second is GPU (if available)
        let cpu_rpm = fan_readings.first().map(|(_, rpm)| *rpm).unwrap_or(0);
        let gpu_rpm = fan_readings.get(1).map(|(_, rpm)| *rpm).unwrap_or(0);

        // Calculate percentages (assuming max RPM around 5000)
        let cpu_percentage = if cpu_rpm > 0 {
            ((cpu_rpm as f64 / 5000.0) * 100.0).min(100.0).max(0.0) as u8
        } else {
            0
        };

        let gpu_percentage = if gpu_rpm > 0 {
            ((gpu_rpm as f64 / 5000.0) * 100.0).min(100.0).max(0.0) as u8
        } else {
            0
        };

        tracing::debug!("Fan speeds: {:?} (CPU: {} RPM, GPU: {} RPM)", fan_readings, cpu_rpm, gpu_rpm);

        Ok(FanSpeeds {
            cpu_rpm,
            gpu_rpm,
            cpu_percentage,
            gpu_percentage,
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        // Power draw is often not available via sysfs
        // We can estimate from battery discharge rate if on battery
        let battery_status = self.read_battery_status();
        let is_discharging = battery_status.as_deref() == Some("Discharging");

        if is_discharging {
            // Try to read current and voltage to calculate power
            // This is hardware-specific and may not be available
            // For now, return zeros
            tracing::debug!("Battery is discharging, but power calculation not implemented");
        }

        Ok(PowerDraw {
            cpu: 0.0,
            gpu: None,
            total: 0.0,
        })
    }
}

/// Mock implementation for testing
#[derive(Clone)]
pub struct MockSysMonitor {
    cpu_temp: f64,
    cpu_fan_rpm: u32,
    gpu_fan_rpm: u32,
}

impl MockSysMonitor {
    pub fn new() -> Self {
        Self {
            cpu_temp: 45.0,
            cpu_fan_rpm: 2000,
            gpu_fan_rpm: 0,
        }
    }

    pub fn with_temp(mut self, temp: f64) -> Self {
        self.cpu_temp = temp;
        self
    }

    pub fn with_cpu_fan(mut self, rpm: u32) -> Self {
        self.cpu_fan_rpm = rpm;
        self
    }

    pub fn with_gpu_fan(mut self, rpm: u32) -> Self {
        self.gpu_fan_rpm = rpm;
        self
    }
}

impl Default for MockSysMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SysMonitorTrait for MockSysMonitor {
    async fn get_temperatures(&self) -> Result<Temperature> {
        Ok(Temperature {
            cpu: self.cpu_temp as f32,
            gpu: None,
        })
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        let cpu_percentage = if self.cpu_fan_rpm > 0 {
            ((self.cpu_fan_rpm as f64 / 5000.0) * 100.0).min(100.0).max(0.0) as u8
        } else {
            0
        };

        let gpu_percentage = if self.gpu_fan_rpm > 0 {
            ((self.gpu_fan_rpm as f64 / 5000.0) * 100.0).min(100.0).max(0.0) as u8
        } else {
            0
        };

        Ok(FanSpeeds {
            cpu_rpm: self.cpu_fan_rpm,
            gpu_rpm: self.gpu_fan_rpm,
            cpu_percentage,
            gpu_percentage,
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        Ok(PowerDraw {
            cpu: 15.0,
            gpu: None,
            total: 15.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_mock_sys_monitor() {
        let monitor = MockSysMonitor::new()
            .with_temp(65.0)
            .with_cpu_fan(3000);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let temps = rt.block_on(monitor.get_temperatures()).unwrap();
        assert_eq!(temps.cpu, 65.0);

        let fans = rt.block_on(monitor.get_fan_speeds()).unwrap();
        assert_eq!(fans.cpu_rpm, 3000);
        assert_eq!(fans.cpu_percentage, 60); // 3000/5000 * 100
    }

    #[test]
    fn test_real_sys_monitor_with_missing_paths() {
        // Use a temp directory that won't have sysfs files
        let temp_dir = TempDir::new().unwrap();
        let monitor = RealSysMonitor::with_paths(
            temp_dir.path().join("thermal"),
            temp_dir.path().join("hwmon"),
            temp_dir.path().join("power_supply"),
        );

        let rt = tokio::runtime::Runtime::new().unwrap();

        // Should return zeros without crashing
        let temps = rt.block_on(monitor.get_temperatures()).unwrap();
        assert_eq!(temps.cpu, 0.0);

        let fans = rt.block_on(monitor.get_fan_speeds()).unwrap();
        assert_eq!(fans.cpu_rpm, 0);
    }
}
