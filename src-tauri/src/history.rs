use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::usage::ProviderUsage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailySnapshot {
    pub date: String,
    pub provider_id: String,
    pub provider_name: String,
    pub account_type: String,
    pub cost: f64,
    pub tokens: u64,
    pub requests: u64,
}

fn history_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".aimeter")
        .join("history")
}

pub fn save_snapshot(usages: &[ProviderUsage]) {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    save_snapshot_to(usages, &history_dir(), &today);
}

pub fn load_history(days: usize) -> Vec<DailySnapshot> {
    load_history_from(days, &history_dir())
}

fn save_snapshot_to(usages: &[ProviderUsage], dir: &Path, date: &str) {
    if fs::create_dir_all(dir).is_err() {
        return;
    }

    let snapshots: Vec<DailySnapshot> = usages
        .iter()
        .filter(|u| u.error.is_none())
        .map(|u| DailySnapshot {
            date: date.to_string(),
            provider_id: u.provider_id.clone(),
            provider_name: u.provider_name.clone(),
            account_type: serde_json::to_string(&u.account_type)
                .unwrap_or_else(|_| "\"api\"".to_string())
                .trim_matches('"')
                .to_string(),
            cost: u.cost_used,
            tokens: u.tokens_used,
            requests: u.requests_today,
        })
        .collect();

    if snapshots.is_empty() {
        return;
    }

    let final_path = dir.join(format!("{}.json", date));
    let tmp_path = dir.join(format!("{}.json.tmp", date));
    let json = serde_json::to_string_pretty(&snapshots).unwrap_or_default();
    if fs::write(&tmp_path, &json).is_ok() {
        let _ = fs::rename(&tmp_path, &final_path);
    }

    cleanup_old_snapshots_in(90, dir);
}

fn load_history_from(days: usize, dir: &Path) -> Vec<DailySnapshot> {
    if !dir.exists() {
        return Vec::new();
    }

    let cutoff = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(days as i64))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let mut snapshots = Vec::new();

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return snapshots,
    };

    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".json") {
            continue;
        }
        let date = name.trim_end_matches(".json");
        if date < cutoff.as_str() {
            continue;
        }

        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(daily) = serde_json::from_str::<Vec<DailySnapshot>>(&content) {
                snapshots.extend(daily);
            }
        }
    }

    snapshots.sort_by(|a, b| a.date.cmp(&b.date));
    snapshots
}

fn cleanup_old_snapshots_in(retain_days: usize, dir: &Path) {
    if !dir.exists() {
        return;
    }

    let cutoff = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(retain_days as i64))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") {
                let date = name.trim_end_matches(".json");
                if date < cutoff.as_str() {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::AccountType;

    fn make_usage(id: &str, cost: f64, error: Option<String>) -> ProviderUsage {
        ProviderUsage {
            provider_id: id.to_string(),
            provider_name: format!("Provider {}", id),
            account_type: AccountType::Api,
            cost_used: cost,
            cost_limit: None,
            quota_used: None,
            quota_limit: None,
            requests_today: 10,
            tokens_used: 5000,
            last_updated: "2026-05-06T00:00:00Z".to_string(),
            error,
        }
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let usages = vec![make_usage("test", 12.5, None)];
        save_snapshot_to(&usages, dir.path(), "2026-05-06");
        let loaded = load_history_from(30, dir.path());
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].provider_id, "test");
        assert_eq!(loaded[0].cost, 12.5);
    }

    #[test]
    fn test_save_filters_error_entries() {
        let dir = tempfile::tempdir().unwrap();
        let usages = vec![
            make_usage("ok", 5.0, None),
            make_usage("err", 0.0, Some("fail".to_string())),
        ];
        save_snapshot_to(&usages, dir.path(), "2026-05-06");
        let loaded = load_history_from(30, dir.path());
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].provider_id, "ok");
    }

    #[test]
    fn test_save_empty_creates_no_file() {
        let dir = tempfile::tempdir().unwrap();
        let usages = vec![make_usage("err", 0.0, Some("fail".to_string()))];
        save_snapshot_to(&usages, dir.path(), "2026-05-06");
        let files: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".json"))
            .collect();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_save_overwrites_same_day() {
        let dir = tempfile::tempdir().unwrap();
        save_snapshot_to(&[make_usage("a", 1.0, None)], dir.path(), "2026-05-06");
        save_snapshot_to(&[make_usage("a", 99.0, None)], dir.path(), "2026-05-06");
        let loaded = load_history_from(30, dir.path());
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].cost, 99.0);
    }

    #[test]
    fn test_save_atomic_write() {
        let dir = tempfile::tempdir().unwrap();
        save_snapshot_to(&[make_usage("a", 5.0, None)], dir.path(), "2026-05-06");
        let final_path = dir.path().join("2026-05-06.json");
        let tmp_path = dir.path().join("2026-05-06.json.tmp");
        assert!(final_path.exists());
        assert!(!tmp_path.exists());
        let content = fs::read_to_string(&final_path).unwrap();
        let parsed: Vec<DailySnapshot> = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed.len(), 1);
    }

    #[test]
    fn test_load_filters_by_date_range() {
        let dir = tempfile::tempdir().unwrap();
        save_snapshot_to(&[make_usage("old", 1.0, None)], dir.path(), "2020-01-01");
        save_snapshot_to(&[make_usage("new", 2.0, None)], dir.path(), "2026-05-06");
        let loaded = load_history_from(30, dir.path());
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].provider_id, "new");
    }

    #[test]
    fn test_load_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = load_history_from(30, dir.path());
        assert!(loaded.is_empty());
    }

    #[test]
    fn test_load_sorts_chronologically() {
        let dir = tempfile::tempdir().unwrap();
        save_snapshot_to(&[make_usage("b", 2.0, None)], dir.path(), "2026-05-05");
        save_snapshot_to(&[make_usage("a", 1.0, None)], dir.path(), "2026-05-03");
        let loaded = load_history_from(30, dir.path());
        assert_eq!(loaded[0].date, "2026-05-03");
        assert_eq!(loaded[1].date, "2026-05-05");
    }

    #[test]
    fn test_cleanup_deletes_old_files() {
        let dir = tempfile::tempdir().unwrap();
        save_snapshot_to(&[make_usage("old", 1.0, None)], dir.path(), "2020-01-01");
        save_snapshot_to(&[make_usage("new", 2.0, None)], dir.path(), "2026-05-06");
        cleanup_old_snapshots_in(90, dir.path());
        assert!(!dir.path().join("2020-01-01.json").exists());
        assert!(dir.path().join("2026-05-06.json").exists());
    }

    #[test]
    fn test_cleanup_missing_dir() {
        let dir = tempfile::tempdir().unwrap();
        let nonexistent = dir.path().join("nope");
        cleanup_old_snapshots_in(90, &nonexistent);
    }
}
