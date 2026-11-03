import { createApp, nextTick } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "@css/style.css";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);
app.mount("#app");

let initialized = false;
let fallbackTimeout: ReturnType<typeof setTimeout> | undefined;

async function showMainWindow(): Promise<void> {
    if (initialized) {
        return;
    }
    initialized = true;

    if (fallbackTimeout !== undefined) {
        clearTimeout(fallbackTimeout);
        fallbackTimeout = undefined;
    }

    try {
        // Show the main windows
        const main_window = getCurrentWebviewWindow();
        await main_window.show();

        // Close the splashscreen
        const splashscreen = await Window.getByLabel("splashscreen");
        await splashscreen?.close();

        await main_window.setFocus();
    } catch (error) {
        console.error("Failed to show main window:", error);
    }
}

void nextTick().then(() => {
    fallbackTimeout = setTimeout(() => {
        void showMainWindow();
    }, 10_000);

    requestAnimationFrame(() => {
        requestAnimationFrame(() => {
            void showMainWindow();
        });
    });
});
