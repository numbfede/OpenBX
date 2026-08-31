use std::os::raw::{c_ulong, c_uchar};

use crate::model::{GpuVendor, SystemInfo};
use crate::registry::{read_dword, read_string, Hive};

#[repr(C)]
struct SystemPowerStatus {
    ac_line_status: c_uchar,
    battery_flag: c_uchar,
    battery_life_percent: c_uchar,
    system_status_flag: c_uchar,
    battery_life_time: c_ulong,
    battery_full_life_time: c_ulong,
}

#[repr(C)]
struct MemoryStatusEx {
    length: u32,
    memory_load: u32,
    total_phys: u64,
    avail_phys: u64,
    total_page_file: u64,
    avail_page_file: u64,
    total_virtual: u64,
    avail_virtual: u64,
    avail_extended_virtual: u64,
}

#[link(name = "kernel32")]
extern "system" {
    fn GetSystemPowerStatus(status: *mut SystemPowerStatus) -> i32;
    fn GlobalMemoryStatusEx(status: *mut MemoryStatusEx) -> i32;
}

pub fn collect_system_info() -> SystemInfo {
    let cpu = read_string(
        Hive::Hklm,
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0",
        "ProcessorNameString",
    )
    .filter(|value| !value.trim().is_empty())
    .unwrap_or_else(|| "CPU sconosciuta".into());

    let gpu = primary_gpu_from_registry().unwrap_or_else(|| "GPU sconosciuta".into());
    let gpu_vendor = vendor_from_name(&gpu);
    let ram_gb = total_ram_gb().max(1);

    let product = read_string(Hive::Hklm, r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", "ProductName")
        .unwrap_or_else(|| "Windows".into());
    let build = read_string(Hive::Hklm, r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", "CurrentBuildNumber")
        .unwrap_or_default();
    let windows_build = parse_build(build);
    let mut windows = if product.to_ascii_lowercase().contains("windows") {
        product
    } else {
        format!("Windows {product}")
    };
    if windows_build >= 22000 {
        windows = windows.replace("Windows 10", "Windows 11");
    }

    let (on_ac_power, has_battery) = power_status();
    let is_laptop = has_battery;

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
        is_dev: cfg!(debug_assertions),
    }
}

fn total_ram_gb() -> u64 {
    unsafe {
        let mut status = MemoryStatusEx {
            length: std::mem::size_of::<MemoryStatusEx>() as u32,
            memory_load: 0,
            total_phys: 0,
            avail_phys: 0,
            total_page_file: 0,
            avail_page_file: 0,
            total_virtual: 0,
            avail_virtual: 0,
            avail_extended_virtual: 0,
        };
        if GlobalMemoryStatusEx(&mut status) == 0 || status.total_phys == 0 {
            return 1;
        }
        (status.total_phys / 1024 / 1024 / 1024).max(1)
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

fn pick_primary_gpu(gpus: Vec<(String, u64)>) -> Option<String> {
    gpus.into_iter()
        .filter(|(name, _)| !is_virtual_gpu(name))
        .max_by_key(|(name, ram)| gpu_priority(name) + ram / (1024 * 1024))
        .map(|(name, _)| name)
}

fn is_virtual_gpu(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    [
        "parsec",
        "virtual display",
        "virtual desktop",
        "microsoft basic",
        "microsoft remote",
        "remote desktop",
        "citrix",
        "teamviewer",
        "anydesk",
        "rustdesk",
        "vmware",
        "virtualbox",
        "hyper-v",
        "hyperv",
        "spacedesk",
        "duet display",
        "steam streaming",
        "sunshine",
        "moonlight",
        "idd ",
        "indirect display",
        "usb display",
        "usb-c virtual",
        "meta virtual",
        "oculus virtual",
        "nvidia virtual",
        "rdp",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

fn gpu_priority(name: &str) -> u64 {
    let lower = name.to_ascii_lowercase();
    if lower.contains("geforce") || lower.contains("rtx") || lower.contains("gtx") || lower.contains("quadro") {
        3_000_000
    } else if lower.contains("radeon") && (lower.contains("rx") || lower.contains("xt") || lower.contains("pro")) {
        3_000_000
    } else if lower.contains("arc") {
        2_500_000
    } else if lower.contains("nvidia") || lower.contains("amd") || lower.contains("radeon") {
        2_000_000
    } else if lower.contains("intel") {
        1_000_000
    } else {
        100_000
    }
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
    let mut found = Vec::new();
    for i in 0..32 {
        let name = format!("{i:04}");
        if let Ok(key) = class.open_subkey_with_flags(&name, KEY_READ) {
            if let Ok(desc) = key.get_value::<String, _>("DriverDesc") {
                if !is_virtual_gpu(&desc) {
                    found.push((desc, 0));
                }
            }
        }
    }
    pick_primary_gpu(found)
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

    #[test]
    fn parsec_and_virtual_adapters_are_ignored() {
        assert!(is_virtual_gpu("Parsec Virtual Display Adapter"));
        assert!(is_virtual_gpu("Microsoft Remote Display Adapter"));
        let picked = pick_primary_gpu(vec![
            ("Parsec Virtual Display Adapter".into(), 0),
            ("NVIDIA GeForce RTX 5070 Ti".into(), 16 * 1024 * 1024 * 1024),
            ("Intel UHD Graphics".into(), 128 * 1024 * 1024),
        ]);
        assert_eq!(picked.as_deref(), Some("NVIDIA GeForce RTX 5070 Ti"));
    }
}
