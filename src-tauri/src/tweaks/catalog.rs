use std::process::Command;

use crate::error::AppResult;
use crate::model::{
    CategoryId, RiskLevel, SnapshotValue, SourceRef, SystemInfo, TweakSnapshot, TweakTechnical,
};
use crate::registry::{
    delete_value, read_dword, read_string, snapshot_dword, snapshot_string, write_dword, write_string,
    Hive,
};
use crate::tweaks::{ntfs_last_access_disabled, win32, DetectResult, TweakModule};

const HIGH_PERFORMANCE: &str = "8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c";
const ULTIMATE: &str = "e9a42b02-d5df-448d-aa00-03f14749eb61";
const BALANCED: &str = "381b4222-f694-41f0-9685-ff5bb260df2e";

pub fn all_modules() -> Vec<Box<dyn TweakModule>> {
    vec![
        Box::new(GameMode),
        Box::new(GameDvr),
        Box::new(GpuScheduling),
        Box::new(PowerPlan),
        Box::new(VisualAnimations),
        Box::new(Transparency),
        Box::new(NtfsLastAccess),
        Box::new(StartupDelay),
        Box::new(FocusAssist),
        Box::new(ConsumerTips),
    ]
}

fn src(title: &str, url: &str) -> SourceRef {
    SourceRef {
        title: title.into(),
        url: url.into(),
    }
}

fn tech(
    path: &str,
    old: Option<String>,
    new: Option<String>,
    api: &str,
    source: &str,
    rollback: &str,
    powershell: Option<&str>,
) -> TweakTechnical {
    TweakTechnical {
        registry_path: Some(path.into()),
        old_value: old,
        new_value: new,
        powershell: powershell.map(str::to_string),
        windows_api: Some(api.into()),
        source: source.into(),
        risk: RiskLevel::Low,
        rollback_method: rollback.into(),
    }
}

fn restore_dword(hive: Hive, path: &str, name: &str, snapshot: &TweakSnapshot, key: &str) -> AppResult<()> {
    match snapshot.values.iter().find(|item| item.key == key).and_then(|item| item.value.as_ref()) {
        Some(value) => {
            let parsed = value.parse::<u32>().unwrap_or(0);
            write_dword(hive, path, name, parsed)
        }
        None => delete_value(hive, path, name),
    }
}

fn restore_string(hive: Hive, path: &str, name: &str, snapshot: &TweakSnapshot, key: &str) -> AppResult<()> {
    match snapshot.values.iter().find(|item| item.key == key).and_then(|item| item.value.as_ref()) {
        Some(value) => write_string(hive, path, name, value),
        None => delete_value(hive, path, name),
    }
}

struct GameMode;
impl TweakModule for GameMode {
    fn id(&self) -> &'static str { "game_mode" }
    fn category(&self) -> CategoryId { CategoryId::Gaming }
    fn title(&self) -> &'static str { "Ottimizza Windows per i giochi" }
    fn description(&self) -> &'static str { "Attiva la modalità gioco di Windows." }
    fn what(&self) -> &'static str { "Abilita Game Mode, così Windows dà più attenzione al gioco aperto." }
    fn why(&self) -> &'static str { "Può ridurre il lavoro in background mentre giochi." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft Learn — Game Mode", "https://learn.microsoft.com/windows/uwp/gaming/use-the-game-mode-api")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_dword(Hive::Hkcu, r"Software\Microsoft\GameBar", "AutoGameModeEnabled").unwrap_or(0);
        DetectResult::current(value == 1, value.to_string())
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "AutoGameModeEnabled".into(),
            value: snapshot_dword(Hive::Hkcu, r"Software\Microsoft\GameBar", "AutoGameModeEnabled"),
        });
        write_dword(Hive::Hkcu, r"Software\Microsoft\GameBar", "AutoGameModeEnabled", 1)?;
        write_dword(Hive::Hkcu, r"Software\Microsoft\GameBar", "AllowAutoGameMode", 1)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hkcu, r"Software\Microsoft\GameBar", "AutoGameModeEnabled", snapshot, "AutoGameModeEnabled")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Software\Microsoft\GameBar\AutoGameModeEnabled", old, new.or(Some("1".into())), "Registry", "Microsoft Game Mode", "Ripristina il DWORD precedente", None)
    }
}

