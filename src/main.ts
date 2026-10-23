import { createApp, nextTick } from "vue";
import App from "./App.vue";
import "@css/style.css";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { Window } from "@tauri-apps/api/window";

createApp(App).mount("#app");

void nextTick().then(() => {
    requestAnimationFrame(() => {
        requestAnimationFrame(async () => {
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
        });
    });
});
