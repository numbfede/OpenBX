use tauri::{AppHandle, Manager};

use crate::backup::list_backups;
use crate::detect::collect_system_info;
use crate::elevate;
use crate::engine::{apply_game_mode, optimize, restore, scan};
use crate::error::AppResult;
use crate::games::{optimize_game, scan_games};
use crate::model::{
    AppSettings, BackupKind, BackupSummary, GameEntry, OptimizeResult, ScanResult, SystemInfo,
};
use crate::paths::{append_log, data_dir, log_path};
use crate::settings::{load_settings, save_settings};

#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    collect_system_info()
}

#[tauri::command]
pub fn get_settings() -> AppSettings {
    load_settings()
}

#[tauri::command]
pub fn save_settings_cmd(settings: AppSettings) -> AppResult<AppSettings> {
    save_settings(&settings)
}

#[tauri::command]
pub fn scan_system() -> ScanResult {
    scan()
}

#[tauri::command]
pub fn optimize_system(app: AppHandle, tweak_ids: Option<Vec<String>>) -> AppResult<OptimizeResult> {
    optimize(&app, tweak_ids, BackupKind::Optimize, "Optimize")
}

#[tauri::command]
pub fn restore_backup(app: AppHandle, backup_id: String) -> AppResult<OptimizeResult> {
    restore(&app, &backup_id)
}

#[tauri::command]
pub fn list_backups_cmd() -> AppResult<Vec<BackupSummary>> {
    list_backups()
}

#[tauri::command]
pub fn apply_game_mode_cmd(app: AppHandle, preset: String) -> AppResult<OptimizeResult> {
    apply_game_mode(&app, &preset)
}

#[tauri::command]
pub fn scan_games_cmd() -> Vec<GameEntry> {
    scan_games()
}

#[tauri::command]
pub fn optimize_game_cmd(app: AppHandle, game_id: String) -> AppResult<OptimizeResult> {
    optimize_game(&app, &game_id)
}

#[tauri::command]
pub fn relaunch_elevated(app: AppHandle) -> AppResult<()> {
    if cfg!(debug_assertions) {
        return Err(crate::error::AppError::Message(
            "In sviluppo Windows blocca localhost dopo l'autorizzazione. Continua senza questo passo, oppure chiudi e riapri il terminale come amministratore. Con l'app installata da GitHub Releases, Autorizza funziona."
                .into(),
        ));
    }

    elevate::relaunch_self()?;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    crate::paths::append_log("relaunch elevated requested");
    std::process::exit(0);
}

#[tauri::command]
pub fn export_logs() -> AppResult<String> {
    let source = log_path()?;
    if !source.exists() {
        append_log("log created on export");
    }
    let dest = data_dir()?.join(format!(
        "openbx-log-{}.log",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    ));
    std::fs::copy(&source, &dest)?;
    Ok(dest.display().to_string())
}

#[tauri::command]
pub fn check_for_updates() -> crate::update::UpdateInfo {
    crate::update::check()
}

#[tauri::command]
pub fn open_external_url(url: String) -> AppResult<()> {
    let allowed = url.starts_with("https://learn.microsoft.com")
        || url.starts_with("https://support.microsoft.com")
        || url.starts_with("https://support.xbox.com")
        || url.starts_with("https://devblogs.microsoft.com")
        || url.starts_with("https://www.nvidia.com")
        || url.starts_with("https://www.amd.com")
        || url.starts_with("https://www.intel.com")
        || url.starts_with("https://github.com/numbfede/OpenBX");
    if !allowed {
        return Err(crate::error::AppError::Message("Link non consentito.".into()));
    }
    std::process::Command::new("explorer").arg(url).spawn()?;
    Ok(())
}
