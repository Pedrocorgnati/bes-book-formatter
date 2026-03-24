# MODULE-8-REMEDIATION-TASKS.md

**Data:** 2026-03-22
**Originado de:** READY-TO-DEPLOY.md — Veredito NO-GO
**Total de tasks:** 4 bloqueadores + 11 recomendações

---

## Bloqueadores P0/P1 (obrigatórios antes do GO)

### RT-01 — Criar rota `/project/[id]/integration` [P0]

**Arquivo a criar:** `src/routes/project/[id]/integration/+page.svelte`
**Arquivos a modificar:**
- `src/routes/project/[id]/+layout.svelte` — adicionar tab `integration`
- `src/lib/components/layout/Sidebar.svelte` — adicionar link de atalho

**Conteúdo esperado da página:**
- Mostrar status BES workspace (`validate_bes_workspace`)
- Botão "Sync Progress" → `sync_editorial_progress`
- Botão "Update F10" → `update_editorial_f10`
- Docs BES carregados → `read_bes_docs` com cache TTL
- CLI status: `bes-format check` output
- Empty state: "Configure seu workspace BES para ver a integração"
- Error state: "Não foi possível conectar ao workspace BES" + retry

**Integração com ipc/bes.ts:** todos os IPC commands já existem.

**Critério de aceite:**
- [ ] Rota acessível via tab no editor de projeto
- [ ] Rota acessível via Sidebar
- [ ] 4 estados: loading, empty, error, success
- [ ] TASK-1 re-auditada com PASS

---

### RT-02 — Dashboard loading state [P1]

**Arquivo:** `src/routes/+page.svelte`

**Problema:** Quando `$projectsStore.loading === true`, a tela renderiza o grid vazio instantaneamente.

**Correção:**
```svelte
{#if $projectsStore.loading}
  <!-- Skeleton grid: 6 placeholders -->
  <div class="projects-grid projects-grid--skeleton" aria-label="Carregando projetos..." aria-busy="true">
    {#each Array(6) as _}
      <div class="project-card project-card--skeleton" aria-hidden="true">
        <div class="skeleton-pulse" style="height: 120px; border-radius: 8px;"></div>
        <div class="skeleton-pulse" style="height: 16px; width: 60%; margin-top: 8px;"></div>
      </div>
    {/each}
  </div>
{:else if $projectsStore.error}
  <!-- Error state já existe -->
{:else if $projectsStore.list.length === 0}
  <!-- EmptyState já existe -->
{:else}
  <!-- Grid de projetos já existe -->
{/if}
```

**Critério de aceite:**
- [ ] Skeleton visível durante load inicial (>200ms)
- [ ] `aria-busy="true"` durante loading
- [ ] TASK-6 re-auditada com PASS para Dashboard

---

### RT-03 — PreviewSidebar states [P1]

**Arquivo:** `src/lib/components/preview/PreviewSidebar.svelte`

**Problemas:**
1. `chaptersLoading` existe no state mas não é renderizado no template
2. Error state de capítulos apenas usa `console.error` sem feedback visual

**Correção:**
```svelte
<!-- Seção de capítulos -->
{#if chaptersLoading}
  <div class="sidebar-section sidebar-section--loading" aria-busy="true">
    {#each Array(5) as _}
      <div class="skeleton-pulse" style="height: 24px; margin-bottom: 4px;"></div>
    {/each}
  </div>
{:else if chaptersError}
  <div class="sidebar-empty" role="alert">
    <p class="sidebar-empty__message">{chaptersError}</p>
    <button onclick={loadChapters} class="btn btn--sm btn--ghost">Tentar novamente</button>
  </div>
{:else if chapters.length === 0}
  <p class="sidebar-empty__message">Nenhum capítulo encontrado</p>
{:else}
  <!-- lista de capítulos -->
{/if}
```

Mesmo padrão para `illustrationsLoading`/`projectsLoading`.

**Critério de aceite:**
- [ ] Loading skeleton visível ao abrir Preview com projeto grande
- [ ] Error state distinct com botão de retry
- [ ] Empty state mostra mensagem clara
- [ ] TASK-6 re-auditada com PASS para PreviewSidebar

---

### RT-04 — Toast warning contrast [P1 — WCAG AA]

**Arquivo:** `src/app.css` (ou tokens CSS globais)

**Problema:** `--color-warning` usa `#D97706` (amber-600) com texto branco = ~3.0:1 (mínimo: 4.5:1).

**Correção:**

| Uso | Background atual | Texto atual | Background novo | Texto novo | Ratio |
|-----|-----------------|-------------|----------------|------------|-------|
| Toast warning | `#D97706` | `#FFFFFF` | `#92400E` (amber-900) | `#FEF3C7` | ~6.1:1 ✓ |
| Badge warning | `#FEF3C7` | `#D97706` | `#FEF3C7` | `#78350F` | ~7.5:1 ✓ |

