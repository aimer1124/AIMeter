import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { useUsage } from "./useUsage";
import { invoke } from "@tauri-apps/api/core";

const mockedInvoke = vi.mocked(invoke);

beforeEach(() => {
  mockedInvoke.mockReset();
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
});

describe("useUsage", () => {
  it("fetches on mount and sets loading to false", async () => {
    mockedInvoke.mockResolvedValue([]);
    vi.useRealTimers();
    const { result } = renderHook(() => useUsage());
    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });
    expect(mockedInvoke).toHaveBeenCalledWith("get_usage_summary");
  });

  it("sets error state on rejection", async () => {
    mockedInvoke.mockRejectedValue(new Error("network"));
    vi.useRealTimers();
    const { result } = renderHook(() => useUsage());
    await waitFor(() => {
      expect(result.current.error).toBe("Error: network");
    });
  });

  it("clears interval on unmount", () => {
    mockedInvoke.mockResolvedValue([]);
    const clearSpy = vi.spyOn(globalThis, "clearInterval");
    const { unmount } = renderHook(() => useUsage());
    unmount();
    expect(clearSpy).toHaveBeenCalled();
    clearSpy.mockRestore();
  });
});
