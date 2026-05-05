use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub id: String,
    pub severity: InsightSeverity,
    pub title: String,
    pub description: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InsightSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageDataPoint {
    pub timestamp: String,
    pub cost: f64,
    pub tokens: u64,
    pub requests: u64,
}

pub fn analyze_spending_patterns(
    history: &[UsageDataPoint],
    budget_limit: Option<f64>,
) -> Vec<Insight> {
    let mut insights = Vec::new();

    if history.len() < 2 {
        return insights;
    }

    // Detect spending acceleration
    let recent = &history[history.len().saturating_sub(7)..];
    let older = &history[..history.len().saturating_sub(7)];

    if !recent.is_empty() && !older.is_empty() {
        let recent_avg: f64 = recent.iter().map(|d| d.cost).sum::<f64>() / recent.len() as f64;
        let older_avg: f64 = older.iter().map(|d| d.cost).sum::<f64>() / older.len() as f64;

        if older_avg > 0.0 {
            let growth_rate = (recent_avg - older_avg) / older_avg;

            if growth_rate > 0.5 {
                insights.push(Insight {
                    id: "spending_spike".to_string(),
                    severity: InsightSeverity::Warning,
                    title: "Spending acceleration detected".to_string(),
                    description: format!(
                        "Your recent daily spend is {:.0}% higher than your historical average.",
                        growth_rate * 100.0
                    ),
                    suggestion: Some(
                        "Consider reviewing which tasks are consuming the most tokens.".to_string(),
                    ),
                });
            }
        }
    }

    // Budget burn rate warning
    if let Some(limit) = budget_limit {
        let total_spent: f64 = history.iter().map(|d| d.cost).sum();
        let days_elapsed = history.len() as f64;
        let daily_rate = total_spent / days_elapsed;
        let remaining = limit - total_spent;

        if remaining > 0.0 && daily_rate > 0.0 {
            let days_until_exhausted = remaining / daily_rate;

            if days_until_exhausted < 3.0 {
                insights.push(Insight {
                    id: "budget_critical".to_string(),
                    severity: InsightSeverity::Critical,
                    title: "Budget nearly exhausted".to_string(),
                    description: format!(
                        "At current rate (${:.2}/day), budget will be exhausted in {:.1} days.",
                        daily_rate, days_until_exhausted
                    ),
                    suggestion: Some(
                        "Reduce usage or increase budget limit to avoid interruption.".to_string(),
                    ),
                });
            } else if days_until_exhausted < 7.0 {
                insights.push(Insight {
                    id: "budget_warning".to_string(),
                    severity: InsightSeverity::Warning,
                    title: "Budget running low".to_string(),
                    description: format!(
                        "Estimated {:.0} days remaining at ${:.2}/day spend rate.",
                        days_until_exhausted, daily_rate
                    ),
                    suggestion: None,
                });
            }
        }
    }

    // Detect anomalous single-day spikes
    if history.len() >= 3 {
        let costs: Vec<f64> = history.iter().map(|d| d.cost).collect();
        let mean = costs.iter().sum::<f64>() / costs.len() as f64;
        let variance =
            costs.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / costs.len() as f64;
        let std_dev = variance.sqrt();

        if let Some(last) = costs.last() {
            if *last > mean + 2.0 * std_dev && std_dev > 0.0 {
                insights.push(Insight {
                    id: "anomaly_detected".to_string(),
                    severity: InsightSeverity::Warning,
                    title: "Unusual spending spike".to_string(),
                    description: format!(
                        "Today's spend (${:.2}) is {:.1}x your daily average (${:.2}).",
                        last,
                        last / mean,
                        mean
                    ),
                    suggestion: Some(
                        "Check if a runaway process or large refactor caused this spike.".to_string(),
                    ),
                });
            }
        }
    }

    // Provider efficiency comparison
    insights
}

pub fn compare_providers(
    provider_data: &[(String, Vec<UsageDataPoint>)],
) -> Vec<Insight> {
    let mut insights = Vec::new();

    if provider_data.len() < 2 {
        return insights;
    }

    let mut cost_per_token: Vec<(String, f64)> = provider_data
        .iter()
        .filter_map(|(name, data)| {
            let total_cost: f64 = data.iter().map(|d| d.cost).sum();
            let total_tokens: u64 = data.iter().map(|d| d.tokens).sum();
            if total_tokens > 0 {
                Some((name.clone(), total_cost / total_tokens as f64 * 1_000_000.0))
            } else {
                None
            }
        })
        .collect();

    cost_per_token.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    if cost_per_token.len() >= 2 {
        let cheapest = &cost_per_token[0];
        let most_expensive = &cost_per_token[cost_per_token.len() - 1];

        if most_expensive.1 > cheapest.1 * 2.0 {
            insights.push(Insight {
                id: "provider_efficiency".to_string(),
                severity: InsightSeverity::Info,
                title: "Cost efficiency varies across providers".to_string(),
                description: format!(
                    "{} costs ${:.4}/1M tokens vs {} at ${:.4}/1M tokens.",
                    cheapest.0, cheapest.1, most_expensive.0, most_expensive.1
                ),
                suggestion: Some(format!(
                    "Consider shifting more workload to {} for cost savings.",
                    cheapest.0
                )),
            });
        }
    }

    insights
}
