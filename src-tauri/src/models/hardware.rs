use serde::{Deserialize, Serialize};
use crate::models::fan::FanSpeeds;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Temperature {
    pub cpu: f32,
    pub gpu: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerDraw {
    pub cpu: f32, // Watts
    pub gpu: Option<f32>,
    pub total: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareMetrics {
    pub temperatures: Temperature,
    pub fan_speeds: FanSpeeds,
    pub power_draw: PowerDraw,
}
