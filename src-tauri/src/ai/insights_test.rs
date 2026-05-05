#[cfg(test)]
mod tests {
    use crate::ai::insights::*;

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
    fn detects_spending_spike() {
        // 10 days of low spend, then 7 days of high spend
        let mut costs: Vec<f64> = vec![1.0; 10];
        costs.extend(vec![5.0; 7]);
        let insights = analyze_spending_patterns(&make_history(&costs), None);
        assert!(insights.iter().any(|i| i.id == "spending_spike"));
    }

    #[test]
    fn detects_budget_critical() {
        let costs = vec![10.0; 10]; // $100 spent, $10/day
        let insights = analyze_spending_patterns(&make_history(&costs), Some(120.0));
        assert!(insights.iter().any(|i| i.id == "budget_critical"));
    }

    #[test]
    fn detects_budget_warning() {
        let costs = vec![5.0; 10]; // $50 spent, $5/day
        let insights = analyze_spending_patterns(&make_history(&costs), Some(80.0));
        assert!(insights.iter().any(|i| i.id == "budget_warning"));
    }

    #[test]
    fn detects_anomaly() {
        let mut costs = vec![2.0; 15];
        costs.push(20.0); // huge spike on last day
        let insights = analyze_spending_patterns(&make_history(&costs), None);
        assert!(insights.iter().any(|i| i.id == "anomaly_detected"));
    }

    #[test]
    fn no_insights_for_stable_usage() {
        let costs = vec![5.0; 30]; // perfectly stable
        let insights = analyze_spending_patterns(&make_history(&costs), Some(500.0));
        assert!(insights.is_empty());
    }

    #[test]
    fn compares_providers() {
        let provider_a = (
            "Claude".to_string(),
            make_history(&[10.0, 10.0, 10.0]),
        );
        let provider_b = (
            "Codex".to_string(),
            vec![
                UsageDataPoint {
                    timestamp: "2026-05-01".to_string(),
                    cost: 10.0,
                    tokens: 20_000, // much cheaper per token
                    requests: 50,
                },
                UsageDataPoint {
                    timestamp: "2026-05-02".to_string(),
                    cost: 10.0,
                    tokens: 20_000,
                    requests: 50,
                },
            ],
        );
        let insights = compare_providers(&[provider_a, provider_b]);
        assert!(insights.iter().any(|i| i.id == "provider_efficiency"));
    }
}
