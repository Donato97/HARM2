import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  build: {
    watch: {
      include: ["frontend/**/*"],
    },
    manifest: true,
    rollupOptions: {
      input: {
        editor: "frontend/editor.tsx",
        htmx: "frontend/htmx.ts",
        css: "frontend/style.css",
      },
    },
  },
});
