use crate::error::AppResult;
use crate::model::AppSettings;
use crate::paths::settings_path;
use crate::registry::{write_string, delete_value, Hive};

pub fn load_settings() -> AppSettings {
    settings_path()
        .ok()
        .and_then(|path| std::fs::read_to_string(path).ok())
        .and_then(|raw| serde_json::from_str(&raw).ok())
        .unwrap_or_default()
}

pub fn save_settings(settings: &AppSettings) -> AppResult<AppSettings> {
    let path = settings_path()?;
    std::fs::write(path, serde_json::to_string_pretty(settings)?)?;
    apply_autostart(settings.start_with_windows)?;
    Ok(settings.clone())
}

fn apply_autostart(enabled: bool) -> AppResult<()> {
    let exe = std::env::current_exe()?.display().to_string();
    const PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    if enabled {
        write_string(Hive::Hkcu, PATH, "OpenBX", &format!("\"{exe}\""))?;
    } else {
        let _ = delete_value(Hive::Hkcu, PATH, "OpenBX");
    }
    Ok(())
}
