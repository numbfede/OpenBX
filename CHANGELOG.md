# Changelog

All notable changes to OpenBX are documented in this file.

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
