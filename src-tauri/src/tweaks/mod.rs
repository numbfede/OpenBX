mod catalog;
mod win32;

use crate::error::AppResult;
use crate::model::{
    CategoryId, RiskLevel, SnapshotValue, SourceRef, SystemInfo, TweakSnapshot, TweakState,
    TweakTechnical,
};

pub use catalog::all_modules;

pub trait TweakModule: Send + Sync {
    fn id(&self) -> &'static str;
    fn category(&self) -> CategoryId;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn what(&self) -> &'static str;
    fn why(&self) -> &'static str;
    fn risk(&self) -> RiskLevel {
        RiskLevel::Low
    }
    fn reversible(&self) -> bool {
        true
    }
    fn safe_mode_allowed(&self) -> bool {
        true
    }
    fn sources(&self) -> Vec<SourceRef>;
    fn detect(&self, ctx: &SystemInfo) -> DetectResult;
    fn apply(&self, ctx: &SystemInfo, snapshot: &mut TweakSnapshot) -> AppResult<()>;
    fn verify(&self, ctx: &SystemInfo) -> bool;
    fn rollback(&self, snapshot: &TweakSnapshot) -> AppResult<()>;
    fn technical(&self, ctx: &SystemInfo, old: Option<String>, new: Option<String>) -> TweakTechnical;
}

#[derive(Debug, Clone)]
pub struct DetectResult {
    pub applicable: bool,
    pub optimized: bool,
    pub skipped_reason: Option<String>,
    pub current_value: Option<String>,
}

impl DetectResult {
    pub fn skip(reason: impl Into<String>) -> Self {
        Self {
            applicable: false,
            optimized: false,
            skipped_reason: Some(reason.into()),
            current_value: None,
        }
    }

    pub fn current(optimized: bool, value: impl Into<String>) -> Self {
        Self {
            applicable: true,
            optimized,
            skipped_reason: None,
            current_value: Some(value.into()),
        }
    }
}

pub fn to_state(module: &dyn TweakModule, ctx: &SystemInfo) -> TweakState {
    let detect = module.detect(ctx);
    TweakState {
        id: module.id().into(),
        category: module.category(),
        title: module.title().into(),
        description: module.description().into(),
        what: module.what().into(),
        why: module.why().into(),
        risk: module.risk(),
        reversible: module.reversible(),
        safe_mode_allowed: module.safe_mode_allowed(),
        applicable: detect.applicable,
        optimized: detect.optimized,
        skipped_reason: detect.skipped_reason,
        sources: module.sources(),
        technical: Some(module.technical(ctx, detect.current_value, None)),
    }
}

#[allow(dead_code)]
pub fn snapshot_for(module: &dyn TweakModule, ctx: &SystemInfo) -> TweakSnapshot {
    let detect = module.detect(ctx);
    TweakSnapshot {
        id: module.id().into(),
        values: vec![SnapshotValue {
            key: "current".into(),
            value: detect.current_value,
        }],
    }
}

pub fn ntfs_last_access_disabled(value: u32) -> bool {
    matches!(value & 0x7FFF_FFFF, 1 | 3)
}

pub fn game_mode_bundle(preset: &str) -> &'static [&'static str] {
    match preset {
        "competitive" => &["game_mode", "game_dvr", "focus_assist", "power_plan", "gpu_scheduling"],
        "balanced" | "streaming" => &["game_mode"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ntfs_values_match_microsoft_encoding() {
        assert!(ntfs_last_access_disabled(1));
        assert!(ntfs_last_access_disabled(3));
        assert!(ntfs_last_access_disabled(0x8000_0003));
        assert!(!ntfs_last_access_disabled(0));
        assert!(!ntfs_last_access_disabled(2));
    }

    #[test]
    fn game_mode_bundles_are_conservative() {
        assert!(game_mode_bundle("streaming").len() < game_mode_bundle("competitive").len());
        assert_eq!(game_mode_bundle("default").len(), 0);
        assert!(game_mode_bundle("competitive").contains(&"game_mode"));
    }
}
