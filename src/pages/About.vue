<script setup lang="ts">
import {
  CodeBracketIcon,
  CpuChipIcon,
  UserIcon,
  ArrowTopRightOnSquareIcon,
  DocumentTextIcon,
  SparklesIcon,
  CommandLineIcon
} from '@heroicons/vue/24/outline';

interface LinkItem {
  name: string;
  description: string;
  tag: string;
  url: string;
}

const repositoryLinks = [
  {
    name: 'GitHub Repository',
    description: 'Vollständiger Quellcode und Dokumentation',
    tag: 'Repository',
    url: 'https://github.com/Endkind/Ubtech-Alpha-1S',
    icon: CodeBracketIcon,
  },
  {
    name: 'MIT-Lizenz',
    description: 'Open-Source MIT-Lizenz (LICENSE)',
    tag: 'MIT',
    url: 'https://github.com/Endkind/Ubtech-Alpha-1S/blob/master/LICENSE',
    icon: DocumentTextIcon,
  },
];

const technologies: LinkItem[] = [
  {name: 'Tauri', description: 'Leichtes Desktop-Framework', tag: 'Backend & App Shell', url: 'https://tauri.app'},
  {name: 'Vue.js', description: 'Reaktives Frontend-Framework', tag: 'UI Framework', url: 'https://vuejs.org'},
  {
    name: 'Pinia',
    description: 'Intuitive State-Management-Bibliothek für Vue',
    tag: 'State Management',
    url: 'https://pinia.vuejs.org'
  },
  {name: 'Tailwind CSS', description: 'Utility-First CSS Framework', tag: 'Styling', url: 'https://tailwindcss.com'},
  {
    name: 'daisyUI',
    description: 'Komponenten-Bibliothek für Tailwind',
    tag: 'UI Components',
    url: 'https://daisyui.com'
  },
  {name: 'Heroicons', description: 'Saubere Icon-Bibliothek', tag: 'Icons', url: 'https://heroicons.com'},
  {
    name: 'pre-commit',
    description: 'Automatisierte Git-Hook-Prüfungen & Code-Qualität',
    tag: 'Tooling',
    url: 'https://pre-commit.com'
  },
];

const aiAssistants: LinkItem[] = [
  {
    name: 'JetBrains Junie',
    description: 'Autonomer Coding-Agent & direkte Code-Reviews in der IDE',
    tag: 'Coding Agent',
    url: 'https://www.jetbrains.com/junie/',
  },
  {
    name: 'ChatGPT',
    description: 'Konzeption, Architekturberatung & Code-Unterstützung',
    tag: 'KI-Assistenz',
    url: 'https://chatgpt.com',
  },
  {
    name: 'GitHub Copilot',
    description: 'KI-Pair-Programming & Code-Vorschläge',
    tag: 'Code Completion',
    url: 'https://github.com/features/copilot',
  },
];
</script>

