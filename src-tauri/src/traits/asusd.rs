use async_trait::async_trait;

use crate::models::*;
use crate::utils::Result;

#[async_trait]
pub trait AsusdTrait: Send + Sync {
    // Power profiles
    async fn get_profile(&self) -> Result<PowerProfile>;
    async fn set_profile(&self, profile: PowerProfile) -> Result<()>;

    // Charge limit
    async fn get_charge_limit(&self) -> Result<ChargeLimit>;
    async fn set_charge_limit(&self, limit: u8) -> Result<()>;

    // Fan speeds
    async fn get_fan_speeds(&self) -> Result<FanSpeeds>;

    // Fan curves
    async fn get_fan_curve(&self, profile: PowerProfile) -> Result<FanCurve>;
    async fn set_fan_curve(&self, profile: PowerProfile, curve: FanCurve) -> Result<()>;

    // Lighting
    async fn get_aura_settings(&self) -> Result<AuraSettings>;
    async fn set_aura_settings(&self, settings: AuraSettings) -> Result<()>;

    // Hardware monitoring
    async fn get_temperatures(&self) -> Result<Temperature>;
    async fn get_power_draw(&self) -> Result<PowerDraw>;

    // Check feature support
    async fn supports_fan_curves(&self) -> bool;
    async fn supports_ani_me(&self) -> bool;
}
