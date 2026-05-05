import { ProviderUsage } from "../hooks/useUsage";

function UsageCard({ usage }: { usage: ProviderUsage }) {
  const percentage = usage.cost_limit
    ? (usage.cost_used / usage.cost_limit) * 100
    : null;

  const getStatusColor = () => {
    if (usage.error) return "var(--color-neutral)";
    if (!percentage) return "var(--color-success)";
    if (percentage >= 90) return "var(--color-danger)";
    if (percentage >= 70) return "var(--color-warning)";
    return "var(--color-success)";
  };

  const formatTokens = (tokens: number) => {
    if (tokens >= 1_000_000_000) return `${(tokens / 1_000_000_000).toFixed(1)}B`;
    if (tokens >= 1_000_000) return `${(tokens / 1_000_000).toFixed(1)}M`;
    if (tokens >= 1_000) return `${(tokens / 1_000).toFixed(1)}K`;
    return tokens.toString();
  };

  const formatCost = (cost: number) => {
    if (cost >= 1000) return `$${(cost / 1000).toFixed(1)}K`;
    return `$${cost.toFixed(2)}`;
  };

  return (
    <div className="usage-card">
      <div className="card-header">
        <h3>{usage.provider_name}</h3>
        <span className="status-dot" style={{ background: getStatusColor() }} />
      </div>

      {usage.error ? (
        <div className="card-error">{usage.error}</div>
      ) : (
        <>
          <div className="card-cost">
            <span className="cost-used">{formatCost(usage.cost_used)}</span>
            {usage.cost_limit && (
              <span className="cost-limit"> / {formatCost(usage.cost_limit)}</span>
            )}
          </div>
          {percentage !== null && (
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{
                  width: `${Math.min(percentage, 100)}%`,
                  background: getStatusColor(),
                }}
              />
            </div>
          )}
          <div className="card-stats">
            <div className="stat">
              <span className="stat-label">Requests today</span>
              <span className="stat-value">{usage.requests_today.toLocaleString()}</span>
            </div>
            <div className="stat">
              <span className="stat-label">Tokens</span>
              <span className="stat-value">{formatTokens(usage.tokens_used)}</span>
            </div>
          </div>
        </>
      )}
    </div>
  );
}

export default UsageCard;
