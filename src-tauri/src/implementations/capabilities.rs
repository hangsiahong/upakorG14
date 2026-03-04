use async_trait::async_trait;
use std::path::PathBuf;
use crate::utils::Result;
use crate::models::HardwareCapabilities;

/// Trait for detecting hardware capabilities
#[async_trait]
pub trait CapabilityDetector: Send + Sync {
    /// Detect all hardware capabilities
    async fn detect_capabilities(&self) -> Result<HardwareCapabilities>;
}

/// Real capability detector using sysfs and DMI info
pub struct RealCapabilityDetector {
    sysfs_base: PathBuf,
}

impl RealCapabilityDetector {
    /// Create a new detector with default sysfs paths
    pub fn new() -> Self {
        Self {
            sysfs_base: PathBuf::from("/sys"),
        }
    }

    /// Create a detector with custom sysfs base (for testing)
    pub fn with_sysfs_base(sysfs_base: PathBuf) -> Self {
        Self { sysfs_base }
    }

    /// Read DMI product name
    fn read_product_name(&self) -> Option<String> {
        let product_name_path = self.sysfs_base.join("class/dmi/id/product_name");
        std::fs::read_to_string(product_name_path)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Read DMI board name
    fn read_board_name(&self) -> Option<String> {
        let board_name_path = self.sysfs_base.join("class/dmi/id/board_name");
        std::fs::read_to_string(board_name_path)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Read DMI BIOS version
    fn read_bios_version(&self) -> Option<String> {
        let bios_version_path = self.sysfs_base.join("class/dmi/id/bios_version");
        std::fs::read_to_string(bios_version_path)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Detect device series from model name
    fn detect_series(&self, model: &str) -> Option<String> {
        let model_lower = model.to_lowercase();

        if model_lower.contains("zephyrus") {
            Some("ROG Zephyrus".to_string())
        } else if model_lower.contains("strix") {
            Some("ROG Strix".to_string())
        } else if model_lower.contains("tuf") {
            Some("TUF Gaming".to_string())
        } else if model_lower.contains("rog") {
            Some("ROG".to_string())
        } else if model_lower.contains("asus") {
            Some("ASUS".to_string())
        } else {
            None
        }
    }

    /// Check for GPU switching support
    fn detect_gpu_switching(&self) -> (bool, bool) {
        // Check for NVIDIA GPU in sysfs
        let drm_path = self.sysfs_base.join("class/drm");

        if let Ok(entries) = std::fs::read_dir(&drm_path) {
            let entries_vec: Vec<_> = entries.flatten().collect();

            let has_nvidia = entries_vec.iter().any(|entry| {
                entry.path()
                    .to_string_lossy()
                    .contains("card0-NVIDIA")
            });

            let has_amd = entries_vec.iter().any(|entry| {
                let path_str = entry.path().to_string_lossy().to_string();
                path_str.contains("amdgpu")
            });

            return (true, has_nvidia || has_amd);
        }

        (false, false)
    }

    /// Check for dedicated GPU
    fn detect_dedicated_gpu(&self) -> bool {
        // Check for multiple GPUs
        let drm_path = self.sysfs_base.join("class/drm");

        if let Ok(entries) = std::fs::read_dir(&drm_path) {
            let gpu_count = entries
                .flatten()
                .filter(|entry| {
                    entry.path().to_string_lossy().contains("card")
                })
                .count();

            gpu_count > 1
        } else {
            false
        }
    }

    /// Detect ASUS-specific features
    fn detect_asus_features(&self) -> (bool, bool, bool) {
        // Check for ASUS-specific hwmon
        let hwmon_path = self.sysfs_base.join("class/hwmon");

        let mut has_fan_curves = false;
        let mut has_rgb = false;

        if let Ok(entries) = std::fs::read_dir(&hwmon_path) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Check for ASUS-specific fan curve files
                let fan_curve_path = path.join("pwm1_auto_point1_pwm");
                if fan_curve_path.exists() {
                    has_fan_curves = true;
                }

                // Check for ASUS RGB
                let name_file = path.join("name");
                if let Ok(name) = std::fs::read_to_string(&name_file) {
                    if name.contains("asus") {
                        has_rgb = true;
                    }
                }
            }
        }

        // AniMe Matrix is only on specific models
        let anime_matrix = self.read_product_name()
            .map(|name| name.contains("G14") || name.contains("Flow"))
            .unwrap_or(false);

        (has_fan_curves, has_rgb, anime_matrix)
    }
}

impl Default for RealCapabilityDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CapabilityDetector for RealCapabilityDetector {
    async fn detect_capabilities(&self) -> Result<HardwareCapabilities> {
        let mut capabilities = HardwareCapabilities::default();

        // Read device information
        capabilities.device_info.model = self.read_product_name();
        capabilities.device_info.board = self.read_board_name();
        capabilities.device_info.bios_version = self.read_bios_version();

        // Detect series from model
        if let Some(ref model) = capabilities.device_info.model {
            capabilities.device_info.series = self.detect_series(model);
        }

        // Detect GPU capabilities
        let (gpu_switching, dedicated) = self.detect_gpu_switching();
        capabilities.gpu_switching = gpu_switching;
        capabilities.dedicated_gpu = dedicated || self.detect_dedicated_gpu();

        // Detect ASUS-specific features
        let (fan_curves, rgb, anime_matrix) = self.detect_asus_features();
        capabilities.fan_curves = fan_curves;
        capabilities.rgb_lighting = rgb;
        capabilities.anime_matrix = anime_matrix;

        // Charge limit and performance profiles are generally available
        capabilities.charge_limit = true;
        capabilities.performance_profiles = true;

        tracing::info!(
            model = ?capabilities.device_info.model,
            series = ?capabilities.device_info.series,
            gpu_switching = capabilities.gpu_switching,
            fan_curves = capabilities.fan_curves,
            "Detected hardware capabilities"
        );

        Ok(capabilities)
    }
}

/// Mock capability detector for testing
#[derive(Clone)]
pub struct MockCapabilityDetector {
    capabilities: HardwareCapabilities,
}

impl MockCapabilityDetector {
    /// Create a mock detector with specific capabilities
    pub fn with_capabilities(capabilities: HardwareCapabilities) -> Self {
        Self { capabilities }
    }

