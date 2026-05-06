import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import App from "./App";

import { invoke } from "@tauri-apps/api/core";

vi.mock("./hooks/useUsage", () => ({
  useUsage: vi.fn().mockReturnValue({
    usages: [],
    loading: false,
    error: null,
    refresh: vi.fn(),
  }),
}));

vi.mock("./hooks/useInsights", () => ({
  useInsights: vi.fn().mockReturnValue({
    insights: [],
    prediction: null,
    loading: false,
  }),
}));

const mockedInvoke = vi.mocked(invoke);
mockedInvoke.mockImplementation(async () => []);

describe("App", () => {
  it("renders Dashboard by default", () => {
    render(<App />);
    expect(screen.getByText("AIMeter")).toBeInTheDocument();
    expect(screen.getByText("Today")).toBeInTheDocument();
  });

  it("switches to Settings view", async () => {
    render(<App />);
    await userEvent.click(screen.getByText("Settings"));
    expect(screen.getByText("Provider Configuration")).toBeInTheDocument();
  });

  it("highlights active nav button", () => {
    render(<App />);
    const dashBtn = screen.getByText("Dashboard");
    expect(dashBtn.className).toContain("active");
  });
});
