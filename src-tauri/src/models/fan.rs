use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurvePoint {
    pub temperature: u8, // Celsius
    pub speed: u8,       // Percentage 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    pub cpu_curve: Vec<FanCurvePoint>,
    pub gpu_curve: Vec<FanCurvePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanSpeeds {
    pub cpu_rpm: u32,
    pub gpu_rpm: u32,
    pub cpu_percentage: u8,
    pub gpu_percentage: u8,
}