<template>
  <div class="p-6 max-w-4xl mx-auto space-y-6">
    <!-- Header / Hero Card -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body">
        <div class="flex items-center gap-3">
          <div class="p-3 bg-primary/10 rounded-2xl text-primary">
            <CpuChipIcon class="size-8"/>
          </div>
          <div>
            <h1 class="card-title text-2xl font-bold">Ubtech Alpha 1S</h1>
            <p class="text-sm text-base-content/70">Steuerungs- & Verwaltungssoftware</p>
          </div>
        </div>

        <p class="mt-4 text-base-content/80 leading-relaxed">
          Diese Anwendung wurde speziell für den humanoiden Roboter <span class="font-semibold text-base-content">Ubtech Alpha 1S</span>
          entwickelt,
          um eine intuitive Steuerung und Interaktion über moderne Desktop-Technologien zu ermöglichen.
        </p>
      </div>
    </div>

    <!-- Developer & Repository Links Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Entwickler Card -->
      <a
          href="https://github.com/Endkind"
          target="_blank"
          rel="noopener noreferrer"
          class="card bg-base-100 shadow-xl border border-base-200 hover:border-primary/50 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-base-100 transition-all cursor-pointer group"
      >
        <div class="card-body">
          <div class="flex items-center justify-between mb-2">
            <div class="flex items-center gap-2">
              <UserIcon class="size-5 text-secondary"/>
              <h2 class="card-title text-lg">Entwickler</h2>
            </div>
            <ArrowTopRightOnSquareIcon class="size-4 text-base-content/40 group-hover:text-primary transition-colors"/>
          </div>
          <p class="text-sm text-base-content/70">
            Dieses Programm wurde entwickelt und programmiert von:
          </p>
          <div class="mt-3 p-3 bg-base-200/60 rounded-xl flex items-center justify-between">
            <div class="font-medium text-base">Endkind Ender</div>
            <span class="badge badge-primary badge-outline text-xs">@Endkind</span>
          </div>
        </div>
      </a>

      <!-- Repository & Lizenz Card -->
      <div class="card bg-base-100 shadow-xl border border-base-200">
        <div class="card-body">
          <div class="flex items-center gap-2 mb-2">
            <CommandLineIcon class="size-5 text-primary"/>
            <h2 class="card-title text-lg">Quellcode & Lizenz</h2>
          </div>
          <p class="text-sm text-base-content/70 mb-3">
            Open-Source-Ressourcen und Lizenzinformationen:
          </p>
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
            <a
                v-for="item in repositoryLinks"
                :key="item.name"
                class="p-3 bg-base-200/50 hover:bg-base-200 rounded-xl border border-base-200 hover:border-primary/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-base-100 transition-all cursor-pointer flex flex-col justify-between group"
                :href="item.url"
                target="_blank"
                rel="noopener noreferrer"
            >
              <div>
                <div class="flex items-center justify-between mb-1">
                  <span class="font-bold text-sm text-base-content group-hover:text-primary transition-colors">
                    {{ item.name }}
                  </span>
                  <ArrowTopRightOnSquareIcon
                      class="size-3.5 text-base-content/40 group-hover:text-primary transition-colors"/>
                </div>
                <p class="text-xs text-base-content/70">{{ item.description }}</p>
              </div>
              <div class="mt-2">
                <span class="badge badge-xs badge-ghost text-[10px]">{{ item.tag }}</span>
              </div>
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- Tech Stack Card -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body">
        <div class="flex items-center gap-2 mb-2">
          <CodeBracketIcon class="size-5 text-primary"/>
          <h2 class="card-title text-lg">Eingesetzte Technologien & Werkzeuge</h2>
        </div>
        <p class="text-sm text-base-content/70 mb-4">
          Die Anwendung und der Entwicklungs-Workflow basieren auf modernen, modularen Komponenten (klicken für
          Details):
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          <a
              v-for="tech in technologies"
              :key="tech.name"
              class="p-3 bg-base-200/50 hover:bg-base-200 rounded-xl border border-base-200 hover:border-primary/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2 focus-visible:ring-offset-base-100 transition-all cursor-pointer flex flex-col justify-between group"
              :href="tech.url"
              target="_blank"
              rel="noopener noreferrer"
          >
            <div>
              <div class="flex items-center justify-between mb-1">
                <span class="font-bold text-sm text-base-content group-hover:text-primary transition-colors">
                  {{ tech.name }}
                </span>
                <ArrowTopRightOnSquareIcon
                    class="size-3.5 text-base-content/40 group-hover:text-primary transition-colors"/>
              </div>
              <p class="text-xs text-base-content/70">{{ tech.description }}</p>
            </div>
            <div class="mt-2">
              <span class="badge badge-xs badge-ghost text-[10px]">{{ tech.tag }}</span>
            </div>
          </a>
        </div>
      </div>
    </div>

    <!-- AI Assistance & Code Reviewers Card -->
    <div class="card bg-base-100 shadow-xl border border-base-200">
      <div class="card-body">
        <div class="flex items-center gap-2 mb-2">
          <SparklesIcon class="size-5 text-accent"/>
          <h2 class="card-title text-lg">KI-Unterstützung & Code-Review</h2>
        </div>
        <p class="text-sm text-base-content/70 mb-4">
          Folgende KI-Systeme haben als Programmierunterstützung und Code-Reviewer aktiv mitgeholfen:
        </p>

        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
          <a
              v-for="ai in aiAssistants"
              :key="ai.name"
              class="p-3 bg-base-200/50 hover:bg-base-200 rounded-xl border border-base-200 hover:border-accent/40 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-accent focus-visible:ring-offset-2 focus-visible:ring-offset-base-100 transition-all cursor-pointer flex flex-col justify-between group"
              :href="ai.url"
              target="_blank"
              rel="noopener noreferrer"
          >
            <div>
              <div class="flex items-center justify-between mb-1">
                <span class="font-bold text-sm text-base-content group-hover:text-accent transition-colors">
                  {{ ai.name }}
                </span>
                <ArrowTopRightOnSquareIcon
                    class="size-3.5 text-base-content/40 group-hover:text-accent transition-colors"/>
              </div>
              <p class="text-xs text-base-content/70">{{ ai.description }}</p>
            </div>
            <div class="mt-2">
              <span class="badge badge-xs badge-ghost text-[10px]">{{ ai.tag }}</span>
            </div>
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
