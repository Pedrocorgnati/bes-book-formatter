# MODULE-8 — Auditoria de Performance e Build (TASK-9)
**Tipo de análise:** Estática (revisão de código + artefatos de build reais)
**Data:** 2026-03-22
**Nota:** Build JS/CSS medido com artefatos reais em `build/`. Benchmarks de runtime marcados como ESTIMADO onde não há medição real disponível.

---

## 1. Análise de Artefatos de Build

### Build Frontend (SvelteKit + adapter-static)

Diretório: `build/_app/immutable/`
Build gerado em: 2026-03-22 00:43

#### JavaScript (medição real)

| Métrica | Valor | Target | Status |
|---------|-------|--------|--------|
| Total JS (uncompressed) | **191.678 bytes** (~187 KB) | — | — |
| Total JS (gzip estimado) | **~72 KB** | < 250 KB gzipped | PASS |
| Número de arquivos JS | 31 arquivos | — | — |

**Breakdown de chunks principais:**
| Arquivo | Tamanho (raw) | Tipo |
|---------|--------------|------|
| `EWTEs3BK.js` (chunks/) | 48.010 bytes | Provavelmente Svelte runtime + stores |
| `BBQCYCg0.js` (chunks/) | 32.804 bytes | Provavelmente i18n + componentes principais |
| `N92MnPTj.js` (chunks/) | 23.872 bytes | Provavelmente rotas/layout |
| `DsvzRBBp.js` (chunks/) | 12.496 bytes | Componente mid-size |
| `0.iYUov8U3.js` (nodes/) | 28.985 bytes | Node de rota principal |
| `11.BJrtTqni.js` (nodes/) | 6.652 bytes | Node de rota |
| Demais 25 arquivos | < 6 KB cada | Lazy-loaded por rota |

**Observação:** O SvelteKit com adapter-static divide automaticamente o código por rota (code splitting). O chunk de entrada (`start.CgxyuBww.js`: 83 bytes + `app.FwHjouuh.js`: 2.944 bytes) é mínimo. A maioria das rotas é lazy-loaded — padrão adequado para app desktop Tauri.

**Gzip estimado por file:**
- Método: `find build -name "*.js" | xargs gzip -c | wc -c` → 73.816 bytes (~72 KB)
- **72 KB gzipped para TODOS os JS** — muito abaixo do target de 250 KB. PASS.

#### CSS (medição real)

| Métrica | Valor | Target | Status |
|---------|-------|--------|--------|
| Total CSS (uncompressed) | **25.868 bytes** (~25 KB) | — | — |
| Total CSS (gzip estimado) | **~6,5 KB** | < 50 KB gzipped | PASS |
| Número de arquivos CSS | 7 arquivos | — | — |

**Breakdown de CSS:**
| Arquivo | Tamanho | Descrição |
|---------|---------|-----------|
| `0.C9X0HwTB.css` | 18.266 bytes | CSS global (app.css: tokens, reset, skip-nav, scrollbar, animações) |
| `11.BHu-jG_0.css` | 2.697 bytes | Rota typography/output |
| `3.bvB2rJdc.css` | 1.562 bytes | Componente layout |
| `2.Cu_zOYQL.css` | 907 bytes | Componente |
| `5.CjhIC0ix.css` | 791 bytes | Componente |
| `1.DO5iI0FN.css` | 713 bytes | Componente |
| `EmptyState.CYxj-ct9.css` | 932 bytes | EmptyState isolado |

**Gzip estimado:** `find build -name "*.css" | xargs gzip -c | wc -c` → 6.636 bytes (~6.5 KB). PASS.

#### Tamanho total do build

| Diretório | Tamanho |
|-----------|---------|
| `build/` total | **384 KB** |
| `build/_app/immutable/` | **336 KB** |
| `build/images/` | SVGs (favicon, logos, empty states) |

O build completo ocupa 384 KB em disco — excelente para um app Tauri que embute os assets no binário. O binário Rust será a parte maior da distribuição.

---

## 2. Configuração de Build

### Vite (`vite.config.ts`)
```
plugins: [sveltekit()]
server.port: 1420 (strictPort)
clearScreen: false
```

**Observações:**
- Nenhuma configuração de chunking manual (`build.rollupOptions.output.manualChunks`) — o SvelteKit gerencia automaticamente via file-based routing. Adequado.
- Sem `build.minify: false` — compressão padrão (esbuild) ativa. Correto.
- Sem `build.sourcemap: true` em produção — correto para distribuição.
- **Gap:** Sem configuração de `build.target` explícita para WebView do Tauri. O Tauri 2 usa um WebView moderno (WebKit no macOS/Linux, WebView2 no Windows) — o target padrão do Vite (`modules`) é compatível mas poderia ser explicitado.

