<script setup lang="ts">
  import { PAGES } from "@ts/pages.ts";
  import { computed, onMounted, ref, shallowRef } from "vue";
  import { getVersion } from "@tauri-apps/api/app";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import pkg from "../package.json";
  import About from "@pages/About.vue";
  import Settings from "@pages/Settings.vue";
  import Home from "@pages/Home.vue";

  type Page = typeof PAGES[keyof typeof PAGES];

  const current_page = shallowRef<Page>(PAGES.HOME);

  function set_page(page: Page) {
    current_page.value = page;
  }

  const version = ref(pkg.version || "0.1.0");
  const releaseTag = computed(() => `v${version.value.replace(/^v/, "")}`);
  const releaseUrl = computed(
    () => `https://github.com/Endkind/Ubtech-Alpha-1S/releases/tag/${releaseTag.value}`
  );

  onMounted(async () => {
    try {
      version.value = await getVersion();
    } catch (error) {
      console.error("Fehler beim Abrufen der Tauri-Version:", error);
      version.value = pkg.version;
    }
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
        <label for="drawer" aria-label="open sidebar" class="btn btn-square btn-ghost drawer-button">
          <!-- Sidebar toggle icon -->
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor" class="my-1.5 inline-block size-4"><path d="M4 4m0 2a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v12a2 2 0 0 1 -2 2h-12a2 2 0 0 1 -2 -2z"></path><path d="M9 4v16"></path><path d="M14 10l2 2l-2 2"></path></svg>
        </label>
        <div class="px-4">{{ current_page.display_name }}</div>
      </nav>
      <!-- Page content here -->
      <div class="flex flex-1 min-h-0 overflow-auto">
        <div class="grow p-2">
          <Home v-if="current_page == PAGES.HOME" />
          <About v-else-if="current_page == PAGES.ABOUT" />
          <Settings v-else-if="current_page == PAGES.SETTINGS" />
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
                class="is-drawer-close:tooltip is-drawer-close:tooltip-right"
                :data-tip="page.display_name"
                @click="set_page(page)"
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
          >
            {{ releaseTag }}
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>
