<script setup lang="ts">
  import { computed, onMounted, ref, useTemplateRef } from "vue";
  import { invoke } from "@tauri-apps/api/core";
  import { Config as ConfigSchema } from "@ts/schemas/config.ts";

  const MAC_REGEX = /^([0-9A-Fa-f]{2}[:\-]){5}([0-9A-Fa-f]{2})$/;

  const mac_address = ref<string>('');
  const mac_address_input = useTemplateRef<HTMLInputElement>('mac-input');

  const delay = ref<number>(500);

  const is_mac_address_valid = computed(() => MAC_REGEX.test(mac_address.value));
  const is_delay_valid = computed(() => typeof delay.value === 'number' && !Number.isNaN(delay.value) && delay.value >= 0 && delay.value <= 2000);
  const enable_save_btn = computed(() => is_mac_address_valid.value && is_delay_valid.value);

  async function save() {
    if (!is_mac_address_valid.value) {
      mac_address_input.value?.reportValidity();
      return;
    }

    if (!is_delay_valid.value) {
      return;
    }

    const settings = await invoke("save_settings", {
      update: {
        mac_address: mac_address.value,
        delay: delay.value,
      } as ConfigSchema,
    }) as ConfigSchema;

    mac_address.value = settings.mac_address;
    delay.value = settings.delay;
  }

  async function get_settings() {
    const settings = await invoke("get_settings") as ConfigSchema;

    mac_address.value = settings.mac_address;
    delay.value = settings.delay;
  }

  onMounted(() => {
    get_settings();
  });
</script>

<template>
  <div class="flex flex-col gap-3 items-start">
    <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
      <legend class="fieldset-legend">MAC-Adresse</legend>
      <input
        ref="mac-input"
        type="text"
        id="mac"
        class="input validator"
        required
        placeholder="00:00:00:00:00:00"
        :pattern="MAC_REGEX.source"
        minlength="17"
        maxlength="17"
        autocomplete="off"
        spellcheck="false"
        title="Bitte eine gültige MAC-Adresse eingeben (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)"
        v-model="mac_address"
      />
      <p class="validator-hint hidden">
        Gültige MAC-Adresse erforderlich (z. B. 00:11:22:33:44:55 oder 00-11-22-33-44-55)
      </p>
    </fieldset>

    <fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
      <legend class="fieldset-legend">Standard-Verzögerung nach jeder Aktion ({{ delay }} ms)</legend>
      <input
        type="range"
        id="delay"
        min="0"
        max="2000"
        step="100"
        class="range"
        v-model.number="delay"
      />
    </fieldset>

    <button class="btn btn-primary" :disabled="!enable_save_btn" @click="save">
      Speichern
    </button>
  </div>
</template>

<style scoped>

</style>
