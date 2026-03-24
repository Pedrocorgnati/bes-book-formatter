# Hardcodes Task — BES Book Formatter
> Stack: SvelteKit + Rust/Axum + Tauri (adaptado do template nextjs:hardcodes)

## Tasks Executadas

### T001 — COMPLETED
**Adicionar `bes_first_launch` a STORAGE_KEYS**
- Criado: `STORAGE_KEYS.FIRST_LAUNCH = 'bes_first_launch'` em `src/lib/constants/storage-keys.ts`
- Corrigido: `src/routes/+page.svelte` — usa `STORAGE_KEYS.FIRST_LAUNCH` em vez de string literal

### T002 — COMPLETED
**Usar `IllustrationState` enum nas comparações de estado de ilustração**
- Corrigido: `src/lib/components/illustrations/IllustrationCard.svelte` (4 comparações)
- Corrigido: `src/lib/components/illustrations/IllustrationGallery.svelte` (4 comparações no `counts`)

### T003 — COMPLETED
**Corrigir rotas hardcoded para constantes existentes**
- Corrigido: `src/lib/components/project/ImportWizard.svelte` — `goto(\`/project/${project.id}\`)` → `goto(PROJECT_ROUTES.ROOT(project.id))`
- Corrigido: `src/routes/project/[id]/+layout.svelte` — 2× `goto('/')` → `goto(ROUTES.HOME)`

### T004 — COMPLETED
**Adicionar toast durations e limits a `src/lib/constants/timing.ts`**
- Adicionado: `TIMING.TOAST_SUCCESS = 4000`, `TIMING.TOAST_WARNING = 6000`, `TIMING.TOAST_INFO = 4000`, `TIMING.TOAST_ERROR_PERSISTENT = 0`
- Adicionado: `MAX_VISIBLE_TOASTS = 3`, `PREVIEW_PAGE_CACHE_SIZE = 10`
- Corrigido: `src/lib/stores/toastStore.ts` — usa constantes em vez de magic numbers
- Corrigido: `src/lib/components/preview/PageSpreadViewer.svelte` — usa `PREVIEW_PAGE_CACHE_SIZE`

### T005 — COMPLETED
**Criar `src/lib/constants/ui-tabs.ts` para tab identifiers**
- Criado: `PREVIEW_TABS`, `FORMAT_TABS`, `COVER_SECTIONS`, `SETTINGS_TABS`, `ILLUSTRATION_FILTERS`
- Corrigido: `PreviewSidebar.svelte` (9 comparações + 6 assignments)
- Corrigido: `FormatSelector.svelte` (3 comparações + 2 assignments)
- Corrigido: `CoverEditor.svelte` (3 comparações + 3 assignments)
- Corrigido: `project/[id]/settings/+page.svelte` (2 comparações + 2 assignments)
- Corrigido: `IllustrationGallery.svelte` — `Filter` type → `IllustrationFilter`, 2 comparações + filters array

### T006 — COMPLETED
**Centralizar preference DB keys em `ipc/preferences.ts`**
- Extraído: `PREF_DB_KEYS` constant com `theme`, `ui_language`, `analytics_opt_in`
- Extraído: `DEFAULT_UI_LANGUAGE = 'pt-BR'`, `DEFAULT_THEME = 'light'`
- Eliminado: `keyMap` inline duplicado

### T007 — COMPLETED
**Corrigir toast messages hardcoded no `GenerationPanel.svelte`**
- Adicionadas chaves i18n: `generation.preflightError`, `generation.generateFormatError`, `generation.partialError` (pt-BR, en-US, es-ES)
- Corrigido: `GenerationPanel.svelte` — 3 strings PT hardcoded → `t('generation.*')`

### T008 — COMPLETED
**Mover magic limits para constantes**
- Adicionado: `PROJECTS_LIST_LIMIT = 20`, `ALT_TEXT_MIN_LENGTH = 10` em `timing.ts`
- Corrigido: `ipc/projects.ts` — `limit = 20` → `limit = PROJECTS_LIST_LIMIT`
- Corrigido: `IllustrationGallery.svelte` — `>= 10` → `>= ALT_TEXT_MIN_LENGTH`
