use tauri::{AppHandle, Manager};

/// Create the system tray with menu items
/// NOTE: Tauri v2 tray API is different from v1
/// This is a placeholder implementation that will be enhanced
/// once we have proper tray icons and understand the v2 API better
pub fn create_system_tray() {
    // Tray initialization happens through tauri.conf.json in Tauri v2
    // We'll handle tray events through the Builder
    tracing::info!("System tray placeholder created");
}

/// Handle system tray events
pub fn handle_tray_event(app: &AppHandle, event: &str) {
    match event {
        "click" => {
            // Toggle window visibility on click
            if let Some(window) = app.get_webview_window("main") {
                if window.is_visible().unwrap() {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
        "double_click" => {
            // Show window and focus on double click
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {
            tracing::warn!("Unknown tray event: {}", event);
        }
    }
}

/// Update the system tray tooltip with current status
pub fn update_tray_tooltip(_app: &AppHandle, cpu_temp: f32, battery_percent: u8, profile: &str) {
    let tooltip = format!(
        "Upakor G14\nCPU: {:.1}°C\nBattery: {}%\nProfile: {}",
        cpu_temp, battery_percent, profile
    );

    tracing::info!("Tray tooltip would be updated to: {}", tooltip);
    // TODO: Implement actual tooltip update when Tauri v2 API is clear
}


