import type { ConfigRepairReport, RecoveryEventType } from "@ts/schemas/config.ts";

export interface ReportDialogProperties {
  isOpen: boolean;
  type: RecoveryEventType;
  title: string;
  description: string;
}

/**
 * Berechnet die Anzeigeeigenschaften des Konfigurations-Repair-Dialogs
 * aus einem ConfigRepairReport.
 *
 * @param report Der Reparaturbericht vom Backend.
 * @param hasConflict True, wenn ein Konfigurationskonflikt aktiv ist (unterdrückt den Repair-Dialog).
 */
export function resolveReportProperties(
  report: ConfigRepairReport | null,
  hasConflict: boolean
): ReportDialogProperties {
  if (!report || hasConflict) {
    return {
      isOpen: false,
      type: "info" as RecoveryEventType,
      title: "",
      description: "",
    };
  }

  const hasFallback =
    report.repairs?.some((r) => r.repair_method === "default_fallback") ||
    !!report.unrecoverable_error;

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

  let description =
    "Einige Konfigurationswerte wurden automatisch in das erwartete Format umgewandelt.";
  if (report.recovery_event?.description) {
    description = report.recovery_event.description;
  } else if (report.unrecoverable_error) {
    description =
      "Die ursprüngliche Konfigurationsdatei konnte nicht geladen werden. Deshalb wurde eine neue Konfiguration mit den Standardwerten erstellt.";
  } else if (hasFallback) {
    description =
      "Einige Konfigurationswerte waren ungültig und wurden durch Standardwerte ersetzt.";
  }

  return {
    isOpen: true,
    type,
    title,
    description,
  };
}
