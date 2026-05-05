use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;

use super::ProviderConfig;
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
    input_tokens: u64,
    output_tokens: u64,
    cache_read_input_tokens: u64,
    cache_creation_input_tokens: u64,
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
        // Default to Sonnet pricing
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

pub async fn fetch_usage(
    config: &ProviderConfig,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    let stats_path = claude_dir().join("stats-cache.json");

    if !stats_path.exists() {
        return Ok(ProviderUsage {
            provider_id: config.id.clone(),
            provider_name: config.name.clone(),
            cost_used: 0.0,
            cost_limit: config.budget_limit,
            requests_today: 0,
            tokens_used: 0,
            last_updated: chrono::Utc::now().to_rfc3339(),
            error: Some("Claude Code stats not found. Run Claude Code at least once.".to_string()),
        });
    }

    let content = std::fs::read_to_string(&stats_path)?;
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

    Ok(ProviderUsage {
        provider_id: config.id.clone(),
        provider_name: config.name.clone(),
        cost_used: (total_cost * 100.0).round() / 100.0,
        cost_limit: config.budget_limit,
        requests_today,
        tokens_used: total_tokens,
        last_updated: chrono::Utc::now().to_rfc3339(),
        error: None,
    })
}
