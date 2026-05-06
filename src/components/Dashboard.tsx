import { useState } from "react";
import { useUsage } from "../hooks/useUsage";
import { useInsights } from "../hooks/useInsights";
import UsageCard from "./UsageCard";
import InsightsPanel from "./ai/InsightsPanel";
import PredictionChart from "./ai/PredictionChart";

type Period = "today" | "week" | "month" | "all";

const PERIOD_LABELS: Record<Period, string> = {
  today: "Today",
  week: "This Week",
  month: "This Month",
  all: "All Time",
};

function Dashboard() {
  const [period, setPeriod] = useState<Period>("today");
  const { usages, loading } = useUsage();

  const history = usages.map((u) => ({
    timestamp: u.last_updated,
    cost: u.cost_used,
    tokens: u.tokens_used,
    requests: u.requests_today,
  }));

  const totalBudget =
    usages.reduce((sum, u) => sum + (u.cost_limit ?? 0), 0) || null;

  const { insights, prediction, loading: aiLoading } = useInsights(
    history,
    totalBudget
  );

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  const totalCost = usages.reduce((sum, u) => sum + u.cost_used, 0);
  const hasSubscription = usages.some((u) => u.account_type !== "api");

  return (
    <div className="dashboard">
      <div className="period-tabs">
        {(Object.keys(PERIOD_LABELS) as Period[]).map((p) => (
          <button
            key={p}
            className={period === p ? "active" : ""}
            onClick={() => setPeriod(p)}
          >
            {PERIOD_LABELS[p]}
          </button>
        ))}
      </div>

      <div className="total-cost">
        <span className="label">
          {hasSubscription ? "Usage" : "Spend"} — {PERIOD_LABELS[period]}
        </span>
        <span className="value">
          {hasSubscription ? `${totalCost.toFixed(2)}` : `$${totalCost.toFixed(2)}`}
        </span>
      </div>

      <InsightsPanel
        insights={insights}
        prediction={prediction}
        loading={aiLoading}
      />

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
