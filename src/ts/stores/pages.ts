import { defineStore } from "pinia";
import { shallowRef, ref } from "vue";
import { PAGES, type Page } from "@ts/pages.ts";

export const usePagesStore = defineStore("pages", () => {
    const current_page = shallowRef<Page>(PAGES.HOME);
    const kargs = ref<Record<string, unknown>>({});

    function set_page(page: Page, new_kargs: Record<string, unknown> = {}) {
        current_page.value = page;
        kargs.value = new_kargs;
    }

    return {
        current_page,
        kargs,
        set_page,
    };
});

export const usePageStore = usePagesStore;
