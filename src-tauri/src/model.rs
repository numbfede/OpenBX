use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum CategoryId {
    Performance,
    Gaming,
    Memory,
    Startup,
    Windows,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceRef {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    pub cpu: String,
    pub gpu: String,
    pub gpu_vendor: GpuVendor,
    pub ram_gb: u64,
    pub windows: String,
    pub windows_build: u32,
    pub is_laptop: bool,
    pub on_ac_power: bool,
    pub is_elevated: bool,
    pub hags_supported: bool,
    pub is_dev: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakTechnical {
    pub registry_path: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub powershell: Option<String>,
    pub windows_api: Option<String>,
    pub source: String,
    pub risk: RiskLevel,
    pub rollback_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakState {
    pub id: String,
    pub category: CategoryId,
    pub title: String,
    pub description: String,
    pub what: String,
    pub why: String,
    pub risk: RiskLevel,
    pub reversible: bool,
    pub safe_mode_allowed: bool,
    pub listed: bool,
    pub counts_toward_score: bool,
    pub home_optimize: bool,
    pub applicable: bool,
    pub optimized: bool,
    pub skipped_reason: Option<String>,
    pub sources: Vec<SourceRef>,
    pub technical: Option<TweakTechnical>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorySummary {
    pub id: CategoryId,
    pub title: String,
    pub description: String,
    pub optimized: u32,
    pub applicable: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub score: u32,
    pub applicable: u32,
    pub optimized: u32,
    pub pending: u32,
    pub tweaks: Vec<TweakState>,
    pub categories: Vec<CategorySummary>,
    pub scanned_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeEvent {
    pub step: String,
    pub message: String,
    pub current: u32,
    pub total: u32,
    pub tweak_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangedTweak {
    pub id: String,
    pub title: String,
    pub success: bool,
    pub message: String,
    pub technical: Option<TweakTechnical>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResult {
    pub backup_id: Option<String>,
    pub applied: u32,
    pub failed: u32,
    pub skipped: u32,
    pub score_before: u32,
    pub score_after: u32,
    pub changes: Vec<ChangedTweak>,
    pub finished_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BackupKind {
    Optimize,
    Gamemode,
    Game,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupSummary {
    pub id: String,
    pub created_at: String,
    pub change_count: u32,
    pub score_before: u32,
    pub kind: BackupKind,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweakSnapshot {
    pub id: String,
    pub values: Vec<SnapshotValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotValue {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupManifest {
    pub id: String,
    pub created_at: String,
    pub score_before: u32,
    pub kind: BackupKind,
    pub label: String,
    pub snapshots: Vec<TweakSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameEntry {
    pub id: String,
    pub name: String,
    pub source: String,
    pub exe_path: Option<String>,
    pub optimized: bool,
    pub applicable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct AppSettings {
    pub start_with_windows: bool,
    pub minimize_to_tray: bool,
    pub notifications: bool,
    pub check_for_updates: bool,
    pub theme: String,
    pub create_backup_automatically: bool,
    pub ask_before_applying: bool,
    pub safe_mode: bool,
    pub developer_mode: bool,
    pub show_technical_details: bool,
    pub first_run_completed: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            start_with_windows: false,
            minimize_to_tray: false,
            notifications: true,
            check_for_updates: true,
            theme: "dark".into(),
            create_backup_automatically: true,
            ask_before_applying: true,
            safe_mode: true,
            developer_mode: false,
            show_technical_details: false,
            first_run_completed: false,
        }
    }
}

pub fn compute_score(optimized: u32, applicable: u32) -> u32 {
    if applicable == 0 {
        100
    } else {
        ((optimized as f32 / applicable as f32) * 100.0).round() as u32
    }
}

#[allow(dead_code)]
pub fn score_label(score: u32) -> &'static str {
    if score >= 90 {
        "FULLY OPTIMIZED"
    } else if score >= 60 {
        "PC READY"
    } else {
        "CAN BE OPTIMIZED"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_is_ratio_not_invented() {
        assert_eq!(compute_score(5, 10), 50);
        assert_eq!(compute_score(9, 10), 90);
        assert_eq!(compute_score(0, 0), 100);
    }

    #[test]
    fn labels_match_spec() {
        assert_eq!(score_label(54), "CAN BE OPTIMIZED");
        assert_eq!(score_label(60), "PC READY");
        assert_eq!(score_label(87), "PC READY");
        assert_eq!(score_label(96), "FULLY OPTIMIZED");
    }
}