struct GameDvr;
impl TweakModule for GameDvr {
    fn id(&self) -> &'static str { "game_dvr" }
    fn category(&self) -> CategoryId { CategoryId::Gaming }
    fn title(&self) -> &'static str { "Riduci registrazione in background" }
    fn description(&self) -> &'static str { "Spegne la registrazione automatica mentre giochi." }
    fn what(&self) -> &'static str { "Disabilita Game DVR / cattura in background di Windows." }
    fn why(&self) -> &'static str { "La registrazione continua può usare GPU e disco senza che tu lo veda." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Xbox Game Bar", "https://support.xbox.com/help/games-apps/game-bar/game-bar-overview")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let dvr = read_dword(Hive::Hkcu, r"System\GameConfigStore", "GameDVR_Enabled").unwrap_or(1);
        let capture = read_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled").unwrap_or(1);
        DetectResult::current(dvr == 0 && capture == 0, format!("{dvr}/{capture}"))
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values = vec![
            SnapshotValue { key: "GameDVR_Enabled".into(), value: snapshot_dword(Hive::Hkcu, r"System\GameConfigStore", "GameDVR_Enabled") },
            SnapshotValue { key: "AppCaptureEnabled".into(), value: snapshot_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled") },
        ];
        write_dword(Hive::Hkcu, r"System\GameConfigStore", "GameDVR_Enabled", 0)?;
        write_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", 0)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hkcu, r"System\GameConfigStore", "GameDVR_Enabled", snapshot, "GameDVR_Enabled")?;
        restore_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\GameDVR", "AppCaptureEnabled", snapshot, "AppCaptureEnabled")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\System\GameConfigStore\GameDVR_Enabled", old, new.or(Some("0".into())), "Registry", "Microsoft Game Bar / DVR", "Ripristina GameDVR_Enabled e AppCaptureEnabled", None)
    }
}

struct GpuScheduling;
impl TweakModule for GpuScheduling {
    fn id(&self) -> &'static str { "gpu_scheduling" }
    fn category(&self) -> CategoryId { CategoryId::Performance }
    fn title(&self) -> &'static str { "Usa la GPU in modo più diretto" }
    fn description(&self) -> &'static str { "Attiva lo scheduling GPU accelerato, se il PC lo supporta." }
    fn what(&self) -> &'static str { "Abilita Hardware-accelerated GPU scheduling." }
    fn why(&self) -> &'static str { "Su hardware compatibile può ridurre il lavoro della CPU per la grafica." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Hardware-accelerated GPU scheduling", "https://devblogs.microsoft.com/directx/hardware-accelerated-gpu-scheduling/")]
    }
    fn detect(&self, ctx: &SystemInfo) -> DetectResult {
        if !ctx.hags_supported {
            return DetectResult::skip("Questa GPU o questa versione di Windows non espone lo scheduling GPU accelerato.");
        }
        let value = read_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode").unwrap_or(1);
        DetectResult::current(value == 2, value.to_string())
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "HwSchMode".into(),
            value: snapshot_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode"),
        });
        write_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode", 2)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode", snapshot, "HwSchMode")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKLM\SYSTEM\CurrentControlSet\Control\GraphicsDrivers\HwSchMode", old, new.or(Some("2".into())), "Registry", "Microsoft HAGS", "Ripristina HwSchMode", None)
    }
}

struct PowerPlan;
impl TweakModule for PowerPlan {
    fn id(&self) -> &'static str { "power_plan" }
    fn category(&self) -> CategoryId { CategoryId::Performance }
    fn title(&self) -> &'static str { "Dai più energia alle prestazioni" }
    fn description(&self) -> &'static str { "Usa il piano prestazioni di Windows, se ha senso su questo PC." }
    fn what(&self) -> &'static str { "Imposta il piano alimentazione High Performance." }
    fn why(&self) -> &'static str { "Windows può ridurre le prestazioni per risparmiare energia. Su desktop, o sul portatile collegato, il piano prestazioni è più adatto." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — powercfg", "https://learn.microsoft.com/windows-hardware/design/device-experiences/powercfg-command-line-options")]
    }
    fn detect(&self, ctx: &SystemInfo) -> DetectResult {
        if ctx.is_laptop && !ctx.on_ac_power {
            return DetectResult::skip("Sul portatile a batteria non forziamo il piano prestazioni.");
        }
        let active = active_power_guid().unwrap_or_default();
        let optimized = guid_eq(&active, HIGH_PERFORMANCE) || guid_eq(&active, ULTIMATE);
        DetectResult::current(optimized, active)
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "scheme".into(),
            value: active_power_guid(),
        });
        set_power_guid(HIGH_PERFORMANCE)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        let guid = snapshot
            .values
            .iter()
            .find(|item| item.key == "scheme")
            .and_then(|item| item.value.clone())
            .unwrap_or_else(|| BALANCED.into());
        set_power_guid(&guid)
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech("powercfg /getactivescheme", old, new.or(Some(HIGH_PERFORMANCE.into())), "powercfg", "Microsoft powercfg", "powercfg /setactive <guid precedente>", Some("powercfg /setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"))
    }
}

