use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::{PowerProfile, GpuMode, AuraMode};

/// Application settings that persist across sessions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    pub power: PowerSettings,
    pub gpu: GpuSettings,
    pub fans: FanSettings,
    pub lighting: LightingSettings,
    pub ui: UiSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            power: PowerSettings::default(),
            gpu: GpuSettings::default(),
            fans: FanSettings::default(),
            lighting: LightingSettings::default(),
            ui: UiSettings::default(),
        }
    }
}

/// Power management settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PowerSettings {
    /// Default power profile to apply on startup
    pub default_profile: PowerProfile,
    /// Battery charge limit (50-100)
    pub charge_limit: u8,
    /// Auto-switch profile based on AC power state
    pub auto_switch_profile: bool,
    /// Profile to use when on battery
    pub battery_profile: PowerProfile,
    /// Profile to use when plugged in
    pub ac_profile: PowerProfile,
}

impl Default for PowerSettings {
    fn default() -> Self {
        Self {
            default_profile: PowerProfile::Balanced,
            charge_limit: 100,
            auto_switch_profile: false,
            battery_profile: PowerProfile::Quiet,
            ac_profile: PowerProfile::Performance,
        }
    }
}

/// GPU mode settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpuSettings {
    /// Preferred GPU mode
    pub mode: GpuMode,
    /// Auto-switch to integrated when on battery
    pub auto_switch_on_battery: bool,
}

impl Default for GpuSettings {
    fn default() -> Self {
        Self {
            mode: GpuMode::Hybrid,
            auto_switch_on_battery: false,
        }
    }
}

/// Fan control settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FanSettings {
    /// Enable custom fan curves
    pub enable_custom_curves: bool,
    /// Minimum fan speed percentage (0-100)
    pub min_fan_speed: u8,
    /// Maximum fan speed percentage (0-100)
    pub max_fan_speed: u8,
}

impl Default for FanSettings {
    fn default() -> Self {
        Self {
            enable_custom_curves: false,
            min_fan_speed: 0,
            max_fan_speed: 100,
        }
    }
}

/// RGB lighting settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightingSettings {
    /// Enable RGB lighting control
    pub enable_control: bool,
    /// Default Aura mode
    pub aura_mode: AuraMode,
    /// Brightness level (0-100)
    pub brightness: u8,
}

impl Default for LightingSettings {
    fn default() -> Self {
        Self {
            enable_control: false,
            aura_mode: AuraMode::Static,
            brightness: 100,
        }
    }
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UiSettings {
    /// Theme preference
    pub theme: Theme,
    /// Start minimized to tray
    pub start_minimized: bool,
    /// Show notifications
    pub show_notifications: bool,
    /// Auto-start with system
    pub auto_start: bool,
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            start_minimized: false,
            show_notifications: true,
            auto_start: false,
        }
    }
}

/// UI theme options
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// Configuration file manager
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new ConfigManager with default XDG config path
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::NoConfigDir)?
            .join("upakorG14");

        // Ensure config directory exists
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| ConfigError::CreateDirFailed(config_dir.clone(), e))?;

        let config_path = config_dir.join("config.toml");

        Ok(Self { config_path })
    }

    /// Create a ConfigManager with a custom config path (useful for testing)
    pub fn with_path(path: PathBuf) -> Self {
        Self { config_path: path }
    }

    /// Load settings from config file, or create default if doesn't exist
    pub fn load_settings(&self) -> Result<Settings, ConfigError> {
        if !self.config_path.exists() {
            tracing::info!("Config file not found, creating default: {:?}", self.config_path);
            let default = Settings::default();
            self.save_settings(&default)?;
            return Ok(default);
        }

        let contents = std::fs::read_to_string(&self.config_path)
            .map_err(|e| ConfigError::ReadFailed(self.config_path.clone(), e))?;

        let settings: Settings = toml::from_str(&contents)
            .map_err(|e| ConfigError::ParseFailed(self.config_path.clone(), e))?;

        tracing::info!("Loaded settings from: {:?}", self.config_path);
        Ok(settings)
    }

    /// Save settings to config file
    pub fn save_settings(&self, settings: &Settings) -> Result<(), ConfigError> {
        let toml_string = toml::to_string_pretty(settings)
            .map_err(|e| ConfigError::SerializeFailed(e))?;

        std::fs::write(&self.config_path, toml_string)
            .map_err(|e| ConfigError::WriteFailed(self.config_path.clone(), e))?;

        tracing::info!("Saved settings to: {:?}", self.config_path);
        Ok(())
    }

    /// Get the config file path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to create ConfigManager")
    }
}

/// Errors that can occur during config operations
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("No config directory found")]
    NoConfigDir,

    #[error("Failed to create config directory {:?}: {}", .0, .1)]
    CreateDirFailed(PathBuf, std::io::Error),

    #[error("Failed to read config file {:?}: {}", .0, .1)]
    ReadFailed(PathBuf, std::io::Error),

    #[error("Failed to parse config file {:?}: {}", .0, .1)]
    ParseFailed(PathBuf, toml::de::Error),

    #[error("Failed to serialize settings: {}", .0)]
    SerializeFailed(toml::ser::Error),

    #[error("Failed to write config file {:?}: {}", .0, .1)]
    WriteFailed(PathBuf, std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.power.default_profile, PowerProfile::Balanced);
        assert_eq!(settings.power.charge_limit, 100);
        assert_eq!(settings.gpu.mode, GpuMode::Hybrid);
    }

    #[test]
    fn test_save_and_load_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let manager = ConfigManager::with_path(config_path);

        let mut settings = Settings::default();
        settings.power.charge_limit = 80;
        settings.ui.theme = Theme::Light;

        manager.save_settings(&settings).unwrap();
        let loaded = manager.load_settings().unwrap();

        assert_eq!(loaded, settings);
    }

    #[test]
    fn test_load_creates_default_if_missing() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let manager = ConfigManager::with_path(config_path.clone());

        let settings = manager.load_settings().unwrap();
        assert_eq!(settings, Settings::default());
        assert!(config_path.exists());
    }
}
