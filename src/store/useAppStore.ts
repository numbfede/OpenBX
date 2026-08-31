import { create } from "zustand";
import { api, isTauri, onOptimizeProgress } from "../services/api";
import {
  DEFAULT_SETTINGS,
  type AppSettings,
  type BackupSummary,
  type GameEntry,
  type GameModePreset,
  type OptimizeEvent,
  type OptimizeResult,
  type ScanResult,
  type ScreenId,
  type SystemInfo,
  type TweakState,
} from "../services/types";

export type OverlayId =
  | null
  | "firstrun"
  | "optimize"
  | "result"
  | "beforeafter"
  | "elevation"
  | "tweak-detail"
  | "confirm-optimize"
  | "confirm-restore"
  | "confirm-safemode"
  | "error";

export interface ToastItem {
  id: string;
  title: string;
  body?: string;
  tone: "ready" | "warn" | "danger";
}

interface AppState {
  ready: boolean;
  screen: ScreenId;
  overlay: OverlayId;
  desktopAvailable: boolean;
  system: SystemInfo | null;
  settings: AppSettings;
  scan: ScanResult | null;
  backups: BackupSummary[];
  games: GameEntry[];
  selectedTweak: TweakState | null;
  pendingBackupId: string | null;
  optimizeProgress: OptimizeEvent | null;
  lastResult: OptimizeResult | null;
  lastError: { title: string; body: string; details?: string } | null;
  toasts: ToastItem[];
  busy: boolean;
  setScreen: (screen: ScreenId) => void;
  setOverlay: (overlay: OverlayId) => void;
  openTweak: (tweak: TweakState) => void;
  pushToast: (toast: Omit<ToastItem, "id">) => void;
  dismissToast: (id: string) => void;
  bootstrap: () => Promise<void>;
  refreshScan: () => Promise<void>;
  requestOptimize: (tweakIds?: string[]) => void;
  runOptimize: (tweakIds?: string[]) => Promise<void>;
  requestRestore: (backupId: string) => void;
  runRestore: () => Promise<void>;
  applyGameMode: (preset: GameModePreset) => Promise<void>;
  refreshGames: () => Promise<void>;
  optimizeGame: (gameId: string) => Promise<void>;
  updateSettings: (patch: Partial<AppSettings>, options?: { allowDisableSafeMode?: boolean }) => Promise<void>;
  relaunchElevated: () => Promise<void>;
  exportLogs: () => Promise<void>;
}

function toastId(): string {
  return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
}

async function safe<T>(fn: () => Promise<T>): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    const message = error instanceof Error ? error.message : String(error);
    throw new Error(message);
  }
}

