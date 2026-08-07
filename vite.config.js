import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(({ mode }) => {
    const config = {
        plugins: [solid(), tailwindcss()],
        build: {
            manifest: true,
            rollupOptions: {
                input: {
                    editor: "frontend/editor.tsx",
                    htmx: "frontend/htmx.ts",
                    css: "frontend/style.css",
                    main: "frontend/main.ts"
                },
            },
        },
    };
    if (mode === "") {
        config.build.watch = {
            include: ["frontend/**/*"],
        };
    }
    return config;
});
