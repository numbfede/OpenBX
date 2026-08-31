use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_SET_VALUE};
use winreg::RegKey;

use crate::error::{AppError, AppResult};

#[derive(Clone, Copy)]
pub enum Hive {
    Hkcu,
    Hklm,
}

impl Hive {
    fn open(self) -> RegKey {
        match self {
            Hive::Hkcu => RegKey::predef(HKEY_CURRENT_USER),
            Hive::Hklm => RegKey::predef(HKEY_LOCAL_MACHINE),
        }
    }
}

pub fn read_dword(hive: Hive, path: &str, name: &str) -> Option<u32> {
    hive.open()
        .open_subkey_with_flags(path, KEY_READ)
        .ok()
        .and_then(|key| key.get_value::<u32, _>(name).ok())
}

pub fn read_string(hive: Hive, path: &str, name: &str) -> Option<String> {
    hive.open()
        .open_subkey_with_flags(path, KEY_READ)
        .ok()
        .and_then(|key| key.get_value::<String, _>(name).ok())
}

pub fn write_dword(hive: Hive, path: &str, name: &str, value: u32) -> AppResult<()> {
    let root = hive.open();
    let (key, _) = root
        .create_subkey_with_flags(path, KEY_SET_VALUE | KEY_READ)
        .map_err(|_| AppError::AccessDenied)?;
    key.set_value(name, &value).map_err(|_| AppError::AccessDenied)
}

pub fn write_string(hive: Hive, path: &str, name: &str, value: &str) -> AppResult<()> {
    let root = hive.open();
    let (key, _) = root
        .create_subkey_with_flags(path, KEY_SET_VALUE | KEY_READ)
        .map_err(|_| AppError::AccessDenied)?;
    key.set_value(name, &value.to_string())
        .map_err(|_| AppError::AccessDenied)
}

pub fn delete_value(hive: Hive, path: &str, name: &str) -> AppResult<()> {
    let key = hive
        .open()
        .open_subkey_with_flags(path, KEY_SET_VALUE)
        .map_err(|_| AppError::AccessDenied)?;
    match key.delete_value(name) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(AppError::AccessDenied),
    }
}

pub fn snapshot_dword(hive: Hive, path: &str, name: &str) -> Option<String> {
    read_dword(hive, path, name).map(|value| value.to_string())
}

pub fn snapshot_string(hive: Hive, path: &str, name: &str) -> Option<String> {
    read_string(hive, path, name)
}
