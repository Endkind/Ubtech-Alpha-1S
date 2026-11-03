import { describe, it, expect, vi, beforeEach } from "vitest";
import { invoke } from "@tauri-apps/api/core";
import { dirname, join, basename, isAbsolute } from "../src/ts/utils/path.ts";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("path utils", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("dirname", () => {
    it("should call path_dirname with path", async () => {
      vi.mocked(invoke).mockResolvedValueOnce("C:\\Projekte\\sdk");
      const result = await dirname("C:\\Projekte\\sdk\\main.py");
      expect(invoke).toHaveBeenCalledWith("path_dirname", { path: "C:\\Projekte\\sdk\\main.py" });
      expect(result).toBe("C:\\Projekte\\sdk");
    });
  });

  describe("join", () => {
    it("should call path_join with paths array", async () => {
      vi.mocked(invoke).mockResolvedValueOnce("C:\\Projekte\\sdk\\.venv");
      const result = await join("C:\\Projekte\\sdk", ".venv");
      expect(invoke).toHaveBeenCalledWith("path_join", { paths: ["C:\\Projekte\\sdk", ".venv"] });
      expect(result).toBe("C:\\Projekte\\sdk\\.venv");
    });

    it("should support multiple paths", async () => {
      vi.mocked(invoke).mockResolvedValueOnce("/home/user/project/src");
      const result = await join("/home/user", "project", "src");
      expect(invoke).toHaveBeenCalledWith("path_join", { paths: ["/home/user", "project", "src"] });
      expect(result).toBe("/home/user/project/src");
    });
  });

  describe("basename", () => {
    it("should call path_basename with path", async () => {
      vi.mocked(invoke).mockResolvedValueOnce("main.py");
      const result = await basename("C:\\Projekte\\sdk\\main.py");
      expect(invoke).toHaveBeenCalledWith("path_basename", { path: "C:\\Projekte\\sdk\\main.py" });
      expect(result).toBe("main.py");
    });
  });

  describe("isAbsolute", () => {
    it("should call path_is_absolute and return boolean", async () => {
      vi.mocked(invoke).mockResolvedValueOnce(true);
      const result = await isAbsolute("C:\\Projekte");
      expect(invoke).toHaveBeenCalledWith("path_is_absolute", { path: "C:\\Projekte" });
      expect(result).toBe(true);
    });
  });
});