```css
/* src/app.css — light theme */
--color-warning: #92400E;       /* era #D97706 */
--color-warning-bg: #FEF3C7;
--color-warning-text: #78350F;
```

```css
/* dark theme */
--color-warning: #FCD34D;       /* amber-300, sobre dark bg */
--color-warning-bg: #451A03;
--color-warning-text: #FDE68A;
```

**Critério de aceite:**
- [ ] Toast warning visível e legível
- [ ] Ratio ≥ 4.5:1 (verificar com axe-core ou browser devtools)
- [ ] TASK-8 re-auditada sem violations WCAG AA

---

## Issues P2 (recomendadas antes do QA)

### RT-05 — `get_cover_config` import faltando [P2]

**Arquivo:** `src-tauri/src/lib.rs`

**Problema:** `get_cover_config` está no `invoke_handler` (linha ~152) mas não está no bloco `use commands::cover::{...}`.

**Correção:**
```rust
use commands::cover::{
    calculate_spine_width, export_cover_image, generate_cover_pdf, get_cover_config, // ← adicionar
    get_cover_templates, save_cover_config,
};
```

**Critério de aceite:**
- [ ] `cargo build --release` sem erros

---

### RT-06 — Silent errors: detectTypoIssues + FontUploader [P2]

**Arquivos:**
- `src/routes/project/[id]/preview/+page.svelte` — `detectTypoIssuesFromToolbar`
- `src/lib/components/typography/FontUploader.svelte` — `ipc('select_font_file')`

**Correção:** adicionar `toast.error()` nos catch de cada um.

---

### RT-07 — PreviewSidebar load errors com toast [P2]

**Arquivo:** `src/lib/components/preview/PreviewSidebar.svelte`

**Correção:** substituir `console.error` por `toast.error(t('error.loadFailed'))` em `loadChapters`, `loadIllustrations`, `openProject`.

---

### RT-08 — Typst preview PNG cache [P2]

**Arquivo:** `src-tauri/src/services/preview_service.rs` (ou equivalente)

**Problema:** `render_preview_page` recompila o Typst a cada chamada, mesmo sem mudanças no manuscrito. Para documentos grandes (200+ páginas), P95 > 500ms.

**Correção:** implementar cache de PNG por (project_id, page_number, content_hash). Invalidar apenas quando o manuscrito muda.

**Estimativa:** 4-6h (maior esforço deste batch)

---

### RT-09 — schema_version inserts faltando em M004-M007 [P2]

**Arquivos:** `src-tauri/migrations/M004_generation_results.sql`, `M005_annotations.sql`, `M006_bes_document_cache.sql`, `M007_cover_configs.sql`

**Correção:** adicionar ao final de cada arquivo:
```sql
INSERT INTO schema_version (migration_name) VALUES ('M004_generation_results');
```
(ajustar nome conforme o arquivo)

---

## Issues P3 (nice-to-have antes do release)

### RT-10 — Genre.Poetry sincronizar TS↔Rust [P3]
Adicionar `POETRY = 'poetry'` ao enum `Genre` em `src/lib/types/enums.ts`.

### RT-11 — Renomear TypographyDefaults para clareza [P3]
Manter `TypographyDefaults` mas adicionar alias/export `TypographyConfig = TypographyDefaults` em `$lib/types/index.ts` para alinhar com spec.

### RT-12 — currentProject store alias [P3]
Exportar `export const currentProject = derived(projectsStore, $s => $s.current)` em `src/lib/stores/projectStore.ts` para manter C10 do contrato.

### RT-13 — id="main-content" duplicado [P3]
Verificar e remover duplicata em AppShell e +layout.svelte.

### RT-14 — catch vazio em guard project/[id]/+layout.svelte [P3]
Adicionar `goto('/')` ou `toast.error()` no bloco `catch {}` vazio (linha ~69).

---

## Sequência de execução

```
FASE 1 — Bloqueadores (6h estimado):
  RT-01 (Rota integration) → 3-4h
  RT-02 (Dashboard loading) → 1h
  RT-03 (PreviewSidebar states) → 1.5h
  RT-04 (Toast contrast) → 0.5h

FASE 2 — P2 (8h estimado):
  RT-05 (get_cover_config import) → 0.5h
  RT-06 (silent errors preview/font) → 0.5h
  RT-07 (PreviewSidebar toast) → 0.5h
  RT-08 (Typst PNG cache) → 4-6h
  RT-09 (schema_version inserts) → 0.5h

FASE 3 — P3 (3h estimado):
  RT-10 a RT-14 (batch de 5 fixes) → 3h

FASE 4 — Re-auditoria:
  Re-executar TASK-1, TASK-2, TASK-6, TASK-7, TASK-8
  Re-executar TASK-12 (Go/No-Go)
  Se GO: prosseguir F9 (/qa:prep)
```

---

*Gerado por auto-flow execute — module-8-cross-rock-integration — 2026-03-22*
