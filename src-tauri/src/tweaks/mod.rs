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
    fn listed(&self) -> bool {
        true
    }
    fn counts_toward_score(&self) -> bool {
        true
    }
    fn home_optimize(&self) -> bool {
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
        listed: module.listed(),
        counts_toward_score: module.counts_toward_score(),
        home_optimize: module.home_optimize(),
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

pub fn dx_flag_enabled(raw: &str, key: &str) -> bool {
    dx_get(raw, key).as_deref() == Some("1")
}

pub fn dx_get(raw: &str, key: &str) -> Option<String> {
    for part in raw.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let Some((name, value)) = part.split_once('=') else { continue };
        if name.trim().eq_ignore_ascii_case(key) {
            return Some(value.trim().to_string());
        }
    }
    None
}

pub fn dx_set(raw: &str, key: &str, value: &str) -> String {
    let mut parts: Vec<(String, String)> = raw
        .split(';')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            let (name, val) = part.split_once('=')?;
            Some((name.trim().to_string(), val.trim().to_string()))
        })
        .collect();
    if let Some(existing) = parts.iter_mut().find(|(name, _)| name.eq_ignore_ascii_case(key)) {
        existing.1 = value.to_string();
    } else {
        parts.push((key.to_string(), value.to_string()));
    }
    let mut out = String::new();
    for (name, val) in parts {
        out.push_str(&name);
        out.push('=');
        out.push_str(&val);
        out.push(';');
    }
    out
}

pub fn game_mode_bundle(preset: &str) -> &'static [&'static str] {
    match preset {
        "competitive" => &[
            "game_mode",
            "game_dvr",
            "focus_assist",
            "power_plan",
            "windowed_games",
            "gpu_scheduling_off",
        ],
        "balanced" | "streaming" => &["game_mode", "windowed_games"],
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
        assert!(game_mode_bundle("competitive").contains(&"windowed_games"));
        assert!(game_mode_bundle("competitive").contains(&"gpu_scheduling_off"));
        assert!(!game_mode_bundle("competitive").contains(&"gpu_scheduling"));
    }

    #[test]
    fn directx_preference_strings_merge() {
        let merged = dx_set("GpuPreference=1;", "GpuPreference", "2");
        let merged = dx_set(&merged, "SwapEffectUpgradeEnable", "1");
        assert!(dx_flag_enabled(&merged, "SwapEffectUpgradeEnable"));
        assert_eq!(dx_get(&merged, "GpuPreference").as_deref(), Some("2"));
        assert!(merged.contains("GpuPreference=2"));
        assert!(merged.ends_with(';'));
    }
}
