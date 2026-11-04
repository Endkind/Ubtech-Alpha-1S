<script setup lang="ts">
  import { PAGES } from "@ts/pages.ts";
  import { computed, onMounted, ref } from "vue";
  import { getVersion } from "@tauri-apps/api/app";
  import { invoke } from "@tauri-apps/api/core";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import pkg from "../package.json";
  import About from "@pages/About.vue";
  import Settings from "@pages/Settings.vue";
  import Home from "@pages/Home.vue";
  import Editor from "@pages/Editor.vue";
  import Library from "@pages/Library.vue";
  import ConfigDialog from "./components/ConfigDialog.vue";
  import { usePagesStore } from "@stores/pages.ts";
  import type { ConfigConflict, ResolutionChoice, ConfigRepairReport, RecoveryEventType } from "@ts/schemas/config.ts";
  import { CheckIcon } from "@heroicons/vue/24/outline";

  const pages_store = usePagesStore();

  const version = ref(pkg.version || "0.1.0");
  const releaseTag = computed(() => `v${version.value.replace(/^v/, "")}`);
  const releaseUrl = computed(
    () => `https://github.com/Endkind/Ubtech-Alpha-1S/releases/tag/${releaseTag.value}`
  );

  const conflict = ref<ConfigConflict | null>(null);
  const is_resolving = ref<boolean>(false);
  const resolve_error = ref<string | null>(null);

  const repair_report = ref<ConfigRepairReport | null>(null);

  const hasFallback = computed(() => {
    if (!repair_report.value) return false;
    if (repair_report.value.unrecoverable_error) return true;
    return repair_report.value.repairs?.some((r) => r.repair_method === "default_fallback");
  });

  const isRepairModalOpen = computed(() => {
    return conflict.value === null && repair_report.value !== null;
  });

  const reportType = computed<RecoveryEventType>(() => {
    if (!repair_report.value) return "info";
    if (repair_report.value.recovery_event?.event_type) {
      return repair_report.value.recovery_event.event_type;
    }
    if (repair_report.value.unrecoverable_error) {
      return "warning";
    }
    if (hasFallback.value) {
      return "warning";
    }
    return "info";
  });

  const reportTitle = computed(() => {
    if (!repair_report.value) return "";
    if (repair_report.value.recovery_event?.title) {
      return repair_report.value.recovery_event.title;
    }
    if (repair_report.value.unrecoverable_error) {
      return "Konfiguration mit Standardwerten wiederhergestellt";
    }
    if (hasFallback.value) {
      return "Konfiguration teilweise repariert";
    }
    return "Konfigurationswerte automatisch konvertiert";
  });

  const reportDescription = computed(() => {
    if (!repair_report.value) return "";
    if (repair_report.value.recovery_event?.description) {
      return repair_report.value.recovery_event.description;
    }
    if (repair_report.value.unrecoverable_error) {
      return "Die ursprüngliche Konfigurationsdatei konnte nicht geladen werden. Deshalb wurde eine neue Konfiguration mit den Standardwerten erstellt.";
    }
    if (hasFallback.value) {
      return "Einige Konfigurationswerte waren ungültig und wurden durch Standardwerte ersetzt.";
    }
    return "Einige Konfigurationswerte wurden automatisch in das erwartete Format umgewandelt.";
  });

  async function checkConflict() {
    try {
      const res = await invoke<ConfigConflict | null>("get_config_conflict");
      if (res && res.original && res.recovery) {
        conflict.value = res;
      } else {
        conflict.value = null;
      }
    } catch (error) {
      console.error("Fehler beim Prüfen von Konfigurationskonflikten:", error);
    }
  }

  async function checkRepairReport() {
    try {
      const res = await invoke<ConfigRepairReport | null>("get_repair_report");
      if (res && ((res.repairs && res.repairs.length > 0) || res.unrecoverable_error || res.recovery_event)) {
        repair_report.value = res;
      } else {
        repair_report.value = null;
      }
    } catch (error) {
      console.error("Fehler beim Abrufen des Reparaturberichts:", error);
    }
  }

  async function dismissRepairReport() {
    try {
      await invoke("dismiss_repair_report");
    } catch (error) {
      console.error("Fehler beim Schließen des Reparaturberichts:", error);
    } finally {
      repair_report.value = null;
    }
  }

  async function resolveConflict(choice: ResolutionChoice) {
    is_resolving.value = true;
    resolve_error.value = null;
    try {
      await invoke("resolve_config_conflict", { choice });
      conflict.value = null;
    } catch (error) {
      console.error("Fehler beim Auflösen des Konflikts:", error);
      resolve_error.value = typeof error === 'string' ? error : 'Fehler beim Speichern der ausgewählten Konfiguration.';
    } finally {
      is_resolving.value = false;
    }
  }

  onMounted(async () => {
    try {
      version.value = await getVersion();
    } catch (error) {
      console.error("Fehler beim Abrufen der Tauri-Version:", error);
      version.value = pkg.version;
    }
    await checkConflict();
    await checkRepairReport();
  });

  async function openRelease() {
    try {
      await openUrl(releaseUrl.value);
    } catch {
      window.open(releaseUrl.value, "_blank");
    }
  }
