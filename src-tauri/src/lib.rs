mod utils;
pub mod models;
pub mod traits;
pub mod implementations;
pub mod tray;
pub mod autostart;

// Re-export for main.rs access
pub use implementations::mock;
pub use models::{PowerProfile, HardwareMetrics, Settings, ConfigManager};
pub use autostart::AutoStartManager;

use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub asusd: Arc<Mutex<Box<dyn traits::AsusdTrait>>>,
    pub supergfxctl: Arc<Mutex<Box<dyn traits::SupergfxctlTrait>>>,
    pub config: Arc<Mutex<ConfigManager>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // TODO: Add system tray initialization when Tauri v2 API is clear
        // .plugin(tauri_plugin_system_tray::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
