# OpenBX

A Windows optimizer you can actually use without knowing what the Registry is.

> You use the program. You should not have to understand the program.

OpenBX is an open-source desktop app for Windows 10/11. It reads real system settings, applies only documented low-risk changes, verifies each one, and can restore everything.

No fake FPS. No invented percentages. No network tweaks. No telemetry. No account.

[Features](#features) · [Download](#download) · [Build from source](#build-from-source) · [Safety](#safety) · [Contributing](#contributing)

## Why it exists

Most Windows “optimizers” dump hundreds of registry checkboxes on the user, copy YouTube myths, or invent benchmark numbers.

OpenBX does the opposite:

1. **Can this PC be optimized?**
2. **What should I press?**

A ten-year-old should be able to use it. Technical details stay behind **Details** / Developer mode.

## Features

- **One-click optimize** — scan, automatic backup, apply only compatible safe tweaks, verify, show the result
- **Honest score** — `(optimized applicable settings / applicable settings) × 100`. Nothing else
- **Human categories** — Performance, Gaming, Memory, Startup, Windows. Not “registry / services / scheduler”
- **Hardware-aware** — irrelevant tweaks (wrong GPU vendor, unsupported OS, laptop on battery) are hidden and excluded from the score
- **Game Mode** — Competitive, Balanced, Streaming, or Default (restore)
- **Game profiles** — local Steam / Epic / Start Menu detection. Per-game change is only the documented Windows GPU preference
- **Restore** — every change stores the previous value
- **Safe Mode on by default** — only reversible, documented, low-risk modules
- **Offline first** — the app does not need a server
- **Privacy** — no accounts, no telemetry, no remote hardware inventory

## What it will never do

- Touch DHCP, IP, gateway, DNS, TCP, or NIC configuration
- Claim FPS or latency gains without a real benchmark
- Disable Windows Security or Windows Update
- Apply irreversible changes without confirmation
- Ship placebo tweaks popular on YouTube/TikTok

See [docs/tweaks.md](docs/tweaks.md) for every setting the app is allowed to change.

## Download

Windows 10 version 2004 (build 19041) or later, or Windows 11.

Build the installer from this repository:

```bash
npm install
npm run tauri build
```

The NSIS setup file is written to `src-tauri/target/release/bundle/nsis/`.

Some optimizations need administrator permission. OpenBX starts unelevated, explains why in plain language, and can relaunch with a Windows UAC prompt.

## Build from source

### Requirements

- Windows 10 2004+ or Windows 11
- [Node.js](https://nodejs.org/) 20+ (22+ preferred)
- [Rust](https://rustup.rs/) stable (MSVC toolchain)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **Desktop development with C++** workload
- WebView2 (included on Windows 11)

### Develop

```bash
git clone https://github.com/numbfede/OpenBX.git
cd OpenBX
npm install
npm run tauri dev
```

### Tests

```bash
cargo test --manifest-path src-tauri/Cargo.toml
```

See [tests/README.md](tests/README.md) for the manual smoke checklist.

## How the score works

The number on the Home screen is not a benchmark.

```
score = round(optimized_applicable / applicable * 100)
```

| Score | Label |
| --- | --- |
| 0–59 | CAN BE OPTIMIZED |
| 60–89 | PC READY |
| 90–100 | FULLY OPTIMIZED |

If a setting does not apply to this PC, it is not shown and it does not affect the score.

## Architecture

```
React UI  --invoke-->  Tauri commands  -->  TweakEngine
                                           |     |
                                      BackupStore  SystemDetector
                                           |
                                      tweak modules
```

Each optimization is an independent module:

- `detect()` — is it applicable, and is it already in the desired state?
- `apply()` — snapshot the previous value, then change Windows
- `verify()` — read the setting back; failures are not counted as success
- `rollback()` — restore the previous value

Stack: **Tauri 2 + React + TypeScript + Tailwind** (UI) and **Rust** (system changes).

Details: [docs/architecture.md](docs/architecture.md).

## Privacy

- No account
- No telemetry (not even opt-in in this release)
- No data sent to a server
- Backups and logs stay in `%LOCALAPPDATA%\OpenBX\`

## Language

The interface currently ships in **Italian**, with English primary actions (`OPTIMIZE MY PC`, `GAME MODE`, `RESTORE`, `DONE`). Translations are welcome — see [CONTRIBUTING.md](CONTRIBUTING.md).

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

The short version: a new tweak is accepted only if it is documented, reversible, compatible, and not a placebo. Cite Microsoft Learn, Microsoft Support, NVIDIA, AMD, or Intel.

## Security

Please report issues that could brick a machine, leak data, or survive rollback. See [SECURITY.md](SECURITY.md).

## License

[MIT](LICENSE)

## Disclaimer

OpenBX changes Windows settings on your PC. Create a restore point if you want extra insurance. The authors are not responsible for misconfiguration of your system. Safe Mode is on by default for a reason.
