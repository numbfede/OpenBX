# OpenBX tweaks

Every optimization is a module with `detect`, `apply`, `verify`, and `rollback`.

The Home score is:

```
(optimized applicable tweaks / applicable tweaks) × 100
```

If a tweak is not relevant to the machine, it is **hidden** and excluded from the score.

The shipping UI uses Italian copy. This document uses English so contributors can review changes without speaking Italian.

## Implemented (Safe Mode)

| ID | What the user sees (EN) | Mechanism | Source |
|---|---|---|---|
| `game_mode` | Optimize Windows for games | `HKCU\Software\Microsoft\GameBar\AutoGameModeEnabled` | [Microsoft Game Mode](https://learn.microsoft.com/windows/uwp/gaming/use-the-game-mode-api) |
| `game_dvr` | Reduce background recording | Game DVR + AppCapture | [Xbox Game Bar](https://support.xbox.com/help/games-apps/game-bar/game-bar-overview) |
| `gpu_scheduling` | Use the GPU more directly | `HwSchMode=2` when supported | [Microsoft HAGS](https://devblogs.microsoft.com/directx/hardware-accelerated-gpu-scheduling/) |
| `power_plan` | Give more power to performance | `powercfg /setactive` High Performance; skipped on battery | [powercfg](https://learn.microsoft.com/windows-hardware/design/device-experiences/powercfg-command-line-options) |
| `visual_animations` | Simplify Windows animations | `MinAnimate` + `SystemParametersInfoW` | [SystemParametersInfo](https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-systemparametersinfow) |
| `transparency` | Reduce transparency effects | `EnableTransparency=0` | [Microsoft personalization](https://support.microsoft.com/windows/personalize-your-desktop-background-9394b71b-cc1f-53d9-8182-707e10592f14) |
| `ntfs_last_access` | Optimize file handling | `NtfsDisableLastAccessUpdate` | [fsutil behavior](https://learn.microsoft.com/windows-server/administration/windows-commands/fsutil-behavior) |
| `startup_delay` | Start apps without the extra wait | `StartupDelayInMSec=0` | [Startup apps](https://support.microsoft.com/windows/configure-startup-applications-in-windows-115a420a-0b9e63b6-5b57-7b0b-93d2-5ab01fb9b808) |
| `focus_assist` | Fewer notifications while you use the PC | Global toast setting | [Notifications](https://support.microsoft.com/windows/change-notification-and-quick-settings-in-windows-0b9e63b6-5b57-7b0b-93d2-5ab01fb9b808) |
| `consumer_tips` | Remove suggestions that waste resources | ContentDeliveryManager | [Tips](https://support.microsoft.com/windows/get-help-and-tips-in-the-tips-app-f7e3d1d0-6d57-0b8a-2b8e-7b4a5e2d7d2f) |

## Game Mode presets

- **Competitive:** `game_mode`, `game_dvr`, `focus_assist`, `power_plan`, `gpu_scheduling`
- **Balanced:** `game_mode`
- **Streaming:** `game_mode` only, so OBS and other apps keep running
- **Default:** restores the last Game Mode backup

## Per-game

Only the documented Windows Graphics preference:

```
HKCU\Software\Microsoft\DirectX\UserGpuPreferences\<exe> = GpuPreference=2;
```

No game-specific “FPS packs”.

## Never implemented

- Network: DHCP, IP, DNS, TCP, NIC
- Placebo: HPET, timer resolution, pagefile, core parking, BCDEdit
- Dangerous: Defender / Update disable, YouTube service lists, fake RAM cleaners