### SvelteKit (`svelte.config.js`)
```
adapter: adapter-static
fallback: '200.html'
precompress: false
strict: false
```

**Observações:**
- `precompress: false` — correto para Tauri (Tauri serve os arquivos diretamente do sistema de arquivos, não via HTTP server que se beneficiaria de pre-compressão).
- `strict: false` — permite rotas dinâmicas sem `prerender = true`. Necessário para a SPA Tauri.
- `fallback: '200.html'` — padrão correto para SPA client-side routing.

### Rust Release Profile (`Cargo.toml`)
```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = "z"
```

**Observação:** Configuração agressiva de otimização de tamanho (`opt-level = "z"` = tamanho mínimo, `lto = true` = Link Time Optimization, `codegen-units = 1` = máxima oportunidade de inlining). Isso resulta em binários menores mas tempos de compilação mais longos — correto para distribuição de produção.

---

## 3. Análise de Performance de Runtime

### 3.1 Typst Preview — Target: < 500ms P95

**Fonte:** `src-tauri/src/services/preview_service.rs`, `src-tauri/src/commands/preview.rs`

**Fluxo de execução do `render_preview_page`:**
1. Resolve projeto do DB (`ProjectRepository::find_by_id`)
2. Parse do manuscrito (`ParserService::parse_manuscript`)
3. Carrega `TypographyConfig` do DB
4. Computa cache key SHA256 do conteúdo
5. Se cache válido: usa `.typ` existente (skip geração)
6. Se cache inválido: regenera `main.typ` + escreve hash
7. Executa `typst compile --format png --ppi {dpi}` via sidecar
8. Lê PNGs gerados e codifica em base64
9. Retorna `render_ms` (tempo medido com `Instant::now()`)

**Mecanismo de cache:** Hash SHA256 do `project_id + config_json + manuscript_json`. Se o manuscrito e a configuração não mudaram desde o último render, o `.typ` é reutilizado. O Typst, no entanto, é reinvocado mesmo com cache de `.typ` (há remoção dos PNGs antigos somente quando `need_regen = true`). **Gap:** Quando `need_regen = false` (cache hit), o Typst ainda é recompilado do `.typ` existente — não há cache de PNG no caminho feliz.

**Timeout configurado:** `SidecarManager::spawn_typst(&compile_args, 10_000)` — 10 segundos.

**Estimativa de performance:**
- Cache miss (primeiro render): DB queries + parse Markdown + Typst compile → **ESTIMADO 2-8s** para manuscritos de 200-300 páginas. Acima de 500ms.
- Cache hit (`.typ` existe, manuscrito não mudou): Typst recompila do `.typ` → **ESTIMADO 500ms-3s** dependendo do tamanho do documento.
- Para documentos pequenos (<50 páginas): **ESTIMADO 200-800ms** — margem para atingir 500ms P95.

**ESTIMADO — Veredicto:** O target de < 500ms P95 para `render_preview_page` **provavelmente não é atingido** para documentos grandes em cache miss. Para cache hit em documentos pequenos, pode ser atingido. Necessária medição real com `render_ms` logado. O mecanismo de retorno de `render_ms` no `PreviewPageResponse` está implementado e disponível para monitoramento.

**Recomendações:**
- Adicionar cache de PNG gerado: se hash não mudou E PNGs existem E não foram removidos → retornar PNGs direto sem reinvocar Typst
- Logging de `render_ms` para benchmark em múltiplos tamanhos de manuscrito

### 3.2 App Boot — Target: < 3s

**Fonte:** `src-tauri/src/lib.rs`, `src/routes/+layout.svelte`

**Fluxo de boot:**
```
Rust: tauri::Builder::default()
  → plugins init (shell, fs, dialog, store)
  → setup callback: db::create_pool(app_data_dir).await
  → migration_service.apply_pending().await
  → app_handle.manage(pool)
  → invoke_handler registration

Frontend: onMount()
  → initPreferences() (IPC → get_preferences)
  → initLocale() (localStorage read)
  → initAnalytics() (local only)
  → ipcInitDatabase() (IPC → init_database)
  → ipcGetProjects() (IPC → get_projects)
  → loading = false
```

**Observação crítica:** O Rust `lib.rs:58` usa `tauri::async_runtime::block_on(async { ... })` para inicializar o pool SQLite e aplicar migrations **de forma síncrona no setup**. Isso bloqueia a thread principal do Tauri até que as migrations sejam aplicadas.

