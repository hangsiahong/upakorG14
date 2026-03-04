use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::{Result, UpakorError};

/// Manager for application auto-start functionality
pub struct AutoStartManager {
    autostart_dir: PathBuf,
    desktop_file_path: PathBuf,
    app_exec_path: PathBuf,
}

impl AutoStartManager {
    /// Create a new AutoStartManager
    pub fn new() -> Result<Self> {
        // Get XDG config directory
        let config_dir = dirs::config_dir()
            .ok_or(UpakorError::Unknown {
                message: "Could not find XDG config directory".to_string(),
                source: None,
            })?;

        let autostart_dir = config_dir.join("autostart");
        let desktop_file_path = autostart_dir.join("upakor-g14.desktop");

        // Get the current executable path
        let app_exec_path = std::env::current_exe()
            .map_err(|e| UpakorError::Unknown {
                message: format!("Could not get executable path: {}", e),
                source: None,
            })?;

        Ok(Self {
            autostart_dir,
            desktop_file_path,
            app_exec_path,
        })
    }

    /// Create an AutoStartManager with custom paths (useful for testing)
    pub fn with_paths(autostart_dir: PathBuf, app_exec_path: PathBuf) -> Self {
        let desktop_file_path = autostart_dir.join("upakor-g14.desktop");
        Self {
            autostart_dir,
            desktop_file_path,
            app_exec_path,
        }
    }

    /// Check if auto-start is enabled
    pub fn is_enabled(&self) -> bool {
        self.desktop_file_path.exists()
    }

    /// Enable auto-start
    pub fn enable(&self) -> Result<()> {
        // Ensure autostart directory exists
        if !self.autostart_dir.exists() {
            fs::create_dir_all(&self.autostart_dir)
                .map_err(|e| UpakorError::Unknown {
                    message: format!("Failed to create autostart directory: {}", e),
                    source: None,
                })?;
        }

        // Create .desktop file content
        let desktop_content = format!(
            "[Desktop Entry]\n\
             Type=Application\n\
             Name=Upakor-G14\n\
             Comment=ASUS G14 laptop control and monitoring\n\
             Exec={}\n\
             Icon=upakor-g14\n\
             Terminal=false\n\
             Categories=System;Settings;\n\
             X-GNOME-Autostart-enabled=true\n\
             X-KDE-autostart-after=panel\n",
            self.app_exec_path.display()
        );

        // Write the .desktop file
        fs::write(&self.desktop_file_path, desktop_content)
            .map_err(|e| UpakorError::Unknown {
                message: format!("Failed to write autostart file: {}", e),
                source: None,
            })?;

        tracing::info!("Auto-start enabled: {:?}", self.desktop_file_path);
        Ok(())
    }

    /// Disable auto-start
    pub fn disable(&self) -> Result<()> {
        if self.desktop_file_path.exists() {
            fs::remove_file(&self.desktop_file_path)
                .map_err(|e| UpakorError::Unknown {
                    message: format!("Failed to remove autostart file: {}", e),
                    source: None,
                })?;

            tracing::info!("Auto-start disabled: {:?}", self.desktop_file_path);
        }

        Ok(())
    }

    /// Get the path to the autostart desktop file
    pub fn desktop_file_path(&self) -> &Path {
        &self.desktop_file_path
    }
}

impl Default for AutoStartManager {
    fn default() -> Self {
        Self::new().expect("Failed to create AutoStartManager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_autostart_enable_disable() {
        let temp_dir = TempDir::new().unwrap();
        let autostart_dir = temp_dir.path().join("autostart");
        let exec_path = PathBuf::from("/usr/bin/upakor-g14");

        let manager = AutoStartManager::with_paths(autostart_dir.clone(), exec_path);

        // Initially disabled
        assert!(!manager.is_enabled());

        // Enable auto-start
        manager.enable().unwrap();
        assert!(manager.is_enabled());
        assert!(manager.desktop_file_path().exists());

        // Check .desktop file content
        let content = fs::read_to_string(manager.desktop_file_path()).unwrap();
        assert!(content.contains("Upakor-G14"));
        assert!(content.contains("/usr/bin/upakor-g14"));

        // Disable auto-start
        manager.disable().unwrap();
        assert!(!manager.is_enabled());
        assert!(!manager.desktop_file_path().exists());
    }

    #[test]
    fn test_autostart_enable_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let autostart_dir = temp_dir.path().join("nested/autostart");
        let exec_path = PathBuf::from("/usr/bin/upakor-g14");

        let manager = AutoStartManager::with_paths(autostart_dir, exec_path);

        // Enable should create parent directories
        manager.enable().unwrap();
        assert!(manager.desktop_file_path().exists());
    }
}
