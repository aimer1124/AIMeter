import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface Insight {
  id: string;
  severity: "info" | "warning" | "critical";
  title: string;
  description: string;
  suggestion: string | null;
}

export interface Prediction {
  provider_id: string;
  daily_forecast: ForecastPoint[];
  estimated_monthly_cost: number;
  days_until_budget_exhausted: number | null;
  confidence: number;
}

export interface ForecastPoint {
  date: string;
  predicted_cost: number;
  lower_bound: number;
  upper_bound: number;
}

interface UsageDataPoint {
  timestamp: string;
  cost: number;
  tokens: number;
  requests: number;
}

export function useInsights(history: UsageDataPoint[], budgetLimit: number | null) {
  const [insights, setInsights] = useState<Insight[]>([]);
  const [prediction, setPrediction] = useState<Prediction | null>(null);
  const [loading, setLoading] = useState(false);

  const analyze = useCallback(async () => {
    if (history.length < 2) return;
    setLoading(true);
    try {
      const [insightResults, predictionResult] = await Promise.all([
        invoke<Insight[]>("get_insights", {
          history,
          budgetLimit,
        }),
        invoke<Prediction>("get_predictions", {
          history,
          budgetLimit,
          forecastDays: 14,
        }),
      ]);
      setInsights(insightResults);
      setPrediction(predictionResult);
    } catch (err) {
      console.error("AI analysis failed:", err);
    } finally {
      setLoading(false);
    }
  }, [history, budgetLimit]);

  useEffect(() => {
    analyze();
  }, [analyze]);

  return { insights, prediction, loading, reanalyze: analyze };
}
