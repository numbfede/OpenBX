use serde::{Deserialize, Serialize};

const CURRENT: &str = env!("CARGO_PKG_VERSION");
const RELEASES_URL: &str = "https://api.github.com/repos/numbfede/OpenBX/releases/latest";
const RELEASES_PAGE: &str = "https://github.com/numbfede/OpenBX/releases/latest";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_url: Option<String>,
    pub available: bool,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    draft: Option<bool>,
    prerelease: Option<bool>,
}

pub fn check() -> UpdateInfo {
    let current_version = CURRENT.to_string();
    match fetch_latest() {
        Some(release) if release.draft.unwrap_or(false) || release.prerelease.unwrap_or(false) => {
            UpdateInfo {
                current_version,
                latest_version: None,
                release_url: None,
                available: false,
            }
        }
        Some(release) => {
            let latest = release.tag_name.trim().trim_start_matches('v').to_string();
            let available = is_newer(&latest, CURRENT);
            UpdateInfo {
                current_version,
                latest_version: Some(latest),
                release_url: Some(if available {
                    release.html_url
                } else {
                    RELEASES_PAGE.into()
                }),
                available,
            }
        }
        None => UpdateInfo {
            current_version,
            latest_version: None,
            release_url: Some(RELEASES_PAGE.into()),
            available: false,
        },
    }
}

fn fetch_latest() -> Option<GithubRelease> {
    let body = ureq::get(RELEASES_URL)
        .set("User-Agent", "OpenBX")
        .set("Accept", "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(4))
        .call()
        .ok()?
        .into_string()
        .ok()?;
    serde_json::from_str(&body).ok()
}

fn is_newer(latest: &str, current: &str) -> bool {
    version_tuple(latest) > version_tuple(current)
}

fn version_tuple(value: &str) -> (u64, u64, u64) {
    let mut parts = value.split('.').filter_map(|part| {
        part.chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>()
            .parse::<u64>()
            .ok()
    });
    (
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
        parts.next().unwrap_or(0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newer_version_is_detected() {
        assert!(is_newer("0.1.1", "0.1.0"));
        assert!(is_newer("0.2.0", "0.1.9"));
        assert!(!is_newer("0.1.0", "0.1.1"));
        assert!(!is_newer("0.1.1", "0.1.1"));
    }
}
