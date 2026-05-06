import { describe, it, expect } from "vitest";
import { formatTokens, formatCost, getStatusColor, planLabel } from "./format";

describe("formatTokens", () => {
  it("returns raw number below 1000", () => {
    expect(formatTokens(0)).toBe("0");
    expect(formatTokens(999)).toBe("999");
  });

  it("formats thousands as K", () => {
    expect(formatTokens(1000)).toBe("1.0K");
    expect(formatTokens(1500)).toBe("1.5K");
    expect(formatTokens(999999)).toBe("1000.0K");
  });

  it("formats millions as M", () => {
    expect(formatTokens(1_000_000)).toBe("1.0M");
    expect(formatTokens(45_000_000)).toBe("45.0M");
  });

  it("formats billions as B", () => {
    expect(formatTokens(1_500_000_000)).toBe("1.5B");
  });
});

describe("formatCost", () => {
  it("formats small amounts with two decimals", () => {
    expect(formatCost(0)).toBe("$0.00");
    expect(formatCost(0.1)).toBe("$0.10");
    expect(formatCost(999.99)).toBe("$999.99");
  });

  it("formats thousands as $K", () => {
    expect(formatCost(1000)).toBe("$1.0K");
    expect(formatCost(5432.1)).toBe("$5.4K");
  });
});

describe("getStatusColor", () => {
  it("returns neutral for errors", () => {
    expect(getStatusColor(50, true)).toBe("var(--color-neutral)");
  });

  it("returns success for null percentage", () => {
    expect(getStatusColor(null, false)).toBe("var(--color-success)");
  });

  it("returns success below 70%", () => {
    expect(getStatusColor(50, false)).toBe("var(--color-success)");
  });

  it("returns warning at 70%+", () => {
    expect(getStatusColor(70, false)).toBe("var(--color-warning)");
    expect(getStatusColor(89.9, false)).toBe("var(--color-warning)");
  });

  it("returns danger at 90%+", () => {
    expect(getStatusColor(90, false)).toBe("var(--color-danger)");
    expect(getStatusColor(100, false)).toBe("var(--color-danger)");
  });
});

describe("planLabel", () => {
  it("maps known account types", () => {
    expect(planLabel("pro")).toBe("Pro");
    expect(planLabel("max100")).toBe("Max $100");
    expect(planLabel("max200")).toBe("Max $200");
    expect(planLabel("api")).toBe("API");
  });

  it("defaults unknown types to API", () => {
    expect(planLabel("unknown")).toBe("API");
  });
});
