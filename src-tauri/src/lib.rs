mod utils;
pub mod models;
pub mod traits;
pub mod implementations;

// Re-export for main.rs access
pub use implementations::mock;
pub use models::{PowerProfile, HardwareMetrics};

use implementations::{create_asusd, create_supergfxctl};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub asusd: Arc<Mutex<Box<dyn traits::AsusdTrait>>>,
    pub supergfxctl: Arc<Mutex<Box<dyn traits::SupergfxctlTrait>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
