use chrono::Utc;
use tauri::{AppHandle, Emitter};
use uuid::Uuid;

use crate::backup::{latest_of_kind, load_manifest, save_manifest};
use crate::detect::collect_system_info;
use crate::error::{AppError, AppResult};
use crate::model::{
    BackupKind, BackupManifest, CategoryId, CategorySummary, ChangedTweak, OptimizeEvent,
    OptimizeResult, ScanResult, TweakState,
};
use crate::model::compute_score;
use crate::settings::load_settings;
use crate::tweaks::{all_modules, game_mode_bundle, to_state, TweakModule};

pub fn scan() -> ScanResult {
    let ctx = collect_system_info();
    let tweaks: Vec<TweakState> = all_modules().iter().map(|module| to_state(module.as_ref(), &ctx)).collect();
    summarize(tweaks)
}

fn summarize(tweaks: Vec<TweakState>) -> ScanResult {
    let applicable = tweaks.iter().filter(|item| item.applicable).count() as u32;
    let optimized = tweaks.iter().filter(|item| item.applicable && item.optimized).count() as u32;
    let categories = [
        (CategoryId::Performance, "Performance", "Rendi Windows più reattivo."),
        (CategoryId::Gaming, "Gaming", "Ottimizza Windows per giocare."),
        (CategoryId::Memory, "Memory", "Riduci il lavoro inutile in background."),
        (CategoryId::Startup, "Startup", "Fai avviare il PC più velocemente."),
        (CategoryId::Windows, "Windows", "Rimuovi impostazioni che possono rallentare il sistema."),
    ]
    .into_iter()
    .map(|(id, title, description)| {
        let group: Vec<_> = tweaks.iter().filter(|item| item.category == id && item.applicable).collect();
        CategorySummary {
            id,
            title: title.into(),
            description: description.into(),
            optimized: group.iter().filter(|item| item.optimized).count() as u32,
            applicable: group.len() as u32,
        }
    })
    .collect();

    ScanResult {
        score: compute_score(optimized, applicable),
        applicable,
        optimized,
        pending: applicable.saturating_sub(optimized),
        tweaks,
        categories,
        scanned_at: Utc::now().to_rfc3339(),
    }
}

pub fn optimize(app: &AppHandle, tweak_ids: Option<Vec<String>>, kind: BackupKind, label: &str) -> AppResult<OptimizeResult> {
    let settings = load_settings();
    let ctx = collect_system_info();
    let modules = all_modules();
    let before = scan();
    emit(app, "scanning", "Scanning...", 0, 1, None);

    let selected: Vec<&Box<dyn TweakModule>> = modules
        .iter()
        .filter(|module| match &tweak_ids {
            Some(ids) => ids.iter().any(|id| id == module.id()),
            None => true,
        })
        .filter(|module| {
            let detect = module.detect(&ctx);
            detect.applicable && !detect.optimized
        })
        .filter(|module| {
            if settings.safe_mode {
                module.safe_mode_allowed() && module.reversible() && matches!(module.risk(), crate::model::RiskLevel::Low)
            } else {
                true
            }
        })
        .collect();

    if selected.is_empty() {
        emit(app, "done", "DONE", 1, 1, None);
        return Ok(OptimizeResult {
            backup_id: None,
            applied: 0,
            failed: 0,
            skipped: 0,
            score_before: before.score,
            score_after: before.score,
            changes: vec![],
            finished_at: Utc::now().to_rfc3339(),
        });
    }

    emit(app, "backup", "Creating backup...", 0, selected.len() as u32, None);
    let mut snapshots = Vec::new();
    let mut changes = Vec::new();
    let mut applied = 0;
    let mut failed = 0;

    let total = selected.len() as u32;
    for (index, module) in selected.iter().enumerate() {
        let step = step_for(module.category());
        emit(app, step, message_for(module.category()), index as u32, total, Some(module.id()));
        let mut snapshot = crate::model::TweakSnapshot {
            id: module.id().into(),
            values: vec![],
        };
        match module.apply(&ctx, &mut snapshot) {
            Ok(()) => {
                emit(app, "verifying", "Verifying changes...", index as u32 + 1, total, Some(module.id()));
                let ok = module.verify(&ctx);
                if ok {
                    applied += 1;
                    if settings.create_backup_automatically {
                        snapshots.push(snapshot.clone());
                    }
                    changes.push(ChangedTweak {
                        id: module.id().into(),
                        title: module.title().into(),
                        success: true,
                        message: "Modifica verificata.".into(),
                        technical: Some(module.technical(&ctx, snapshot.values.first().and_then(|item| item.value.clone()), None)),
                    });
                } else {
                    failed += 1;
                    let _ = module.rollback(&snapshot);
                    changes.push(ChangedTweak {
                        id: module.id().into(),
                        title: module.title().into(),
                        success: false,
                        message: "Windows non ha permesso di modificare questa impostazione.".into(),
                        technical: Some(module.technical(&ctx, None, None)),
                    });
                }
            }
            Err(error) => {
                failed += 1;
                changes.push(ChangedTweak {
                    id: module.id().into(),
                    title: module.title().into(),
                    success: false,
                    message: error.to_string(),
                    technical: Some(module.technical(&ctx, None, None)),
                });
            }
        }
        crate::paths::append_log(&format!("apply {} ok={}", module.id(), changes.last().map(|c| c.success).unwrap_or(false)));
    }

    let backup_id = if settings.create_backup_automatically && applied > 0 {
        let manifest = BackupManifest {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now().to_rfc3339(),
            score_before: before.score,
            kind,
            label: label.into(),
            snapshots,
        };
        save_manifest(&manifest)?.id.into()
    } else {
        None
    };

    let after = scan();
    emit(app, "done", "DONE", total, total, None);
    Ok(OptimizeResult {
        backup_id,
        applied,
        failed,
        skipped: 0,
        score_before: before.score,
        score_after: after.score,
        changes,
        finished_at: Utc::now().to_rfc3339(),
    })
}

