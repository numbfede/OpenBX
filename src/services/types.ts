export type CategoryId = "performance" | "gaming" | "memory" | "startup" | "windows";
export type RiskLevel = "low" | "medium" | "high";
export type ScreenId = "home" | "categories" | "gamemode" | "games" | "restore" | "settings";
export type GameModePreset = "competitive" | "balanced" | "streaming" | "default";
export type OptimizeStep =
  | "scanning"
  | "backup"
  | "windows"
  | "gaming"
  | "startup"
  | "verifying"
  | "done"
  | "failed";

export interface SourceRef {
  title: string;
  url: string;
}

export interface SystemInfo {
  cpu: string;
  gpu: string;
  gpuVendor: "nvidia" | "amd" | "intel" | "other";
  ramGb: number;
  windows: string;
  windowsBuild: number;
  isLaptop: boolean;
  onAcPower: boolean;
  isElevated: boolean;
  hagsSupported: boolean;
}

export interface TweakState {
  id: string;
  category: CategoryId;
  title: string;
  description: string;
  what: string;
  why: string;
  risk: RiskLevel;
  reversible: boolean;
  safeModeAllowed: boolean;
  applicable: boolean;
  optimized: boolean;
  skippedReason?: string | null;
  sources: SourceRef[];
  technical?: TweakTechnical | null;
}

export interface TweakTechnical {
  registryPath?: string | null;
  oldValue?: string | null;
  newValue?: string | null;
  powershell?: string | null;
  windowsApi?: string | null;
  source: string;
  risk: RiskLevel;
  rollbackMethod: string;
}

export interface CategorySummary {
  id: CategoryId;
  title: string;
  description: string;
  optimized: number;
  applicable: number;
}

export interface ScanResult {
  score: number;
  applicable: number;
  optimized: number;
  pending: number;
  tweaks: TweakState[];
  categories: CategorySummary[];
  scannedAt: string;
}

export interface OptimizeEvent {
  step: OptimizeStep;
  message: string;
  current: number;
  total: number;
  tweakId?: string | null;
}

export interface ChangedTweak {
  id: string;
  title: string;
  success: boolean;
  message: string;
  technical?: TweakTechnical | null;
}

export interface OptimizeResult {
  backupId?: string | null;
  applied: number;
  failed: number;
  skipped: number;
  scoreBefore: number;
  scoreAfter: number;
  changes: ChangedTweak[];
  finishedAt: string;
}

export interface BackupSummary {
  id: string;
  createdAt: string;
  changeCount: number;
  scoreBefore: number;
  kind: "optimize" | "gamemode" | "game";
  label: string;
}

export interface GameEntry {
  id: string;
  name: string;
  source: "steam" | "epic" | "startmenu";
  exePath?: string | null;
  optimized: boolean;
  applicable: boolean;
}

export interface AppSettings {
  startWithWindows: boolean;
  minimizeToTray: boolean;
  notifications: boolean;
  theme: "dark" | "light";
  createBackupAutomatically: boolean;
  askBeforeApplying: boolean;
  safeMode: boolean;
  developerMode: boolean;
  showTechnicalDetails: boolean;
  firstRunCompleted: boolean;
}

export const DEFAULT_SETTINGS: AppSettings = {
  startWithWindows: false,
  minimizeToTray: false,
  notifications: true,
  theme: "dark",
  createBackupAutomatically: true,
  askBeforeApplying: true,
  safeMode: true,
  developerMode: false,
  showTechnicalDetails: false,
  firstRunCompleted: false,
};

export const CATEGORY_META: Record<
  CategoryId,
  { title: string; description: string; kicker: string }
> = {
  performance: {
    title: "Performance",
    description: "Rendi Windows più reattivo.",
    kicker: "( 01 )",
  },
  gaming: {
    title: "Gaming",
    description: "Ottimizza Windows per giocare.",
    kicker: "( 02 )",
  },
  memory: {
    title: "Memory",
    description: "Riduci il lavoro inutile in background.",
    kicker: "( 03 )",
  },
  startup: {
    title: "Startup",
    description: "Fai avviare il PC più velocemente.",
    kicker: "( 04 )",
  },
  windows: {
    title: "Windows",
    description: "Rimuovi impostazioni che possono rallentare il sistema.",
    kicker: "( 05 )",
  },
};