- Primeira execução (migrations pendentes): Criação de tabelas SQLite → **ESTIMADO 100-500ms**
- Execuções subsequentes (sem migrations): Pool creation → **ESTIMADO 50-150ms**
- Frontend IPC roundtrip (3-4 chamadas sequenciais): **ESTIMADO 100-300ms total**

**ESTIMADO — Veredicto:** Boot em < 3s é **provável** para execuções normais (sem migrations novas). O `block_on` no setup pode adicionar latência visível na primeira execução, mas migrations são operação única. O target de 3s parece atingível.

**Gap identificado:** Sem indicador visual de progresso durante o `block_on` de migrations no primeiro boot. O usuário vê a tela de loading genérica sem saber que há uma operação de banco de dados em curso.

### 3.3 EPUB Generation — Target: < 30s

**Fonte:** `src-tauri/src/services/epub_service.rs`, `src-tauri/src/commands/generation.rs`

**Fluxo de geração EPUB:**
1. `EpubService::generate()`: carrega projeto, parse manuscrito, carrega TypographyConfig
2. Renderiza HTML de cada capítulo via `comrak` (Markdown → HTML)
3. Constrói estrutura EPUB (OPF, NCX, NAV, CSS, capítulos, imagens)
4. Cria arquivo ZIP em memória
5. Escreve em disco
6. Executa EPUBCheck: `SidecarManager::spawn_epubcheck(epub_path, 60_000)` — timeout 60s

**Sem mecanismo de progress callback:** Não há `emit()` para o frontend durante a geração. A UI do frontend não foi verificada em detalhe para polling, mas o comando IPC é assíncrono — o frontend aguarda a Promise resolver.

**Timeout configurado:** 60s para EPUBCheck (validação pós-geração). A geração em si (comrak + ZIP) não tem timeout explícito além do timeout global do IPC.

**Estimativa:**
- Manuscrito pequeno (<100 páginas, poucas imagens): **ESTIMADO 3-10s**
- Manuscrito grande (400 páginas, 50 ilustrações): **ESTIMADO 15-45s**

**ESTIMADO — Veredicto:** O target de < 30s é **provavelmente atingível** para manuscritos típicos sem EPUBCheck. Com EPUBCheck incluso, manuscritos grandes podem exceder 30s. **Gap:** Sem progress callback, o usuário não tem feedback durante geração longa.

**Nota de geração PDF:** `pdf_print_service.rs` usa Typst com timeout de 120s (`spawn_typst(..., 120_000)`) seguido de Ghostscript com timeout de 120s. PDFs de livros longos podem facilmente exceder 30s — mas PDF não foi listado no target da TASK-9.

---

## 4. Análise de Código de Performance (Frontend)

### SvelteKit + Svelte 5
| Padrão | Status | Evidência |
|--------|--------|-----------|
| Svelte 5 com runes (`$state`, `$derived`, `$effect`) | PASS | Todos os componentes auditados usam runes |
| `$derived` para valores computados (sem recalcular na mão) | PASS | `+layout.svelte:29,40`; `AppShell.svelte:25-36` |
| Lazy loading de componentes dev-only | PASS | `+layout.svelte:122` — `import()` dinâmico para `DataTestOverlay` |
| Animações CSS via `transition:` Svelte (não JS) | PASS | `Toast.svelte:24-26` — `in:fly / out:fly` |
| CSS scoped por componente (sem CSS global desnecessário) | PASS | Cada `.svelte` tem `<style>` scoped |
| `font-display: swap` nas fontes | PASS | `app.css:11,17,23,28,35,41,48` — todas as fontes usam `font-display: swap` |

### Dependências de Produção
```json
"@tauri-apps/api": "^2.0.0"
"@tauri-apps/plugin-shell": "^2.0.0"
"@tauri-apps/plugin-sql": "^2.0.0"
"@tauri-apps/plugin-store": "^2.0.0"
"@tauri-apps/plugin-fs": "^2.0.0"
"lucide-svelte": "^0.454.0"
```

**Observação:** `lucide-svelte` é a única dependência de UI de terceiros. Tree-shaking do Svelte/Vite elimina ícones não utilizados. Impacto mínimo no bundle.

**Sem dependências pesadas:** Não há lodash, moment.js, axios, RxJS, ou outras bibliotecas de grande porte. Bundle slim é esperado.

