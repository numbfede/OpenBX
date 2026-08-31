use std::path::PathBuf;

use crate::error::{AppError, AppResult};

pub fn data_dir() -> AppResult<PathBuf> {
    let base = dirs::data_local_dir().ok_or_else(|| AppError::Message("LocalAppData non trovato.".into()))?;
    let dir = base.join("OpenBX");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn backups_dir() -> AppResult<PathBuf> {
    let dir = data_dir()?.join("backups");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn settings_path() -> AppResult<PathBuf> {
    Ok(data_dir()?.join("settings.json"))
}

pub fn log_path() -> AppResult<PathBuf> {
    Ok(data_dir()?.join("logs").join("openbx.log"))
}

pub fn append_log(line: &str) {
    if let Ok(path) = log_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let stamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .and_then(|mut file| {
                use std::io::Write;
                writeln!(file, "[{stamp}] {line}")
            });
    }
}