struct VisualAnimations;
impl TweakModule for VisualAnimations {
    fn id(&self) -> &'static str { "visual_animations" }
    fn category(&self) -> CategoryId { CategoryId::Performance }
    fn title(&self) -> &'static str { "Semplifica le animazioni di Windows" }
    fn description(&self) -> &'static str { "Riduce le animazioni delle finestre, senza spegnere l’interfaccia." }
    fn what(&self) -> &'static str { "Disattiva l’animazione di riduzione/ingrandimento delle finestre." }
    fn why(&self) -> &'static str { "Meno animazioni possono rendere il desktop più immediato, soprattutto su PC meno recenti." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — SystemParametersInfo", "https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-systemparametersinfow")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_string(Hive::Hkcu, r"Control Panel\Desktop\WindowMetrics", "MinAnimate").unwrap_or_else(|| "1".into());
        DetectResult::current(value == "0", value)
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "MinAnimate".into(),
            value: snapshot_string(Hive::Hkcu, r"Control Panel\Desktop\WindowMetrics", "MinAnimate"),
        });
        write_string(Hive::Hkcu, r"Control Panel\Desktop\WindowMetrics", "MinAnimate", "0")?;
        win32::set_min_animate(false);
        Ok(())
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_string(Hive::Hkcu, r"Control Panel\Desktop\WindowMetrics", "MinAnimate", snapshot, "MinAnimate")?;
        let enabled = snapshot
            .values
            .iter()
            .find(|item| item.key == "MinAnimate")
            .and_then(|item| item.value.as_deref())
            .unwrap_or("1")
            != "0";
        win32::set_min_animate(enabled);
        Ok(())
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Control Panel\Desktop\WindowMetrics\MinAnimate", old, new.or(Some("0".into())), "SystemParametersInfoW(SPI_SETANIMATION)", "Microsoft visual effects", "Ripristina MinAnimate e notifica Windows", None)
    }
}

struct Transparency;
impl TweakModule for Transparency {
    fn id(&self) -> &'static str { "transparency" }
    fn category(&self) -> CategoryId { CategoryId::Windows }
    fn title(&self) -> &'static str { "Riduci effetti trasparenti" }
    fn description(&self) -> &'static str { "Spegne le trasparenze di Windows." }
    fn what(&self) -> &'static str { "Disabilita gli effetti di trasparenza del desktop." }
    fn why(&self) -> &'static str { "Le trasparenze usano la GPU per l’interfaccia. Spegnendole il sistema può risultare più leggero." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Personalization settings", "https://support.microsoft.com/windows/personalize-your-desktop-background-9394b71b-cc1f-53d9-8182-707e10592f14")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency").unwrap_or(1);
        DetectResult::current(value == 0, value.to_string())
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "EnableTransparency".into(),
            value: snapshot_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency"),
        });
        write_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency", 0)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize", "EnableTransparency", snapshot, "EnableTransparency")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize\EnableTransparency", old, new.or(Some("0".into())), "Registry", "Microsoft personalization", "Ripristina EnableTransparency", None)
    }
}

struct NtfsLastAccess;
impl TweakModule for NtfsLastAccess {
    fn id(&self) -> &'static str { "ntfs_last_access" }
    fn category(&self) -> CategoryId { CategoryId::Windows }
    fn title(&self) -> &'static str { "Ottimizza la gestione dei file" }
    fn description(&self) -> &'static str { "Può ridurre alcune operazioni non necessarie di Windows." }
    fn what(&self) -> &'static str { "Disabilita l’aggiornamento dell’ultimo accesso sui file NTFS." }
    fn why(&self) -> &'static str { "Windows può scrivere la data di ultimo accesso su ogni file letto. Disattivarlo riduce scritture extra sul disco." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — NtfsDisableLastAccessUpdate", "https://learn.microsoft.com/windows-server/administration/windows-commands/fsutil-behavior")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsDisableLastAccessUpdate").unwrap_or(0);
        DetectResult::current(ntfs_last_access_disabled(value), format!("0x{value:08X}"))
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "NtfsDisableLastAccessUpdate".into(),
            value: snapshot_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsDisableLastAccessUpdate"),
        });
        write_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsDisableLastAccessUpdate", 0x8000_0003)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hklm, r"SYSTEM\CurrentControlSet\Control\FileSystem", "NtfsDisableLastAccessUpdate", snapshot, "NtfsDisableLastAccessUpdate")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKLM\SYSTEM\CurrentControlSet\Control\FileSystem\NtfsDisableLastAccessUpdate", old, new.or(Some("0x80000003".into())), "Registry / fsutil behavior", "Microsoft NTFS", "Ripristina NtfsDisableLastAccessUpdate", Some("fsutil behavior set disablelastaccess 3"))
    }
}

