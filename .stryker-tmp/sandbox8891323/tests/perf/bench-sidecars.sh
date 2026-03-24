#!/bin/bash
# bench-sidecars.sh — Benchmark de sidecars externos do BES Book Formatter
# Mede latência de Typst, Ghostscript e EPUBCheck com hyperfine
#
# Uso: ./tests/perf/bench-sidecars.sh [smoke|full]
# Requisitos: hyperfine, typst, gs (ghostscript), java (para EPUBCheck)
#
# smoke: 3 execuções por cenário (validação rápida)
# full:  10 execuções por cenário (dados estatísticos)

set -euo pipefail

MODE="${1:-smoke}"
RESULTS_DIR="tests/perf/results"
FIXTURES_DIR="tests/perf/fixtures"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
REPORT_FILE="${RESULTS_DIR}/sidecars_${TIMESTAMP}.json"

mkdir -p "$RESULTS_DIR" "$FIXTURES_DIR"

# ── Cores ──
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
fail()  { echo -e "${RED}[FAIL]${NC} $1"; }

# ── Configuração por modo ──
if [[ "$MODE" == "full" ]]; then
    RUNS=10
    WARMUP=3
    info "Modo FULL: ${RUNS} execuções, ${WARMUP} warmup"
else
    RUNS=3
    WARMUP=1
    info "Modo SMOKE: ${RUNS} execuções, ${WARMUP} warmup"
fi

# ── Verificar dependências ──
check_tool() {
    if command -v "$1" &>/dev/null; then
        local version
        version=$($2 2>&1 | head -1)
        ok "$1 encontrado: $version"
        return 0
    else
        warn "$1 não encontrado — pulando benchmarks de $1"
        return 1
    fi
}

HAS_HYPERFINE=false
HAS_TYPST=false
HAS_GS=false
HAS_JAVA=false

echo ""
info "Verificando dependências..."
check_tool "hyperfine" "hyperfine --version" && HAS_HYPERFINE=true
check_tool "typst" "typst --version" && HAS_TYPST=true
check_tool "gs" "gs --version" && HAS_GS=true
check_tool "java" "java -version" && HAS_JAVA=true

if [[ "$HAS_HYPERFINE" == "false" ]]; then
    fail "hyperfine é obrigatório. Instale com: cargo install hyperfine (ou brew install hyperfine)"
    echo ""
    echo "Alternativa sem hyperfine — use 'time' manualmente:"
    echo "  time typst compile tests/perf/fixtures/single-page.typ /dev/null"
    exit 1
fi

# ── Gerar fixtures ──
echo ""
info "Gerando fixtures de teste..."

# Fixture: página única Typst (preview)
cat > "${FIXTURES_DIR}/single-page.typ" << 'TYPST_EOF'
#set page(width: 6in, height: 9in, margin: (top: 1in, bottom: 1in, left: 0.75in, right: 0.75in))
#set text(font: "Linux Libertine", size: 11pt, lang: "pt")
#set par(justify: true, leading: 0.65em, first-line-indent: 1.5em)

= Capítulo 1: O Início

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.

Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.

== Seção 1.1

Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.
TYPST_EOF

# Fixture: livro completo Typst (20 capítulos ~200 páginas)
generate_typst_book() {
    local output_file="$1"
    local chapters="$2"

    cat > "$output_file" << 'HEADER'
#set page(width: 6in, height: 9in, margin: (top: 1in, bottom: 1in, left: 0.75in, right: 0.75in))
#set text(font: "Linux Libertine", size: 11pt, lang: "pt")
#set par(justify: true, leading: 0.65em, first-line-indent: 1.5em)

#outline(title: "Sumário", depth: 2)
#pagebreak()

HEADER

    for i in $(seq 1 "$chapters"); do
        cat >> "$output_file" << CHAPTER
= Capítulo ${i}: Título do Capítulo Número ${i}

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.

Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo.

== Seção ${i}.1

Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem.

== Seção ${i}.2

Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur.

Vel illum qui dolorem eum fugiat quo voluptas nulla pariatur. At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident.

#pagebreak()

CHAPTER
    done
}

