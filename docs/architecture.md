# Architecture

OpenBX is a Tauri 2 desktop app. The React UI never talks to the Windows Registry. It only displays state that the Rust backend has read and verified.

```
React UI  --invoke-->  Tauri commands  -->  TweakEngine
                                           |     |
                                      BackupStore  SystemDetector
                                           |
                                      tweak modules
```

## Layers

| Layer | Location | Responsibility |
| --- | --- | --- |
| UI | `src/` | Screens, glass components, human copy |
| Commands | `src-tauri/src/commands.rs` | IPC surface used by the frontend |
| Engine | `src-tauri/src/engine.rs` | Scan, filter (Safe Mode), apply, verify, restore |
| Tweaks | `src-tauri/src/tweaks/` | Independent `detect/apply/verify/rollback` modules |
| Backup | `src-tauri/src/backup.rs` | JSON manifests under `%LOCALAPPDATA%\OpenBX\backups\` |
| Detect | `src-tauri/src/detect.rs` | CPU, GPU, RAM, Windows, laptop/AC, elevation |
| Games | `src-tauri/src/games.rs` | Steam / Epic / Start Menu discovery, GPU preference |

## Rules the engine enforces

- A backup snapshot is written with the **previous** values before a change is counted as success.
- `verify()` re-reads Windows after every change. Failed tweaks are rolled back and are not counted as success.
- Safe Mode (default on) applies only low-risk, reversible, documented modules.
- Offline first. Internet is used only to open documentation links the user clicked.
- No DHCP / IP / DNS / TCP / NIC modules exist in the tree.

## Local data

```
%LOCALAPPDATA%\OpenBX\
  settings.json
  backups\<id>\manifest.json
  logs\openbx.log
```
