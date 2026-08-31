import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import type {
  AppSettings,
  BackupSummary,
  GameEntry,
  GameModePreset,
  OptimizeEvent,
  OptimizeResult,
  ScanResult,
  SystemInfo,
  UpdateInfo,
} from "./types";

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri()) {
    throw new Error("OpenBX deve essere aperto come applicazione desktop.");
  }
  return invoke<T>(command, args);
}

export const api = {
  getSystemInfo: () => call<SystemInfo>("get_system_info"),
  getSettings: () => call<AppSettings>("get_settings"),
  saveSettings: (settings: AppSettings) => call<AppSettings>("save_settings_cmd", { settings }),
  scan: () => call<ScanResult>("scan_system"),
  optimize: (tweakIds?: string[]) =>
    call<OptimizeResult>("optimize_system", { tweakIds: tweakIds ?? null }),
  restore: (backupId: string) => call<OptimizeResult>("restore_backup", { backupId }),
  listBackups: () => call<BackupSummary[]>("list_backups_cmd"),
  applyGameMode: (preset: GameModePreset) =>
    call<OptimizeResult>("apply_game_mode_cmd", { preset }),
  scanGames: () => call<GameEntry[]>("scan_games_cmd"),
  optimizeGame: (gameId: string) => call<OptimizeResult>("optimize_game_cmd", { gameId }),
  relaunchElevated: () => call<void>("relaunch_elevated"),
  exportLogs: () => call<string>("export_logs"),
  openUrl: (url: string) => call<void>("open_external_url", { url }),
  checkForUpdates: () => call<UpdateInfo>("check_for_updates"),
};

export async function onOptimizeProgress(
  handler: (event: OptimizeEvent) => void,
): Promise<UnlistenFn> {
  if (!isTauri()) return () => undefined;
  return listen<OptimizeEvent>("optimize-progress", (event) => handler(event.payload));
}

export const windowApi = {
  minimize: async () => {
    if (isTauri()) await getCurrentWindow().minimize();
  },
  toggleMaximize: async () => {
    if (isTauri()) await getCurrentWindow().toggleMaximize();
  },
  close: async () => {
    if (isTauri()) await getCurrentWindow().close();
  },
};
