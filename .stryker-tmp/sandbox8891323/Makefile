# Makefile — BES Book Formatter
# Gerado por /dev-bootstrap-create (SystemForge)
# Uso: make <target>

.PHONY: setup reset dev tauri-dev docker-dev docker-watch check lint build test help health bench bench-rust bench-sidecars bench-smoke

# === Bootstrap ===
setup:
	@./scripts/bootstrap.sh

reset:
	@./scripts/bootstrap.sh --reset

health:
	@./scripts/bootstrap.sh --health

# === Development ===
dev:
	npm run dev

tauri-dev:
	npm run tauri:dev

docker-dev:
	docker compose run --rm dev bash

docker-watch:
	docker compose up dev-watch

# === Building ===
check:
	npm run check

lint:
	npm run lint

build:
	npm run build

tauri-build:
	npm run tauri:build

# === Testing ===
test:
	npm run docker:test

# === Performance Benchmarks ===
bench: bench-rust bench-sidecars

bench-rust:
	cd src-tauri && cargo bench

bench-sidecars:
	./tests/perf/bench-sidecars.sh full

bench-smoke:
	cd src-tauri && cargo bench -- --quick
	./tests/perf/bench-sidecars.sh smoke

# === Database ===
seed:
	npm run db:seed

seed-reset:
	npm run db:reset

# === Docker ===
docker-down:
	docker compose down

docker-clean:
	docker compose down -v

# === Help ===
help:
	@echo "BES Book Formatter — Makefile Commands"
	@echo ""
	@echo "Bootstrap:"
	@echo "  make setup         Setup completo do ambiente local"
	@echo "  make reset         Limpa tudo e refaz setup"
	@echo "  make health        Verifica saude do ambiente"
	@echo ""
	@echo "Development:"
	@echo "  make dev           Roda SvelteKit dev server (npm run dev)"
	@echo "  make tauri-dev     Roda Tauri app com hot-reload"
	@echo "  make docker-dev    Abre bash no container Docker (dev)"
	@echo "  make docker-watch  Watch mode SvelteKit em Docker"
	@echo ""
	@echo "Building & Checking:"
	@echo "  make check         Type-check SvelteKit"
	@echo "  make lint          Lint eslint"
	@echo "  make build         Build SvelteKit + Tauri"
	@echo "  make tauri-build   Build Tauri apenas"
	@echo ""
	@echo "Testing & Database:"
	@echo "  make test          Roda testes Rust em Docker"
	@echo "  make seed          Aplica seed ao banco SQLite"
	@echo "  make seed-reset    Reseta e reaplica seeds"
	@echo ""
	@echo "Performance Benchmarks:"
	@echo "  make bench         Rust benchmarks + sidecar benchmarks (full)"
	@echo "  make bench-rust    Criterion benchmarks (SQLite, parser, imagens)"
	@echo "  make bench-sidecars Hyperfine benchmarks (Typst, Ghostscript, EPUBCheck)"
	@echo "  make bench-smoke   Validacao rapida (3 runs por cenario)"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-down   Para containers"
	@echo "  make docker-clean  Para e remove volumes"
	@echo ""
	@echo "Other:"
	@echo "  make help          Mostra esta mensagem"
