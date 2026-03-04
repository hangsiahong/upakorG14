use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PowerProfile {
    Quiet,
    Balanced,
    Performance,
}

impl Default for PowerProfile {
    fn default() -> Self {
        Self::Balanced
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChargeLimit {
    pub limit: u8, // 0-100, 0 means no limit
}

impl Default for ChargeLimit {
    fn default() -> Self {
        Self { limit: 100 }
    }
}