generate_typst_book "${FIXTURES_DIR}/book-20ch.typ" 20
generate_typst_book "${FIXTURES_DIR}/book-50ch.typ" 50
ok "Fixtures Typst geradas"

# ── SLOs (referência para relatório) ──
declare -A SLOS
SLOS["typst_single_page"]=500       # ms
SLOS["typst_20ch_book"]=7000        # ~350ms/página × 20 páginas
SLOS["typst_50ch_book"]=17500       # ~350ms/página × 50 páginas
SLOS["ghostscript_pdfx"]=10000      # ms
SLOS["epubcheck_validation"]=8000   # ms

# ── Benchmarks ──
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
info "INICIANDO BENCHMARKS DE SIDECARS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

RESULTS_JSON='{"timestamp":"'${TIMESTAMP}'","mode":"'${MODE}'","benchmarks":{'
FIRST=true

# ── Typst benchmarks ──
if [[ "$HAS_TYPST" == "true" ]]; then
    info "Benchmark: Typst single-page (preview)"
    hyperfine \
        --runs "$RUNS" \
        --warmup "$WARMUP" \
        --export-json "${RESULTS_DIR}/typst_single_${TIMESTAMP}.json" \
        "typst compile ${FIXTURES_DIR}/single-page.typ ${RESULTS_DIR}/_tmp_single.pdf"
    rm -f "${RESULTS_DIR}/_tmp_single.pdf"
    echo ""

    info "Benchmark: Typst 20-chapter book"
    hyperfine \
        --runs "$RUNS" \
        --warmup "$WARMUP" \
        --export-json "${RESULTS_DIR}/typst_20ch_${TIMESTAMP}.json" \
        "typst compile ${FIXTURES_DIR}/book-20ch.typ ${RESULTS_DIR}/_tmp_20ch.pdf"
    rm -f "${RESULTS_DIR}/_tmp_20ch.pdf"
    echo ""

    info "Benchmark: Typst 50-chapter book"
    hyperfine \
        --runs "$RUNS" \
        --warmup "$WARMUP" \
        --export-json "${RESULTS_DIR}/typst_50ch_${TIMESTAMP}.json" \
        "typst compile ${FIXTURES_DIR}/book-50ch.typ ${RESULTS_DIR}/_tmp_50ch.pdf"
    rm -f "${RESULTS_DIR}/_tmp_50ch.pdf"
    echo ""
fi

# ── Ghostscript benchmark ──
if [[ "$HAS_GS" == "true" && "$HAS_TYPST" == "true" ]]; then
    info "Gerando PDF base para benchmark Ghostscript..."
    typst compile "${FIXTURES_DIR}/book-20ch.typ" "${FIXTURES_DIR}/book-20ch-base.pdf" 2>/dev/null

    info "Benchmark: Ghostscript PDF/X-1a"
    hyperfine \
        --runs "$RUNS" \
        --warmup "$WARMUP" \
        --export-json "${RESULTS_DIR}/gs_pdfx_${TIMESTAMP}.json" \
        "gs -dBATCH -dNOPAUSE -dQUIET -sDEVICE=pdfwrite -dPDFX -sOutputFile=${RESULTS_DIR}/_tmp_pdfx.pdf ${FIXTURES_DIR}/book-20ch-base.pdf"
    rm -f "${RESULTS_DIR}/_tmp_pdfx.pdf"
    echo ""
elif [[ "$HAS_GS" == "true" ]]; then
    warn "Typst não disponível — pulando benchmark Ghostscript (precisa de PDF base)"
fi

