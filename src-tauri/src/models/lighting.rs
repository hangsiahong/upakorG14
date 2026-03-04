use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuraMode {
    Static,
    Breathing,
    Strobe,
    Rainbow,
    Star,
    Rain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuraSettings {
    pub mode: AuraMode,
    pub brightness: u8, // 0-100
}

impl Default for AuraSettings {
    fn default() -> Self {
        Self {
            mode: AuraMode::Static,
            brightness: 100,
        }
    }
}
