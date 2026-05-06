import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import Dashboard from "./Dashboard";

vi.mock("../hooks/useUsage", () => ({
  useUsage: vi.fn(),
}));

vi.mock("../hooks/useInsights", () => ({
  useInsights: vi.fn().mockReturnValue({
    insights: [],
    prediction: null,
    loading: false,
  }),
}));

import { useUsage } from "../hooks/useUsage";

const mockedUseUsage = vi.mocked(useUsage);

describe("Dashboard", () => {
  it("shows loading state", () => {
    mockedUseUsage.mockReturnValue({
      usages: [],
      loading: true,
      error: null,
      refresh: vi.fn(),
    });
    render(<Dashboard />);
    expect(screen.getByText("Loading...")).toBeInTheDocument();
  });

  it("renders period tabs", () => {
    mockedUseUsage.mockReturnValue({
      usages: [],
      loading: false,
      error: null,
      refresh: vi.fn(),
    });
    render(<Dashboard />);
    expect(screen.getByText("Today")).toBeInTheDocument();
    expect(screen.getByText("This Week")).toBeInTheDocument();
    expect(screen.getByText("This Month")).toBeInTheDocument();
    expect(screen.getByText("All Time")).toBeInTheDocument();
  });

  it("shows empty state when no providers", () => {
    mockedUseUsage.mockReturnValue({
      usages: [],
      loading: false,
      error: null,
      refresh: vi.fn(),
    });
    render(<Dashboard />);
    expect(screen.getByText(/No providers configured/)).toBeInTheDocument();
  });

  it("renders usage cards after loading", () => {
    mockedUseUsage.mockReturnValue({
      usages: [
        {
          provider_id: "claude",
          provider_name: "Claude Code",
          account_type: "api",
          cost_used: 25.0,
          cost_limit: 100.0,
          quota_used: null,
          quota_limit: null,
          requests_today: 10,
          tokens_used: 5000,
          last_updated: "",
        },
      ],
      loading: false,
      error: null,
      refresh: vi.fn(),
    });
    render(<Dashboard />);
    expect(screen.getByText("Claude Code")).toBeInTheDocument();
    expect(screen.getAllByText("$25.00").length).toBeGreaterThanOrEqual(1);
  });

  it("shows 'Usage' label for subscription providers", () => {
    mockedUseUsage.mockReturnValue({
      usages: [
        {
          provider_id: "claude",
          provider_name: "Claude Code",
          account_type: "pro",
          cost_used: 0,
          cost_limit: null,
          quota_used: 1000,
          quota_limit: 10000,
          requests_today: 5,
          tokens_used: 1000,
          last_updated: "",
        },
      ],
      loading: false,
      error: null,
      refresh: vi.fn(),
    });
    render(<Dashboard />);
    expect(screen.getByText(/Usage/)).toBeInTheDocument();
  });
});
