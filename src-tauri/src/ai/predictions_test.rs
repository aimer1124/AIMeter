#[cfg(test)]
mod tests {
    use crate::ai::insights::UsageDataPoint;
    use crate::ai::predictions::*;

    fn make_history(costs: &[f64]) -> Vec<UsageDataPoint> {
        costs
            .iter()
            .enumerate()
            .map(|(i, &cost)| UsageDataPoint {
                timestamp: format!("2026-05-{:02}T00:00:00Z", i + 1),
                cost,
                tokens: (cost * 10_000.0) as u64,
                requests: (cost * 5.0) as u64,
            })
            .collect()
    }

    #[test]
    fn forecast_returns_correct_days() {
        let history = make_history(&[5.0; 14]);
        let prediction = forecast_usage(&history, None, 7);
        assert_eq!(prediction.daily_forecast.len(), 7);
    }

    #[test]
    fn estimates_monthly_cost() {
        let history = make_history(&[10.0; 14]);
        let prediction = forecast_usage(&history, None, 14);
        // ~$10/day * 30 = ~$300/month
        assert!(prediction.estimated_monthly_cost > 200.0);
        assert!(prediction.estimated_monthly_cost < 400.0);
    }

    #[test]
    fn calculates_days_until_exhausted() {
        let history = make_history(&[10.0; 7]);
        let prediction = forecast_usage(&history, Some(100.0), 14);
        // $70 spent, $10/day rate, $30 remaining = ~3 days
        assert!(prediction.days_until_budget_exhausted.is_some());
        let days = prediction.days_until_budget_exhausted.unwrap();
        assert!(days > 1.0 && days < 5.0);
    }

    #[test]
    fn empty_history_returns_zero() {
        let prediction = forecast_usage(&[], None, 14);
        assert_eq!(prediction.estimated_monthly_cost, 0.0);
        assert_eq!(prediction.daily_forecast.len(), 0);
    }

    #[test]
    fn confidence_increases_with_more_data() {
        let short = make_history(&[5.0; 3]);
        let long = make_history(&[5.0; 30]);
        let p_short = forecast_usage(&short, None, 14);
        let p_long = forecast_usage(&long, None, 14);
        assert!(p_long.confidence > p_short.confidence);
    }
}
