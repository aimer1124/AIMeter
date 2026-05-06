import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import Settings from "./Settings";
import { invoke } from "@tauri-apps/api/core";

const mockedInvoke = vi.mocked(invoke);

const mockProviders = [
  {
    id: "claude",
    name: "Claude Code",
    provider_type: "claude_code",
    account_type: "api",
    api_key: null,
    enabled: true,
    budget_limit: null,
  },
];

beforeEach(() => {
  mockedInvoke.mockReset();
  mockedInvoke.mockImplementation(async (cmd: string) => {
    if (cmd === "get_providers") return mockProviders;
    if (cmd === "get_autostart_enabled") return false;
    return undefined;
  });
});

describe("Settings", () => {
  it("renders provider list", async () => {
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText("Claude Code")).toBeInTheDocument();
    });
  });

  it("shows API key field only for api type", async () => {
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText("Claude Code")).toBeInTheDocument();
    });
    expect(screen.getByPlaceholderText("Enter API key...")).toBeInTheDocument();
  });

  it("shows quota hint for subscription types", async () => {
    mockedInvoke.mockImplementation(async (cmd: string) => {
      if (cmd === "get_providers")
        return [{ ...mockProviders[0], account_type: "pro" }];
      if (cmd === "get_autostart_enabled") return false;
      return undefined;
    });
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText(/Quota limits are set automatically/)).toBeInTheDocument();
    });
  });

  it("shows budget field for API accounts", async () => {
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText("Budget Limit ($)")).toBeInTheDocument();
    });
  });

  it("toggles autostart", async () => {
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText("Launch at login")).toBeInTheDocument();
    });
    const checkbox = screen.getByText("Launch at login").previousElementSibling as HTMLInputElement;
    await userEvent.click(checkbox);
    expect(mockedInvoke).toHaveBeenCalledWith("set_autostart_enabled", { enabled: true });
  });

  it("handles initial invoke rejection gracefully", async () => {
    mockedInvoke.mockRejectedValue(new Error("fail"));
    render(<Settings />);
    await waitFor(() => {
      expect(screen.getByText("Provider Configuration")).toBeInTheDocument();
    });
  });
});
