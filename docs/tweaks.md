# OpenBX tweaks

Every optimization is a module with `detect`, `apply`, `verify`, and `rollback`.

The Home score is:

```
(optimized scored tweaks / scored tweaks) × 100
```

If a tweak is not relevant to the machine, it is **hidden** and excluded from the score. Optional tweaks (`gpu_scheduling`) are listed but excluded from OPTIMIZE MY PC and from the score.

The shipping UI uses Italian copy. This document uses English so contributors can review changes without speaking Italian.

## Implemented (Safe Mode)

| ID | What the user sees (EN) | Mechanism | Source |
|---|---|---|---|
| `game_mode` | Optimize Windows for games | `HKCU\Software\Microsoft\GameBar\AutoGameModeEnabled` | [Microsoft Game Mode](https://learn.microsoft.com/windows/uwp/gaming/use-the-game-mode-api) |
| `game_dvr` | Turn off overlay and background recording | Game DVR + AppCapture + Game Bar nexus | [Xbox Game Bar](https://support.xbox.com/help/games-apps/game-bar/game-bar-overview) |
| `windowed_games` | Optimize windowed / borderless games | `DirectXUserGlobalSettings` `SwapEffectUpgradeEnable=1;VRROptimizeEnable=1;` (Windows 11 22H2+) | [Optimizations for windowed games](https://devblogs.microsoft.com/directx/optimizations-for-windowed-games-in-windows-11/) |
| `gpu_scheduling` | Use the GPU more directly (optional) | `HwSchMode=2` when supported. Not in Home optimize or Competitive | [Microsoft HAGS](https://devblogs.microsoft.com/directx/hardware-accelerated-gpu-scheduling/) |
| `gpu_scheduling_off` | Hidden Competitive helper | Sets `HwSchMode=1` so Competitive does not leave HAGS on | [Microsoft HAGS](https://devblogs.microsoft.com/directx/hardware-accelerated-gpu-scheduling/) |
| `power_plan` | Give more power to performance | Ultimate Performance if available, else High Performance; skipped on battery | [powercfg](https://learn.microsoft.com/windows-hardware/design/device-experiences/powercfg-command-line-options) |
| `visual_animations` | Simplify Windows animations | `MinAnimate` + `SystemParametersInfoW` | [SystemParametersInfo](https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-systemparametersinfow) |
| `transparency` | Reduce transparency effects | `EnableTransparency=0` | [Microsoft personalization](https://support.microsoft.com/windows/personalize-your-desktop-background-9394b71b-cc1f-53d9-8182-707e10592f14) |
| `ntfs_last_access` | Optimize file handling | `NtfsDisableLastAccessUpdate` | [fsutil behavior](https://learn.microsoft.com/windows-server/administration/windows-commands/fsutil-behavior) |
| `startup_delay` | Start apps without the extra wait | `StartupDelayInMSec=0` | [Startup apps](https://support.microsoft.com/windows/configure-startup-applications-in-windows-115a420a-0bff-4a6f-90e0-19341a412983) |
| `focus_assist` | Fewer notifications while you use the PC | Global toast setting | [Notifications](https://support.microsoft.com/windows/change-notification-and-quick-settings-in-windows-0b9e63b6-5b57-7b0b-93d2-5ab01fb9b808) |
| `consumer_tips` | Remove suggestions that waste resources | ContentDeliveryManager | [Tips](https://support.microsoft.com/windows/get-help-and-tips-in-the-tips-app-f7e3d1d0-6d57-0b8a-2b8e-7b4a5e2d7d2f) |

## Game Mode presets

- **Competitive:** `game_mode`, `game_dvr`, `focus_assist`, `power_plan`, `windowed_games`, `gpu_scheduling_off` (does **not** enable HAGS)
- **Balanced:** `game_mode`, `windowed_games`
- **Streaming:** `game_mode`, `windowed_games`
- **Default:** restores the last Game Mode backup

## Per-game

Documented Windows Graphics settings only:

```
HKCU\Software\Microsoft\DirectX\UserGpuPreferences\<exe> = GpuPreference=2;SwapEffectUpgradeEnable=1;
```

No game-specific “FPS packs”.

## Never implemented

- Network: DHCP, IP, DNS, TCP, NIC
- Placebo: HPET, timer resolution, pagefile, core parking, BCDEdit
- Dangerous: Defender / Update disable, YouTube service lists, fake RAM cleaners
