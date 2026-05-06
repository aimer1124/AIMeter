import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import InsightsPanel from "./InsightsPanel";

describe("InsightsPanel", () => {
  it("shows loading state", () => {
    render(<InsightsPanel insights={[]} prediction={null} loading={true} />);
    expect(screen.getByText("AI Analyzing...")).toBeInTheDocument();
  });

  it("returns null when no data", () => {
    const { container } = render(
      <InsightsPanel insights={[]} prediction={null} loading={false} />
    );
    expect(container.innerHTML).toBe("");
  });

  it("renders insight cards with severity badges", () => {
    const insights = [
      {
        id: "test",
        severity: "warning" as const,
        title: "Spending spike",
        description: "Your spending increased",
        suggestion: "Check usage",
      },
    ];
    render(<InsightsPanel insights={insights} prediction={null} loading={false} />);
    expect(screen.getByText("Spending spike")).toBeInTheDocument();
    expect(screen.getByText("Your spending increased")).toBeInTheDocument();
    expect(screen.getByText("Check usage")).toBeInTheDocument();
  });

  it("renders prediction summary", () => {
    const prediction = {
      provider_id: "test",
      daily_forecast: [],
      estimated_monthly_cost: 150.0,
      days_until_budget_exhausted: 12,
      confidence: 0.85,
    };
    render(<InsightsPanel insights={[]} prediction={prediction} loading={false} />);
    expect(screen.getByText("$150.00")).toBeInTheDocument();
    expect(screen.getByText("12 days")).toBeInTheDocument();
    expect(screen.getByText("85%")).toBeInTheDocument();
  });
});