</script>

<template>
  <div class="drawer lg:drawer-open min-h-screen h-screen">
    <input id="drawer" type="checkbox" class="drawer-toggle inline" />
    <div class="drawer-content flex flex-col h-full overflow-hidden">
      <!-- Navbar -->
      <nav class="navbar w-full bg-base-300">
        <label
          for="drawer"
          tabindex="0"
          role="button"
          aria-label="Seitenleiste umschalten"
          class="btn btn-square btn-ghost drawer-button"
          @keydown.enter.prevent="($event.target as HTMLElement).click()"
          @keydown.space.prevent="($event.target as HTMLElement).click()"
        >
          <!-- Sidebar toggle icon -->
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4"><path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"></path><path d="M9 4v16"></path><path d="M14 10l2 2l-2 2"></path></svg>
        </label>
        <div class="px-4">{{ pages_store.current_page.display_name }}</div>
      </nav>

      <!-- Page content here -->
      <div class="flex flex-1 min-h-0 overflow-auto">
        <div class="grow p-2">
          <Home v-if="pages_store.current_page == PAGES.HOME" v-bind="pages_store.kargs" />
          <Editor v-else-if="pages_store.current_page == PAGES.EDITOR" v-bind="pages_store.kargs" />
          <Library v-else-if="pages_store.current_page == PAGES.LIBRARY" v-bind="pages_store.kargs" />
          <Settings v-else-if="pages_store.current_page == PAGES.SETTINGS" v-bind="pages_store.kargs" />
          <About v-else-if="pages_store.current_page == PAGES.ABOUT" v-bind="pages_store.kargs" />
        </div>
      </div>
    </div>

    <div class="drawer-side is-drawer-close:overflow-visible">
      <label for="drawer" aria-label="close sidebar" class="drawer-overlay"></label>
      <div class="flex min-h-full flex-col items-start bg-base-200 is-drawer-close:w-16 is-drawer-open:w-64">
        <!-- Sidebar content here -->
        <ul class="menu w-full grow">
          <li v-for="page in PAGES" :key="page.display_name">
            <button
                type="button"
                class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                :class="{ active: pages_store.current_page === page }"
                :data-tip="page.display_name"
                :aria-label="page.display_name"
                :aria-current="pages_store.current_page === page ? 'page' : undefined"
                @click="pages_store.set_page(page)"
            >
              <component
                :is="page.hero_icon"
                class="my-1.5 inline-block size-6"
              />
              <span class="is-drawer-close:hidden">{{ page.display_name }}</span>
            </button>
          </li>
        </ul>
        <div
          class="w-full p-3 text-center is-drawer-close:tooltip is-drawer-close:tooltip-right"
          :data-tip="releaseTag"
        >
          <a
            :href="releaseUrl"
            class="text-xs font-mono text-base-content/60 hover:text-primary transition-colors block truncate cursor-pointer py-1"
            @click.prevent="openRelease"
            @keydown.space.prevent="openRelease"
          >
            {{ releaseTag }}
          </a>
        </div>
      </div>
    </div>
  </div>

  <!-- Startup Configuration Conflict Modal Dialog -->
  <ConfigDialog
    :is-open="conflict !== null"
    type="warning"
    title="Konfigurationskonflikt erkannt"
    description="Aufgrund vorheriger Wiederherstellungsmaßnahmen existiert eine abweichende Recovery-Konfiguration. Bitte wähle, welche Konfiguration dauerhaft beibehalten werden soll:"
    required
  >
    <div v-if="conflict" class="grid grid-cols-1 md:grid-cols-2 gap-4 my-3 text-xs font-mono">
      <div class="p-3 bg-base-200 rounded-box border border-base-300 space-y-1">
        <div class="font-bold font-sans text-sm pb-1 border-b border-base-300 text-base-content flex items-center justify-between">
          <span>Originale Konfiguration</span>
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.mac_address !== conflict.recovery.mac_address }">
          <span class="text-base-content/60">MAC:</span> {{ conflict.original.mac_address }}
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.default_delay !== conflict.recovery.default_delay }">
          <span class="text-base-content/60">Delay:</span> {{ conflict.original.default_delay }} ms
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.default_action_duration !== conflict.recovery.default_action_duration }">
          <span class="text-base-content/60">Dauer:</span> {{ conflict.original.default_action_duration }} ms
        </div>
        <div class="truncate" :title="conflict.original.python_sdk" :class="{ 'text-warning font-semibold': conflict.original.python_sdk !== conflict.recovery.python_sdk }">
          <span class="text-base-content/60">SDK:</span> {{ conflict.original.python_sdk || '-' }}
        </div>
        <div class="truncate" :title="conflict.original.workdir" :class="{ 'text-warning font-semibold': conflict.original.workdir !== conflict.recovery.workdir }">
          <span class="text-base-content/60">Workdir:</span> {{ conflict.original.workdir || '-' }}
        </div>
        <div class="truncate" :title="conflict.original.venv" :class="{ 'text-warning font-semibold': conflict.original.venv !== conflict.recovery.venv }">
          <span class="text-base-content/60">venv:</span> {{ conflict.original.venv || '-' }}
        </div>
      </div>

      <div class="p-3 bg-base-200 rounded-box border border-base-300 space-y-1">
        <div class="font-bold font-sans text-sm pb-1 border-b border-base-300 text-base-content flex items-center justify-between">
          <span>Recovery-Konfiguration</span>
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.mac_address !== conflict.recovery.mac_address }">
          <span class="text-base-content/60">MAC:</span> {{ conflict.recovery.mac_address }}
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.default_delay !== conflict.recovery.default_delay }">
          <span class="text-base-content/60">Delay:</span> {{ conflict.recovery.default_delay }} ms
        </div>
        <div :class="{ 'text-warning font-semibold': conflict.original.default_action_duration !== conflict.recovery.default_action_duration }">
          <span class="text-base-content/60">Dauer:</span> {{ conflict.recovery.default_action_duration }} ms
        </div>
        <div class="truncate" :title="conflict.recovery.python_sdk" :class="{ 'text-warning font-semibold': conflict.original.python_sdk !== conflict.recovery.python_sdk }">
          <span class="text-base-content/60">SDK:</span> {{ conflict.recovery.python_sdk || '-' }}
        </div>
        <div class="truncate" :title="conflict.recovery.workdir" :class="{ 'text-warning font-semibold': conflict.original.workdir !== conflict.recovery.workdir }">
          <span class="text-base-content/60">Workdir:</span> {{ conflict.recovery.workdir || '-' }}
        </div>
        <div class="truncate" :title="conflict.recovery.venv" :class="{ 'text-warning font-semibold': conflict.original.venv !== conflict.recovery.venv }">
          <span class="text-base-content/60">venv:</span> {{ conflict.recovery.venv || '-' }}
        </div>
      </div>
    </div>

    <div v-if="resolve_error" class="alert alert-error text-xs py-2 my-2">
      <span>{{ resolve_error }}</span>
    </div>

    <template #actions>
      <button
        type="button"
        class="btn btn-outline gap-2"
        :disabled="is_resolving"
        @click="resolveConflict('original')"
      >
        <span v-if="is_resolving" class="loading loading-spinner loading-xs"></span>
        <CheckIcon v-else class="size-4" />
        <span>Keep original configuration</span>
      </button>
      <button
        type="button"
        class="btn btn-primary gap-2"
        :disabled="is_resolving"
        @click="resolveConflict('recovery')"
      >
        <span v-if="is_resolving" class="loading loading-spinner loading-xs"></span>
        <CheckIcon v-else class="size-4" />
        <span>Keep recovery configuration</span>
      </button>
    </template>
  </ConfigDialog>

  <!-- Configuration Repair / Recovery Modal Dialog -->
  <ConfigDialog
    :is-open="isRepairModalOpen"
    :type="reportType"
    :title="reportTitle"
    :description="reportDescription"
    @close="dismissRepairReport"
  >
    <div v-if="repair_report" class="space-y-3 my-2 text-xs">
      <!-- Recovery Details Card if present -->
      <div
        v-if="repair_report.recovery_event?.details"
        class="p-3 bg-base-200 rounded-box border border-base-300 space-y-1.5"
      >
        <div
          v-if="repair_report.recovery_event.details.affected_file"
          class="flex flex-col sm:flex-row sm:justify-between gap-1"
        >
          <span class="font-semibold text-base-content/70">Betroffene Datei:</span>
          <span class="font-mono break-all">{{ repair_report.recovery_event.details.affected_file }}</span>
        </div>
        <div
          v-if="repair_report.recovery_event.details.error_cause"
          class="flex flex-col sm:flex-row sm:justify-between gap-1"
        >
          <span class="font-semibold text-base-content/70">Ursache des Fehlers:</span>
          <span class="font-mono text-error break-all">{{ repair_report.recovery_event.details.error_cause }}</span>
        </div>
        <div
          v-if="repair_report.recovery_event.details.recovery_action"
          class="flex flex-col sm:flex-row sm:justify-between gap-1"
        >
          <span class="font-semibold text-base-content/70">Recovery-Maßnahme:</span>
          <span>{{ repair_report.recovery_event.details.recovery_action }}</span>
        </div>
        <div
          v-if="repair_report.recovery_event.details.source"
          class="flex flex-col sm:flex-row sm:justify-between gap-1"
        >
          <span class="font-semibold text-base-content/70">Verwendete Quelle:</span>
          <span>{{ repair_report.recovery_event.details.source }}</span>
        </div>
        <div
          v-if="repair_report.recovery_event.details.result"
          class="flex flex-col sm:flex-row sm:justify-between gap-1"
        >
          <span class="font-semibold text-base-content/70">Ergebnis:</span>
          <span>{{ repair_report.recovery_event.details.result }}</span>
        </div>
      </div>

      <!-- Unrecoverable error if no details card -->
      <div
        v-else-if="repair_report.unrecoverable_error"
        class="p-3 bg-base-200 rounded-box border border-base-300 space-y-1.5"
      >
        <div class="flex flex-col sm:flex-row sm:justify-between gap-1">
          <span class="font-semibold text-base-content/70">Ursache des Fehlers:</span>
          <span class="font-mono text-error break-all">{{ repair_report.unrecoverable_error }}</span>
        </div>
        <div class="flex flex-col sm:flex-row sm:justify-between gap-1">
          <span class="font-semibold text-base-content/70">Ergebnis:</span>
          <span>Standardkonfiguration wurde geladen.</span>
        </div>
      </div>

      <!-- Repairs Table if repairs are present -->
      <div v-if="repair_report.repairs && repair_report.repairs.length > 0" class="w-full">
        <div class="overflow-x-auto max-h-56 overflow-y-auto rounded bg-base-200 border border-base-300 p-2 text-xs">
          <table class="table table-xs w-full">
            <thead>
              <tr class="text-base-content/70 border-b border-base-300">
                <th>Feld</th>
                <th>Ursprünglicher Wert</th>
                <th>Erwarteter Typ</th>
                <th>Grund</th>
                <th>Ergebniswert</th>
                <th>Status</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="repair in repair_report.repairs" :key="repair.field" class="border-b border-base-300/50">
                <td class="font-mono font-semibold">{{ repair.field }}</td>
                <td class="font-mono">{{ JSON.stringify(repair.original_value) }}</td>
                <td class="font-mono">{{ repair.expected_type }}</td>
                <td>{{ repair.reason }}</td>
                <td class="font-mono">{{ JSON.stringify(repair.resulting_value) }}</td>
                <td>
                  <span
                    v-if="repair.repair_method === 'converted' && repair.success"
                    class="badge badge-info badge-xs font-semibold"
                  >
                    Konvertiert
                  </span>
                  <span
                    v-else
                    class="badge badge-warning badge-xs font-semibold"
                  >
                    Standardwert
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </ConfigDialog>
</template>

<style scoped>

</style>
