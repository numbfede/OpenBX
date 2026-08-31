use tauri::AppHandle;

use crate::backup::list_backups;
use crate::detect::collect_system_info;
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
pub fn relaunch_elevated() -> AppResult<()> {
    let exe = std::env::current_exe()?;
    std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "Start-Process -FilePath '{}' -Verb RunAs",
                exe.display().to_string().replace('\'', "''")
            ),
        ])
        .spawn()?;
    append_log("relaunch elevated requested");
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
pub fn open_external_url(url: String) -> AppResult<()> {
    if !(url.starts_with("https://learn.microsoft.com")
        || url.starts_with("https://support.microsoft.com")
        || url.starts_with("https://support.xbox.com")
        || url.starts_with("https://devblogs.microsoft.com")
        || url.starts_with("https://www.nvidia.com")
        || url.starts_with("https://www.amd.com")
        || url.starts_with("https://www.intel.com"))
    {
        return Err(crate::error::AppError::Message("Link non consentito.".into()));
    }
    std::process::Command::new("explorer").arg(url).spawn()?;
    Ok(())
}
