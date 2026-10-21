# Pfad-Aliase konfigurieren (Vite & TypeScript)

Um die Effizienz beim Entwickeln zu maximieren, die Lesbarkeit zu verbessern und fehleranfällige relative Pfade (z. B. `../../`) zu vermeiden, werden im Projekt Pfad-Aliase verwendet.

Die Konfiguration muss an zwei Stellen synchron gepflegt werden:
1. **Vite (`vite.config.ts`)**: Zuständig für das Bundling und das Auflösen der Module zur Laufzeit / beim Build.
2. **TypeScript (`tsconfig.json`)**: Zuständig für die statische Typanalyse, Autovervollständigung und Navigation in der IDE.

---

## 1. Vite-Konfiguration (`vite.config.ts`)

In `vite.config.ts` wird das Modul `fileURLToPath` und `URL` aus `node:url` importiert, um absolute Pfade robust und plattformunabhängig aufzulösen.

```typescript
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@assets": fileURLToPath(new URL("./src/assets", import.meta.url)),
      "@components": fileURLToPath(new URL("./src/components", import.meta.url)),
      "@css": fileURLToPath(new URL("./src/css", import.meta.url)),
      "@pages": fileURLToPath(new URL("./src/pages", import.meta.url)),
      "@ts": fileURLToPath(new URL("./src/ts", import.meta.url)),
      // Hier weitere Aliase ergänzen...
    },
  },
});
```

---

## 2. TypeScript-Konfiguration (`tsconfig.json`)

Damit der TypeScript-Compiler (`vue-tsc`) und die IDE die Aliase korrekt auflösen können, müssen diese unter `compilerOptions.paths` eingetragen werden. Es wird empfohlen, sowohl das Verzeichnis selbst als auch Unterpfade (`/*`) zu mappen.

```json
{
  "compilerOptions": {
    /* Weitere Optionen... */
    "paths": {
      "@assets": ["./src/assets"],
      "@assets/*": ["./src/assets/*"],
      "@components": ["./src/components"],
      "@components/*": ["./src/components/*"],
      "@css": ["./src/css"],
      "@css/*": ["./src/css/*"],
      "@pages": ["./src/pages"],
      "@pages/*": ["./src/pages/*"],
      "@ts": ["./src/ts"],
      "@ts/*": ["./src/ts/*"]
    }
  }
}
```

---

## 3. Schritt-für-Schritt: Neuen Alias hinzufügen

Um einen neuen Alias hinzuzufügen (z. B. `@services` -> `./src/services`):

1. **Vite anpassen (`vite.config.ts`)**:
   ```typescript
   "@services": fileURLToPath(new URL("./src/services", import.meta.url)),
   ```

2. **TypeScript anpassen (`tsconfig.json`)**:
   ```json
   "@services": ["./src/services"],
   "@services/*": ["./src/services/*"]
   ```

3. **Verifizieren**:
   Führe den Build-Befehl aus, um sicherzustellen, dass sowohl die Typüberprüfung als auch das Bundling fehlerfrei funktionieren:
   ```bash
   pnpm run build
   ```

---

## 4. Verwendung im Code

```vue
<script setup lang="ts">
import { PAGES } from "@ts/pages.ts";
import HeaderComponent from "@components/HeaderComponent.vue";
import "@css/style.css";
</script>
```
