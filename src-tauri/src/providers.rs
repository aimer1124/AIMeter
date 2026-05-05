use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    pub name: String,
    pub provider_type: ProviderType,
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

fn default_providers() -> Vec<ProviderConfig> {
    vec![
        ProviderConfig {
            id: "claude-code".to_string(),
            name: "Claude Code".to_string(),
            provider_type: ProviderType::ClaudeCode,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
        ProviderConfig {
            id: "openai-codex".to_string(),
            name: "OpenAI Codex".to_string(),
            provider_type: ProviderType::OpenaiCodex,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
        ProviderConfig {
            id: "github-copilot".to_string(),
            name: "GitHub Copilot".to_string(),
            provider_type: ProviderType::GithubCopilot,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
        ProviderConfig {
            id: "cursor".to_string(),
            name: "Cursor".to_string(),
            provider_type: ProviderType::Cursor,
            api_key: None,
            enabled: false,
            budget_limit: None,
        },
    ]
}
