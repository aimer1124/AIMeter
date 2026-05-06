use std::collections::HashSet;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

use crate::usage::ProviderUsage;

pub struct NotificationState {
    sent: HashSet<(String, u8)>,
}

fn evaluate_threshold(pct: f64) -> Option<u8> {
    if pct >= 100.0 {
        Some(100)
    } else if pct >= 90.0 {
        Some(90)
    } else if pct >= 70.0 {
        Some(70)
    } else {
        None
    }
}

fn compute_pending_notifications(
    sent: &HashSet<(String, u8)>,
    usages: &[ProviderUsage],
) -> Vec<(String, u8, String, f64)> {
    let mut pending = Vec::new();
    for usage in usages {
        let Some(limit) = usage.cost_limit else {
            continue;
        };
        if limit <= 0.0 {
            continue;
        }
        let pct = (usage.cost_used / limit) * 100.0;
        if let Some(key) = evaluate_threshold(pct) {
            let notif_key = (usage.provider_id.clone(), key);
            if !sent.contains(&notif_key) {
                let severity = match key {
                    100 => "exceeded",
                    90 => "critical",
                    _ => "warning",
                };
                pending.push((usage.provider_name.clone(), key, severity.to_string(), pct));
            }
        }
    }
    pending
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            sent: HashSet::new(),
        }
    }

    pub fn check_and_notify(&mut self, app: &AppHandle, usages: &[ProviderUsage]) {
        let pending = compute_pending_notifications(&self.sent, usages);
        for (provider_name, key, severity, pct) in pending {
            let provider_id = usages
                .iter()
                .find(|u| u.provider_name == provider_name)
                .map(|u| u.provider_id.clone())
                .unwrap_or_default();
            self.sent.insert((provider_id, key));
            send_budget_notification(app, &provider_name, pct, &severity);
        }
    }
}

fn send_budget_notification(app: &AppHandle, provider: &str, pct: f64, severity: &str) {
    let title = format!("AIMeter: {} Budget Alert", provider);
    let body = match severity {
        "exceeded" => format!(
            "{} has exceeded its budget ({:.0}% used)!",
            provider, pct
        ),
        "critical" => format!(
            "{} is at {:.0}% of budget — approaching limit",
            provider, pct
        ),
        _ => format!("{} has reached {:.0}% of budget", provider, pct),
    };
    let _ = app.notification().builder().title(&title).body(&body).show();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::AccountType;

    fn make_usage(cost: f64, limit: Option<f64>) -> ProviderUsage {
        ProviderUsage {
            provider_id: "test".to_string(),
            provider_name: "Test".to_string(),
            account_type: AccountType::Api,
            cost_used: cost,
            cost_limit: limit,
            quota_used: None,
            quota_limit: None,
            requests_today: 0,
            tokens_used: 0,
            last_updated: String::new(),
            error: None,
        }
    }

    #[test]
    fn test_below_70_no_alert() {
        assert_eq!(evaluate_threshold(50.0), None);
    }

    #[test]
    fn test_at_70_warning() {
        assert_eq!(evaluate_threshold(75.0), Some(70));
    }

    #[test]
    fn test_at_90_critical() {
        assert_eq!(evaluate_threshold(92.0), Some(90));
    }

    #[test]
    fn test_at_100_exceeded() {
        assert_eq!(evaluate_threshold(105.0), Some(100));
    }

    #[test]
    fn test_highest_threshold_wins() {
        assert_eq!(evaluate_threshold(95.0), Some(90));
        assert_ne!(evaluate_threshold(95.0), Some(70));
    }

    #[test]
    fn test_deduplication() {
        let usages = vec![make_usage(80.0, Some(100.0))];
        let mut sent = HashSet::new();
        let first = compute_pending_notifications(&sent, &usages);
        assert_eq!(first.len(), 1);
        sent.insert(("test".to_string(), first[0].1));
        let second = compute_pending_notifications(&sent, &usages);
        assert_eq!(second.len(), 0);
    }

    #[test]
    fn test_no_limit_skipped() {
        let usages = vec![make_usage(80.0, None)];
        let sent = HashSet::new();
        let pending = compute_pending_notifications(&sent, &usages);
        assert!(pending.is_empty());
    }

    #[test]
    fn test_zero_limit_skipped() {
        let usages = vec![make_usage(80.0, Some(0.0))];
        let sent = HashSet::new();
        let pending = compute_pending_notifications(&sent, &usages);
        assert!(pending.is_empty());
    }
}
