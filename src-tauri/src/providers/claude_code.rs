use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::{AccountType, ProviderConfig};
use crate::usage::ProviderUsage;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatsCache {
    daily_activity: Vec<DailyActivity>,
    #[allow(dead_code)]
    daily_model_tokens: Vec<DailyModelTokens>,
    model_usage: HashMap<String, ModelTokenUsage>,
    #[allow(dead_code)]
    total_messages: u64,
    #[allow(dead_code)]
    total_sessions: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DailyActivity {
    date: String,
    message_count: u64,
    #[allow(dead_code)]
    session_count: u64,
    #[allow(dead_code)]
    tool_call_count: u64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct DailyModelTokens {
    #[allow(dead_code)]
    date: String,
    #[allow(dead_code)]
    tokens_by_model: HashMap<String, u64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ModelTokenUsage {
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    cache_read_input_tokens: u64,
    #[serde(default)]
    cache_creation_input_tokens: u64,
    #[serde(default)]
    #[allow(dead_code)]
    cost_usd: f64,
}

struct ModelPricing {
    input_per_mtok: f64,
    output_per_mtok: f64,
    cache_read_per_mtok: f64,
    cache_write_per_mtok: f64,
}

fn get_pricing(model: &str) -> ModelPricing {
    let model_lower = model.to_lowercase();
    if model_lower.contains("opus") {
        ModelPricing {
            input_per_mtok: 15.0,
            output_per_mtok: 75.0,
            cache_read_per_mtok: 1.50,
            cache_write_per_mtok: 3.75,
        }
    } else if model_lower.contains("haiku") {
        ModelPricing {
            input_per_mtok: 0.80,
            output_per_mtok: 4.0,
            cache_read_per_mtok: 0.08,
            cache_write_per_mtok: 0.20,
        }
    } else {
        ModelPricing {
            input_per_mtok: 3.0,
            output_per_mtok: 15.0,
            cache_read_per_mtok: 0.30,
            cache_write_per_mtok: 0.75,
        }
    }
}

fn calculate_model_cost(usage: &ModelTokenUsage, pricing: &ModelPricing) -> f64 {
    let mtok = 1_000_000.0;
    (usage.input_tokens as f64 / mtok) * pricing.input_per_mtok
        + (usage.output_tokens as f64 / mtok) * pricing.output_per_mtok
        + (usage.cache_read_input_tokens as f64 / mtok) * pricing.cache_read_per_mtok
        + (usage.cache_creation_input_tokens as f64 / mtok) * pricing.cache_write_per_mtok
}

fn claude_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".claude")
}

fn compute_usage_from_path(
    config: &ProviderConfig,
    stats_path: &Path,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    if !stats_path.exists() {
        return Ok(ProviderUsage {
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
            error: Some("Claude Code stats not found. Run Claude Code at least once.".to_string()),
        });
    }

    let content = std::fs::read_to_string(stats_path)?;
    let stats: StatsCache = serde_json::from_str(&content)?;

    let mut total_cost = 0.0;
    let mut total_tokens: u64 = 0;

    for (model, usage) in &stats.model_usage {
        let pricing = get_pricing(model);
        total_cost += calculate_model_cost(usage, &pricing);
        total_tokens += usage.input_tokens
            + usage.output_tokens
            + usage.cache_read_input_tokens
            + usage.cache_creation_input_tokens;
    }

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let requests_today = stats
        .daily_activity
        .iter()
        .find(|d| d.date == today)
        .map(|d| d.message_count)
        .unwrap_or(0);

    let (quota_used, quota_limit) = match config.account_type {
        AccountType::Pro => Some((total_tokens as f64, 45_000_000.0)),
        AccountType::Max100 => Some((total_tokens as f64, 225_000_000.0)),
        AccountType::Max200 => Some((total_tokens as f64, 450_000_000.0)),
        AccountType::Api => None,
    }
    .map_or((None, None), |(u, l)| (Some(u), Some(l)));

    Ok(ProviderUsage {
        provider_id: config.id.clone(),
        provider_name: config.name.clone(),
        account_type: config.account_type.clone(),
        cost_used: (total_cost * 100.0).round() / 100.0,
        cost_limit: config.budget_limit,
        quota_used,
        quota_limit,
        requests_today,
        tokens_used: total_tokens,
        last_updated: chrono::Utc::now().to_rfc3339(),
        error: None,
    })
}

pub async fn fetch_usage(
    config: &ProviderConfig,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    compute_usage_from_path(config, &claude_dir().join("stats-cache.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_config(account_type: AccountType) -> ProviderConfig {
        ProviderConfig {
            id: "claude-code".to_string(),
            name: "Claude Code".to_string(),
            provider_type: super::super::ProviderType::ClaudeCode,
            account_type,
            api_key: None,
            enabled: true,
            budget_limit: Some(100.0),
        }
    }

    fn sample_stats_json() -> &'static str {
        r#"{
            "version": 1,
            "lastComputedDate": "2026-05-06",
            "dailyActivity": [
                {"date": "2026-05-06", "messageCount": 42, "sessionCount": 3, "toolCallCount": 10}
            ],
            "dailyModelTokens": [],
            "modelUsage": {
                "claude-sonnet-4-6": {
                    "inputTokens": 1000000,
                    "outputTokens": 500000,
                    "cacheReadInputTokens": 2000000,
                    "cacheCreationInputTokens": 100000,
                    "costUsd": 0
                }
            },
            "totalSessions": 5,
            "totalMessages": 100
        }"#
    }

    #[test]
    fn test_get_pricing_opus() {
        let p = get_pricing("claude-opus-4-6");
        assert_eq!(p.input_per_mtok, 15.0);
        assert_eq!(p.output_per_mtok, 75.0);
    }

    #[test]
    fn test_get_pricing_haiku() {
        let p = get_pricing("claude-haiku-4-5-20251001");
        assert_eq!(p.input_per_mtok, 0.80);
        assert_eq!(p.output_per_mtok, 4.0);
    }

    #[test]
    fn test_get_pricing_sonnet_default() {
        let p = get_pricing("claude-sonnet-4-6");
        assert_eq!(p.input_per_mtok, 3.0);
        let p2 = get_pricing("unknown-model");
        assert_eq!(p2.input_per_mtok, 3.0);
    }

    #[test]
    fn test_get_pricing_case_insensitive() {
        let p = get_pricing("CLAUDE-OPUS-4");
        assert_eq!(p.input_per_mtok, 15.0);
    }

    #[test]
    fn test_calculate_cost_basic() {
        let usage = ModelTokenUsage {
            input_tokens: 1_000_000,
            output_tokens: 1_000_000,
            cache_read_input_tokens: 0,
            cache_creation_input_tokens: 0,
            cost_usd: 0.0,
        };
        let pricing = get_pricing("claude-sonnet-4-6");
        let cost = calculate_model_cost(&usage, &pricing);
        assert!((cost - 18.0).abs() < 0.001); // 3 + 15
    }

    #[test]
    fn test_calculate_cost_zero_tokens() {
        let usage = ModelTokenUsage {
            input_tokens: 0,
            output_tokens: 0,
            cache_read_input_tokens: 0,
            cache_creation_input_tokens: 0,
            cost_usd: 0.0,
        };
        let pricing = get_pricing("claude-sonnet-4-6");
        assert_eq!(calculate_model_cost(&usage, &pricing), 0.0);
    }

    #[test]
    fn test_calculate_cost_cache_tokens() {
        let usage = ModelTokenUsage {
            input_tokens: 0,
            output_tokens: 0,
            cache_read_input_tokens: 10_000_000,
            cache_creation_input_tokens: 1_000_000,
            cost_usd: 0.0,
        };
        let pricing = get_pricing("claude-sonnet-4-6");
        let cost = calculate_model_cost(&usage, &pricing);
        // 10M * 0.30/M + 1M * 0.75/M = 3.0 + 0.75 = 3.75
        assert!((cost - 3.75).abs() < 0.001);
    }

    #[test]
    fn test_serde_missing_cost_usd() {
        let json = r#"{"inputTokens": 100, "outputTokens": 200}"#;
        let parsed: ModelTokenUsage = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.input_tokens, 100);
        assert_eq!(parsed.output_tokens, 200);
        assert_eq!(parsed.cost_usd, 0.0);
    }

    #[test]
    fn test_serde_missing_all_optional() {
        let json = r#"{}"#;
        let parsed: ModelTokenUsage = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.input_tokens, 0);
        assert_eq!(parsed.output_tokens, 0);
        assert_eq!(parsed.cache_read_input_tokens, 0);
    }

    #[test]
    fn test_serde_malformed_json() {
        let result = serde_json::from_str::<StatsCache>("not json at all");
        assert!(result.is_err());
    }

    #[test]
    fn test_serde_missing_top_level_fields() {
        let json = r#"{"version": 1}"#;
        let result = serde_json::from_str::<StatsCache>(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_from_valid_stats() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("stats-cache.json");
        std::fs::write(&path, sample_stats_json()).unwrap();

        let config = make_config(AccountType::Api);
        let result = compute_usage_from_path(&config, &path).unwrap();
        assert!(result.cost_used > 0.0);
        assert!(result.tokens_used > 0);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_fetch_missing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nonexistent.json");

        let config = make_config(AccountType::Api);
        let result = compute_usage_from_path(&config, &path).unwrap();
        assert!(result.error.is_some());
        assert_eq!(result.cost_used, 0.0);
    }

    #[test]
    fn test_quota_mapping_by_account_type() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("stats-cache.json");
        std::fs::write(&path, sample_stats_json()).unwrap();

        let api = compute_usage_from_path(&make_config(AccountType::Api), &path).unwrap();
        assert!(api.quota_used.is_none());
        assert!(api.quota_limit.is_none());

        let pro = compute_usage_from_path(&make_config(AccountType::Pro), &path).unwrap();
        assert!(pro.quota_used.is_some());
        assert_eq!(pro.quota_limit, Some(45_000_000.0));

        let max100 = compute_usage_from_path(&make_config(AccountType::Max100), &path).unwrap();
        assert_eq!(max100.quota_limit, Some(225_000_000.0));

        let max200 = compute_usage_from_path(&make_config(AccountType::Max200), &path).unwrap();
        assert_eq!(max200.quota_limit, Some(450_000_000.0));
    }
}
