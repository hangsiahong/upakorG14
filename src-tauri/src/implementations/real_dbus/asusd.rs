use async_trait::async_trait;
use zbus::Connection;

use crate::traits::AsusdTrait;
use crate::models::*;
use crate::utils::{Result, UpakorError};

pub struct RealAsusd {
    connection: Connection,
}

impl RealAsusd {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system()
            .await
            .map_err(|e| UpakorError::DbusConnection(e.to_string()))?;

        // TODO: Test actual asusd connection
        // For now, just verify we can connect to system bus
        Ok(Self { connection })
    }
}

#[async_trait]
impl AsusdTrait for RealAsusd {
    async fn get_profile(&self) -> Result<PowerProfile> {
        // TODO: Implement with actual D-Bus calls to xyz.ljones.Asusd
        Ok(PowerProfile::Balanced)
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        // TODO: Implement with actual D-Bus calls
        Ok(ChargeLimit { limit: 100 })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        // TODO: Implement with actual D-Bus calls
        Ok(FanSpeeds {
            cpu_rpm: 0,
            gpu_rpm: 0,
            cpu_percentage: 0,
            gpu_percentage: 0,
        })
    }

    async fn get_fan_curve(&self, _profile: PowerProfile) -> Result<FanCurve> {
        // TODO: Implement with actual D-Bus calls
        Ok(FanCurve {
            cpu_curve: vec![],
            gpu_curve: vec![],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        // TODO: Implement with actual D-Bus calls
        Ok(AuraSettings::default())
    }

    async fn set_aura_settings(&self, _settings: AuraSettings) -> Result<()> {
        // TODO: Implement with actual D-Bus calls
        Ok(())
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        // TODO: Implement with actual D-Bus calls
        Ok(Temperature {
            cpu: 0.0,
            gpu: None,
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        // TODO: Implement with actual D-Bus calls
        Ok(PowerDraw {
            cpu: 0.0,
            gpu: None,
            total: 0.0,
        })
    }

    async fn supports_fan_curves(&self) -> bool {
        // TODO: Query device capabilities
        true
    }

    async fn supports_ani_me(&self) -> bool {
        // TODO: Query device capabilities
        false
    }
}
