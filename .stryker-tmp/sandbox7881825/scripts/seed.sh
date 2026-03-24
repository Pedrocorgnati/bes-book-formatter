#!/usr/bin/env bash
# =========================================================================
# BES Book Formatter — Script de Seed
# Aplica o seed.sql ao banco SQLite do app (desenvolvimento)
# =========================================================================
# Uso:
#   ./scripts/seed.sh            # aplica seed ao banco do app
#   ./scripts/seed.sh --reset    # apaga o banco e reaplica todas as migrations + seed
#   ./scripts/seed.sh --db PATH  # usa banco customizado (ex: ./test.db)
#
# Pré-requisitos: sqlite3 instalado no PATH
# =========================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
SEED_FILE="$WORKSPACE_DIR/src-tauri/seeds/seed.sql"
MIGRATIONS_DIR="$WORKSPACE_DIR/src-tauri/migrations"
APP_IDENTIFIER="com.corgnati.bes-book-formatter"
DB_NAME="bes-book-formatter.db"

# ── Detectar caminho do banco do app por OS ──────────────────────────────
detect_db_path() {
    local os
    os="$(uname -s)"
    case "$os" in
        Linux)
            echo "${XDG_DATA_HOME:-$HOME/.local/share}/$APP_IDENTIFIER/$DB_NAME"
            ;;
        Darwin)
            echo "$HOME/Library/Application Support/$APP_IDENTIFIER/$DB_NAME"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            echo "${APPDATA:-$HOME/AppData/Roaming}/$APP_IDENTIFIER/$DB_NAME"
            ;;
        *)
            echo "SO não reconhecido: $os" >&2
            exit 1
            ;;
    esac
}

# ── Verificar dependências ───────────────────────────────────────────────
check_deps() {
    if ! command -v sqlite3 &>/dev/null; then
        echo "ERRO: sqlite3 não encontrado no PATH."
        echo "  Linux:  sudo apt install sqlite3"
        echo "  macOS:  brew install sqlite"
        exit 1
    fi
}

# ── Aplicar migrations ───────────────────────────────────────────────────
apply_migrations() {
    local db="$1"
    echo "Aplicando migrations em: $db"
    for migration in "$MIGRATIONS_DIR"/M*.sql; do
        echo "  → $(basename "$migration")"
        sqlite3 "$db" < "$migration"
    done
}

# ── Main ─────────────────────────────────────────────────────────────────
main() {
    check_deps

    local db_path=""
    local reset=false

    # Processar argumentos
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --reset)
                reset=true
                shift
                ;;
            --db)
                db_path="$2"
                shift 2
                ;;
            *)
                echo "Uso: $0 [--reset] [--db CAMINHO]"
                exit 1
                ;;
        esac
    done

    # Resolver caminho do banco
    if [[ -z "$db_path" ]]; then
        db_path="$(detect_db_path)"
    fi

    echo "======================================================"
    echo " BES Book Formatter — Seed de Desenvolvimento"
    echo "======================================================"
    echo " Banco: $db_path"
    echo " Seed:  $SEED_FILE"
    echo ""

    # Verificar se o arquivo de seed existe
    if [[ ! -f "$SEED_FILE" ]]; then
        echo "ERRO: seed.sql não encontrado em $SEED_FILE"
        exit 1
    fi

    # Reset: apagar banco e recriar
    if [[ "$reset" == true ]]; then
        echo "⚠ RESET solicitado. O banco será apagado e recriado."
        read -r -p "  Confirmar? (s/N): " confirm
        if [[ "${confirm,,}" != "s" ]]; then
            echo "Cancelado."
            exit 0
        fi
        rm -f "$db_path" "${db_path}-shm" "${db_path}-wal"
        echo "  Banco apagado."
        mkdir -p "$(dirname "$db_path")"
        apply_migrations "$db_path"
        echo ""
    fi

    # Verificar se o banco existe
    if [[ ! -f "$db_path" ]]; then
        echo "AVISO: banco não encontrado em $db_path"
        echo "  Execute o app pelo menos uma vez antes de aplicar o seed,"
        echo "  ou use --reset para criar o banco e aplicar migrations."
        echo ""
        read -r -p "  Criar banco e aplicar migrations agora? (s/N): " confirm
        if [[ "${confirm,,}" != "s" ]]; then
            echo "Cancelado."
            exit 0
        fi
        mkdir -p "$(dirname "$db_path")"
        apply_migrations "$db_path"
        echo ""
    fi

    # Aplicar seed
    echo "Aplicando seed..."
    sqlite3 "$db_path" < "$SEED_FILE"
    echo ""

    # Resumo
    echo "Resumo do banco após seed:"
    sqlite3 -column -header "$db_path" \
        "SELECT 'projects' AS tabela, COUNT(*) AS registros FROM projects
         UNION ALL SELECT 'illustrations', COUNT(*) FROM illustrations
         UNION ALL SELECT 'user_preferences', COUNT(*) FROM user_preferences
         UNION ALL SELECT 'typography_configs', COUNT(*) FROM typography_configs
         UNION ALL SELECT 'generation_results', COUNT(*) FROM generation_results
         UNION ALL SELECT 'annotations', COUNT(*) FROM annotations
         UNION ALL SELECT 'bes_document_cache', COUNT(*) FROM bes_document_cache
         UNION ALL SELECT 'cover_configs', COUNT(*) FROM cover_configs;"

    echo ""
    echo "======================================================"
    echo " Seed aplicado com sucesso!"
    echo "======================================================"
}

main "$@"
