# Changelog

All notable changes to OpenBX are documented in this file.

## 0.1.5 — 2026-08-31

### Fixed

- Game scan no longer opens flashing PowerShell windows for Start Menu shortcuts
- System detection no longer launches PowerShell/WMI, so startup does not freeze the UI
- `powercfg` runs hidden, without a console window
- Game search runs in the background and shows “Cerchiamo i giochi…” instead of locking the window

## 0.1.4 — 2026-08-31

### Added

- Windows 11 **Optimizations for windowed games** and variable refresh (`SwapEffectUpgradeEnable`, `VRROptimizeEnable`)
- Game Bar overlay is turned off together with Game DVR
- Per-game optimize now also enables windowed-game optimizations
- Competitive Game Mode no longer enables Hardware-accelerated GPU scheduling (HAGS); it turns HAGS off when it was on
- Desktop power plan prefers Ultimate Performance, with High Performance as fallback
- Optional HAGS remains available from Details, excluded from the Home score

### Fixed

- Home copy states the score is Windows settings, not an FPS increase
- Tray icon was created without an image (blank square in the notification area)

## 0.1.3 — 2026-08-31

### Fixed

- The Windows permission screen (**Autorizza**) now appears every time OpenBX starts without administrator rights, not only on first run
- **OPTIMIZE MY PC** asks for permission again if the app is still running without it
- Settings includes an Autorizza action when permission is missing

## 0.1.2 — 2026-08-31

### Fixed

- GPU card no longer shows virtual adapters such as **Parsec Virtual Display Adapter**; OpenBX picks the real GPU (NVIDIA / AMD / Intel)
- Rainbow Six Siege and other Ubisoft Connect games were missing because the scanner only looked at Steam, Epic, and a short Start Menu list

### Added

- Game scan now includes Ubisoft Connect, Xbox Games, Steam from the registry, and more Start Menu titles

## 0.1.1 — 2026-08-31

### Added

- In-app notice when a newer GitHub Release exists (optional, off if you disable “Check for updates”)
- Install and update guide: [docs/install.md](docs/install.md)

### Fixed

- Elevating the **dev** build no longer opens PowerShell and then a localhost error page
- Installed app relaunches with a normal Windows UAC prompt, without a terminal window

## 0.1.0 — 2026-08-31

### Added

- First public release: Home score, one-click optimize, automatic backup, restore
- Local system detection (CPU, GPU, RAM, Windows edition, laptop vs desktop, AC power, elevation)
- Ten documented Safe Mode tweaks with `detect` / `apply` / `verify` / `rollback`
- Game Mode presets: Competitive, Balanced, Streaming, Default
- Local game discovery (Steam, Epic, Start Menu) and per-game GPU high-performance preference
- Settings, Safe Mode on by default, Developer mode, log export
- NSIS installer packaging
- MIT license, contributing guide, and security policy

### Guarantees

- No network optimizer
- No telemetry
- No invented FPS or benchmark numbers
