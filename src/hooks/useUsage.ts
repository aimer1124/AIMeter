import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface ProviderUsage {
  provider_id: string;
  provider_name: string;
  account_type: string;
  cost_used: number;
  cost_limit: number | null;
  quota_used: number | null;
  quota_limit: number | null;
  requests_today: number;
  tokens_used: number;
  last_updated: string;
  error?: string;
}

export function useUsage(refreshInterval = 60_000) {
  const [usages, setUsages] = useState<ProviderUsage[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetch = useCallback(async () => {
    try {
      const data = await invoke<ProviderUsage[]>("get_usage_summary");
      setUsages(data);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    fetch();
    const interval = setInterval(fetch, refreshInterval);
    return () => clearInterval(interval);
  }, [fetch, refreshInterval]);

  return { usages, loading, error, refresh: fetch };
}
