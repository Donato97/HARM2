# Fase 1: Build del Frontend (Vite)
FROM node:22-slim AS frontend-builder
WORKDIR /app

COPY package.json package-lock.json ./

RUN npm ci

COPY . .

RUN npm run build

# Fase 2: Build del Backend (Rust)
FROM rust:1.88-slim-bookworm as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev

RUN cargo build --release --bin web

# Fase 3: Build dell'immagine finale
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/web /app/web

COPY --from=frontend-builder /app/dist /app/dist

COPY crates/web/db.sqlite /app/crates/web/db.sqlite

CMD ["/app/web"]