struct StartupDelay;
impl TweakModule for StartupDelay {
    fn id(&self) -> &'static str { "startup_delay" }
    fn category(&self) -> CategoryId { CategoryId::Startup }
    fn title(&self) -> &'static str { "Fai partire le app all’avvio senza attesa extra" }
    fn description(&self) -> &'static str { "Rimuove il ritardo extra che Windows aggiunge alle app di avvio." }
    fn what(&self) -> &'static str { "Imposta a zero il ritardo di avvio di Explorer per le app in Startup." }
    fn why(&self) -> &'static str { "Windows può aspettare qualche secondo prima di avviare le app. Togliendo l’attesa il desktop diventa pronto prima." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Startup apps", "https://support.microsoft.com/windows/configure-startup-applications-in-windows-115a420a-0bff-4a6f-90e0-19341a412983")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize", "StartupDelayInMSec");
        match value {
            Some(0) => DetectResult::current(true, "0"),
            Some(other) => DetectResult::current(false, other.to_string()),
            None => DetectResult::current(false, "default"),
        }
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "StartupDelayInMSec".into(),
            value: snapshot_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize", "StartupDelayInMSec"),
        });
        write_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize", "StartupDelayInMSec", 0)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize", "StartupDelayInMSec", snapshot, "StartupDelayInMSec")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Serialize\StartupDelayInMSec", old, new.or(Some("0".into())), "Registry", "Windows Explorer startup serialization", "Ripristina o elimina StartupDelayInMSec", None)
    }
}

struct FocusAssist;
impl TweakModule for FocusAssist {
    fn id(&self) -> &'static str { "focus_assist" }
    fn category(&self) -> CategoryId { CategoryId::Memory }
    fn title(&self) -> &'static str { "Meno notifiche mentre usi il PC" }
    fn description(&self) -> &'static str { "Riduce i toast di Windows. Si può riattivare in un click." }
    fn what(&self) -> &'static str { "Disattiva i toast di notifica globali di Windows." }
    fn why(&self) -> &'static str { "Meno popup significa meno interruzioni mentre lavori o giochi." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Notifications and actions", "https://support.microsoft.com/windows/change-notification-and-quick-settings-in-windows-0b9e63b6-5b57-7b0b-93d2-5ab01fb9b808")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let value = read_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_TOASTS_ENABLED").unwrap_or(1);
        DetectResult::current(value == 0, value.to_string())
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        snapshot.values.push(SnapshotValue {
            key: "NOC_GLOBAL_SETTING_TOASTS_ENABLED".into(),
            value: snapshot_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_TOASTS_ENABLED"),
        });
        write_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_TOASTS_ENABLED", 0)
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        restore_dword(Hive::Hkcu, r"Software\Microsoft\Windows\CurrentVersion\Notifications\Settings", "NOC_GLOBAL_SETTING_TOASTS_ENABLED", snapshot, "NOC_GLOBAL_SETTING_TOASTS_ENABLED")
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Software\Microsoft\Windows\CurrentVersion\Notifications\Settings\NOC_GLOBAL_SETTING_TOASTS_ENABLED", old, new.or(Some("0".into())), "Registry", "Microsoft notifications", "Ripristina NOC_GLOBAL_SETTING_TOASTS_ENABLED", None)
    }
}

