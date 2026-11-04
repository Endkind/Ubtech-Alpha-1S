import { describe, it, expect } from "vitest";
import type { ConfigRepairReport, RecoveryEventType } from "../src/ts/schemas/config.ts";

function resolveReportProperties(report: ConfigRepairReport | null, hasConflict: boolean) {
  if (!report || hasConflict) {
    return {
      isOpen: false,
      type: "info" as RecoveryEventType,
      title: "",
      description: "",
    };
  }

  const hasFallback = report.repairs?.some((r) => r.repair_method === "default_fallback") || !!report.unrecoverable_error;

  let type: RecoveryEventType = "info";
  if (report.recovery_event?.event_type) {
    type = report.recovery_event.event_type;
  } else if (report.unrecoverable_error || hasFallback) {
    type = "warning";
  }

  let title = "Konfigurationswerte automatisch konvertiert";
  if (report.recovery_event?.title) {
    title = report.recovery_event.title;
  } else if (report.unrecoverable_error) {
    title = "Konfiguration mit Standardwerten wiederhergestellt";
  } else if (hasFallback) {
    title = "Konfiguration teilweise repariert";
  }

  let description = "Einige Konfigurationswerte wurden automatisch in das erwartete Format umgewandelt.";
  if (report.recovery_event?.description) {
    description = report.recovery_event.description;
  } else if (report.unrecoverable_error) {
    description = "Die ursprüngliche Konfigurationsdatei konnte nicht geladen werden. Deshalb wurde eine neue Konfiguration mit den Standardwerten erstellt.";
  } else if (hasFallback) {
    description = "Einige Konfigurationswerte waren ungültig und wurden durch Standardwerte ersetzt.";
  }

  return {
    isOpen: true,
    type,
    title,
    description,
  };
}

describe("Config Report and Dialog Resolution", () => {
  it("Scenario 1: should resolve info event for pure type conversions", () => {
    const report: ConfigRepairReport = {
      repairs: [
        {
          field: "default_action_duration",
          original_value: "50",
          expected_type: "usize",
          reason: "Converted string to unsigned integer",
          resulting_value: 50,
          repair_method: "converted",
          success: true,
        },
      ],
      unrecoverable_error: null,
      recovery_event: null,
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("info");
    expect(res.title).toBe("Konfigurationswerte automatisch konvertiert");
  });

  it("Scenario 2: should resolve warning event for default fallback repairs", () => {
    const report: ConfigRepairReport = {
      repairs: [
        {
          field: "default_action_duration",
          original_value: "invalid",
          expected_type: "usize",
          reason: "Cannot convert string to unsigned integer",
          resulting_value: 50,
          repair_method: "default_fallback",
          success: false,
        },
      ],
      unrecoverable_error: null,
      recovery_event: null,
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("warning");
    expect(res.title).toBe("Konfiguration teilweise repariert");
  });

  it("Scenario 3: should resolve multiple repairs with fallback as warning", () => {
    const report: ConfigRepairReport = {
      repairs: [
        {
          field: "default_delay",
          original_value: "750",
          expected_type: "usize",
          reason: "Converted string to unsigned integer",
          resulting_value: 750,
          repair_method: "converted",
          success: true,
        },
        {
          field: "default_action_duration",
          original_value: -10,
          expected_type: "usize",
          reason: "Negative value for unsigned integer",
          resulting_value: 50,
          repair_method: "default_fallback",
          success: false,
        },
      ],
      unrecoverable_error: null,
      recovery_event: null,
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("warning");
    expect(res.title).toBe("Konfiguration teilweise repariert");
  });

  it("Scenario 4: should resolve warning event for unrecoverable error recreated with defaults", () => {
    const report: ConfigRepairReport = {
      repairs: [],
      unrecoverable_error: "Unrecoverable configuration error: TOML parse error",
      recovery_event: {
        event_type: "warning",
        title: "Konfiguration mit Standardwerten wiederhergestellt",
        description: "Die ursprüngliche Konfigurationsdatei konnte nicht geladen werden. Deshalb wurde eine neue Konfiguration mit den Standardwerten erstellt.",
        details: {
          affected_file: "/path/config.toml",
          error_cause: "TOML parse error",
          recovery_action: "Neue Standardkonfiguration erstellt und gespeichert",
          source: "Standardkonfiguration",
          result: "Standardwerte aktiv und gespeichert",
        },
      },
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("warning");
    expect(res.title).toBe("Konfiguration mit Standardwerten wiederhergestellt");
    expect(report.recovery_event?.details?.source).toBe("Standardkonfiguration");
  });

  it("Scenario 5: should resolve success event for backup recovery", () => {
    const report: ConfigRepairReport = {
      repairs: [],
      unrecoverable_error: null,
      recovery_event: {
        event_type: "success",
        title: "Konfiguration aus Sicherung wiederhergestellt",
        description: "Die Konfiguration wurde erfolgreich aus einer vorhandenen Recovery-Sicherungsdatei wiederhergestellt.",
        details: {
          affected_file: "/path/config.toml",
          source: "Recovery-Sicherungsdatei",
          result: "Erfolgreich wiederhergestellt",
        },
      },
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("success");
    expect(res.title).toBe("Konfiguration aus Sicherung wiederhergestellt");
  });

  it("Scenario 6: should resolve error event for failed recovery", () => {
    const report: ConfigRepairReport = {
      repairs: [],
      unrecoverable_error: "Persistent IO error: Permission denied",
      recovery_event: {
        event_type: "error",
        title: "Wiederherstellung fehlgeschlagen",
        description: "Die ursprüngliche Konfigurationsdatei konnte wegen eines persistenten E/A-Fehlers nicht geladen werden.",
        details: {
          affected_file: "/path/config.toml",
          error_cause: "Permission denied",
          result: "Recovery-Modus aktiviert",
        },
      },
    };

    const res = resolveReportProperties(report, false);
    expect(res.isOpen).toBe(true);
    expect(res.type).toBe("error");
    expect(res.title).toBe("Wiederherstellung fehlgeschlagen");
  });

  it("Scenario 7: should suppress repair modal while conflict is active", () => {
    const report: ConfigRepairReport = {
      repairs: [],
      unrecoverable_error: null,
      recovery_event: {
        event_type: "success",
        title: "Konfiguration aus Sicherung wiederhergestellt",
        description: "Wiederhergestellt",
      },
    };

    const res = resolveReportProperties(report, true);
    expect(res.isOpen).toBe(false);
  });
});
