# BES Book Formatter — Dockerfile (Tauri 2 Build Environment)
# Uso: build de binários Linux (.deb, .AppImage) em CI/CD
#
# NÃO é um servidor web — app desktop Tauri não roda como container.
# Este Dockerfile serve para build cross-platform e CI.

# ─────────────────────────────────────────────
# Stage 1: Dependências Node.js
# ─────────────────────────────────────────────
FROM node:20-slim AS node-deps
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci --legacy-peer-deps

# ─────────────────────────────────────────────
# Stage 2: Ambiente de build Tauri (Linux)
# ─────────────────────────────────────────────
FROM rust:1.77-slim-bullseye AS builder

# System deps obrigatórios para Tauri 2 no Linux
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Webkit (motor do Tauri)
    libwebkit2gtk-4.1-dev \
    # Ferramentas de build
    build-essential \
    curl \
    wget \
    file \
    pkg-config \
    # SSL
    libssl-dev \
    libglib2.0-dev \
    # Ícone e recursos
    librsvg2-dev \
    # AppImage
    libayatana-appindicator3-dev \
    # Squashfs para AppImage
    libfuse2 \
    # Compressão
    libarchive-dev \
    && rm -rf /var/lib/apt/lists/*

# Node.js 20 (via nodesource)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Cachear dependências Rust em layer separado
WORKDIR /app/src-tauri
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock* ./
# Dummy build para cachear deps do Cargo
RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo fetch && \
    rm -rf src

# Copiar node_modules do stage anterior
WORKDIR /app
COPY --from=node-deps /app/node_modules ./node_modules

# Copiar todo o projeto
COPY . .

# Build de produção Tauri (gera .deb e .AppImage em src-tauri/target/release/bundle/)
ARG TAURI_PRIVATE_KEY=""
ARG TAURI_KEY_PASSWORD=""
ENV TAURI_PRIVATE_KEY=${TAURI_PRIVATE_KEY}
ENV TAURI_KEY_PASSWORD=${TAURI_KEY_PASSWORD}

RUN npm run tauri:build

# ─────────────────────────────────────────────
# Stage 3: Extrator de artefatos
# Copia apenas os bundles gerados, sem o toolchain
# ─────────────────────────────────────────────
FROM debian:bullseye-slim AS artifacts
WORKDIR /artifacts
COPY --from=builder /app/src-tauri/target/release/bundle ./bundle
# Os bundles ficam em:
#   bundle/deb/*.deb
#   bundle/appimage/*.AppImage
CMD ["ls", "-lh", "/artifacts/bundle"]
