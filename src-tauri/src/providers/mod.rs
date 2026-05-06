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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_providers_count() {
        assert_eq!(default_providers().len(), 4);
    }

    #[test]
    fn test_default_providers_claude_enabled() {
        let providers = default_providers();
        let claude = providers.iter().find(|p| p.id == "claude-code").unwrap();
        assert!(claude.enabled);
    }

    #[test]
    fn test_default_providers_others_disabled() {
        let providers = default_providers();
        for p in providers.iter().filter(|p| p.id != "claude-code") {
            assert!(!p.enabled, "{} should be disabled by default", p.id);
        }
    }

    #[test]
    fn test_default_account_type_is_api() {
        for p in default_providers() {
            assert!(matches!(p.account_type, AccountType::Api));
        }
    }

    #[tokio::test]
    async fn test_unsupported_provider_returns_error() {
        let config = ProviderConfig {
            id: "copilot".to_string(),
            name: "Copilot".to_string(),
            provider_type: ProviderType::GithubCopilot,
            account_type: AccountType::Api,
            api_key: None,
            enabled: true,
            budget_limit: None,
        };
        let result = fetch_usage(&config).await.unwrap();
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("not yet support"));
    }

    #[test]
    fn test_provider_config_serde_roundtrip() {
        let config = ProviderConfig {
            id: "test".to_string(),
            name: "Test".to_string(),
            provider_type: ProviderType::ClaudeCode,
            account_type: AccountType::Max100,
            api_key: Some("secret".to_string()),
            enabled: true,
            budget_limit: Some(99.99),
        };
        let json = serde_json::to_string(&config).unwrap();
        let parsed: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.id, "test");
        assert_eq!(parsed.budget_limit, Some(99.99));
        assert!(matches!(parsed.account_type, AccountType::Max100));
    }

    #[test]
    fn test_account_type_serde_names() {
        assert_eq!(serde_json::to_string(&AccountType::Api).unwrap(), "\"api\"");
        assert_eq!(serde_json::to_string(&AccountType::Pro).unwrap(), "\"pro\"");
        assert_eq!(serde_json::to_string(&AccountType::Max100).unwrap(), "\"max100\"");
        assert_eq!(serde_json::to_string(&AccountType::Max200).unwrap(), "\"max200\"");
    }

    #[test]
    fn test_provider_type_serde_names() {
        assert_eq!(serde_json::to_string(&ProviderType::ClaudeCode).unwrap(), "\"claude_code\"");
        assert_eq!(serde_json::to_string(&ProviderType::OpenaiCodex).unwrap(), "\"openai_codex\"");
        assert_eq!(serde_json::to_string(&ProviderType::GithubCopilot).unwrap(), "\"github_copilot\"");
        assert_eq!(serde_json::to_string(&ProviderType::Cursor).unwrap(), "\"cursor\"");
        assert_eq!(serde_json::to_string(&ProviderType::Custom).unwrap(), "\"custom\"");
    }
}