pub fn restore(app: &AppHandle, backup_id: &str) -> AppResult<OptimizeResult> {
    let manifest = load_manifest(backup_id)?;
    let modules = all_modules();
    let before = scan();
    emit(app, "backup", "Ripristino in corso...", 0, manifest.snapshots.len() as u32, None);
    let mut changes = Vec::new();
    let mut applied = 0;
    let mut failed = 0;
    let total = manifest.snapshots.len() as u32;

    for (index, snapshot) in manifest.snapshots.iter().rev().enumerate() {
        emit(app, "windows", "Ripristino impostazioni...", index as u32, total, Some(&snapshot.id));
        if let Some(module) = modules.iter().find(|item| item.id() == snapshot.id) {
            match module.rollback(snapshot) {
                Ok(()) => {
                    applied += 1;
                    changes.push(ChangedTweak {
                        id: module.id().into(),
                        title: module.title().into(),
                        success: true,
                        message: "Ripristinato.".into(),
                        technical: None,
                    });
                }
                Err(error) => {
                    failed += 1;
                    changes.push(ChangedTweak {
                        id: snapshot.id.clone(),
                        title: snapshot.id.clone(),
                        success: false,
                        message: error.to_string(),
                        technical: None,
                    });
                }
            }
        }
    }

    let after = scan();
    emit(app, "done", "DONE", total, total, None);
    Ok(OptimizeResult {
        backup_id: Some(backup_id.into()),
        applied,
        failed,
        skipped: 0,
        score_before: before.score,
        score_after: after.score,
        changes,
        finished_at: Utc::now().to_rfc3339(),
    })
}

pub fn apply_game_mode(app: &AppHandle, preset: &str) -> AppResult<OptimizeResult> {
    if preset == "default" {
        if let Some(manifest) = latest_of_kind(BackupKind::Gamemode)? {
            return restore(app, &manifest.id);
        }
        return Err(AppError::Message("Nessuna modalità di gioco precedente da ripristinare.".into()));
    }
    let ids = game_mode_bundle(preset).iter().map(|id| (*id).to_string()).collect();
    optimize(app, Some(ids), BackupKind::Gamemode, &format!("Game Mode {preset}"))
}

fn emit(app: &AppHandle, step: &str, message: &str, current: u32, total: u32, tweak_id: Option<&str>) {
    let _ = app.emit(
        "optimize-progress",
        OptimizeEvent {
            step: step.into(),
            message: message.into(),
            current,
            total,
            tweak_id: tweak_id.map(str::to_string),
        },
    );
}

fn step_for(category: CategoryId) -> &'static str {
    match category {
        CategoryId::Gaming => "gaming",
        CategoryId::Startup => "startup",
        _ => "windows",
    }
}

fn message_for(category: CategoryId) -> &'static str {
    match category {
        CategoryId::Gaming => "Optimizing gaming...",
        CategoryId::Startup => "Optimizing startup...",
        _ => "Optimizing Windows...",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_ignores_non_applicable() {
        let tweaks = vec![TweakState {
            id: "a".into(),
            category: CategoryId::Gaming,
            title: "A".into(),
            description: String::new(),
            what: String::new(),
            why: String::new(),
            risk: crate::model::RiskLevel::Low,
            reversible: true,
            safe_mode_allowed: true,
            applicable: false,
            optimized: false,
            skipped_reason: Some("AMD only".into()),
            sources: vec![],
            technical: None,
        }];
        let result = summarize(tweaks);
        assert_eq!(result.applicable, 0);
        assert_eq!(result.score, 100);
    }
}
