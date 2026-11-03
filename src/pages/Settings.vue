<script setup lang="ts">
  import { computed, onMounted, ref, useTemplateRef } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { Config as ConfigSchema } from "@ts/schemas/config.ts";
  import { dirname, join } from "@ts/utils/path.ts";
  import {
    SignalIcon,
    ClockIcon,
    BoltIcon,
    CommandLineIcon,
    FolderIcon,
    FolderOpenIcon,
    DocumentIcon,
    CheckIcon,
    AdjustmentsHorizontalIcon,
  } from "@heroicons/vue/24/outline";

  const MAC_REGEX = /^(([0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}|([0-9A-Fa-f]{2}-){5}[0-9A-Fa-f]{2})$/;

  const mac_address = ref<string>('');
  const mac_address_input = useTemplateRef<HTMLInputElement>('mac-input');

  const default_delay = ref<number>(500);
  const default_action_duration = ref<number>(50);

  const python_sdk = ref<string>('');
  const workdir = ref<string>('');
  const venv = ref<string>('');

  const is_windows = ref<boolean>(navigator.userAgent.toLowerCase().includes('win'));

  const save_success = ref<boolean>(false);
  const is_saving = ref<boolean>(false);

  const is_mac_address_valid = computed(() => MAC_REGEX.test(mac_address.value));
  const is_default_delay_valid = computed(() => typeof default_delay.value === 'number' && !Number.isNaN(default_delay.value) && default_delay.value >= 0 && default_delay.value <= 2000);
  const is_default_action_duration_valid = computed(() => typeof default_action_duration.value === 'number' && !Number.isNaN(default_action_duration.value) && default_action_duration.value >= 10 && default_action_duration.value <= 200);
  const enable_save_btn = computed(() => is_mac_address_valid.value && is_default_delay_valid.value && is_default_action_duration_valid.value);

  const python_sdk_placeholder = computed(() =>
    is_windows.value
      ? 'z. B. C:\\Projekte\\Ubtech-Alpha-1S-SDK\\main.py'
      : 'z. B. /home/user/Ubtech-Alpha-1S-SDK/main.py'
  );

  const workdir_placeholder = computed(() =>
    is_windows.value
      ? 'z. B. C:\\Projekte\\Ubtech-Alpha-1S-SDK'
      : 'z. B. /home/user/Ubtech-Alpha-1S-SDK'
  );

  const venv_placeholder = computed(() =>
    is_windows.value
      ? 'z. B. C:\\Projekte\\Ubtech-Alpha-1S-SDK\\.venv'
      : 'z. B. /home/user/Ubtech-Alpha-1S-SDK/.venv'
  );

  async function selectPythonSdk() {
    try {
      const selected = await invoke<string | null>("pick_file", {
        defaultPath: workdir.value || undefined,
        filterName: "Python-Skript",
        extensions: ["py"],
      });

      if (selected) {
        python_sdk.value = selected;
        if (!workdir.value || workdir.value.trim() === '') {
          workdir.value = await dirname(selected);
        }
        if (!venv.value || venv.value.trim() === '') {
          venv.value = await join(workdir.value, ".venv");
        }
      }
    } catch (error) {
      console.error("Fehler bei der Dateiauswahl:", error);
    }
  }

  async function selectWorkdir() {
    try {
      const selected = await invoke<string | null>("pick_directory", {
        defaultPath: workdir.value || undefined,
      });

      if (selected) {
        workdir.value = selected;
        if (!python_sdk.value || python_sdk.value.trim() === '') {
          python_sdk.value = await join(selected, "main.py");
        }
        if (!venv.value || venv.value.trim() === '') {
          venv.value = await join(selected, ".venv");
        }
      }
    } catch (error) {
      console.error("Fehler bei der Ordnerauswahl:", error);
    }
  }

  async function selectVenv() {
    try {
      const selected = await invoke<string | null>("pick_directory", {
        defaultPath: workdir.value || undefined,
      });

      if (selected) {
        venv.value = selected;
        if (!workdir.value || workdir.value.trim() === '') {
          workdir.value = await dirname(selected);
        }
        if (!python_sdk.value || python_sdk.value.trim() === '') {
          python_sdk.value = await join(workdir.value, "main.py");
        }
      }
    } catch (error) {
      console.error("Fehler bei der Ordnerauswahl:", error);
    }
  }

  async function save() {
    if (!is_mac_address_valid.value) {
      mac_address_input.value?.reportValidity();
      return;
    }

    if (!is_default_delay_valid.value || !is_default_action_duration_valid.value) {
      return;
    }

    is_saving.value = true;
    try {
      const settings = await invoke("save_settings", {
        update: {
          mac_address: mac_address.value,
          default_delay: default_delay.value,
          default_action_duration: default_action_duration.value,
          python_sdk: python_sdk.value,
          workdir: workdir.value,
          venv: venv.value,
        } as ConfigSchema,
      }) as ConfigSchema;

      mac_address.value = settings.mac_address;
      default_delay.value = settings.default_delay;
      default_action_duration.value = settings.default_action_duration;
      python_sdk.value = settings.python_sdk;
      workdir.value = settings.workdir;
      venv.value = settings.venv;

      save_success.value = true;
      setTimeout(() => {
        save_success.value = false;
      }, 3000);
    } catch (error) {
      console.error("Fehler beim Speichern der Einstellungen:", error);
    } finally {
      is_saving.value = false;
    }
  }

  async function get_settings() {
    try {
      const settings = await invoke("get_settings") as ConfigSchema;

      mac_address.value = settings.mac_address;
      default_delay.value = settings.default_delay;
      default_action_duration.value = settings.default_action_duration;
      python_sdk.value = settings.python_sdk;
      workdir.value = settings.workdir;
      venv.value = settings.venv;
    } catch (error) {
      console.error("Fehler beim Abrufen der Einstellungen:", error);
    }
  }

  onMounted(async () => {
    try {
      const platform = await invoke<string>("get_family");
      is_windows.value = platform === 'windows';
    } catch (error) {
      console.warn("Plattform konnte nicht über Tauri ermittelt werden, Fallback auf userAgent:", error);
    }
    await get_settings();
  });
</script>

<template>
  <div class="p-6 max-w-4xl mx-auto space-y-6">
    <!-- Header Banner -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body p-6">
        <div class="flex items-center gap-3">
          <div class="p-3 bg-primary/10 rounded-2xl text-primary">
            <AdjustmentsHorizontalIcon class="size-8" />
          </div>
          <div>
            <h1 class="card-title text-2xl font-bold">Einstellungen</h1>
            <p class="text-sm text-base-content/70">
              Konfiguriere Verbindung, Aktions- & Verzögerungszeiten sowie die Python-Umgebung
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Verbindung / Bluetooth Section -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body p-6 space-y-4">
        <div class="flex items-center gap-2 pb-2 border-b border-base-200">
          <SignalIcon class="size-5 text-primary" />
          <h2 class="text-lg font-bold">Roboter-Verbindung</h2>
        </div>

        <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4 w-full">
          <legend class="fieldset-legend font-semibold">MAC-Adresse</legend>
          <input
            ref="mac-input"
            type="text"
            id="mac"
            aria-label="MAC-Adresse"
            class="input validator w-full max-w-md font-mono"
            required
            placeholder="00:00:00:00:00:00"
            :pattern="MAC_REGEX.source"
            minlength="17"
            maxlength="17"
            autocomplete="off"
            spellcheck="false"
            title="Bitte eine gültige MAC-Adresse eingeben (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)"
            :disabled="is_saving"
            v-model="mac_address"
          />
          <p class="validator-hint hidden text-xs text-error mt-1">
            Gültige MAC-Adresse erforderlich (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)
          </p>
        </fieldset>
      </div>
    </div>

    <!-- Aktionen & Verzögerungen (Getrennt) -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body p-6 space-y-4">
        <div class="flex items-center gap-2 pb-2 border-b border-base-200">
          <BoltIcon class="size-5 text-primary" />
          <h2 class="text-lg font-bold">Aktionen & Verzögerungszeiten</h2>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Standard-Aktionsdauer -->
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4 flex flex-col justify-between">
            <legend class="fieldset-legend font-semibold flex items-center gap-1.5">
              <BoltIcon class="size-4 text-warning" />
              <span>Standarddauer einer Aktion ({{ default_action_duration }} ms)</span>
            </legend>
            <div class="space-y-2 mt-2">
              <input
                type="range"
                id="default_action_duration"
                aria-label="Standarddauer einer Aktion in Millisekunden"
                min="10"
                max="200"
                step="5"
                class="range range-warning w-full"
                :disabled="is_saving"
                v-model.number="default_action_duration"
              />
              <div class="flex justify-between text-xs text-base-content/60 font-mono">
                <span>10 ms</span>
                <span class="font-semibold text-warning">{{ default_action_duration }} ms</span>
                <span>200 ms</span>
              </div>
              <p class="text-xs text-base-content/70 mt-1">
                Standard-Ausführungsdauer für eine einzelne Roboteraktion (10–200 ms, Standard: 50 ms).
              </p>
            </div>
          </fieldset>

          <!-- Standard-Verzögerung -->
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4 flex flex-col justify-between">
            <legend class="fieldset-legend font-semibold flex items-center gap-1.5">
              <ClockIcon class="size-4 text-primary" />
              <span>Standard-Verzögerung ({{ default_delay }} ms)</span>
            </legend>
            <div class="space-y-2 mt-2">
              <input
                type="range"
                id="default_delay"
                aria-label="Standard-Verzögerung in Millisekunden"
                min="0"
                max="2000"
                step="100"
                class="range range-primary w-full"
                :disabled="is_saving"
                v-model.number="default_delay"
              />
              <div class="flex justify-between text-xs text-base-content/60 font-mono">
                <span>0 ms</span>
                <span class="font-semibold text-primary">{{ default_delay }} ms</span>
                <span>2000 ms</span>
              </div>
              <p class="text-xs text-base-content/70 mt-1">
                Pause/Wartezeit nach jeder ausgeführten Aktion (0–2000 ms, Standard: 500 ms).
              </p>
            </div>
          </fieldset>
        </div>
      </div>
    </div>

    <!-- Python SDK, Workdir & venv Dateiauswahl -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body p-6 space-y-4">
        <div class="flex items-center gap-2 pb-2 border-b border-base-200">
          <CommandLineIcon class="size-5 text-primary" />
          <h2 class="text-lg font-bold">Python-Umgebung & Pfade</h2>
        </div>

        <div class="space-y-4">
          <!-- Python SDK File Entry -->
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4">
            <legend class="fieldset-legend font-semibold flex items-center gap-1.5">
              <DocumentIcon class="size-4 text-primary" />
              <span>Ubtech Alpha 1S SDK (Main-Skript / main.py)</span>
            </legend>
            <div class="flex flex-col sm:flex-row gap-2 mt-1">
              <input
                type="text"
                id="python_sdk"
                aria-label="Ubtech Alpha 1S SDK Pfad zur Python-Main-Datei"
                class="input w-full font-mono text-sm"
                :placeholder="python_sdk_placeholder"
                :disabled="is_saving"
                v-model="python_sdk"
              />
              <button
                type="button"
                class="btn btn-outline gap-2 shrink-0"
                :disabled="is_saving"
                @click="selectPythonSdk"
              >
                <DocumentIcon class="size-4" />
                <span>Datei auswählen</span>
              </button>
            </div>
            <p class="text-xs text-base-content/60 mt-1">
              Pfad zur Python-Main-Datei des „Ubtech Alpha 1S SDK“-Projekts (z. B. `main.py`).
            </p>
          </fieldset>

          <!-- Workdir Folder Entry -->
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4">
            <legend class="fieldset-legend font-semibold flex items-center gap-1.5">
              <FolderIcon class="size-4 text-secondary" />
              <span>Arbeitsverzeichnis (Workdir)</span>
            </legend>
            <div class="flex flex-col sm:flex-row gap-2 mt-1">
              <input
                type="text"
                id="workdir"
                aria-label="Arbeitsverzeichnis Pfad"
                class="input w-full font-mono text-sm"
                :placeholder="workdir_placeholder"
                :disabled="is_saving"
                v-model="workdir"
              />
              <button
                type="button"
                class="btn btn-outline gap-2 shrink-0"
                :disabled="is_saving"
                @click="selectWorkdir"
              >
                <FolderOpenIcon class="size-4" />
                <span>Ordner auswählen</span>
              </button>
            </div>
            <p class="text-xs text-base-content/60 mt-1">
              Basis-Arbeitsverzeichnis des SDK-Projekts für Skripte und Aktionen.
            </p>
          </fieldset>

          <!-- venv Folder Entry -->
          <fieldset class="fieldset bg-base-200 border-base-300 rounded-box border p-4">
            <legend class="fieldset-legend font-semibold flex items-center gap-1.5">
              <FolderIcon class="size-4 text-accent" />
              <span>Virtuelle Umgebung (venv)</span>
            </legend>
            <div class="flex flex-col sm:flex-row gap-2 mt-1">
              <input
                type="text"
                id="venv"
                aria-label="Virtuelle Umgebung Pfad"
                class="input w-full font-mono text-sm"
                :placeholder="venv_placeholder"
                :disabled="is_saving"
                v-model="venv"
              />
              <button
                type="button"
                class="btn btn-outline gap-2 shrink-0"
                :disabled="is_saving"
                @click="selectVenv"
              >
                <FolderOpenIcon class="size-4" />
                <span>Ordner auswählen</span>
              </button>
            </div>
            <p class="text-xs text-base-content/60 mt-1">
              Pfad zum Virtualenv-Verzeichnis (.venv) des SDK-Projekts.
            </p>
          </fieldset>
        </div>
      </div>
    </div>

    <!-- Speichern Button & Status -->
    <div class="flex items-center gap-4">
      <button
        type="button"
        class="btn btn-primary gap-2 shadow-lg shadow-primary/20"
        :disabled="!enable_save_btn || is_saving"
        @click="save"
      >
        <span v-if="is_saving" class="loading loading-spinner loading-xs"></span>
        <CheckIcon v-else class="size-5" />
        <span>{{ is_saving ? 'Speichert...' : 'Einstellungen speichern' }}</span>
      </button>

      <div
        v-if="save_success"
        class="badge badge-success gap-1.5 p-3 text-success-content font-medium transition-all"
      >
        <CheckIcon class="size-4" />
        <span>Erfolgreich gespeichert!</span>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>
