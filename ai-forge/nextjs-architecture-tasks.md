<metadata>
source: /nextjs:architecture (adapted for SvelteKit + Tauri)
repo: output/workspace/bes-book-formatter
date: 2026-03-22
estimated-hours: 2
complexity: low
</metadata>

<overview>
Corrigir violações de cohesão IPC e chamadas diretas invoke() bypassando o wrapper. Criar módulos ipc/illustrations.ts e ipc/preview.ts. Conectar preferencesStore ao backend Rust.
</overview>

<task-list>

### T001 — Criar `src/lib/ipc/illustrations.ts`

**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- criar: `src/lib/ipc/illustrations.ts`

**Descrição:**
Extrair as 4 funções de ilustrações de `typography.ts` para novo módulo dedicado.

**Critérios de Aceite:**
- [ ] Arquivo criado com `ipcValidateIllustrationDpi`, `ipcProcessIllustration`, `ipcListIllustrations`, `ipcUpdateIllustrationAltText`
- [ ] Usa `ipc<T>()` wrapper (não `invoke()` direto)
- [ ] Tipagem correta mantida
- [ ] Estimativa: 0.25h

---

### T002 — Criar `src/lib/ipc/preview.ts`

**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- criar: `src/lib/ipc/preview.ts`

**Descrição:**
Criar módulo IPC para todos os comandos de preview: `toggle_distraction_free`, `add_annotation`, `get_annotations`, `delete_annotation`, além de mover `ipcDetectOrphansWidows` de typography.ts.

**Critérios de Aceite:**
- [ ] Funções: `ipcToggleDistractionFree`, `ipcGetAnnotations`, `ipcAddAnnotation`, `ipcDeleteAnnotation`, `ipcDetectOrphansWidows`
- [ ] Usa `ipc<T>()` wrapper
- [ ] Tipagem correta

**Estimativa:** 0.25h

---

### T003 — Corrigir `src/lib/ipc/typography.ts`

**Tipo:** SEQUENTIAL
**Dependências:** T001, T002
**Arquivos:**
- modificar: `src/lib/ipc/typography.ts`

**Descrição:**
Remover as 5 funções extraídas para illustrations.ts e preview.ts.

**Critérios de Aceite:**
- [ ] `ipcValidateIllustrationDpi`, `ipcProcessIllustration`, `ipcListIllustrations`, `ipcUpdateIllustrationAltText` removidas
- [ ] `ipcDetectOrphansWidows` removida
- [ ] Restam apenas funções de tipografia (config, fontes, TOC)
- [ ] Build sem erros

**Estimativa:** 0.1h

---

### T004 — Corrigir `DistractionFreeMode.svelte`

**Tipo:** SEQUENTIAL
**Dependências:** T002
**Arquivos:**
- modificar: `src/lib/components/preview/DistractionFreeMode.svelte`

**Descrição:**
Substituir `invoke('toggle_distraction_free', ...)` por `ipcToggleDistractionFree` do módulo `ipc/preview.ts`.

**Critérios de Aceite:**
- [ ] Import `invoke` removido
- [ ] Usa `ipcToggleDistractionFree` de `$lib/ipc/preview`

**Estimativa:** 0.1h

---

### T005 — Corrigir `AnnotationLayer.svelte`

**Tipo:** SEQUENTIAL
**Dependências:** T002
**Arquivos:**
- modificar: `src/lib/components/preview/AnnotationLayer.svelte`

**Descrição:**
Substituir 3 `invoke()` diretos por funções de `ipc/preview.ts`.

**Critérios de Aceite:**
- [ ] `invoke('get_annotations', ...)` → `ipcGetAnnotations`
- [ ] `invoke('add_annotation', ...)` → `ipcAddAnnotation`
- [ ] `invoke('delete_annotation', ...)` → `ipcDeleteAnnotation`
- [ ] Import `invoke` removido

**Estimativa:** 0.2h

---

### T006 — Corrigir `PreviewSidebar.svelte`

**Tipo:** SEQUENTIAL
**Dependências:** T001, T002
**Arquivos:**
- modificar: `src/lib/components/preview/PreviewSidebar.svelte`

