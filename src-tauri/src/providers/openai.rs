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

fn empty_usage(config: &ProviderConfig, error: String) -> ProviderUsage {
    ProviderUsage {
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
        error: Some(error),
    }
}

async fn fetch_usage_with_base_url(
    config: &ProviderConfig,
    base_url: &str,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    let api_key = match &config.api_key {
        Some(key) if !key.is_empty() => key,
        _ => {
            return Ok(empty_usage(
                config,
                "API key required. Use an Admin API key with usage.read scope.".to_string(),
            ));
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
        .get(format!(
            "{}/v1/organization/usage/completions",
            base_url
        ))
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
        return Ok(empty_usage(
            config,
            format!("OpenAI API error ({}): {}", status, body),
        ));
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
        account_type: config.account_type.clone(),
        cost_used: (total_cost * 100.0).round() / 100.0,
        cost_limit: config.budget_limit,
        quota_used: None,
        quota_limit: None,
        requests_today: total_requests,
        tokens_used: total_tokens,
        last_updated: chrono::Utc::now().to_rfc3339(),
        error: None,
    })
}

pub async fn fetch_usage(
    config: &ProviderConfig,
) -> Result<ProviderUsage, Box<dyn std::error::Error>> {
    fetch_usage_with_base_url(config, "https://api.openai.com").await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::{AccountType, ProviderType};

    fn make_config(api_key: Option<&str>) -> ProviderConfig {
        ProviderConfig {
            id: "openai".to_string(),
            name: "OpenAI".to_string(),
            provider_type: ProviderType::OpenaiCodex,
            account_type: AccountType::Api,
            api_key: api_key.map(|s| s.to_string()),
            enabled: true,
            budget_limit: Some(50.0),
        }
    }

    #[tokio::test]
    async fn test_missing_api_key() {
        let config = make_config(None);
        let result = fetch_usage_with_base_url(&config, "http://unused").await.unwrap();
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("API key required"));
    }

    #[tokio::test]
    async fn test_empty_api_key() {
        let config = make_config(Some(""));
        let result = fetch_usage_with_base_url(&config, "http://unused").await.unwrap();
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_success_response() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .match_header("Authorization", "Bearer test-key")
            .with_status(200)
            .with_body(r#"{"data":[{"results":[{"input_tokens":1000,"output_tokens":500,"num_model_requests":5,"cost_in_major":0.05}]}]}"#)
            .create_async()
            .await;

        let config = make_config(Some("test-key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert!(result.error.is_none());
        assert_eq!(result.cost_used, 0.05);
        assert_eq!(result.tokens_used, 1500);
        assert_eq!(result.requests_today, 5);
    }

    #[tokio::test]
    async fn test_success_multiple_buckets() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .with_status(200)
            .with_body(r#"{"data":[{"results":[{"cost_in_major":0.10,"input_tokens":100,"output_tokens":50,"num_model_requests":2}]},{"results":[{"cost_in_major":0.20,"input_tokens":200,"output_tokens":100,"num_model_requests":3}]}]}"#)
            .create_async()
            .await;

        let config = make_config(Some("key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.cost_used, 0.30);
        assert_eq!(result.tokens_used, 450);
        assert_eq!(result.requests_today, 5);
    }

    #[tokio::test]
    async fn test_empty_data_response() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .with_status(200)
            .with_body(r#"{"data":[]}"#)
            .create_async()
            .await;

        let config = make_config(Some("key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.cost_used, 0.0);
        assert_eq!(result.tokens_used, 0);
    }

    #[tokio::test]
    async fn test_api_error_401() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .with_status(401)
            .with_body("Unauthorized")
            .create_async()
            .await;

        let config = make_config(Some("bad-key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert!(result.error.unwrap().contains("401"));
    }

    #[tokio::test]
    async fn test_api_error_500() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let config = make_config(Some("key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert!(result.error.unwrap().contains("500"));
    }

    #[tokio::test]
    async fn test_cost_rounding() {
        let mut server = mockito::Server::new_async().await;
        let mock = server
            .mock("GET", mockito::Matcher::Regex(r"^/v1/organization/usage/completions.*$".to_string()))
            .with_status(200)
            .with_body(r#"{"data":[{"results":[{"cost_in_major":0.1249999,"input_tokens":0,"output_tokens":0,"num_model_requests":0}]}]}"#)
            .create_async()
            .await;

        let config = make_config(Some("key"));
        let result = fetch_usage_with_base_url(&config, &server.url()).await.unwrap();
        mock.assert_async().await;
        assert_eq!(result.cost_used, 0.12);
    }
}