### Tauri Plugins
Todos os plugins Tauri são nativos Rust — zero overhead de JS além do thin wrapper IPC:
- `tauri-plugin-sql`: SQLite via Rust (sqlx)
- `tauri-plugin-shell`: Spawn de sidecars (Typst, Ghostscript, EPUBCheck)
- `tauri-plugin-fs`: Acesso ao filesystem nativo
- `tauri-plugin-store`: Persistência de preferências
- `tauri-plugin-dialog`: Dialog de seleção de pasta

---

## 5. Mecanismo de Cache e Otimizações Identificadas

| Mecanismo | Status | Detalhe |
|-----------|--------|---------|
| Cache hash SHA256 para preview `.typ` | IMPLEMENTADO | `preview_service.rs:17-22,329-338` |
| PNG cleanup seletivo (só em cache miss) | IMPLEMENTADO | `preview_service.rs:358-369` |
| Pool SQLite compartilhado como managed state | IMPLEMENTADO | `lib.rs:79` — `app_handle.manage(pool)` |
| LTO + codegen-units=1 no release | IMPLEMENTADO | `Cargo.toml:57-60` |
| Code splitting por rota (SvelteKit) | IMPLEMENTADO | Build confirma 12 node files separados |
| `font-display: swap` (FOUT tolerado) | IMPLEMENTADO | `app.css` — não bloqueia render |
| Cache de PNG após compilação Typst | NAO IMPLEMENTADO | Gap: Typst é recompilado mesmo em cache hit |
| Progress callbacks em geração longa | NAO IMPLEMENTADO | Gap: sem `emit()` do Rust para o frontend |
| Preloading de rotas críticas | NAO IMPLEMENTADO | SvelteKit default — adequado para Tauri |

---

## 6. Issues Identificados

### BLOQUEADORES
Nenhum bloqueador crítico de performance identificado na análise estática.

### RESSALVAS
| # | Área | Descrição | Impacto |
|---|------|-----------|--------|
| P01 | Preview Service | Typst é recompilado mesmo em cache hit de `.typ` (PNGs não são cacheados) | Alto — latência extra em renders sequenciais |
| P02 | EPUB Generation | Sem progress callback para o frontend — usuário sem feedback em gerações > 5s | Médio — UX |
| P03 | App Boot | `block_on` síncrono no setup para migrations — pode causar freeze visual curto na primeira execução | Baixo (único) |
| P04 | EPUB com EPUBCheck | Timeout 60s para EPUBCheck pode ultrapassar target de 30s para livros grandes | Médio — depende do tamanho do manuscrito |
| P05 | vite.config.ts | Sem `build.target` explícito para WebView Tauri — padrão `modules` é compatível mas não otimizado | Baixo |

---

## 7. Targets de Performance — Resumo

| Target | Valor Alvo | Status | Método de Análise |
|--------|-----------|--------|--------------------|
| JS bundle gzipped | < 250 KB | **PASS** ✓ (~72 KB medido) | Medição real via gzip |
| CSS bundle gzipped | < 50 KB | **PASS** ✓ (~6.5 KB medido) | Medição real via gzip |
| Typst preview P95 | < 500ms | **ESTIMADO — Provável FAIL** para docs grandes | Análise estática do fluxo |
| App boot | < 3s | **ESTIMADO — Provável PASS** | Análise estática do fluxo |
| EPUB generation | < 30s | **ESTIMADO — PASS COM RESSALVAS** (sem EPUBCheck em docs grandes) | Análise estática do fluxo |

---

## Verdict

**PASS COM RESSALVAS (ESTIMADO)**

Os targets de bundle size (JS e CSS) foram **medidos** com os artefatos de build reais e ambos passam com grande margem. Os targets de runtime (Typst preview, boot, EPUB) foram analisados estaticamente e não podem ser confirmados sem execução real da aplicação.

**O principal gap de performance identificado é o cache de preview:** o `PreviewService` reutiliza o arquivo `.typ` (evitando regerar o documento Typst), mas não cacheia os PNGs resultantes, forçando Typst a recompilar em toda navegação de página — o que representa uma oportunidade clara de melhoria para atingir 500ms P95 de forma consistente.

**Ações Recomendadas:**
1. Implementar cache de PNG por (hash, page_number) — retornar imagem diretamente se hash não mudou e arquivo PNG existe
2. Adicionar `emit()` do Rust para frontend durante geração EPUB longa (canal de progresso)
3. Medir `render_ms` em livros de diferentes tamanhos (50/200/400 páginas) para validar o target P95 empiricamente
4. Considerar separar EPUBCheck do fluxo principal de geração para não impactar o target de 30s
