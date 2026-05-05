import { Prediction } from "../../hooks/useInsights";

interface PredictionChartProps {
  prediction: Prediction;
}

function PredictionChart({ prediction }: PredictionChartProps) {
  const { daily_forecast } = prediction;

  if (daily_forecast.length === 0) return null;

  const maxCost = Math.max(...daily_forecast.map((f) => f.upper_bound));
  const chartHeight = 80;
  const chartWidth = 280;
  const barWidth = chartWidth / daily_forecast.length - 2;

  return (
    <div className="prediction-chart">
      <div className="chart-header">
        <span className="chart-title">14-Day Cost Forecast</span>
      </div>
      <svg
        viewBox={`0 0 ${chartWidth} ${chartHeight + 20}`}
        className="chart-svg"
      >
        {daily_forecast.map((point, i) => {
          const x = i * (barWidth + 2);
          const barHeight = maxCost > 0
            ? (point.predicted_cost / maxCost) * chartHeight
            : 0;
          const boundHeight = maxCost > 0
            ? ((point.upper_bound - point.lower_bound) / maxCost) * chartHeight
            : 0;
          const boundY = maxCost > 0
            ? chartHeight - (point.upper_bound / maxCost) * chartHeight
            : chartHeight;

          return (
            <g key={i}>
              <rect
                x={x}
                y={boundY}
                width={barWidth}
                height={boundHeight}
                fill="var(--color-primary)"
                opacity={0.15}
                rx={2}
              />
              <rect
                x={x}
                y={chartHeight - barHeight}
                width={barWidth}
                height={barHeight}
                fill="var(--color-primary)"
                opacity={0.7}
                rx={2}
              />
              {i % 3 === 0 && (
                <text
                  x={x + barWidth / 2}
                  y={chartHeight + 14}
                  textAnchor="middle"
                  fill="var(--color-text-muted)"
                  fontSize="8"
                >
                  {point.date}
                </text>
              )}
            </g>
          );
        })}
      </svg>
    </div>
  );
}

export default PredictionChart;
