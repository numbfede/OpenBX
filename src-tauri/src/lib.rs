mod backup;
mod commands;
mod detect;
mod engine;
mod error;
mod games;
mod model;
mod paths;
mod registry;
mod settings;
mod tweaks;

use tauri::Manager;

use commands::{
    apply_game_mode_cmd, export_logs, get_settings, get_system_info, list_backups_cmd, open_external_url,
    optimize_game_cmd, optimize_system, relaunch_elevated, restore_backup, save_settings_cmd, scan_games_cmd,
    scan_system,
};
use settings::load_settings;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            crate::paths::append_log("OpenBX started");
            let handle = app.handle().clone();
            let _ = tauri::tray::TrayIconBuilder::new()
                .tooltip("OpenBX")
                .on_tray_icon_event(move |_tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        if let Some(window) = handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app);
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if load_settings().minimize_to_tray {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_system_info,
            get_settings,
            save_settings_cmd,
            scan_system,
            optimize_system,
            restore_backup,
            list_backups_cmd,
            apply_game_mode_cmd,
            scan_games_cmd,
            optimize_game_cmd,
            relaunch_elevated,
            export_logs,
            open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("OpenBX failed to start");
}
