// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use upakorg14_lib::*;
use upakorg14_lib::models::*;

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create runtime
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

    // Create implementations
    #[cfg(feature = "mock")]
    let (asusd, supergfxctl) = {
        use upakorg14_lib::mock::{MockAsusd, MockSupergfxctl};
        use upakorg14_lib::traits::{AsusdTrait, SupergfxctlTrait};
        (
            Box::new(MockAsusd::new()) as Box<dyn AsusdTrait>,
            Box::new(MockSupergfxctl::new()) as Box<dyn SupergfxctlTrait>,
        )
    };

    #[cfg(not(feature = "mock"))]
    let (asusd, supergfxctl) = rt.block_on(async {
        use upakorg14_lib::implementations::real_dbus::{RealAsusd, RealSupergfxctl};
        use upakorg14_lib::traits::{AsusdTrait, SupergfxctlTrait};
        let asusd = RealAsusd::new().await.expect("Failed to connect to asusd");
        let supergfxctl = RealSupergfxctl::new().await.expect("Failed to connect to supergfxd");
        (
            Box::new(asusd) as Box<dyn AsusdTrait>,
            Box::new(supergfxctl) as Box<dyn SupergfxctlTrait>,
        )
    });

    // Initialize config manager
    let config = ConfigManager::new()
        .expect("Failed to initialize config manager");

    #[tauri::command]
    async fn get_power_profile(state: State<'_, upakorg14_lib::AppState>) -> Result<PowerProfile, String> {
        let asusd = state.asusd.lock().await;
        asusd.get_profile().await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn set_power_profile(state: State<'_, upakorg14_lib::AppState>, profile: String) -> Result<(), String> {
        let asusd = state.asusd.lock().await;
        let profile = match profile.as_str() {
            "quiet" => PowerProfile::Quiet,
            "balanced" => PowerProfile::Balanced,
            "performance" => PowerProfile::Performance,
            _ => return Err("Invalid profile name".to_string()),
        };
        asusd.set_profile(profile).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn get_charge_limit(state: State<'_, upakorg14_lib::AppState>) -> Result<ChargeLimit, String> {
        let asusd = state.asusd.lock().await;
        asusd.get_charge_limit().await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn set_charge_limit(state: State<'_, upakorg14_lib::AppState>, limit: u8) -> Result<(), String> {
        let asusd = state.asusd.lock().await;
        asusd.set_charge_limit(limit).await.map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn get_hardware_metrics(state: State<'_, upakorg14_lib::AppState>) -> Result<HardwareMetrics, String> {
        let asusd = state.asusd.lock().await;

        let temps = asusd.get_temperatures().await.map_err(|e| e.to_string())?;
        let fans = asusd.get_fan_speeds().await.map_err(|e| e.to_string())?;
        let power = asusd.get_power_draw().await.map_err(|e| e.to_string())?;

        Ok(HardwareMetrics {
            temperatures: temps,
            fan_speeds: fans,
            power_draw: power,
        })
    }

    #[tauri::command]
    async fn get_settings(state: State<'_, upakorg14_lib::AppState>) -> Result<Settings, String> {
        let config = state.config.lock().await;
        config.load_settings().map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn save_settings(state: State<'_, upakorg14_lib::AppState>, settings: Settings) -> Result<(), String> {
        let config = state.config.lock().await;
        config.save_settings(&settings).map_err(|e| e.to_string())
    }

    #[tauri::command]
    async fn reset_settings(state: State<'_, upakorg14_lib::AppState>) -> Result<Settings, String> {
        let config = state.config.lock().await;
        let default = Settings::default();
        config.save_settings(&default).map_err(|e| e.to_string())?;
        Ok(default)
    }

    #[tauri::command]
    async fn get_config_path(state: State<'_, upakorg14_lib::AppState>) -> Result<String, String> {
        let config = state.config.lock().await;
        Ok(config.config_path().to_string_lossy().to_string())
    }

    tauri::Builder::default()
        .manage(upakorg14_lib::AppState {
            asusd: std::sync::Arc::new(tokio::sync::Mutex::new(asusd)),
            supergfxctl: std::sync::Arc::new(tokio::sync::Mutex::new(supergfxctl)),
            config: std::sync::Arc::new(tokio::sync::Mutex::new(config)),
        })
        .invoke_handler(tauri::generate_handler![
            get_power_profile,
            set_power_profile,
            get_charge_limit,
            set_charge_limit,
            get_hardware_metrics,
            get_settings,
            save_settings,
            reset_settings,
            get_config_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
