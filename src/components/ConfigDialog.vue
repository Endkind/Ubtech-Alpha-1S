<script setup lang="ts">
  import { computed, useTemplateRef, watch } from "vue";
  import type { RecoveryEventType } from "@ts/schemas/config.ts";
  import {
    InformationCircleIcon,
    CheckCircleIcon,
    ExclamationTriangleIcon,
    XCircleIcon,
    XMarkIcon,
  } from "@heroicons/vue/24/outline";

  const props = withDefaults(
    defineProps<{
      isOpen: boolean;
      type?: RecoveryEventType;
      title: string;
      description?: string;
    }>(),
    {
      type: "info",
      description: "",
    }
  );

  const emit = defineEmits<{
    (e: "close"): void;
  }>();

  const dialog = useTemplateRef<HTMLDialogElement>('config-dialog');

  watch(
    () => props.isOpen,
    (open) => {
      if (open) {
        dialog.value?.showModal();
      } else {
        dialog.value?.close();
      }
    }
  );

  function handleCancel() {
    emit('close');
  }

  function handleClose() {
    emit('close');
  }

  const iconComponent = computed(() => {
    switch (props.type) {
      case "success":
        return CheckCircleIcon;
      case "warning":
        return ExclamationTriangleIcon;
      case "error":
        return XCircleIcon;
      case "info":
      default:
        return InformationCircleIcon;
    }
  });

  const borderClass = computed(() => {
    switch (props.type) {
      case "success":
        return "border-success/30";
      case "warning":
        return "border-warning/30";
      case "error":
        return "border-error/30";
      case "info":
      default:
        return "border-info/30";
    }
  });

  const titleColorClass = computed(() => {
    switch (props.type) {
      case "success":
        return "text-success";
      case "warning":
        return "text-warning";
      case "error":
        return "text-error";
      case "info":
      default:
        return "text-info";
    }
  });
</script>

<template>
  <dialog
    ref="config-dialog"
    class="modal z-50"
    @cancel="handleCancel"
    @close="handleClose"
  >
    <div class="modal-box max-w-3xl border shadow-2xl relative bg-base-100" :class="borderClass">
      <!-- Close button (top right) -->
      <button
        type="button"
        class="btn btn-ghost btn-sm btn-circle absolute right-4 top-4"
        aria-label="Schließen"
        @click="$emit('close')"
      >
        <XMarkIcon class="size-5" />
      </button>

      <!-- Header with semantic icon and title -->
      <h3 class="text-lg font-bold flex items-center gap-2 pr-8" :class="titleColorClass">
        <component :is="iconComponent" class="size-6 shrink-0" />
        <span>{{ title }}</span>
      </h3>

      <!-- Description -->
      <p v-if="description" class="py-3 text-sm text-base-content/80">
        {{ description }}
      </p>

      <!-- Main dialog content -->
      <slot />

      <!-- Decision / Actions slot -->
      <div v-if="$slots.actions" class="modal-action flex flex-col sm:flex-row justify-end gap-2 mt-4">
        <slot name="actions" />
      </div>
    </div>
  </dialog>
</template>

<style scoped>
</style>
