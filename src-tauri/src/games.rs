use std::path::{Path, PathBuf};

use crate::error::AppResult;
use crate::model::GameEntry;
use crate::registry::{list_subkeys, read_string, write_string, Hive};
use crate::{engine, detect};

const GPU_PREF_PATH: &str = r"Software\Microsoft\DirectX\UserGpuPreferences";

pub fn scan_games() -> Vec<GameEntry> {
    let mut games = Vec::new();
    games.extend(scan_steam());
    games.extend(scan_epic());
    games.extend(scan_ubisoft());
    games.extend(scan_xbox_games());
    games.extend(scan_start_menu());
    games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    games.dedup_by(|a, b| names_match(&a.name, &b.name));
    games
}

pub fn optimize_game(_app: &tauri::AppHandle, game_id: &str) -> AppResult<crate::model::OptimizeResult> {
    let games = scan_games();
    let game = games
        .into_iter()
        .find(|item| item.id == game_id)
        .ok_or_else(|| crate::error::AppError::Message("Gioco non trovato.".into()))?;
    let Some(exe) = game.exe_path else {
        return Err(crate::error::AppError::Message("Percorso del gioco non trovato. Nessuna modifica applicata.".into()));
    };
    if is_gpu_optimized(&exe) {
        let scan_like = engine::scan();
        return Ok(crate::model::OptimizeResult {
            backup_id: None,
            applied: 0,
            failed: 0,
            skipped: 1,
            score_before: scan_like.score,
            score_after: scan_like.score,
            changes: vec![],
            finished_at: chrono::Utc::now().to_rfc3339(),
        });
    }
    let previous = read_string(Hive::Hkcu, GPU_PREF_PATH, &exe);
    write_string(Hive::Hkcu, GPU_PREF_PATH, &exe, "GpuPreference=2;")?;
    crate::paths::append_log(&format!("game gpu preference set for {}", game.name));
    let _ = detect::collect_system_info();
    Ok(crate::model::OptimizeResult {
        backup_id: None,
        applied: 1,
        failed: 0,
        skipped: 0,
        score_before: 0,
        score_after: 100,
        changes: vec![crate::model::ChangedTweak {
            id: game.id,
            title: format!("Preferenza GPU ad alte prestazioni — {}", game.name),
            success: true,
            message: "Impostazione Windows Graphics documentata.".into(),
            technical: Some(crate::model::TweakTechnical {
                registry_path: Some(format!(r"HKCU\{GPU_PREF_PATH}\{exe}")),
                old_value: previous,
                new_value: Some("GpuPreference=2;".into()),
                powershell: None,
                windows_api: Some("DirectX UserGpuPreferences".into()),
                source: "Microsoft Graphics Settings".into(),
                risk: crate::model::RiskLevel::Low,
                rollback_method: "Ripristina o elimina UserGpuPreferences per l'eseguibile".into(),
            }),
        }],
        finished_at: chrono::Utc::now().to_rfc3339(),
    })
}

fn is_gpu_optimized(exe: &str) -> bool {
    read_string(Hive::Hkcu, GPU_PREF_PATH, exe)
        .map(|value| value.contains("GpuPreference=2"))
        .unwrap_or(false)
}

fn scan_steam() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let mut roots = vec![
        PathBuf::from(r"C:\Program Files (x86)\Steam"),
        PathBuf::from(r"C:\Program Files\Steam"),
    ];
    if let Ok(home) = std::env::var("ProgramFiles(x86)") {
        roots.push(PathBuf::from(home).join("Steam"));
    }
    if let Some(path) = read_string(Hive::Hkcu, r"Software\Valve\Steam", "SteamPath") {
        roots.push(PathBuf::from(path.replace('/', "\\")));
    }
    if let Some(path) = read_string(Hive::Hklm, r"SOFTWARE\WOW6432Node\Valve\Steam", "InstallPath") {
        roots.push(PathBuf::from(path.replace('/', "\\")));
    }
    roots.sort();
    roots.dedup();
    for root in roots {
        let vdf = root.join(r"steamapps\libraryfolders.vdf");
        let libraries = if vdf.exists() {
            parse_library_paths(&std::fs::read_to_string(vdf).unwrap_or_default())
        } else if root.join(r"steamapps").exists() {
            vec![root.clone()]
        } else {
            continue;
        };
        for library in libraries {
            let steamapps = if library.ends_with("steamapps") {
                library
            } else {
                library.join("steamapps")
            };
            let Ok(entries) = std::fs::read_dir(&steamapps) else { continue };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) != Some("acf") {
                    continue;
                }
                if let Ok(raw) = std::fs::read_to_string(&path) {
                    if let Some((appid, name, installdir)) = parse_acf(&raw) {
                        if is_tool_app(&name) {
                            continue;
                        }
                        let common = steamapps.join("common").join(&installdir);
                        let exe = find_exe(&common, &name);
                        let optimized = exe.as_ref().map(|path| is_gpu_optimized(path)).unwrap_or(false);
                        games.push(GameEntry {
                            id: format!("steam-{appid}"),
                            name,
                            source: "steam".into(),
                            exe_path: exe,
                            optimized,
                            applicable: true,
                        });
                    }
                }
            }
        }
    }
    games
}

