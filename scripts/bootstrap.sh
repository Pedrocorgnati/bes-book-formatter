#!/usr/bin/env bash
# bootstrap.sh — Setup completo do ambiente local
# BES Book Formatter — SvelteKit + Rust + Tauri + SQLite
# Gerado por /dev-bootstrap-create (SystemForge)
# Uso: ./scripts/bootstrap.sh [--reset] [--health]
set -euo pipefail

# === Cores ===
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log()  { echo -e "${BLUE}[bootstrap]${NC} $1"; }
ok()   { echo -e "${GREEN}[ok]${NC} $1"; }
warn() { echo -e "${YELLOW}[warn]${NC} $1"; }
err()  { echo -e "${RED}[erro]${NC} $1" >&2; }

# === Pre-requisitos ===
check_prereqs() {
  log "Verificando pre-requisitos..."
  local missing=()

  command -v git >/dev/null 2>&1 || missing+=("git")
  command -v node >/dev/null 2>&1 || missing+=("node")
  command -v npm >/dev/null 2>&1 || missing+=("npm")
  command -v docker >/dev/null 2>&1 || missing+=("docker")
  command -v docker compose >/dev/null 2>&1 || missing+=("docker compose")

  if [ ${#missing[@]} -gt 0 ]; then
    err "Faltando: ${missing[*]}"
    err "Instale os pre-requisitos acima e tente novamente."
    exit 1
  fi
  ok "Pre-requisitos verificados"
}

# === .env ===
ensure_env() {
  if [ -f .env ]; then
    ok ".env ja existe"
    return
  fi

  if [ -f .env.example ]; then
    cp .env.example .env
    ok ".env criado a partir de .env.example"
    warn "Revise .env e preencha valores sensiveis antes de continuar"
  else
    warn ".env nao encontrado. Crie manualmente ou execute /env-creation"
  fi
}

# === .env.docker ===
ensure_env_docker() {
  if [ -f .env.docker ]; then
    ok ".env.docker ja existe"
    return
  fi

  if [ -f .env.docker.example ]; then
    cp .env.docker.example .env.docker
    ok ".env.docker criado a partir de .env.docker.example"
  else
    warn ".env.docker nao encontrado"
  fi
}

# === Dependencias ===
install_deps() {
  log "Instalando dependencias..."
  npm ci --legacy-peer-deps
  ok "Dependencias instaladas"
}

# === Docker ===
start_services() {
  log "Subindo servicos Docker..."
  docker compose up -d dev

  log "Aguardando health checks..."
  local max_wait=60
  local waited=0
  while [ $waited -lt $max_wait ]; do
    if docker compose ps --format json 2>/dev/null | grep -q '"Status"' || \
       docker compose ps 2>/dev/null | grep -q "dev"; then
      break
    fi
    sleep 2
    waited=$((waited + 2))
  done

  if [ $waited -ge $max_wait ]; then
    warn "Timeout esperando Docker ficar pronto (${max_wait}s)"
    warn "Verifique com: docker compose ps"
  else
    ok "Servicos Docker rodando"
  fi
}

stop_services() {
  log "Parando servicos Docker..."
  docker compose down
  ok "Servicos parados"
}

# === Seeds ===
run_seeds() {
  if [ -f scripts/seed.sh ]; then
    log "Executando seeds..."
    bash scripts/seed.sh
    ok "Seeds aplicados"
  else
    warn "scripts/seed.sh nao encontrado — pulando seeds"
  fi
}

# === Health Check (leve) ===
check_health() {
  log "Verificando saude do ambiente..."
  local errors=0

  # .env
  if [ -f .env ]; then
    ok ".env presente"
  else
    warn ".env ausente"
    errors=$((errors + 1))
  fi

  # .env.docker
  if [ -f .env.docker ]; then
    ok ".env.docker presente"
  else
    warn ".env.docker ausente"
    errors=$((errors + 1))
  fi

  # node_modules
  if [ -d node_modules ]; then
    ok "node_modules presente"
  else
    warn "node_modules nao encontrado — execute npm install"
    errors=$((errors + 1))
  fi

  # Docker containers
  if docker compose ps --status running 2>/dev/null | grep -q "dev"; then
    ok "Container Docker (dev) rodando"
  else
    warn "Container Docker (dev) nao esta rodando"
    # Nao incrementa erro — pode ser intencional
  fi

  if [ $errors -eq 0 ]; then
    ok "Ambiente saudavel"
  else
    warn "$errors problema(s) encontrado(s) — verifique acima"
  fi
}

# === Resumo ===
show_summary() {
  echo ""
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo -e "${GREEN}  BOOTSTRAP COMPLETO${NC}"
  echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
  echo ""
  echo "  Projeto:       bes-book-formatter"
  echo "  Runtime:       node (npm)"
  echo "  Stack:         SvelteKit + Rust + Tauri + SQLite"
  echo ""
  echo "  Para iniciar o dev server (SvelteKit web):"
  echo "    npm run dev"
  echo ""
  echo "  Para iniciar o Tauri app:"
  echo "    npm run tauri:dev"
  echo ""
  echo "  Para dev com Docker (watch mode SvelteKit):"
  echo "    docker compose up dev-watch"
  echo ""
  echo "  Para parar servicos Docker:"
  echo "    docker compose down"
  echo ""
  echo "  Para rodar type-check:"
  echo "    npm run check"
  echo ""
  echo "  Para resetar tudo:"
  echo "    ./scripts/bootstrap.sh --reset"
  echo ""
  echo "  Para health check:"
  echo "    ./scripts/bootstrap.sh --health"
  echo ""
}

# === Reset ===
do_reset() {
  warn "Resetando ambiente..."
  docker compose down -v 2>/dev/null || true
  rm -rf node_modules .svelte-kit build dist .next __pycache__ .venv 2>/dev/null || true
  rm -rf src-tauri/target .tauri 2>/dev/null || true
  rm -f .env 2>/dev/null || true
  ok "Ambiente limpo"
  do_setup
}

# === Setup principal ===
do_setup() {
  log "Iniciando bootstrap de bes-book-formatter..."
  echo ""

  check_prereqs
  ensure_env
  ensure_env_docker
  install_deps
  start_services
  run_seeds
  check_health
  show_summary
}

# === Entrypoint ===
cd "$(dirname "${BASH_SOURCE[0]}")/.."

case "${1:-}" in
  --reset) do_reset ;;
  --health) check_health ;;
  *) do_setup ;;
esac