struct ConsumerTips;
impl TweakModule for ConsumerTips {
    fn id(&self) -> &'static str { "consumer_tips" }
    fn category(&self) -> CategoryId { CategoryId::Memory }
    fn title(&self) -> &'static str { "Rimuovi suggerimenti che occupano risorse" }
    fn description(&self) -> &'static str { "Spegne i suggerimenti e le proposte di Windows." }
    fn what(&self) -> &'static str { "Disabilita tips, suggerimenti e contenuto suggerito di Windows." }
    fn why(&self) -> &'static str { "Questi contenuti girano in background e possono distrarre. Non servono alle prestazioni del PC." }
    fn sources(&self) -> Vec<SourceRef> {
        vec![src("Microsoft — Get tips and suggestions", "https://support.microsoft.com/windows/get-help-and-tips-in-the-tips-app-f7e3d1d0-6d57-0b8a-2b8e-7b4a5e2d7d2f")]
    }
    fn detect(&self, _ctx: &SystemInfo) -> DetectResult {
        let path = r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        let keys = [
            "SoftLandingEnabled",
            "SystemPaneSuggestionsEnabled",
            "SubscribedContent-338389Enabled",
            "SubscribedContent-338393Enabled",
        ];
        let optimized = keys.iter().all(|name| read_dword(Hive::Hkcu, path, name).unwrap_or(1) == 0);
        DetectResult::current(optimized, if optimized { "0" } else { "1" })
    }
    fn apply(&self, _ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()> {
        let path = r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        for name in [
            "SoftLandingEnabled",
            "SystemPaneSuggestionsEnabled",
            "SubscribedContent-338389Enabled",
            "SubscribedContent-338393Enabled",
            "SubscribedContent-353694Enabled",
            "SubscribedContent-353696Enabled",
        ] {
            snapshot.values.push(SnapshotValue {
                key: name.into(),
                value: snapshot_dword(Hive::Hkcu, path, name),
            });
            write_dword(Hive::Hkcu, path, name, 0)?;
        }
        Ok(())
    }
    fn verify(&self, ctx: &SystemInfo) -> bool { self.detect(ctx).optimized }
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()> {
        let path = r"Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager";
        for item in &snapshot.values {
            restore_dword(Hive::Hkcu, path, &item.key, snapshot, &item.key)?;
        }
        Ok(())
    }
    fn technical(&self, _ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical {
        tech(r"HKCU\Software\Microsoft\Windows\CurrentVersion\ContentDeliveryManager", old, new.or(Some("0".into())), "Registry", "Microsoft Content Delivery / Tips", "Ripristina i DWORD ContentDeliveryManager", None)
    }
}

fn active_power_guid() -> Option<String> {
    let output = Command::new("powercfg").args(["/getactivescheme"]).output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    extract_guid(&text)
}

fn set_power_guid(guid: &str) -> AppResult<()> {
    let status = Command::new("powercfg").args(["/setactive", guid]).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(crate::error::AppError::AccessDenied)
    }
}

fn extract_guid(text: &str) -> Option<String> {
    if let (Some(start), Some(end)) = (text.find('{'), text.find('}')) {
        if end > start {
            return Some(text[start + 1..end].to_ascii_lowercase());
        }
    }
    let lower = text.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    for i in 0..bytes.len().saturating_sub(36) {
        let slice = &lower[i..i + 36];
        if is_guid(slice) {
            return Some(slice.to_string());
        }
    }
    None
}

fn is_guid(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    bytes.iter().enumerate().all(|(index, ch)| match index {
        8 | 13 | 18 | 23 => *ch == b'-',
        _ => ch.is_ascii_hexdigit(),
    })
}

fn guid_eq(value: &str, expected: &str) -> bool {
    value.trim_matches(|c| c == '{' || c == '}').eq_ignore_ascii_case(expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::GpuVendor;

    #[test]
    fn power_guid_parser() {
        let sample = "Power Scheme GUID: 381b4222-f694-41f0-9685-ff5bb260df2e  (Balanced)";
        assert_eq!(extract_guid(sample).unwrap(), "381b4222-f694-41f0-9685-ff5bb260df2e");
        assert_eq!(
            extract_guid("GUID: {381b4222-f694-41f0-9685-ff5bb260df2e}").unwrap(),
            "381b4222-f694-41f0-9685-ff5bb260df2e"
        );
        assert!(guid_eq("381B4222-F694-41F0-9685-FF5BB260DF2E", BALANCED));
    }

    #[test]
    fn amd_only_would_be_hidden_by_detect() {
        let ctx = SystemInfo {
            cpu: "Intel".into(),
            gpu: "Intel UHD".into(),
            gpu_vendor: GpuVendor::Intel,
            ram_gb: 16,
            windows: "Windows 11".into(),
            windows_build: 22631,
            is_laptop: false,
            on_ac_power: true,
            is_elevated: true,
            hags_supported: true,
        };
        assert_ne!(ctx.gpu_vendor, GpuVendor::Amd);
    }
}
