use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::traits::AsusdTrait;
use crate::models::*;
use crate::utils::Result;

#[derive(Clone)]
pub struct MockAsusd {
    state: Arc<RwLock<MockState>>,
}

struct MockState {
    profile: PowerProfile,
    charge_limit: u8,
    aura_settings: AuraSettings,
}

impl Default for MockState {
    fn default() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            charge_limit: 100,
            aura_settings: AuraSettings::default(),
        }
    }
}

impl MockAsusd {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(MockState::default())),
        }
    }
}

#[async_trait]
impl AsusdTrait for MockAsusd {
    async fn get_profile(&self) -> Result<PowerProfile> {
        let state = self.state.read().await;
        Ok(state.profile)
    }

    async fn set_profile(&self, profile: PowerProfile) -> Result<()> {
        let mut state = self.state.write().await;
        state.profile = profile;
        Ok(())
    }

    async fn get_charge_limit(&self) -> Result<ChargeLimit> {
        let state = self.state.read().await;
        Ok(ChargeLimit { limit: state.charge_limit })
    }

    async fn set_charge_limit(&self, limit: u8) -> Result<()> {
        if limit > 100 {
            return Err(crate::utils::UpakorError::InvalidValue(
                "Charge limit must be 0-100".to_string(),
            ));
        }
        let mut state = self.state.write().await;
        state.charge_limit = limit;
        Ok(())
    }

    async fn get_fan_speeds(&self) -> Result<FanSpeeds> {
        Ok(FanSpeeds {
            cpu_rpm: 2500,
            gpu_rpm: 0,
            cpu_percentage: 45,
            gpu_percentage: 0,
        })
    }

    async fn get_fan_curve(&self, _profile: PowerProfile) -> Result<FanCurve> {
        Ok(FanCurve {
            cpu_curve: vec![
                FanCurvePoint { temperature: 50, speed: 30 },
                FanCurvePoint { temperature: 70, speed: 50 },
                FanCurvePoint { temperature: 85, speed: 100 },
            ],
            gpu_curve: vec![
                FanCurvePoint { temperature: 55, speed: 30 },
                FanCurvePoint { temperature: 75, speed: 60 },
                FanCurvePoint { temperature: 90, speed: 100 },
            ],
        })
    }

    async fn set_fan_curve(&self, _profile: PowerProfile, _curve: FanCurve) -> Result<()> {
        Ok(())
    }

    async fn get_aura_settings(&self) -> Result<AuraSettings> {
        let state = self.state.read().await;
        Ok(state.aura_settings.clone())
    }

    async fn set_aura_settings(&self, settings: AuraSettings) -> Result<()> {
        let mut state = self.state.write().await;
        state.aura_settings = settings;
        Ok(())
    }

    async fn get_temperatures(&self) -> Result<Temperature> {
        Ok(Temperature {
            cpu: 55.0,
            gpu: Some(45.0),
        })
    }

    async fn get_power_draw(&self) -> Result<PowerDraw> {
        Ok(PowerDraw {
            cpu: 25.0,
            gpu: Some(15.0),
            total: 40.0,
        })
    }

    async fn supports_fan_curves(&self) -> bool {
        true
    }

    async fn supports_ani_me(&self) -> bool {
        true
    }
}
