use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Hardware capabilities detected on the system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HardwareCapabilities {
    /// Fan curve customization support
    pub fan_curves: bool,

    /// AniMe Matrix display support (LED matrix on lid)
    pub anime_matrix: bool,

    /// RGB lighting control support (Aura SYNC)
    pub rgb_lighting: bool,

    /// GPU mode switching support (Integrated/Hybrid/Dedicated)
    pub gpu_switching: bool,

    /// Battery charge limit support
    pub charge_limit: bool,

    /// Performance profile switching support
    pub performance_profiles: bool,

    /// Dedicated GPU availability
    pub dedicated_gpu: bool,

    /// Additional device-specific information
    pub device_info: DeviceInfo,
}

impl Default for HardwareCapabilities {
    fn default() -> Self {
        Self {
            fan_curves: false,
            anime_matrix: false,
            rgb_lighting: false,
            gpu_switching: false,
            charge_limit: true, // Most laptops support this
            performance_profiles: true,
            dedicated_gpu: false,
            device_info: DeviceInfo::default(),
        }
    }
}

/// Device-specific information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    /// Device/model name
    pub model: Option<String>,

    /// Manufacturer
    pub manufacturer: Option<String>,

    /// Board name
    pub board: Option<String>,

    /// BIOS version
    pub bios_version: Option<String>,

    /// Detected device series (e.g., "ROG Zephyrus", "ROG Strix")
    pub series: Option<String>,

    /// Additional capabilities not in the main struct
    pub extra: HashMap<String, String>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            model: None,
            manufacturer: None,
            board: None,
            bios_version: None,
            series: None,
            extra: HashMap::new(),
        }
    }
}

impl HardwareCapabilities {
    /// Get a summary of available features
    pub fn feature_summary(&self) -> Vec<String> {
        let mut features = Vec::new();

        if self.performance_profiles {
            features.push("Performance Profiles".to_string());
        }
        if self.fan_curves {
            features.push("Custom Fan Curves".to_string());
        }
        if self.gpu_switching {
            features.push("GPU Mode Switching".to_string());
        }
        if self.dedicated_gpu {
            features.push("Dedicated GPU".to_string());
        }
        if self.charge_limit {
            features.push("Battery Charge Limit".to_string());
        }
        if self.rgb_lighting {
            features.push("RGB Lighting Control".to_string());
        }
        if self.anime_matrix {
            features.push("AniMe Matrix Display".to_string());
        }

        features
    }

    /// Check if this is an ASUS device
    pub fn is_asus_device(&self) -> bool {
        self.device_info
            .manufacturer
            .as_ref()
            .map(|m| m.contains("ASUS"))
            .unwrap_or(false)
            || self.device_info
                .series
            .as_ref()
            .map(|s| s.contains("ROG") || s.contains("TUF"))
            .unwrap_or(false)
    }

    /// Get the supported GPU modes
    pub fn supported_gpu_modes(&self) -> Vec<&'static str> {
        if !self.gpu_switching {
            return vec!["integrated"];
        }

        let mut modes = vec!["integrated", "hybrid"];
        if self.dedicated_gpu {
            modes.push("dedicated");
        }
        modes
    }

    /// Create a human-readable description
    pub fn description(&self) -> String {
        let mut parts = vec![];

        if let Some(model) = &self.device_info.model {
            parts.push(format!("Model: {}", model));
        } else if let Some(series) = &self.device_info.series {
            parts.push(format!("Series: {}", series));
        }

        parts.push(format!("Features: {}", self.feature_summary().join(", ")));

        if self.is_asus_device() {
            parts.push("ASUS laptop with full support".to_string());
        } else {
            parts.push("Generic laptop (sysfs fallback mode)".to_string());
        }

        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let caps = HardwareCapabilities::default();
        assert_eq!(caps.fan_curves, false);
        assert_eq!(caps.charge_limit, true);
        assert_eq!(caps.performance_profiles, true);
    }

    #[test]
    fn test_feature_summary() {
        let mut caps = HardwareCapabilities::default();
        caps.fan_curves = true;
        caps.gpu_switching = true;

        let summary = caps.feature_summary();
        assert!(summary.contains(&"Custom Fan Curves".to_string()));
        assert!(summary.contains(&"GPU Mode Switching".to_string()));
    }

    #[test]
    fn test_is_asus_device() {
        let mut caps = HardwareCapabilities::default();
        assert!(!caps.is_asus_device());

        caps.device_info.manufacturer = Some("ASUSTeK Computer".to_string());
        assert!(caps.is_asus_device());
    }

    #[test]
    fn test_supported_gpu_modes() {
        let mut caps = HardwareCapabilities::default();
        caps.gpu_switching = false;

        assert_eq!(caps.supported_gpu_modes(), vec!["integrated"]);

        caps.gpu_switching = true;
        caps.dedicated_gpu = true;

        let modes = caps.supported_gpu_modes();
        assert!(modes.contains(&"integrated"));
        assert!(modes.contains(&"hybrid"));
        assert!(modes.contains(&"dedicated"));
    }
}