**Descrição:**
Substituir 3 `invoke()` diretos por funções dos módulos IPC corretos.

**Critérios de Aceite:**
- [ ] `invoke('parse_manuscript', ...)` → `ipcParseManuscript` de `$lib/ipc/generation` (ou criar no módulo adequado)
- [ ] `invoke('list_illustrations', ...)` → `ipcListIllustrations` de `$lib/ipc/illustrations`
- [ ] `invoke('get_project', ...)` → `ipcGetProject` de `$lib/ipc/projects`
- [ ] Import `invoke` removido

**Estimativa:** 0.2h

---

### T007 — Corrigir `ipc/preferences.ts`

**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/ipc/preferences.ts`

**Descrição:**
Substituir `import { invoke }` por `import { ipc }` de `$lib/utils/ipc` para consistência com os outros 5 módulos.

**Critérios de Aceite:**
- [ ] `ipcGetPreferences` usa `ipc<ApiResponse<...>>()`
- [ ] `ipcSetPreference` usa `ipc<ApiResponse<null>>()`
- [ ] `ipcGetTheme` mantido ou simplificado
- [ ] Import `invoke` removido

**Estimativa:** 0.15h

---

### T008 — Conectar `preferencesStore.ts` ao backend

**Tipo:** SEQUENTIAL
**Dependências:** T007
**Arquivos:**
- modificar: `src/lib/stores/preferencesStore.ts`

**Descrição:**
Remover os 3 TODOs e conectar `initPreferences`, `setTheme`, `setLanguage` ao `ipc/preferences.ts` que já existe.

**Critérios de Aceite:**
- [ ] `initPreferences` carrega do backend Rust via `ipcGetPreferences`
- [ ] `setTheme` persiste via `ipcSetPreference('theme', value)`
- [ ] `setLanguage` persiste via `ipcSetPreference('uiLanguage', value)`
- [ ] `setAnalyticsOptIn` persiste via `ipcSetPreference('analyticsOptIn', value)`
- [ ] localStorage mantido como fallback rápido
- [ ] 0 TODOs restantes

**Estimativa:** 0.3h

---

### T009 — Atualizar imports nos componentes de ilustração

**Tipo:** SEQUENTIAL
**Dependências:** T001, T003
**Arquivos:**
- modificar: `src/lib/components/illustrations/IllustrationGallery.svelte`
- modificar: `src/lib/components/illustrations/IllustrationDropzone.svelte`

**Descrição:**
Atualizar imports de `$lib/ipc/typography` para `$lib/ipc/illustrations`.

**Critérios de Aceite:**
- [ ] `IllustrationGallery.svelte` importa de `$lib/ipc/illustrations`
- [ ] `IllustrationDropzone.svelte` importa de `$lib/ipc/illustrations`
- [ ] Build sem erros

**Estimativa:** 0.05h

---

### T010 — Atualizar import em `TypographyPreview.svelte`

**Tipo:** SEQUENTIAL
**Dependências:** T002, T003
**Arquivos:**
- modificar: `src/lib/components/typography/TypographyPreview.svelte`

**Descrição:**
Atualizar import de `ipcDetectOrphansWidows` de `$lib/ipc/typography` para `$lib/ipc/preview`.

**Critérios de Aceite:**
- [ ] Import atualizado
- [ ] Build sem erros

**Estimativa:** 0.05h

</task-list>

<validation-strategy>
- svelte-check sem erros após cada task
- Build de desenvolvimento passa
- Imports corretos verificados com grep
</validation-strategy>

<acceptance-criteria>
- [ ] `ipc/illustrations.ts` criado com 4 funções
- [ ] `ipc/preview.ts` criado com 5 funções
- [ ] `ipc/typography.ts` sem funções de ilustrações ou preview
- [ ] 0 chamadas `invoke()` diretas em componentes (exceto plugin:shell)
- [ ] `ipc/preferences.ts` usa wrapper `ipc<T>()`
- [ ] `preferencesStore.ts` sem TODOs, conectado ao backend
- [ ] Imports de ilustrações apontam para `ipc/illustrations`
- [ ] Import de `ipcDetectOrphansWidows` aponta para `ipc/preview`
- [ ] Build passando
</acceptance-criteria>
