# Hardcodes Task — BES Book Formatter (SvelteKit + Tauri)

> Stack: SvelteKit + Rust/Axum + Tauri + SQLite
> workspace_root: output/workspace/bes-book-formatter
> Gerado por: /nextjs:hardcodes (adaptado para SvelteKit/Tauri)

---

## Resumo de Findings (Phase 1)

| # | Tipo | Valor | Arquivos | Ocorrências |
|---|------|-------|----------|-------------|
| 1 | Storage Key | `'bes_theme'` | preferencesStore.ts | 3 |
| 2 | Storage Key | `'bes_language'` | preferencesStore.ts + i18n/engine.ts | 4 |
| 3 | Storage Key | `'bes_analytics'` | preferencesStore.ts | 2 |
| 4 | Rota | `/import` | Header, Sidebar, +page.svelte (2x) | 4 |
| 5 | Rota | `/settings` | Sidebar.svelte | 1 |
| 6 | Rota | `/` | +layout.svelte (3x), preview, illustrations, cover, output, typography | 8 |
| 7 | Rota | `/project/{id}/*` | +layout.svelte (tabs) | 8 |
| 8 | OutputFormat strings | `'epub3'`, `'pdf_print'`, etc. | FormatSelector.svelte, GenerationPanel.svelte | 15+ |
| 9 | Platform strings | `'kdp'`, `'ingramspark'`, etc. | FormatSelector.svelte | 6+ |
| 10 | Status (GenerationStatus) | `'idle'`/`'preflight'`/`'generating'`/`'done'`/`'error'` | generationStore.ts (local type) | 5 |
| 11 | AnnotationType strings | `'comment'`/`'flag'`/`'highlight'` | AnnotationLayer + PreviewRightPanel | 8+ |
| 12 | TypoIssueType strings | `'orphan'`/`'widow'` | OrphanWidowHighlight, PreviewRightPanel, PreviewSidebar | 6 |
| 13 | Direct invoke() | `'get_annotations'`, `'detect_orphans_widows'` | PreviewRightPanel.svelte | 2 |
| 14 | Direct invoke() | `'render_preview_page'` | PageSpreadViewer.svelte | 2 |
| 15 | Toast cru (sem i18n) | "Salve a configuração de capa antes de..." | CoverPreview.svelte | 2 |
| 16 | Magic Number | `500`ms debounce | PageConfigPanel + TypographyPanel | 2 |
| 17 | Magic Number | `300`ms debounce | PageSpreadViewer | 2 |
| 18 | Magic Number | `1500`ms setTimeout | DataTestOverlay | 1 |
| 19 | Magic Number | `3000`ms setTimeout | utils/a11y.ts | 1 |
| 20 | Magic Number | `50`ms focus | ConfirmDialog | 1 |
| 21 | Magic Number | `500`ms sidecar wait | SidecarStatus | 1 |

---

## Task List

### GRUPO 1 — Criar arquivos de constantes (PARALLEL)

- [x] **T001** — Criar `src/lib/constants/storage-keys.ts`
- [x] **T002** — Criar `src/lib/constants/routes.ts`
- [x] **T003** — Criar `src/lib/constants/timing.ts`
- [x] **T004** — Criar `src/lib/constants/index.ts` (barrel export)

### GRUPO 2 — Atualizar enums.ts (SEQUENTIAL, depende de nada)

- [x] **T005** — Adicionar `GenerationStatus`, `AnnotationType`, `TypoIssueType` a `enums.ts`

### GRUPO 3 — Atualizar IPC layer (PARALLEL)

- [x] **T006** — Adicionar `ipcRenderPreviewPage` em `ipc/preview.ts`
- [x] **T007** — Adicionar `ipcSelectDirectory`, `ipcReadBookConfig` em `ipc/parser.ts`

### GRUPO 4 — Atualizar stores (PARALLEL, depende de T001 + T005)

- [x] **T008** — `generationStore.ts`: usar `GenerationStatus` enum
- [x] **T009** — `preferencesStore.ts`: usar `STORAGE_KEYS`
- [x] **T010** — `i18n/engine.ts`: usar `STORAGE_KEYS`

### GRUPO 5 — Atualizar componentes (SEQUENTIAL, depende de T001–T010)

- [x] **T011** — `FormatSelector.svelte`: usar `OutputFormat` + `Platform` enums
- [x] **T012** — `GenerationPanel.svelte`: usar `OutputFormat` enum; remover `'epub3'` fallback literal
- [x] **T013** — `PreviewRightPanel.svelte`: substituir `invoke()` direto por `ipcGetAnnotations()` + `ipcDetectOrphansWidows()`; usar `AnnotationType` enum
- [x] **T014** — `PageSpreadViewer.svelte`: substituir `invoke()` direto por `ipcRenderPreviewPage()`; usar `TIMING.DEBOUNCE.PREVIEW`
- [x] **T015** — `AnnotationLayer.svelte`: usar `AnnotationType` enum
- [x] **T016** — `OrphanWidowHighlight.svelte` + `PreviewRightPanel.svelte`: usar `TypoIssueType` enum
- [x] **T017** — Rotas (Header, Sidebar, +page.svelte, +layout.svelte, sub-pages): usar `ROUTES`
- [x] **T018** — `PageConfigPanel.svelte` + `TypographyPanel.svelte`: usar `TIMING.DEBOUNCE.CONFIG_SAVE`
- [x] **T019** — `DataTestOverlay.svelte`, `ConfirmDialog.svelte`, `SidecarStatus.svelte`, `utils/a11y.ts`: usar constantes TIMING
- [x] **T020** — `CoverPreview.svelte`: substituir toast literals por `t('cover.preview.saveFirst')` / `t('cover.export.saveFirst')`; adicionar chaves em pt-BR.json, en-US.json, es-ES.json

---

## Impacto

- Total de hardcodes: **67+**
- Arquivos de constantes criados: **4**
- Arquivos de enums: **1 expandido**
- Arquivos de IPC: **2 expandidos**
- Componentes/stores corrigidos: **14+**
- Risco: **BAIXO** (refatoração de strings e tipos; nenhuma lógica de negócio alterada)
