import { ProviderUsage } from "../hooks/useUsage";
import {
  formatTokens,
  formatCost,
  getStatusColor,
  planLabel,
} from "../utils/format";

function UsageCard({ usage }: { usage: ProviderUsage }) {
  const isSubscription = usage.account_type !== "api";

  const percentage = isSubscription
    ? usage.quota_used && usage.quota_limit
      ? (usage.quota_used / usage.quota_limit) * 100
      : null
    : usage.cost_limit
      ? (usage.cost_used / usage.cost_limit) * 100
      : null;

  const statusColor = getStatusColor(percentage, !!usage.error);

  return (
    <div className="usage-card">
      <div className="card-header">
        <h3>{usage.provider_name}</h3>
        <div className="card-header-right">
          <span className="plan-badge">{planLabel(usage.account_type)}</span>
          <span className="status-dot" style={{ background: statusColor }} />
        </div>
      </div>

      {usage.error ? (
        <div className="card-error">{usage.error}</div>
      ) : isSubscription ? (
        <>
          <div className="card-cost">
            <span className="cost-used">
              {percentage !== null ? `${percentage.toFixed(0)}%` : "--"}
            </span>
            <span className="cost-limit"> quota used</span>
          </div>
          {percentage !== null && (
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{
                  width: `${Math.min(percentage, 100)}%`,
                  background: statusColor,
                }}
              />
            </div>
          )}
          <div className="card-stats">
            <div className="stat">
              <span className="stat-label">Tokens used</span>
              <span className="stat-value">
                {usage.quota_used ? formatTokens(usage.quota_used) : "--"}
                {usage.quota_limit ? ` / ${formatTokens(usage.quota_limit)}` : ""}
              </span>
            </div>
            <div className="stat">
              <span className="stat-label">Requests today</span>
              <span className="stat-value">{usage.requests_today.toLocaleString()}</span>
            </div>
          </div>
        </>
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
                  background: statusColor,
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
