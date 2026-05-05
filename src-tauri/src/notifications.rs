use std::collections::HashSet;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

use crate::usage::ProviderUsage;

pub struct NotificationState {
    sent: HashSet<(String, u8)>,
}

impl NotificationState {
    pub fn new() -> Self {
        Self {
            sent: HashSet::new(),
        }
    }

    pub fn check_and_notify(&mut self, app: &AppHandle, usages: &[ProviderUsage]) {
        for usage in usages {
            let Some(limit) = usage.cost_limit else {
                continue;
            };
            if limit <= 0.0 {
                continue;
            }

            let pct = (usage.cost_used / limit) * 100.0;
            let thresholds: [(f64, u8, &str); 3] = [
                (100.0, 100, "exceeded"),
                (90.0, 90, "critical"),
                (70.0, 70, "warning"),
            ];

            for (threshold_pct, key, severity) in thresholds {
                if pct >= threshold_pct {
                    let notif_key = (usage.provider_id.clone(), key);
                    if !self.sent.contains(&notif_key) {
                        self.sent.insert(notif_key);
                        send_budget_notification(app, &usage.provider_name, pct, severity);
                    }
                    break;
                }
            }
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
