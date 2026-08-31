use crate::error::AppResult;
use crate::model::{BackupKind, BackupManifest, BackupSummary};
use crate::paths::backups_dir;

pub fn save_manifest(manifest: &BackupManifest) -> AppResult<BackupSummary> {
    let dir = backups_dir()?.join(&manifest.id);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join("manifest.json"), serde_json::to_string_pretty(manifest)?)?;
    Ok(summary_of(manifest))
}

pub fn list_backups() -> AppResult<Vec<BackupSummary>> {
    let mut items = Vec::new();
    let dir = backups_dir()?;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path().join("manifest.json");
            if let Ok(raw) = std::fs::read_to_string(path) {
                if let Ok(manifest) = serde_json::from_str::<BackupManifest>(&raw) {
                    items.push(summary_of(&manifest));
                }
            }
        }
    }
    items.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(items)
}

pub fn load_manifest(id: &str) -> AppResult<BackupManifest> {
    let path = backups_dir()?.join(id).join("manifest.json");
    let raw = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

pub fn latest_of_kind(kind: BackupKind) -> AppResult<Option<BackupManifest>> {
    let mut matches = Vec::new();
    for summary in list_backups()? {
        if summary.kind == kind {
            if let Ok(manifest) = load_manifest(&summary.id) {
                matches.push(manifest);
            }
        }
    }
    Ok(matches.into_iter().next())
}

fn summary_of(manifest: &BackupManifest) -> BackupSummary {
    BackupSummary {
        id: manifest.id.clone(),
        created_at: manifest.created_at.clone(),
        change_count: manifest.snapshots.len() as u32,
        score_before: manifest.score_before,
        kind: manifest.kind.clone(),
        label: manifest.label.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TweakSnapshot;

    #[test]
    fn manifest_roundtrip_shape() {
        let manifest = BackupManifest {
            id: "test".into(),
            created_at: "2026-08-31T00:00:00Z".into(),
            score_before: 64,
            kind: BackupKind::Optimize,
            label: "Optimize".into(),
            snapshots: vec![TweakSnapshot {
                id: "game_mode".into(),
                values: vec![],
            }],
        };
        let json = serde_json::to_string(&manifest).unwrap();
        let parsed: BackupManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.snapshots.len(), 1);
        assert_eq!(summary_of(&parsed).change_count, 1);
    }
}
