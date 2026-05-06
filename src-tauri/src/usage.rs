use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::providers::{self, AccountType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub provider_id: String,
    pub provider_name: String,
    pub account_type: AccountType,
    pub cost_used: f64,
    pub cost_limit: Option<f64>,
    pub quota_used: Option<f64>,
    pub quota_limit: Option<f64>,
    pub requests_today: u64,
    pub tokens_used: u64,
    pub last_updated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub async fn get_all_usage(app: &AppHandle) -> Result<Vec<ProviderUsage>, Box<dyn std::error::Error>> {
    let configs = providers::load_providers(app).await?;
    let mut results = Vec::new();

    for config in configs.iter().filter(|c| c.enabled) {
        match providers::fetch_usage(config).await {
            Ok(usage) => results.push(usage),
            Err(e) => {
                results.push(ProviderUsage {
                    provider_id: config.id.clone(),
                    provider_name: config.name.clone(),
                    account_type: config.account_type.clone(),
                    cost_used: 0.0,
                    cost_limit: config.budget_limit,
                    quota_used: None,
                    quota_limit: None,
                    requests_today: 0,
                    tokens_used: 0,
                    last_updated: chrono::Utc::now().to_rfc3339(),
                    error: Some(e.to_string()),
                });
            }
        }
    }

    Ok(results)
}
