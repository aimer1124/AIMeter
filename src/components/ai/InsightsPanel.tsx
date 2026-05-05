import { Insight, Prediction } from "../../hooks/useInsights";

interface InsightsPanelProps {
  insights: Insight[];
  prediction: Prediction | null;
  loading: boolean;
}

function InsightsPanel({ insights, prediction, loading }: InsightsPanelProps) {
  if (loading) {
    return (
      <div className="insights-panel loading">
        <div className="ai-badge">AI Analyzing...</div>
      </div>
    );
  }

  if (insights.length === 0 && !prediction) {
    return null;
  }

  const severityIcon = (severity: string) => {
    switch (severity) {
      case "critical":
        return "!!";
      case "warning":
        return "!";
      default:
        return "i";
    }
  };

  return (
    <div className="insights-panel">
      <div className="insights-header">
        <span className="ai-badge">AI Insights</span>
      </div>

      {prediction && (
        <div className="prediction-summary">
          <div className="prediction-item">
            <span className="prediction-label">Est. Monthly Cost</span>
            <span className="prediction-value">
              ${prediction.estimated_monthly_cost.toFixed(2)}
            </span>
          </div>
          {prediction.days_until_budget_exhausted !== null && (
            <div className="prediction-item">
              <span className="prediction-label">Budget Lasts</span>
              <span className="prediction-value">
                {prediction.days_until_budget_exhausted.toFixed(0)} days
              </span>
            </div>
          )}
          <div className="prediction-item">
            <span className="prediction-label">Confidence</span>
            <span className="prediction-value">
              {(prediction.confidence * 100).toFixed(0)}%
            </span>
          </div>
        </div>
      )}

      {insights.length > 0 && (
        <div className="insights-list">
          {insights.map((insight) => (
            <div key={insight.id} className={`insight-card severity-${insight.severity}`}>
              <div className="insight-header">
                <span className={`severity-badge ${insight.severity}`}>
                  {severityIcon(insight.severity)}
                </span>
                <span className="insight-title">{insight.title}</span>
              </div>
              <p className="insight-description">{insight.description}</p>
              {insight.suggestion && (
                <p className="insight-suggestion">{insight.suggestion}</p>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default InsightsPanel;