export const useAppStore = create<AppState>((set, get) => ({
  ready: false,
  screen: "home",
  overlay: null,
  desktopAvailable: isTauri(),
  system: null,
  settings: DEFAULT_SETTINGS,
  scan: null,
  backups: [],
  games: [],
  selectedTweak: null,
  pendingBackupId: null,
  optimizeProgress: null,
  lastResult: null,
  lastError: null,
  toasts: [],
  busy: false,

  setScreen: (screen) => set({ screen, overlay: screen === get().screen ? get().overlay : null }),
  setOverlay: (overlay) => set({ overlay }),
  openTweak: (tweak) => set({ selectedTweak: tweak, overlay: "tweak-detail" }),

  pushToast: (toast) =>
    set((state) => ({
      toasts: [...state.toasts, { ...toast, id: toastId() }].slice(-4),
    })),

  dismissToast: (id) =>
    set((state) => ({ toasts: state.toasts.filter((item) => item.id !== id) })),

  bootstrap: async () => {
    if (!isTauri()) {
      set({
        ready: true,
        desktopAvailable: false,
        lastError: {
          title: "Apri OpenBX sul desktop",
          body: "Questa interfaccia deve girare come applicazione Windows, non nel browser.",
        },
        overlay: "error",
      });
      return;
    }

    try {
      const [system, settings, backups] = await Promise.all([
        api.getSystemInfo(),
        api.getSettings(),
        api.listBackups(),
      ]);
      set({ system, settings, backups, desktopAvailable: true });
      document.documentElement.dataset.theme = settings.theme;

      if (!system.isElevated && !settings.firstRunCompleted) {
        set({ overlay: "elevation", ready: true });
        return;
      }
      if (!settings.firstRunCompleted) {
        set({ overlay: "firstrun", ready: true });
        return;
      }

      await get().refreshScan();
      set({ ready: true });
    } catch (error) {
      set({
        ready: true,
        lastError: {
          title: "Something went wrong",
          body: "Windows non ha permesso di leggere lo stato del PC.",
          details: error instanceof Error ? error.message : String(error),
        },
        overlay: "error",
      });
    }
  },

  refreshScan: async () => {
    const scan = await safe(() => api.scan());
    set({ scan });
  },

  requestOptimize: (tweakIds) => {
    if (get().settings.askBeforeApplying) {
      set({ overlay: "confirm-optimize", pendingBackupId: tweakIds?.[0] ?? null });
      return;
    }
    void get().runOptimize(tweakIds);
  },

  runOptimize: async (tweakIds) => {
    set({ overlay: "optimize", optimizeProgress: { step: "scanning", message: "Scanning...", current: 0, total: 1 }, busy: true });
    const unlisten = await onOptimizeProgress((event) => set({ optimizeProgress: event }));
    try {
      const result = await api.optimize(tweakIds);
      const [scan, backups] = await Promise.all([api.scan(), api.listBackups()]);
      set({
        lastResult: result,
        scan,
        backups,
        overlay: "result",
        busy: false,
      });
    } catch (error) {
      set({
        busy: false,
        overlay: "error",
        lastError: {
          title: "Something went wrong",
          body: "Windows non ha permesso di modificare questa impostazione.",
          details: error instanceof Error ? error.message : String(error),
        },
      });
    } finally {
      unlisten();
    }
  },

  requestRestore: (backupId) => set({ pendingBackupId: backupId, overlay: "confirm-restore" }),

  runRestore: async () => {
    const backupId = get().pendingBackupId;
    if (!backupId) return;
    set({ overlay: "optimize", busy: true, optimizeProgress: { step: "backup", message: "Ripristino in corso...", current: 0, total: 1 } });
    try {
      const result = await api.restore(backupId);
      const [scan, backups] = await Promise.all([api.scan(), api.listBackups()]);
      set({
        lastResult: result,
        scan,
        backups,
        overlay: "result",
        busy: false,
        pendingBackupId: null,
      });
    } catch (error) {
      set({
        busy: false,
        overlay: "error",
        lastError: {
          title: "Something went wrong",
          body: "Non è stato possibile ripristinare le impostazioni precedenti.",
          details: error instanceof Error ? error.message : String(error),
        },
      });
    }
  },

  applyGameMode: async (preset) => {
    set({ overlay: "optimize", busy: true, optimizeProgress: { step: "gaming", message: "Ottimizzazione gaming...", current: 0, total: 1 } });
    const unlisten = await onOptimizeProgress((event) => set({ optimizeProgress: event }));
    try {
      const result = await api.applyGameMode(preset);
      const [scan, backups] = await Promise.all([api.scan(), api.listBackups()]);
      set({ lastResult: result, scan, backups, overlay: "result", busy: false });
    } catch (error) {
      set({
        busy: false,
        overlay: "error",
        lastError: {
          title: "Something went wrong",
          body: "Windows non ha permesso di cambiare la modalità di gioco.",
          details: error instanceof Error ? error.message : String(error),
        },
      });
    } finally {
      unlisten();
    }
  },

  refreshGames: async () => {
    const games = await api.scanGames();
    set({ games });
  },

  optimizeGame: async (gameId) => {
    set({ overlay: "optimize", busy: true, optimizeProgress: { step: "gaming", message: "Ottimizzazione gioco...", current: 0, total: 1 } });
    try {
      const result = await api.optimizeGame(gameId);
      const games = await api.scanGames();
      set({ lastResult: result, games, overlay: "result", busy: false });
    } catch (error) {
      set({
        busy: false,
        overlay: "error",
        lastError: {
          title: "Something went wrong",
          body: "Non è stato possibile ottimizzare questo gioco.",
          details: error instanceof Error ? error.message : String(error),
        },
      });
    }
  },

  updateSettings: async (patch, options) => {
    const current = get().settings;
    if (patch.safeMode === false && current.safeMode && !options?.allowDisableSafeMode) {
      set({ overlay: "confirm-safemode" });
      return;
    }
    const settings = await api.saveSettings({ ...current, ...patch });
    document.documentElement.dataset.theme = settings.theme;
    set({ settings, overlay: get().overlay === "confirm-safemode" ? null : get().overlay });
  },

  relaunchElevated: async () => {
    await api.relaunchElevated();
  },

  exportLogs: async () => {
    const path = await api.exportLogs();
    get().pushToast({
      tone: "ready",
      title: "Log esportati",
      body: path,
    });
  },
}));