    /// Create a mock detector simulating a ROG Zephyrus G14
    pub fn rog_zephyrus_g14() -> Self {
        use crate::models::DeviceInfo;

        let mut device_info = DeviceInfo::default();
        device_info.model = Some("ROG Zephyrus G14 GA402".to_string());
        device_info.manufacturer = Some("ASUSTeK Computer".to_string());
        device_info.series = Some("ROG Zephyrus".to_string());

        let capabilities = HardwareCapabilities {
            fan_curves: true,
            anime_matrix: true,
            rgb_lighting: true,
            gpu_switching: true,
            charge_limit: true,
            performance_profiles: true,
            dedicated_gpu: true,
            device_info,
        };

        Self { capabilities }
    }

    /// Create a mock detector simulating a generic laptop
    pub fn generic_laptop() -> Self {
        let capabilities = HardwareCapabilities::default();
        Self { capabilities }
    }
}

impl Default for MockCapabilityDetector {
    fn default() -> Self {
        Self::generic_laptop()
    }
}

#[async_trait]
impl CapabilityDetector for MockCapabilityDetector {
    async fn detect_capabilities(&self) -> Result<HardwareCapabilities> {
        Ok(self.capabilities.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_real_capability_detector() {
        let detector = RealCapabilityDetector::new();
        let capabilities = detector.detect_capabilities().await.unwrap();

        // Should always have basic capabilities
        assert!(capabilities.charge_limit);
        assert!(capabilities.performance_profiles);
    }

    #[tokio::test]
    async fn test_mock_capability_detector() {
        let detector = MockCapabilityDetector::rog_zephyrus_g14();
        let capabilities = detector.detect_capabilities().await.unwrap();

        assert!(capabilities.fan_curves);
        assert!(capabilities.anime_matrix);
        assert!(capabilities.gpu_switching);
        assert!(capabilities.is_asus_device());
    }

    #[tokio::test]
    async fn test_mock_generic_laptop() {
        let detector = MockCapabilityDetector::generic_laptop();
        let capabilities = detector.detect_capabilities().await.unwrap();

        assert!(!capabilities.fan_curves);
        assert!(!capabilities.is_asus_device());
    }
}
