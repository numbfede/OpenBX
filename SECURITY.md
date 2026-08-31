# Security Policy

OpenBX changes Windows settings on the local machine. Treat every new module as a privilege boundary.

## Supported versions

| Version | Supported |
| --- | --- |
| 0.1.x | Yes |

## Reporting a vulnerability

Please **do not** open a public issue for anything that could:

- brick or lock a Windows install
- persist after Restore
- leak personal data
- escalate privileges beyond the UAC prompt the user already accepted

Use GitHub **Privately report a vulnerability** on this repository, or open an issue titled `SECURITY:` only if the report cannot be exploited by publishing it.

We will acknowledge the report and work on a fix before any disclosure.

## What the app does

- Reads hardware and Windows version **locally**
- Writes only the registry / power-plan values listed in [docs/tweaks.md](docs/tweaks.md)
- Stores backups and logs in `%LOCALAPPDATA%\OpenBX\`
- Opens documentation URLs only from an allow-list of vendor domains
- Optionally asks GitHub Releases for the latest version string (no hardware or personal data). Disable it in Settings.

## What it never does

- No accounts
- No telemetry (not even opt-in in this release)
- No remote hardware inventory
- No DHCP / IP / DNS / TCP / NIC changes
- No disabling of Windows Security or Windows Update

## Privileges

Some keys live under `HKLM` and need administrator rights (`gpu_scheduling`, `gpu_scheduling_off`, `ntfs_last_access`, `power_plan`). The app starts unelevated and can relaunch with a Windows UAC prompt. HKCU tweaks still work without elevation.

## Rollback

Every applied tweak stores the previous value. Restore replays `rollback()` in reverse order. If `verify()` fails after apply, OpenBX rolls that tweak back immediately and does not count it as success.