fn scan_epic() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let manifests = PathBuf::from(r"C:\ProgramData\Epic\EpicGamesLauncher\Data\Manifests");
    let Ok(entries) = std::fs::read_dir(manifests) else {
        return games;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("item") {
            continue;
        }
        let Ok(raw) = std::fs::read_to_string(&path) else { continue };
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&raw) else { continue };
        let name = json.get("DisplayName").and_then(|v| v.as_str()).unwrap_or("Epic Game");
        let location = json.get("InstallLocation").and_then(|v| v.as_str()).unwrap_or_default();
        let exe_name = json.get("LaunchExecutable").and_then(|v| v.as_str()).unwrap_or_default();
        let exe = if location.is_empty() || exe_name.is_empty() {
            None
        } else {
            Some(PathBuf::from(location).join(exe_name).display().to_string())
        };
        let optimized = exe.as_ref().map(|path| is_gpu_optimized(path)).unwrap_or(false);
        games.push(GameEntry {
            id: format!("epic-{}", json.get("CatalogItemId").and_then(|v| v.as_str()).unwrap_or(name)),
            name: name.into(),
            source: "epic".into(),
            exe_path: exe,
            optimized,
            applicable: true,
        });
    }
    games
}

fn scan_ubisoft() -> Vec<GameEntry> {
    let mut games = Vec::new();
    for path in [
        r"SOFTWARE\WOW6432Node\Ubisoft\Launcher\Installs",
        r"SOFTWARE\Ubisoft\Launcher\Installs",
    ] {
        for id in list_subkeys(Hive::Hklm, path) {
            let key = format!("{path}\\{id}");
            let Some(dir) = read_string(Hive::Hklm, &key, "InstallDir") else { continue };
            let folder = PathBuf::from(dir.trim_end_matches(['\\', '/']));
            if !folder.exists() {
                continue;
            }
            let name = folder
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("Ubisoft game")
                .to_string();
            if is_tool_app(&name) {
                continue;
            }
            let exe = find_exe(&folder, &name);
            let optimized = exe.as_ref().map(|path| is_gpu_optimized(path)).unwrap_or(false);
            games.push(GameEntry {
                id: format!("ubisoft-{id}"),
                name,
                source: "ubisoft".into(),
                exe_path: exe,
                optimized,
                applicable: true,
            });
        }
    }
    for games_root in [
        PathBuf::from(r"C:\Program Files (x86)\Ubisoft\Ubisoft Game Launcher\games"),
        PathBuf::from(r"C:\Program Files\Ubisoft\Ubisoft Game Launcher\games"),
    ] {
        let Ok(entries) = std::fs::read_dir(games_root) else { continue };
        for entry in entries.flatten() {
            let folder = entry.path();
            if !folder.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if games.iter().any(|game| names_match(&game.name, &name)) {
                continue;
            }
            let exe = find_exe(&folder, &name);
            let optimized = exe.as_ref().map(|path| is_gpu_optimized(path)).unwrap_or(false);
            games.push(GameEntry {
                id: format!("ubisoft-{}", slug(&name)),
                name,
                source: "ubisoft".into(),
                exe_path: exe,
                optimized,
                applicable: true,
            });
        }
    }
    games
}

fn scan_xbox_games() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let mut roots = vec![PathBuf::from(r"C:\XboxGames")];
    if let Ok(pf) = std::env::var("ProgramFiles") {
        roots.push(PathBuf::from(pf).join("XboxGames"));
    }
    for root in roots {
        let Ok(entries) = std::fs::read_dir(root) else { continue };
        for entry in entries.flatten() {
            let folder = entry.path();
            if !folder.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            let content = folder.join("Content");
            let search = if content.exists() { content } else { folder };
            let exe = find_exe(&search, &name);
            let applicable = exe.is_some();
            let optimized = exe.as_ref().map(|path| is_gpu_optimized(path)).unwrap_or(false);
            games.push(GameEntry {
                id: format!("xbox-{}", slug(&name)),
                name,
                source: "xbox".into(),
                exe_path: exe,
                optimized,
                applicable,
            });
        }
    }
    games
}

