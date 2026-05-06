import { describe, it, expect, vi } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { useInsights } from "./useInsights";
import { invoke } from "@tauri-apps/api/core";

const mockedInvoke = vi.mocked(invoke);

describe("useInsights", () => {
  it("does not invoke when history has fewer than 2 points", () => {
    mockedInvoke.mockReset();
    renderHook(() => useInsights([{ timestamp: "", cost: 1, tokens: 1, requests: 1 }], null));
    expect(mockedInvoke).not.toHaveBeenCalled();
  });

  it("calls both insights and predictions when history is sufficient", async () => {
    mockedInvoke.mockReset();
    mockedInvoke.mockResolvedValue([]);
    const history = [
      { timestamp: "a", cost: 1, tokens: 1, requests: 1 },
      { timestamp: "b", cost: 2, tokens: 2, requests: 2 },
    ];
    renderHook(() => useInsights(history, 100));
    await waitFor(() => {
      expect(mockedInvoke).toHaveBeenCalledWith("get_insights", expect.anything());
      expect(mockedInvoke).toHaveBeenCalledWith("get_predictions", expect.anything());
    });
  });

  it("handles rejection gracefully", async () => {
    const consoleSpy = vi.spyOn(console, "error").mockImplementation(() => {});
    mockedInvoke.mockReset();
    mockedInvoke.mockRejectedValue(new Error("fail"));
    const history = [
      { timestamp: "a", cost: 1, tokens: 1, requests: 1 },
      { timestamp: "b", cost: 2, tokens: 2, requests: 2 },
    ];
    renderHook(() => useInsights(history, null));
    await waitFor(() => {
      expect(consoleSpy).toHaveBeenCalled();
    });
    consoleSpy.mockRestore();
  });
});
