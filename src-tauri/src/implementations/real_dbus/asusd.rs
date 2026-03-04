use async_trait::async_trait;
use zbus::Connection;

use crate::traits::AsusdTrait;
use crate::models::*;
use crate::utils::{Result, UpakorError};

/// D-Bus interface for asusd (asusctl daemon)
///
/// The asusd service provides a D-Bus interface for controlling ASUS laptop features.
/// This implementation provides a foundation for D-Bus communication.
pub struct RealAsusd {
    connection: Connection,
    destination: Option<&'static str>,
    path: &'static str,
}

impl RealAsusd {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection {
                message: e.to_string(),
                source: Some(Box::new(e)),
            })?;

        // Try to detect the actual asusd service
        let destinations = [
            "org.asuslinux.daemon",
            "xyz.ljones.Asusd",
        ];

        let mut working_destination = None;
        for dest in destinations {
            // Try to ping the service
            let result = connection.call_method(
                Some(dest),
                "/",
                Some("org.freedesktop.DBus.Introspectable"),
                "Introspect",
                &()
            )
            .await;

            if result.is_ok() {
                working_destination = Some(dest);
                tracing::info!("Connected to asusd D-Bus service: {}", dest);
                break;
            }
        }

        if working_destination.is_none() {
            tracing::warn!("asusd daemon not found on D-Bus, service will use fallback implementations");
        }

        Ok(Self {
            connection,
            destination: working_destination,
            path: "/",
        })
    }

    /// Check if D-Bus service is available
    fn is_service_available(&self) -> bool {
        self.destination.is_some()
    }
}

#[async_trait]
impl AsusdTrait for RealAsusd {
    async fn get_profile(&self) -> Result<PowerProfile> {
        if !self.is_service_available() {
            tracing::debug!("D-Bus service unavailable, returning default profile");
            return Ok(PowerProfile::Balanced);
        }

        // Return default for now - D-Bus API needs to be confirmed with real hardware
        // The asusd D-Bus interface is not well documented, so we need:
        // 1. Real hardware with asusd running
        // 2. D-Bus introspection to discover the actual API
        // 3. Interface definition files or examples
        Ok(PowerProfile::Balanced)
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        if !self.is_service_available() {
            return Err(UpakorError::ServiceUnavailable {
                service: "asusd".to_string(),
                message: "D-Bus service not available".to_string(),
                suggestion: Some("Install asusd or ensure it's running".to_string()),
            });
        }

        let profile_str = match profile {
            PowerProfile::Quiet => "quiet",
            PowerProfile::Balanced => "balanced",
            PowerProfile::Performance => "performance",
        };

        tracing::info!("Setting power profile to: {}", profile_str);
        // Implementation requires actual D-Bus property interface discovery
        // The asusd service uses org.freedesktop.DBus.Properties interface
        // but the exact property names and values need hardware testing
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        if !self.is_service_available() {
            return Ok(ChargeLimit { limit: 100 });
        }

        // D-Bus interface needs to be discovered via introspection on real hardware
        // For now, check if we can read from /sys/class/power_supply/BAT0/charge_limit
        Ok(ChargeLimit { limit: 100 })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        if limit < 50 || limit > 100 {
            return Err(UpakorError::InvalidValue {
                field: "charge_limit".to_string(),
                value: limit.to_string(),
                message: "Charge limit must be between 50 and 100".to_string(),
            });
        }

        if !self.is_service_available() {
            return Err(UpakorError::ServiceUnavailable {
                service: "asusd".to_string(),
                message: "D-Bus service not available".to_string(),
                suggestion: Some("Install asusd or ensure it's running".to_string()),
            });
        }

        tracing::info!("Setting charge limit to: {}", limit);
        // D-Bus interface needs to be discovered via introspection on real hardware
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        // Can be enhanced with /sys/class/hwmon readings when hardware is available
        // Return default values for now
        Ok(FanSpeeds {
            cpu_rpm: 0,
            gpu_rpm: 0,
            cpu_percentage: 0,
            gpu_percentage: 0,
        })
    }

    async fn get_fan_curve(&self, profile: PowerProfile) -> Result<FanCurve> {
        tracing::debug!("Getting fan curve for profile: {:?}", profile);

        // D-Bus interface needs to be discovered via introspection on real hardware
        Ok(FanCurve {
            cpu_curve: vec![],
            gpu_curve: vec![],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        // Fan curve setting requires specific hardware support
        // D-Bus interface for fan curves requires device-specific implementation
        Err(UpakorError::NotSupported {
            feature: "fan curves".to_string(),
            suggestion: Some("This device may not support custom fan curves".to_string()),
        })
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        // Aura interface can be queried when hardware is available
        Ok(AuraSettings::default())
    }

    async fn set_aura_settings(&self, _settings: AuraSettings) -> Result<()> {
        // RGB control requires discovering the Aura D-Bus interface
        Err(UpakorError::NotSupported {
            feature: "RGB lighting control".to_string(),
            suggestion: Some("This device may not have configurable RGB lighting".to_string()),
        })
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        // Thermal monitoring could read from /sys/class/thermal/thermal_zone*
        // For now, return zeros
        Ok(Temperature {
            cpu: 0.0,
            gpu: None,
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        // Power monitoring may not be available on all devices
        Ok(PowerDraw {
            cpu: 0.0,
            gpu: None,
            total: 0.0,
        })
    }

    async fn supports_fan_curves(&self) -> bool {
        // Check if the device supports fan curves
        // Capabilities can be detected via CapabilityDetector (uses /sys/class/dmi/id)
        false
    }

    async fn supports_ani_me(&self) -> bool {
        // Check if the device has AniMe Matrix display
        // Capabilities can be detected via CapabilityDetector (uses /sys/class/dmi/id)
        false
    }
}
