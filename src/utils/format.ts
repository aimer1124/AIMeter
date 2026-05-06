export function formatTokens(tokens: number): string {
  if (tokens >= 1_000_000_000) return `${(tokens / 1_000_000_000).toFixed(1)}B`;
  if (tokens >= 1_000_000) return `${(tokens / 1_000_000).toFixed(1)}M`;
  if (tokens >= 1_000) return `${(tokens / 1_000).toFixed(1)}K`;
  return tokens.toString();
}

export function formatCost(cost: number): string {
  if (cost >= 1000) return `$${(cost / 1000).toFixed(1)}K`;
  return `$${cost.toFixed(2)}`;
}

export function getStatusColor(
  percentage: number | null,
  hasError: boolean
): string {
  if (hasError) return "var(--color-neutral)";
  if (percentage === null) return "var(--color-success)";
  if (percentage >= 90) return "var(--color-danger)";
  if (percentage >= 70) return "var(--color-warning)";
  return "var(--color-success)";
}

export function planLabel(accountType: string): string {
  switch (accountType) {
    case "pro":
      return "Pro";
    case "max100":
      return "Max $100";
    case "max200":
      return "Max $200";
    default:
      return "API";
  }
}
