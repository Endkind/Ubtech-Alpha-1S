<script setup lang="ts">
import { usePagesStore } from "@stores/pages.ts";
import { PAGES, type Page } from "@ts/pages.ts";
import {
  PencilIcon,
  FolderIcon,
  AdjustmentsHorizontalIcon,
  QuestionMarkCircleIcon,
  CpuChipIcon,
  ArrowRightIcon,
  SparklesIcon,
} from "@heroicons/vue/24/outline";

const pagesStore = usePagesStore();

function navigateTo(page: Page) {
  pagesStore.set_page(page);
}
</script>

<template>
  <div class="p-6 max-w-5xl mx-auto space-y-8">
    <!-- Hero / Welcome Banner -->
    <div class="card bg-base-100 shadow-xl border border-base-200 overflow-hidden relative">
      <div class="absolute -right-12 -bottom-12 opacity-5 pointer-events-none">
        <CpuChipIcon class="size-64" />
      </div>
      <div class="card-body relative z-10 p-6 md:p-8">
        <div class="flex flex-col md:flex-row md:items-center justify-between gap-6">
          <div class="space-y-3">
            <div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-primary/10 text-primary text-xs font-semibold">
              <SparklesIcon class="size-4" />
              <span>Alpha 1S Control Center</span>
            </div>
            <h1 class="text-3xl md:text-4xl font-extrabold tracking-tight text-base-content">
              Willkommen bei Ubtech Alpha 1S
            </h1>
            <p class="text-base text-base-content/70 max-w-2xl leading-relaxed">
              Steuere deinen humanoiden Roboter, erstelle eigene Bewegungsabläufe im Editor oder verwalte gespeicherte Sequenzen in der Bibliothek.
            </p>
          </div>
          <div class="flex items-center gap-3">
            <button
              class="btn btn-primary shadow-lg shadow-primary/20 gap-2"
              @click="navigateTo(PAGES.EDITOR)"
            >
              <PencilIcon class="size-5" />
              <span>Editor öffnen</span>
            </button>
            <button
              class="btn btn-outline gap-2"
              @click="navigateTo(PAGES.LIBRARY)"
            >
              <FolderIcon class="size-5" />
              <span>Bibliothek</span>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Navigation Grid -->
    <div>
      <h2 class="text-xl font-bold text-base-content mb-4 flex items-center gap-2">
        <span>Hauptbereiche</span>
      </h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Editor Card -->
        <div
          class="card bg-base-100 shadow-xl border border-base-200 hover:border-primary/60 hover:shadow-2xl transition-all duration-300 cursor-pointer group flex flex-col justify-between"
          @click="navigateTo(PAGES.EDITOR)"
        >
          <div class="card-body p-6">
            <div class="flex items-start justify-between mb-4">
              <div class="p-3 bg-primary/10 text-primary rounded-2xl group-hover:scale-110 group-hover:bg-primary group-hover:text-primary-content transition-all duration-300">
                <PencilIcon class="size-7" />
              </div>
              <span class="badge badge-primary badge-outline text-xs">Posen & Sequenzen</span>
            </div>
            <h3 class="card-title text-xl font-bold group-hover:text-primary transition-colors flex items-center justify-between">
              <span>Editor</span>
              <ArrowRightIcon class="size-5 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300 text-primary" />
            </h3>
            <p class="text-sm text-base-content/70 mt-2 leading-relaxed">
              Erstelle und programmiere präzise Servo-Posen, passe Winkel an und teste komplexe Bewegungsabläufe in Echtzeit.
            </p>
          </div>
          <div class="px-6 pb-6 pt-0">
            <div class="pt-4 border-t border-base-200/60 flex items-center justify-between text-xs text-base-content/60 font-medium">
              <span>Bewegungen bearbeiten</span>
              <span class="text-primary font-semibold group-hover:underline">Jetzt starten &rarr;</span>
            </div>
          </div>
        </div>

        <!-- Library Card -->
        <div
          class="card bg-base-100 shadow-xl border border-base-200 hover:border-secondary/60 hover:shadow-2xl transition-all duration-300 cursor-pointer group flex flex-col justify-between"
          @click="navigateTo(PAGES.LIBRARY)"
        >
          <div class="card-body p-6">
            <div class="flex items-start justify-between mb-4">
              <div class="p-3 bg-secondary/10 text-secondary rounded-2xl group-hover:scale-110 group-hover:bg-secondary group-hover:text-secondary-content transition-all duration-300">
                <FolderIcon class="size-7" />
              </div>
              <span class="badge badge-secondary badge-outline text-xs">Aktionen & Dateien</span>
            </div>
            <h3 class="card-title text-xl font-bold group-hover:text-secondary transition-colors flex items-center justify-between">
              <span>Bibliothek</span>
              <ArrowRightIcon class="size-5 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300 text-secondary" />
            </h3>
            <p class="text-sm text-base-content/70 mt-2 leading-relaxed">
              Verwalte gespeicherte Aktionsdateien, organisiere deine Bibliothek und übertrage fertige Choreografien auf den Roboter.
            </p>
          </div>
          <div class="px-6 pb-6 pt-0">
            <div class="pt-4 border-t border-base-200/60 flex items-center justify-between text-xs text-base-content/60 font-medium">
              <span>Sammlung durchsuchen</span>
              <span class="text-secondary font-semibold group-hover:underline">Öffnen &rarr;</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Links / Secondary Options -->
    <div>
      <h2 class="text-xl font-bold text-base-content mb-4">Weitere Optionen</h2>
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <!-- Settings Link Card -->
        <div
          class="card bg-base-100 shadow-md border border-base-200 hover:border-base-300 hover:bg-base-200/30 transition-all cursor-pointer p-4 group"
          @click="navigateTo(PAGES.SETTINGS)"
        >
          <div class="flex items-center gap-4">
            <div class="p-2.5 bg-base-200 rounded-xl text-base-content/80 group-hover:text-primary transition-colors">
              <AdjustmentsHorizontalIcon class="size-5" />
            </div>
            <div class="flex-1 min-w-0">
              <h4 class="font-semibold text-sm text-base-content group-hover:text-primary transition-colors">Einstellungen</h4>
              <p class="text-xs text-base-content/60 truncate">MAC-Adresse und Verbindungsparameter anpassen</p>
            </div>
            <ArrowRightIcon class="size-4 text-base-content/40 group-hover:text-primary group-hover:translate-x-1 transition-all" />
          </div>
        </div>

        <!-- About Link Card -->
        <div
          class="card bg-base-100 shadow-md border border-base-200 hover:border-base-300 hover:bg-base-200/30 transition-all cursor-pointer p-4 group"
          @click="navigateTo(PAGES.ABOUT)"
        >
          <div class="flex items-center gap-4">
            <div class="p-2.5 bg-base-200 rounded-xl text-base-content/80 group-hover:text-primary transition-colors">
              <QuestionMarkCircleIcon class="size-5" />
            </div>
            <div class="flex-1 min-w-0">
              <h4 class="font-semibold text-sm text-base-content group-hover:text-primary transition-colors">Über dieses Projekt</h4>
              <p class="text-xs text-base-content/60 truncate">Technologien, Lizenz und Entwicklerinformationen</p>
            </div>
            <ArrowRightIcon class="size-4 text-base-content/40 group-hover:text-primary group-hover:translate-x-1 transition-all" />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
