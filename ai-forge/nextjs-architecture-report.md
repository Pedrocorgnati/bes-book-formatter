# Architecture Report — BES Book Formatter

**source:** /nextjs:architecture (adapted for SvelteKit + Tauri)
**repo:** output/workspace/bes-book-formatter
**date:** 2026-03-22
**stack:** SvelteKit 5 + TypeScript 5.7 + Tauri 2 + Rust/Axum + SQLite

---

## Status: CONCLUÍDO ✅

---

## Resumo Executivo

O projeto tem arquitetura sólida no geral: separação commands → services → repositories no Rust, stores Svelte bem definidas, wrapper IPC genérico (`utils/ipc.ts`), tipos centralizados. Os problemas encontrados são de **cohesão e consistência**, não de design fundamental.

---

## Problemas Encontrados

### P1 — IPC Cohesion Violation (ALTO)

**Evidência:** `rg "ipcListIllustrations\|ipcValidateIllustrationDpi\|ipcProcessIllustration\|ipcUpdateIllustrationAltText" src/`

`src/lib/ipc/typography.ts` contém 4 funções de ilustrações + 1 de preview:

| Função | Deveria estar em |
|--------|-----------------|
| `ipcValidateIllustrationDpi` | `ipc/illustrations.ts` |
| `ipcProcessIllustration` | `ipc/illustrations.ts` |
| `ipcListIllustrations` | `ipc/illustrations.ts` |
| `ipcUpdateIllustrationAltText` | `ipc/illustrations.ts` |
| `ipcDetectOrphansWidows` | `ipc/preview.ts` |

Impacto: `IllustrationGallery.svelte` e `IllustrationDropzone.svelte` importam de `$lib/ipc/typography`, confundindo o leitor sobre a responsabilidade do módulo.

---

### P2 — Chamadas `invoke()` Diretas Bypassando Abstração (MÉDIO)

**Evidência:** `rg "invoke(" src/ --include="*.svelte"`

```
src/lib/components/preview/DistractionFreeMode.svelte:11
src/lib/components/preview/AnnotationLayer.svelte:47,80,104
src/lib/components/preview/PreviewSidebar.svelte:68,83,105
```

3 componentes ignoram o wrapper `ipc<T>()`. Consequência: sem tratamento de erro padronizado, sem tipagem consistente via `ApiResponse<T>`.

---

### P3 — Módulo `ipc/preview.ts` Ausente (MÉDIO)

9 comandos de preview (`toggle_distraction_free`, `add_annotation`, `get_annotations`, `delete_annotation`, `parse_manuscript`, `list_illustrations`, etc.) são chamados via `invoke()` direto pois não há módulo IPC dedicado para preview.

---

### P4 — `ipc/preferences.ts` Usa `invoke` Direto (BAIXO)

**Evidência:** `grep "import.*invoke" src/lib/ipc/preferences.ts`

Único módulo IPC que não usa o wrapper `ipc<T>()` de `utils/ipc.ts`. Os outros 5 módulos (`projects.ts`, `typography.ts`, `cover.ts`, `generation.ts`, `bes.ts`) todos usam o wrapper.

---

### P5 — `preferencesStore.ts` com 3 TODOs Não Implementados (BAIXO)

`initPreferences`, `setTheme`, `setLanguage` têm `// TODO: Implementar backend` apesar de `ipc/preferences.ts` já existir com `ipcGetPreferences` e `ipcSetPreference`.

---

## O que está BEM

- Estrutura commands → services → repositories no Rust
- Wrapper `ipc<T>()` adotado por 5/6 módulos IPC
- Svelte 5 runes usados corretamente ($state, $effect, $props)
- Tipos centralizados em `types/enums.ts` e `types/interfaces.ts`
- Toast system centralizado via `toastStore`
- Error handling tipado no Rust (`error.rs` com `AppError`)
- Stores bem organizadas (7 módulos)
- Sem dependências circulares detectadas

---

## Métricas Pré-Execução

| Métrica | Valor |
|---------|-------|
| Arquivos .svelte | 48 |
| Módulos IPC frontend | 6 (faltam 2: illustrations, preview) |
| Chamadas `invoke()` diretas em componentes | 7 (em 3 componentes) |
| TODOs em preferencesStore | 3 |
| Funções fora do módulo correto | 5 |

---

## Tarefas Executadas

| Task | Status | Resultado |
|------|--------|-----------|
| T001 — Criar `ipc/illustrations.ts` | ✅ DONE | 4 funções extraídas de typography.ts |
| T002 — Criar `ipc/preview.ts` | ✅ DONE | 5 funções + tipo Annotation exportado |
| T002b — Criar `ipc/parser.ts` | ✅ DONE | ipcParseManuscript + ipcCalculateCompleteness |
| T003 — Corrigir `ipc/typography.ts` | ✅ DONE | Removidas 5 funções fora do escopo; adicionado ipcGenerateToc |
| T004 — Corrigir `DistractionFreeMode.svelte` | ✅ DONE | Usa ipcToggleDistractionFree |
| T005 — Corrigir `AnnotationLayer.svelte` | ✅ DONE | 3 invokes → ipcGetAnnotations, ipcAddAnnotation, ipcDeleteAnnotation |
| T006 — Corrigir `PreviewSidebar.svelte` | ✅ DONE | invoke direto removido em 3 pontos |
| T007 — Corrigir `ipc/preferences.ts` | ✅ DONE | Usa ipc<T>() em vez de invoke() |
| T008 — Conectar `preferencesStore.ts` ao backend | ✅ DONE | 3 TODOs removidos; backend + fallback localStorage |
| T009 — Atualizar imports IllustrationGallery + Dropzone | ✅ DONE | Importam de $lib/ipc/illustrations |
| T010 — Atualizar import TypographyPreview | ✅ DONE | ipcDetectOrphansWidows de $lib/ipc/preview |