# ── EPUBCheck benchmark ──
if [[ "$HAS_JAVA" == "true" ]]; then
    EPUBCHECK_JAR=""
    # Procurar EPUBCheck em locais comuns
    for jar_path in \
        "/usr/local/lib/epubcheck/epubcheck.jar" \
        "/usr/share/java/epubcheck.jar" \
        "$HOME/.local/lib/epubcheck/epubcheck.jar" \
        "$(which epubcheck 2>/dev/null || true)"; do
        if [[ -f "$jar_path" ]]; then
            EPUBCHECK_JAR="$jar_path"
            break
        fi
    done

    if [[ -n "$EPUBCHECK_JAR" ]]; then
        # Gerar EPUB mínimo de fixture para validação
        info "Gerando EPUB mínimo de fixture..."
        EPUB_DIR="${FIXTURES_DIR}/_epub_tmp"
        mkdir -p "${EPUB_DIR}/META-INF" "${EPUB_DIR}/OEBPS"

        echo -n "application/epub+zip" > "${EPUB_DIR}/mimetype"
        cat > "${EPUB_DIR}/META-INF/container.xml" << 'CONTAINER_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>
CONTAINER_EOF
        cat > "${EPUB_DIR}/OEBPS/content.opf" << 'OPF_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="uid">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
    <dc:identifier id="uid">urn:uuid:12345678-1234-1234-1234-123456789012</dc:identifier>
    <dc:title>BES Benchmark EPUB</dc:title>
    <dc:language>pt-BR</dc:language>
    <meta property="dcterms:modified">2026-03-22T00:00:00Z</meta>
  </metadata>
  <manifest>
    <item id="chapter1" href="chapter1.xhtml" media-type="application/xhtml+xml"/>
    <item id="nav" href="nav.xhtml" media-type="application/xhtml+xml" properties="nav"/>
  </manifest>
  <spine><itemref idref="chapter1"/></spine>
</package>
OPF_EOF
        cat > "${EPUB_DIR}/OEBPS/chapter1.xhtml" << 'CH_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="pt-BR">
<head><title>Capítulo 1</title></head>
<body><h1>Capítulo 1</h1><p>Lorem ipsum dolor sit amet.</p></body>
</html>
CH_EOF
        cat > "${EPUB_DIR}/OEBPS/nav.xhtml" << 'NAV_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="pt-BR">
<head><title>Sumário</title></head>
<body>
<nav epub:type="toc"><h1>Sumário</h1><ol><li><a href="chapter1.xhtml">Capítulo 1</a></li></ol></nav>
</body>
</html>
NAV_EOF

        # Criar EPUB (zip com mimetype não-comprimido primeiro)
        EPUB_FILE="${FIXTURES_DIR}/benchmark.epub"
        cd "$EPUB_DIR"
        zip -X0 "$EPUB_FILE" mimetype 2>/dev/null
        zip -Xr9D "$EPUB_FILE" META-INF/ OEBPS/ 2>/dev/null
        cd - > /dev/null
        rm -rf "$EPUB_DIR"

        if [[ -f "$EPUB_FILE" ]]; then
            info "Benchmark: EPUBCheck validation"
            hyperfine \
                --runs "$RUNS" \
                --warmup "$WARMUP" \
                --export-json "${RESULTS_DIR}/epubcheck_${TIMESTAMP}.json" \
                "java -jar ${EPUBCHECK_JAR} ${EPUB_FILE} --quiet"
            echo ""
        fi
    else
        warn "EPUBCheck JAR não encontrado — pulando benchmark"
    fi
fi

# ── Relatório ──
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
info "BENCHMARKS CONCLUÍDOS"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
info "Resultados salvos em: ${RESULTS_DIR}/"
echo ""

# Listar resultados com médias
for f in "${RESULTS_DIR}"/*_${TIMESTAMP}.json; do
    if [[ -f "$f" ]]; then
        name=$(basename "$f" | sed "s/_${TIMESTAMP}.json//")
        mean=$(python3 -c "
import json, sys
with open('$f') as fh:
    data = json.load(fh)
    if 'results' in data and len(data['results']) > 0:
        mean_s = data['results'][0].get('mean', 0)
        print(f'{mean_s*1000:.1f}ms')
    else:
        print('N/A')
" 2>/dev/null || echo "N/A")
        echo "  ${name}: média ${mean}"
    fi
done

echo ""
info "Para benchmark completo: ./tests/perf/bench-sidecars.sh full"
info "Para benchmarks Rust:    cd src-tauri && cargo bench"
