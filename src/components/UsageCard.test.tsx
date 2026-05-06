import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import UsageCard from "./UsageCard";
import { ProviderUsage } from "../hooks/useUsage";

function makeUsage(overrides: Partial<ProviderUsage> = {}): ProviderUsage {
  return {
    provider_id: "test",
    provider_name: "Test Provider",
    account_type: "api",
    cost_used: 12.5,
    cost_limit: 100.0,
    quota_used: null,
    quota_limit: null,
    requests_today: 42,
    tokens_used: 50000,
    last_updated: "2026-05-06T00:00:00Z",
    ...overrides,
  };
}

describe("UsageCard", () => {
  it("renders API mode with dollar cost and budget", () => {
    render(<UsageCard usage={makeUsage()} />);
    expect(screen.getByText("$12.50")).toBeInTheDocument();
    expect(screen.getByText("/ $100.00")).toBeInTheDocument();
    expect(screen.getByText("API")).toBeInTheDocument();
  });

  it("renders subscription mode with quota percentage", () => {
    render(
      <UsageCard
        usage={makeUsage({
          account_type: "pro",
          quota_used: 29_250_000,
          quota_limit: 45_000_000,
        })}
      />
    );
    expect(screen.getByText("65%")).toBeInTheDocument();
    expect(screen.getByText(/quota used/)).toBeInTheDocument();
    expect(screen.getByText("Pro")).toBeInTheDocument();
  });

  it("renders error state", () => {
    render(
      <UsageCard usage={makeUsage({ error: "Something broke" })} />
    );
    expect(screen.getByText("Something broke")).toBeInTheDocument();
    expect(screen.queryByText("$12.50")).not.toBeInTheDocument();
  });

  it("caps progress bar at 100%", () => {
    const { container } = render(
      <UsageCard usage={makeUsage({ cost_used: 150, cost_limit: 100 })} />
    );
    const fill = container.querySelector(".progress-fill") as HTMLElement;
    expect(fill.style.width).toBe("100%");
  });

  it("shows plan badge", () => {
    render(<UsageCard usage={makeUsage({ account_type: "max200" })} />);
    expect(screen.getByText("Max $200")).toBeInTheDocument();
  });

  it("shows -- when quota data is null in subscription mode", () => {
    render(
      <UsageCard
        usage={makeUsage({
          account_type: "pro",
          quota_used: null,
          quota_limit: null,
        })}
      />
    );
    expect(screen.getAllByText("--").length).toBeGreaterThanOrEqual(1);
  });
});
