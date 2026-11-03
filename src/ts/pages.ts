import { Component } from "vue";
import {
    AdjustmentsHorizontalIcon,
    HomeIcon,
    QuestionMarkCircleIcon,
    PencilIcon,
    FolderIcon
} from "@heroicons/vue/24/outline";

export type PageInfo = {
    display_name: string;
    hero_icon: Component;
};

export const PAGES = {
    HOME: {
        display_name: "Home",
        hero_icon: HomeIcon
    },
    EDITOR: {
        display_name: "Editor",
        hero_icon: PencilIcon
    },
    LIBRARY: {
        display_name: "Library",
        hero_icon: FolderIcon
    },
    SETTINGS: {
        display_name: "Settings",
        hero_icon: AdjustmentsHorizontalIcon
    },
    ABOUT: {
        display_name: "About",
        hero_icon: QuestionMarkCircleIcon
    },
} as const satisfies Record<string, PageInfo>;

export type Page = typeof PAGES[keyof typeof PAGES];
