use std::os::raw::{c_ulong, c_uchar};
use std::process::Command;

use serde::Deserialize;
use sysinfo::System;

use crate::model::{GpuVendor, SystemInfo};
use crate::registry::{read_dword, Hive};

#[repr(C)]
struct SystemPowerStatus {
    ac_line_status: c_uchar,
    battery_flag: c_uchar,
    battery_life_percent: c_uchar,
    system_status_flag: c_uchar,
    battery_life_time: c_ulong,
    battery_full_life_time: c_ulong,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> i32;
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WmiSnapshot {
    cpu: Option<String>,
    gpus: Option<Vec<String>>,
    ram_gb: Option<u64>,
    windows: Option<String>,
    build: Option<String>,
    chassis_types: Option<Vec<u32>>,
}

pub fn collect_system_info() -> SystemInfo {
    let wmi = query_wmi();
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu = wmi
        .as_ref()
        .and_then(|item| item.cpu.clone())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| {
            sys.cpus()
                .first()
                .map(|cpu| cpu.brand().trim().to_string())
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "CPU sconosciuta".into())
        });

    let gpu = wmi
        .as_ref()
        .and_then(|item| item.gpus.clone())
        .unwrap_or_default()
        .into_iter()
        .find(|name| !name.to_lowercase().contains("microsoft basic"))
        .or_else(|| primary_gpu_from_registry())
        .unwrap_or_else(|| "GPU sconosciuta".into());

    let gpu_vendor = vendor_from_name(&gpu);
    let ram_gb = wmi
        .as_ref()
        .and_then(|item| item.ram_gb)
        .unwrap_or_else(|| sys.total_memory() / 1024 / 1024 / 1024)
        .max(1);

    let windows = wmi
        .as_ref()
        .and_then(|item| item.windows.clone())
        .unwrap_or_else(|| System::long_os_version().unwrap_or_else(|| "Windows".into()));

    let windows_build = wmi
        .as_ref()
        .and_then(|item| item.build.clone())
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| parse_build(System::os_version().unwrap_or_default()));

    let (on_ac_power, has_battery) = power_status();
    let chassis_laptop = wmi
        .as_ref()
        .and_then(|item| item.chassis_types.clone())
        .map(|types| types.iter().any(|code| matches!(code, 8 | 9 | 10 | 11 | 14 | 30 | 31 | 32)))
        .unwrap_or(false);
    let is_laptop = chassis_laptop || has_battery;

    let hags_value = read_dword(
        Hive::Hklm,
        r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers",
        "HwSchMode",
    );
    let hags_supported = windows_build >= 19041 && hags_value != Some(0) && gpu_vendor != GpuVendor::Other;

    SystemInfo {
        cpu: cpu.trim().to_string(),
        gpu: gpu.trim().to_string(),
        gpu_vendor,
        ram_gb,
        windows: windows.trim().to_string(),
        windows_build,
        is_laptop,
        on_ac_power,
        is_elevated: is_process_elevated(),
        hags_supported,
    }
}

fn is_process_elevated() -> bool {
    #[cfg(windows)]
    {
        is_elevated::is_elevated()
    }
    #[cfg(not(windows))]
    {
        false
    }
}

fn power_status() -> (bool, bool) {
    unsafe {
        let mut status = SystemPowerStatus {
            ac_line_status: 255,
            battery_flag: 255,
            battery_life_percent: 255,
            system_status_flag: 0,
            battery_life_time: 0,
            battery_full_life_time: 0,
        };
        if GetSystemPowerStatus(&mut status) == 0 {
            return (true, false);
        }
        let on_ac = status.ac_line_status != 0;
        let has_battery = status.battery_flag != 128;
        (on_ac, has_battery)
    }
}

fn query_wmi() -> Option<WmiSnapshot> {
    let script = r#"
$ErrorActionPreference = 'Stop'
$cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name
$gpus = @(Get-CimInstance Win32_VideoController | ForEach-Object { $_.Name })
$ram = [uint64][math]::Round(((Get-CimInstance Win32_ComputerSystem).TotalPhysicalMemory) / 1GB)
$os = Get-CimInstance Win32_OperatingSystem
$chassis = @()
try { $chassis = @((Get-CimInstance Win32_SystemEnclosure).ChassisTypes) } catch {}
[pscustomobject]@{
  cpu = "$cpu"
  gpus = $gpus
  ramGb = $ram
  windows = $os.Caption
  build = "$($os.BuildNumber)"
  chassisTypes = $chassis
} | ConvertTo-Json -Compress
"#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", script])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    serde_json::from_slice(&output.stdout).ok()
}

fn primary_gpu_from_registry() -> Option<String> {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class = hklm
        .open_subkey_with_flags(
            r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
            KEY_READ,
        )
        .ok()?;
    for i in 0..8 {
        let name = format!("{i:04}");
        if let Ok(key) = class.open_subkey_with_flags(&name, KEY_READ) {
            if let Ok(desc) = key.get_value::<String, _>("DriverDesc") {
                if !desc.to_lowercase().contains("microsoft basic") {
                    return Some(desc);
                }
            }
        }
    }
    None
}

fn vendor_from_name(name: &str) -> GpuVendor {
    let lower = name.to_ascii_lowercase();
    if lower.contains("nvidia") || lower.contains("geforce") || lower.contains("rtx") || lower.contains("gtx") {
        GpuVendor::Nvidia
    } else if lower.contains("amd") || lower.contains("radeon") || lower.contains("ryzen") {
        GpuVendor::Amd
    } else if lower.contains("intel") || lower.contains("arc") || lower.contains("uhd") || lower.contains("iris") {
        GpuVendor::Intel
    } else {
        GpuVendor::Other
    }
}

fn parse_build(value: String) -> u32 {
    value
        .split(|c: char| !c.is_ascii_digit())
        .find(|part| !part.is_empty())
        .and_then(|part| part.parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vendor_detection_hides_wrong_gpu_family() {
        assert_eq!(vendor_from_name("NVIDIA GeForce RTX 4070"), GpuVendor::Nvidia);
        assert_eq!(vendor_from_name("AMD Radeon RX 7800 XT"), GpuVendor::Amd);
        assert_eq!(vendor_from_name("Intel Arc B580"), GpuVendor::Intel);
    }
}
