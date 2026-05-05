use serde::{Deserialize, Serialize};

use super::insights::UsageDataPoint;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub provider_id: String,
    pub daily_forecast: Vec<ForecastPoint>,
    pub estimated_monthly_cost: f64,
    pub days_until_budget_exhausted: Option<f64>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    pub date: String,
    pub predicted_cost: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

pub fn forecast_usage(
    history: &[UsageDataPoint],
    budget_limit: Option<f64>,
    forecast_days: usize,
) -> Prediction {
    let costs: Vec<f64> = history.iter().map(|d| d.cost).collect();

    if costs.is_empty() {
        return Prediction {
            provider_id: String::new(),
            daily_forecast: Vec::new(),
            estimated_monthly_cost: 0.0,
            days_until_budget_exhausted: None,
            confidence: 0.0,
        };
    }

    // Exponential moving average for trend detection
    let alpha = 0.3;
    let mut ema = costs[0];
    for &cost in &costs[1..] {
        ema = alpha * cost + (1.0 - alpha) * ema;
    }

    // Calculate volatility for confidence bounds
    let mean = costs.iter().sum::<f64>() / costs.len() as f64;
    let variance = costs.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / costs.len() as f64;
    let std_dev = variance.sqrt();

    // Linear trend from recent data
    let trend = if costs.len() >= 7 {
        let recent_mean: f64 = costs[costs.len() - 3..].iter().sum::<f64>() / 3.0;
        let older_mean: f64 = costs[costs.len() - 7..costs.len() - 3].iter().sum::<f64>() / 4.0;
        (recent_mean - older_mean) / 4.0
    } else {
        0.0
    };

    // Generate forecast
    let mut forecast = Vec::new();
    for day in 1..=forecast_days {
        let predicted = (ema + trend * day as f64).max(0.0);
        let uncertainty = std_dev * (day as f64).sqrt();

        forecast.push(ForecastPoint {
            date: format!("+{}d", day),
            predicted_cost: predicted,
            lower_bound: (predicted - uncertainty).max(0.0),
            upper_bound: predicted + uncertainty,
        });
    }

    // Estimate monthly cost
    let estimated_monthly = ema * 30.0 + trend * 30.0 * 15.0; // trend averaged over month

    // Days until budget exhausted
    let days_until_exhausted = budget_limit.and_then(|limit| {
        let spent: f64 = costs.iter().sum();
        let remaining = limit - spent;
        if remaining <= 0.0 {
            Some(0.0)
        } else if ema > 0.0 {
            Some(remaining / ema)
        } else {
            None
        }
    });

    // Confidence based on data availability and volatility
    let data_confidence = (history.len() as f64 / 30.0).min(1.0);
    let stability_confidence = if mean > 0.0 {
        1.0 - (std_dev / mean).min(1.0)
    } else {
        0.0
    };
    let confidence = (data_confidence * 0.4 + stability_confidence * 0.6).clamp(0.0, 1.0);

    Prediction {
        provider_id: String::new(),
        daily_forecast: forecast,
        estimated_monthly_cost: estimated_monthly.max(0.0),
        days_until_budget_exhausted: days_until_exhausted,
        confidence,
    }
}
