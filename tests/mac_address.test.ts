import { describe, it, expect } from "vitest";

const MAC_REGEX = /^(([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}|([0-9A-Fa-f]{2}-){5}[0-9A-Fa-f]{2})$/;

describe("MAC address regex validation", () => {
  it("should accept valid colon-separated MAC addresses", () => {
    expect(MAC_REGEX.test("00:11:22:33:44:55")).toBe(true);
    expect(MAC_REGEX.test("AA:BB:CC:DD:EE:FF")).toBe(true);
    expect(MAC_REGEX.test("a1:b2:c3:d4:e5:f6")).toBe(true);
  });

  it("should accept valid hyphen-separated MAC addresses", () => {
    expect(MAC_REGEX.test("00-11-22-33-44-55")).toBe(true);
    expect(MAC_REGEX.test("AA-BB-CC-DD-EE-FF")).toBe(true);
    expect(MAC_REGEX.test("a1-b2-c3-d4-e5-f6")).toBe(true);
  });

  it("should reject mixed-separator MAC addresses", () => {
    expect(MAC_REGEX.test("00:11-22:33-44:55")).toBe(false);
    expect(MAC_REGEX.test("00-11:22:33:44:55")).toBe(false);
    expect(MAC_REGEX.test("00:11:22:33:44-55")).toBe(false);
    expect(MAC_REGEX.test("00:11:22-33:44:55")).toBe(false);
  });

  it("should reject invalid MAC formats", () => {
    expect(MAC_REGEX.test("invalid-mac")).toBe(false);
    expect(MAC_REGEX.test("00:11:22:33:44")).toBe(false);
    expect(MAC_REGEX.test("00:11:22:33:44:55:66")).toBe(false);
    expect(MAC_REGEX.test("00:11:22:33:44:5G")).toBe(false);
  });
});
