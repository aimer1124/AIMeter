use serde::Deserialize;

use super::ProviderConfig;
use crate::usage::ProviderUsage;

#[derive(Deserialize)]
struct UsageResponse {
    data: Vec<UsageBucket>,
}

#[derive(Deserialize)]
struct UsageBucket {
    results: Vec<UsageResult>,
}

#[derive(Deserialize)]
struct UsageResult {
    #[serde(default)]
    input_tokens: u64,
    #[serde(default)]
    output_tokens: u64,
    #[serde(default)]
    num_model_requests: u64,
    #[serde(default)]
    cost_in_major: f64,
}

pub async fn fetch_usage(
    config: &ProviderConfig,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    let api_key = match &config.api_key {
        Some(key) if !key.is_empty() => key,
        _ => {
            return Ok(ProviderUsage {
                provider_id: config.id.clone(),
                provider_name: config.name.clone(),
                cost_used: 0.0,
                cost_limit: config.budget_limit,
                requests_today: 0,
                tokens_used: 0,
                last_updated: chrono::Utc::now().to_rfc3339(),
                error: Some("API key required. Use an Admin API key with usage.read scope.".to_string()),
            });
        }
    };

    let client = reqwest::Client::new();
    let now = chrono::Utc::now();
    let start_of_day = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let response = client
        .get("https://api.openai.com/v1/organization/usage/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .query(&[
            ("start_time", start_of_day.to_string()),
            ("end_time", now.timestamp().to_string()),
        ])
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Ok(ProviderUsage {
            provider_id: config.id.clone(),
            provider_name: config.name.clone(),
            cost_used: 0.0,
            cost_limit: config.budget_limit,
            requests_today: 0,
            tokens_used: 0,
            last_updated: chrono::Utc::now().to_rfc3339(),
            error: Some(format!("OpenAI API error ({}): {}", status, body)),
        });
    }

    let data: UsageResponse = response.json().await?;

    let mut total_cost = 0.0;
    let mut total_tokens: u64 = 0;
    let mut total_requests: u64 = 0;

    for bucket in &data.data {
        for result in &bucket.results {
            total_cost += result.cost_in_major;
            total_tokens += result.input_tokens + result.output_tokens;
            total_requests += result.num_model_requests;
        }
    }

    Ok(ProviderUsage {
        provider_id: config.id.clone(),
        provider_name: config.name.clone(),
        cost_used: (total_cost * 100.0).round() / 100.0,
        cost_limit: config.budget_limit,
        requests_today: total_requests,
        tokens_used: total_tokens,
        last_updated: chrono::Utc::now().to_rfc3339(),
        error: None,
    })
}
