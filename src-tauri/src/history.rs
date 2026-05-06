use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::usage::ProviderUsage;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn snapshot_path(date: &str) -> PathBuf {
    history_dir().join(format!("{}.json", date))
}

pub fn save_snapshot(usages: &[ProviderUsage]) {
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let dir = history_dir();
    if fs::create_dir_all(&dir).is_err() {
        return;
    }

    let snapshots: Vec<DailySnapshot> = usages
        .iter()
        .filter(|u| u.error.is_none())
        .map(|u| DailySnapshot {
            date: today.clone(),
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

    let path = snapshot_path(&today);
    let _ = fs::write(&path, serde_json::to_string_pretty(&snapshots).unwrap_or_default());

    cleanup_old_snapshots(90);
}

pub fn load_history(days: usize) -> Vec<DailySnapshot> {
    let dir = history_dir();
    if !dir.exists() {
        return Vec::new();
    }

    let cutoff = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(days as i64))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let mut snapshots = Vec::new();

    let entries = match fs::read_dir(&dir) {
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

fn cleanup_old_snapshots(retain_days: usize) {
    let dir = history_dir();
    if !dir.exists() {
        return;
    }

    let cutoff = chrono::Utc::now()
        .checked_sub_signed(chrono::Duration::days(retain_days as i64))
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    if let Ok(entries) = fs::read_dir(&dir) {
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
