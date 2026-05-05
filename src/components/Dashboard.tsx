import { useUsage } from "../hooks/useUsage";
import { useInsights } from "../hooks/useInsights";
import UsageCard from "./UsageCard";
import InsightsPanel from "./ai/InsightsPanel";
import PredictionChart from "./ai/PredictionChart";

function Dashboard() {
  const { usages, loading } = useUsage();

  // Build mock history from current data for AI analysis demo
  const history = usages.map((u) => ({
    timestamp: u.last_updated,
    cost: u.cost_used,
    tokens: u.tokens_used,
    requests: u.requests_today,
  }));

  const totalBudget = usages.reduce(
    (sum, u) => sum + (u.cost_limit ?? 0),
    0
  ) || null;

  const { insights, prediction, loading: aiLoading } = useInsights(history, totalBudget);

  if (loading) {
    return <div className="loading">Loading usage data...</div>;
  }

  const totalCost = usages.reduce((sum, u) => sum + u.cost_used, 0);

  return (
    <div className="dashboard">
      <div className="total-cost">
        <span className="label">Total Spend</span>
        <span className="value">${totalCost.toFixed(2)}</span>
      </div>

      <InsightsPanel insights={insights} prediction={prediction} loading={aiLoading} />

      {prediction && prediction.daily_forecast.length > 0 && (
        <PredictionChart prediction={prediction} />
      )}

      <div className="usage-grid">
        {usages.map((usage) => (
          <UsageCard key={usage.provider_id} usage={usage} />
        ))}
      </div>

      {usages.length === 0 && (
        <div className="empty-state">
          <p>No providers configured yet.</p>
          <p>Go to Settings to add your AI tool API keys.</p>
        </div>
      )}
    </div>
  );
}

export default Dashboard;
