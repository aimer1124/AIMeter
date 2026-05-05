use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderUsage {
    pub provider_id: String,
    pub provider_name: String,
    pub cost_used: f64,
    pub cost_limit: Option<f64>,
    pub requests_today: u64,
    pub tokens_used: u64,
    pub last_updated: String,
}

pub async fn get_all_usage() -> Result<Vec<ProviderUsage>, Box<dyn std::error::Error>> {
    // TODO: Implement actual API calls to each provider
    // For now return placeholder data to demonstrate the UI
    Ok(vec![
        ProviderUsage {
            provider_id: "claude-code".to_string(),
            provider_name: "Claude Code".to_string(),
            cost_used: 12.50,
            cost_limit: Some(100.0),
            requests_today: 42,
            tokens_used: 150_000,
            last_updated: chrono::Utc::now().to_rfc3339(),
        },
        ProviderUsage {
            provider_id: "openai-codex".to_string(),
            provider_name: "OpenAI Codex".to_string(),
            cost_used: 8.30,
            cost_limit: Some(50.0),
            requests_today: 28,
            tokens_used: 95_000,
            last_updated: chrono::Utc::now().to_rfc3339(),
        },
    ])
}
