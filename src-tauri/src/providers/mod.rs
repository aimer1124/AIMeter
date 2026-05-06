pub mod claude_code;
pub mod openai;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use crate::usage::ProviderUsage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
    pub account_type: AccountType,
    pub api_key: Option<String>,
    pub enabled: bool,
    pub budget_limit: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    ClaudeCode,
    OpenaiCodex,
    GithubCopilot,
    Cursor,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Api,
    Pro,
    Max100,
    Max200,
}

pub async fn load_providers(app: &AppHandle) -> Result<Vec<ProviderConfig>, Box<dyn std::error::Error>> {
    let store = app.store("config.json")?;
    let providers = store.get("providers");
    match providers {
        Some(value) => Ok(serde_json::from_value(value)?),
        None => Ok(default_providers()),
    }
}

pub async fn save_providers(
    app: &AppHandle,
    providers: Vec<ProviderConfig>,
) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("config.json")?;
    store.set("providers", serde_json::to_value(providers)?);
    store.save()?;
    Ok(())
}

pub async fn fetch_usage(config: &ProviderConfig) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    match config.provider_type {
        ProviderType::ClaudeCode => claude_code::fetch_usage(config).await,
        ProviderType::OpenaiCodex => openai::fetch_usage(config).await,
        _ => Ok(ProviderUsage {
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
            error: Some("This provider does not yet support automatic usage tracking.".to_string()),
        }),
    }
}

fn default_providers() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig {
            id: "claude-code".to_string(),
            name: "Claude Code".to_string(),
            provider_type: ProviderType::ClaudeCode,
            account_type: AccountType::Api,
            api_key: None,
            enabled: true,
            budget_limit: None,
        },
        ProviderConfig {
            id: "openai-codex".to_string(),
            name: "OpenAI Codex".to_string(),
            provider_type: ProviderType::OpenaiCodex,
            account_type: AccountType::Api,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
        ProviderConfig {
            id: "github-copilot".to_string(),
            name: "GitHub Copilot".to_string(),
            provider_type: ProviderType::GithubCopilot,
            account_type: AccountType::Api,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
        ProviderConfig {
            id: "cursor".to_string(),
            name: "Cursor".to_string(),
            provider_type: ProviderType::Cursor,
            account_type: AccountType::Api,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
    ]
}
