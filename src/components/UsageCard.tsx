interface ProviderUsage {
  provider_id: string;
  provider_name: string;
  cost_used: number;
  cost_limit: number | null;
  requests_today: number;
  tokens_used: number;
  last_updated: string;
}

function UsageCard({ usage }: { usage: ProviderUsage }) {
  const percentage = usage.cost_limit
    ? (usage.cost_used / usage.cost_limit) * 100
    : null;

  const getStatusColor = () => {
    if (!percentage) return "var(--color-neutral)";
    if (percentage >= 90) return "var(--color-danger)";
    if (percentage >= 70) return "var(--color-warning)";
    return "var(--color-success)";
  };

  const formatTokens = (tokens: number) => {
    if (tokens >= 1_000_000) return `${(tokens / 1_000_000).toFixed(1)}M`;
    if (tokens >= 1_000) return `${(tokens / 1_000).toFixed(1)}K`;
    return tokens.toString();
  };

  return (
    <div className="usage-card">
      <div className="card-header">
        <h3>{usage.provider_name}</h3>
        <span className="status-dot" style={{ background: getStatusColor() }} />
      </div>
      <div className="card-cost">
        <span className="cost-used">${usage.cost_used.toFixed(2)}</span>
        {usage.cost_limit && (
          <span className="cost-limit"> / ${usage.cost_limit.toFixed(2)}</span>
        )}
      </div>
      {percentage !== null && (
        <div className="progress-bar">
          <div
            className="progress-fill"
            style={{ width: `${Math.min(percentage, 100)}%`, background: getStatusColor() }}
          />
        </div>
      )}
      <div className="card-stats">
        <div className="stat">
          <span className="stat-label">Requests today</span>
          <span className="stat-value">{usage.requests_today}</span>
        </div>
        <div className="stat">
          <span className="stat-label">Tokens</span>
          <span className="stat-value">{formatTokens(usage.tokens_used)}</span>
        </div>
      </div>
    </div>
  );
}

export default UsageCard;
