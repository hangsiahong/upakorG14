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
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

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

        // TODO: Implement actual D-Bus call when service API is confirmed
        // For now, return default
        Ok(PowerProfile::Balanced)
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        if !self.is_service_available() {
            return Err(UpakorError::ServiceUnavailable(
                "asusd D-Bus service not available".to_string()
            ));
        }

        let profile_str = match profile {
            PowerProfile::Quiet => "quiet",
            PowerProfile::Balanced => "balanced",
            PowerProfile::Performance => "performance",
        };

        tracing::info!("Setting power profile to: {}", profile_str);
        // TODO: Implement actual D-Bus property set
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        if !self.is_service_available() {
            return Ok(ChargeLimit { limit: 100 });
        }

        // TODO: Implement actual D-Bus call
        // For now, check if we can read from /sys/class/power_supply/BAT0/charge_limit
        Ok(ChargeLimit { limit: 100 })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        if limit < 50 || limit > 100 {
            return Err(UpakorError::InvalidValue("Charge limit must be between 50 and 100".to_string()));
        }

        if !self.is_service_available() {
            return Err(UpakorError::ServiceUnavailable(
                "asusd D-Bus service not available".to_string()
            ));
        }

        tracing::info!("Setting charge limit to: {}", limit);
        // TODO: Implement actual D-Bus call
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        // TODO: Implement D-Bus call or fallback to /sys/class/hwmon
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

        // TODO: Implement actual D-Bus call
        Ok(FanCurve {
            cpu_curve: vec![],
            gpu_curve: vec![],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        // Fan curve setting requires specific hardware support
        // TODO: Implement when D-Bus interface is available
        Err(UpakorError::NotSupported)
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        // TODO: Implement LED settings query
        Ok(AuraSettings::default())
    }

    async fn set_aura_settings(&self, _settings: AuraSettings) -> Result<()> {
        // TODO: Implement LED settings
        Err(UpakorError::NotSupported)
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        // TODO: Implement fallback to /sys/class/thermal
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
        // TODO: Query device capabilities via D-Bus or /sys
        false
    }

    async fn supports_ani_me(&self) -> bool {
        // Check if the device has AniMe Matrix display
        // TODO: Query device capabilities via D-Bus or /sys
        false
    }
}
