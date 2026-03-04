use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GpuMode {
    Integrated,
    Hybrid,
    Dedicated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStatus {
    pub current_mode: GpuMode,
    pub dedicated_available: bool,
}
