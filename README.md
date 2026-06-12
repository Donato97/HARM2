# HARM2

Workspace Rust multi-target: stesso `app_core` servito sia come server web
standalone (Axum) sia come app desktop (Tauri). Frontend SolidJS + Vite,
SSR con Maud, query con sea-query + sqlx (SQLite/MySQL via `CustomPool`).

## Struttura

```
Cargo.toml          # workspace: crates/app_core, crates/web, crates/desktop
crates/
  app_core/         # lib condivisa: router, layout Maud, state, query
  web/              # binario server Axum standalone (porta 3000)
  desktop/          # app Tauri v2 (usa app_core come backend)
frontend/           # frontend SolidJS (Vite, output in dist/)
```

## Sviluppo

```bash
npm install                 # dipendenze frontend
npm run dev                 # Vite dev server (porta 1420)
cargo run -p web            # server Axum su http://localhost:3000
cargo build                 # build di tutto il workspace
```

Per la build di produzione: `npm run build` genera `dist/` con il manifest
Vite, che `app_core` legge a runtime per iniettare gli asset hashati.
