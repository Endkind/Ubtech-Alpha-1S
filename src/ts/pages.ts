import { Component } from "vue";
import {AdjustmentsHorizontalIcon, HomeIcon, QuestionMarkCircleIcon} from "@heroicons/vue/24/outline";

export type PageInfo = {
    display_name: string;
    hero_icon: Component;
};

export const PAGES = {
    HOME: {
        display_name: "Home",
        hero_icon: HomeIcon
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