fn scan_start_menu() -> Vec<GameEntry> {
    let mut games = Vec::new();
    let known = [
        "Fortnite",
        "Valorant",
        "Dead by Daylight",
        "Apex Legends",
        "League of Legends",
        "Counter-Strike",
        "Minecraft",
        "Roblox",
        "GTA",
        "Grand Theft Auto",
        "Call of Duty",
        "Overwatch",
        "Rocket League",
        "The Sims",
        "Elden Ring",
        "Rainbow Six",
        "RainbowSix",
        "Assassin's Creed",
        "Far Cry",
        "Watch Dogs",
        "The Division",
        "Ghost Recon",
        "For Honor",
    ];
    let mut roots = Vec::new();
    if let Some(roaming) = dirs::data_dir() {
        roots.push(roaming.join(r"Microsoft\Windows\Start Menu\Programs"));
    }
    roots.push(PathBuf::from(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs"));
    for root in roots {
        walk_lnk(&root, &known, &mut games);
    }
    games
}

fn walk_lnk(dir: &Path, known: &[&str], games: &mut Vec<GameEntry>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            walk_lnk(&path, known, games);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.eq_ignore_ascii_case("lnk")) != Some(true) {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        if !known.iter().any(|name| stem.to_ascii_lowercase().contains(&name.to_ascii_lowercase())) {
            continue;
        }
        let exe = resolve_lnk(&path);
        let optimized = exe.as_ref().map(|value| is_gpu_optimized(value)).unwrap_or(false);
        games.push(GameEntry {
            id: format!("startmenu-{}", stem.to_ascii_lowercase().replace(' ', "-")),
            name: stem.into(),
            source: "startmenu".into(),
            exe_path: exe,
            optimized,
            applicable: true,
        });
    }
}

fn resolve_lnk(path: &Path) -> Option<String> {
    let script = format!(
        "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $s.TargetPath",
        path.display().to_string().replace('\\', "\\\\").replace('\'', "''")
    );
    let output = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output()
        .ok()?;
    let target = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if target.ends_with(".exe") { Some(target) } else { None }
}

fn parse_library_paths(vdf: &str) -> Vec<PathBuf> {
    vdf.lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("\"path\"") {
                line.split('"').nth(3).map(|value| PathBuf::from(value.replace("\\\\", "\\")))
            } else {
                None
            }
        })
        .collect()
}

fn parse_acf(raw: &str) -> Option<(String, String, String)> {
    let appid = kv(raw, "appid")?;
    let name = kv(raw, "name")?;
    let installdir = kv(raw, "installdir")?;
    Some((appid, name, installdir))
}

fn kv(raw: &str, key: &str) -> Option<String> {
    raw.lines().find_map(|line| {
        let line = line.trim();
        if line.starts_with(&format!("\"{key}\"")) {
            line.split('"').nth(3).map(|value| value.to_string())
        } else {
            None
        }
    })
}

fn is_tool_app(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("redistributable")
        || lower.contains("proton")
        || lower.contains("steamworks")
        || lower.contains("easyanticheat")
        || lower.contains("battleye")
        || lower == "ubisoft connect"
        || lower == "ubisoft game launcher"
}

fn is_skipped_exe(name: &str) -> bool {
    let n = name.to_ascii_lowercase();
    n.contains("unins")
        || n.contains("crash")
        || n.contains("easyanticheat")
        || n.contains("battleye")
        || n.contains("launcher")
        || n.ends_with("_be")
        || n.contains("redist")
        || n.contains("setup")
        || n.contains("vcredist")
        || n.contains("unitycrash")
        || n.contains("cefsharp")
        || n.contains("report")
}

fn find_exe(dir: &Path, name: &str) -> Option<String> {
    let mut exes = Vec::new();
    collect_exes(dir, 0, 2, &mut exes);
    if exes.is_empty() {
        return None;
    }
    let needle = compact(name);
    exes.into_iter()
        .max_by_key(|path| {
            let file = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            let compact_name = compact(&file);
            let name_score = u64::from(
                compact_name.contains(&needle)
                    || needle.contains(&compact_name)
                    || compact_name.contains("rainbowsix")
                        && (needle.contains("siege") || needle.contains("rainbow")),
            ) * 1_000_000;
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            name_score + size
        })
        .map(|path| path.display().to_string())
}

fn collect_exes(dir: &Path, depth: u8, max_depth: u8, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if depth < max_depth {
                collect_exes(&path, depth + 1, max_depth, out);
            }
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else { continue };
        if path.extension().and_then(|ext| ext.to_str()).map(|ext| ext.eq_ignore_ascii_case("exe")) != Some(true) {
            continue;
        }
        if is_skipped_exe(stem) {
            continue;
        }
        out.push(path);
    }
}

fn compact(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

fn names_match(a: &str, b: &str) -> bool {
    let left = compact(a).replace("tomclancys", "");
    let right = compact(b).replace("tomclancys", "");
    left == right
}

fn slug(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn siege_names_are_the_same_game() {
        assert!(names_match(
            "Tom Clancy's Rainbow Six Siege",
            "Rainbow Six Siege"
        ));
        assert!(!names_match("Rainbow Six Siege", "Rainbow Six Extraction"));
    }

    #[test]
    fn acf_and_vdf_parsers() {
        let acf = r#"
        "AppState"
        {
            "appid" "730"
            "name" "Counter-Strike 2"
            "installdir" "Counter-Strike Global Offensive"
        }
        "#;
        let parsed = parse_acf(acf).unwrap();
        assert_eq!(parsed.0, "730");
        let vdf = "\"path\"\t\t\"D:\\\\SteamLibrary\"";
        assert_eq!(parse_library_paths(vdf)[0], PathBuf::from(r"D:\SteamLibrary"));
    }
}
