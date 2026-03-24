# MODULE-8-STATES-AUDIT.md
**Projeto:** BES Book Formatter (SvelteKit 5 + Tauri 2)
**Data:** 2026-03-22
**Escopo:** TASK-6 — Validação de estados em componentes data-driven (loading / empty / error / success)
**Auditor:** SystemForge (automated review)

---

## 1. Metodologia

Critérios de aprovação por estado:

| Estado | Critério mínimo |
|--------|----------------|
| **loading** | Spinner, skeleton ou indicador textual com `role="status"` |
| **empty** | Mensagem explicativa + CTA quando aplicável |
| **error** | Toast, banner `role="alert"`, ou estado inline visível ao usuário |
| **success** | Dados renderizados corretamente, sem stub/placeholder |

Cada componente data-driven é avaliado nos 4 estados.

---

## 2. Avaliação por Componente

### 2.1 Dashboard — Lista de Projetos (`src/routes/+page.svelte`)

Responsável por exibir a lista de projetos do usuário.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `$projectsStore.loading` — condicional `{#if loading}` presente (`+layout.svelte` / store gerencia globalmente); a tela não trava durante carga | RESSALVA |
| **empty** | `{:else if projects.length === 0}` → `<EmptyState>` com `onCta` para `/import` — mensagem + CTA | OK |
| **error** | `{#if error}` → banner `role="alert"` com título e detalhe do erro | OK |
| **success** | `{:else}` → grid de `<ProjectCard>` com dados reais | OK |

**Ressalva loading (linha 8–10):** `loading` é derivado de `$projectsStore.loading`, mas o template na rota `+page.svelte` não renderiza nenhum skeleton/spinner enquanto `loading === true` e `projects.length === 0`. O `{#if loading} return` no `$effect` (linha 13–14) apenas bloqueia o redirect automático, mas não exibe indicador visual. O usuário vê tela em branco durante a carga inicial.

**Gap:** Falta bloco `{#if loading}` no template com skeleton ou spinner.

### 2.2 IllustrationGallery (`src/lib/components/illustrations/IllustrationGallery.svelte`)

Galeria de ilustrações do livro com filtros.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `role="status" aria-live="polite"` com spinner SVG animado e label textual | OK |
| **empty** | `{:else if filteredIllustrations.length === 0}` → div `role="status"` com ícone SVG + mensagem contextual (diferencia "filtrado" de "vazio total") | OK |
| **error** | `try/catch` → `toast.error(t('illustrations.loadError'))` | RESSALVA |
| **success** | `{:else}` → grid com `<IllustrationCard>` por item | OK |

**Ressalva error:** O erro de carregamento é exibido via toast (efêmero), mas o componente fica no estado "empty" após o erro — não há banner inline persistente que indique que o carregamento falhou vs. que realmente não há ilustrações. Usuário pode confundir erro com galeria vazia.

### 2.3 PageSpreadViewer — Preview do Livro (`src/lib/components/preview/PageSpreadViewer.svelte`)

Renderização das páginas do livro via IPC.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `aria-hidden="true"` com skeleton pulsante (`skeleton-pulse`) + `<span class="sr-only">` acessível | OK |
| **empty** | `{:else if pages.length === 0}` → div `role="status"` com mensagem `t('preview.noProject')` | OK |
| **error** | `{:else if error}` → div `role="alert"` com mensagem de erro + `<button>` Retry | OK |
| **success** | `{:else}` → páginas renderizadas com `<img>` + overlays opcionais | OK |

**Todos os 4 estados implementados corretamente.** Este é o componente mais completo da auditoria.

### 2.4 GenerationResults — Histórico de Geração (`src/lib/components/generation/GenerationResults.svelte`)

Tabela de histórico de arquivos gerados.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `role="status"` com spinner `.spinner` e label textual | OK |
| **empty** | `{:else if history.length === 0}` → `<p role="status">` com `$t('generation.historyEmpty')` | OK |
| **error** | Sem estado de erro inline no componente — erro tratado pelo pai (`GenerationPanel`) via banner `role="alert"` | RESSALVA |
| **success** | `{:else}` → `<table>` com dados reais, badge de status por item | OK |

**Ressalva error:** `GenerationResults` não tem erro próprio — depende do `GenerationPanel` para exibir erros de carga (`ipcGetGenerationResults`). O handler no `$effect` do `GenerationPanel` chama `toastStore.error` + `generationStore.setHistoryLoading(false)`, mas não há estado de erro persistente na UI do componente de histórico.

### 2.5 TypographyPanel — Painel de Configuração (`src/lib/components/typography/TypographyPanel.svelte`)

Configurações de tipografia do livro.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` no header → `<span class="saving-indicator" aria-live="polite">` com spinner e "Salvando" | RESSALVA |
| **empty** | `{#if !config}` → `<div class="panel-empty" role="status">` com `t('typography.loading')` | RESSALVA |
| **error** | Erro de carga via `toast.error` no `onMount` da rota pai | RESSALVA |
| **success** | `{:else}` → painel completo com todos os campos | OK |

**Ressalva loading:** O `loading` no `TypographyPanel` indica "salvando" (debounce), não "carregando dados iniciais". O carregamento inicial é feito no `onMount` da rota `typography/+page.svelte`, que usa `typographyLoadingStore`. O painel não exibe um skeleton durante a carga inicial — mostra o estado `{#if !config}` com mensagem "Carregando..." (que é ambíguo entre "ainda carregando" e "não configurado").

**Ressalva empty:** A mensagem de empty state (`panel-empty`) usa `t('typography.loading')` — o texto da chave de i18n é "carregando" em vez de "não configurado", gerando ambiguidade semântica.

**Ressalva error:** Erro de carga não é surfado inline no `TypographyPanel` — depende do toast gerado na rota pai. Se o toast fechar, nenhum indicador visual de erro persiste.

### 2.6 FontCatalog — Catálogo de Fontes (`src/lib/components/typography/FontCatalog.svelte`)

Lista de fontes disponíveis (bundled + custom).

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `role="status" aria-live="polite"` com spinner e `t('common.loading')` | OK |
| **empty** | `{#if bundledFonts.length === 0}` → `<p class="catalog-empty">` com `t('typography.noFontsFound')`; idem para custom fonts | OK |
| **error** | `toast.error(t('typography.fontsLoadError'))` no catch | RESSALVA |
| **success** | Listas `<ul>` com fontes renderizadas | OK |

**Ressalva error:** Mesmo padrão dos outros componentes — toast efêmero sem estado de erro persistente. Após o toast fechar, a lista fica vazia sem explicação.

### 2.7 PreviewSidebar — Painéis de Capítulos/Ilustrações/Projetos (`src/lib/components/preview/PreviewSidebar.svelte`)

Sidebar com 3 tabs de conteúdo data-driven.

#### Tab Chapters

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `chaptersLoading` existe no estado, mas **não há bloco `{#if chaptersLoading}`** no template do painel chapters — renderiza imediatamente a lista vazia | GAP |
| **empty** | `{#if filteredChapters.length === 0}` → `<div class="panel-empty">` | OK |
| **error** | Apenas `console.error` no catch — sem estado de erro visível | GAP |
| **success** | Lista `<ul>` com capítulos | OK |

#### Tab Gallery (Illustrations)

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{:else if illustrationsLoading}` → `<div class="panel-empty">` com `t('preview.loading')` — texto simples, sem spinner | RESSALVA |
| **empty** | `{:else if illustrations.length === 0}` → `<div class="panel-empty">` | OK |
| **error** | Apenas `console.error` no catch — sem estado de erro visível | GAP |
| **success** | Lista `<ul>` com ilustrações e badges | OK |

#### Tab Projects

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | Não aplicável — dados vêm de store síncrono (`recentProjectsStore`) | N/A |
| **empty** | `{#if recentProjects.length === 0}` → `<div class="panel-empty">` | OK |
| **error** | `openProject` com `console.error` — sem estado de erro | GAP |
| **success** | Lista `<ul>` com projetos | OK |

### 2.8 CoverPreview — Preview da Capa (`src/lib/components/cover/CoverPreview.svelte`)

Preview 3D da capa do livro.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `role="status"` com spinner `.preview__spinner` e texto "Gerando preview…" | OK |
| **empty** | `{:else}` (sem base64) → div `.preview__empty` com ícone SVG + texto + botão CTA "Gerar Preview" | OK |
| **error** | `toast.error(String(e))` no catch — sem estado inline | RESSALVA |
| **success** | `{:else if previewBase64}` → mockup 3D com `<img>` base64 | OK |

**Ressalva error:** Erro de geração de preview tratado via toast (efêmero). Após toast fechar, o componente fica no estado empty sem indicação de que houve uma falha.

### 2.9 GenerationPanel — Painel de Geração (`src/lib/components/generation/GenerationPanel.svelte`)

Orquestrador do fluxo de geração de ebooks.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** (preflight) | `preflightLoading` → delegado ao `PreGenerationChecklist` que exibe spinner | OK |
| **loading** (geração) | `state.status === 'generating'` → exibe `<GenerationProgress>` | OK |
| **empty** | `state.status === 'idle'` + `!showNewGeneration` → botão "Nova Geração" e `<GenerationResults>` com histórico | OK |
| **error** | `{#if state.status === 'error' && state.error}` → banner `role="alert"` com mensagem | OK |
| **success** | `state.status === 'done'` → `<GenerationResults>` com resultado | OK |

**Todos os estados implementados corretamente no orquestrador.**

### 2.10 PreGenerationChecklist (`src/lib/components/generation/PreGenerationChecklist.svelte`)

Checklist de pré-geração com resultado de preflight.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` → div `role="status"` com spinner e texto | OK |
| **empty** | `{:else if result === null}` → botão "Executar Preflight" | OK |
| **error** | `{:else}` → badge `badge--error` com mensagem de falha + lista de blockers | OK |
| **success** | `{:else}` → badge `badge--success` + botão "Gerar" habilitado | OK |

**Todos os 4 estados implementados.**

### 2.11 BesStatusPanel (`src/lib/components/bes/BesStatusPanel.svelte`)

Status de validação do workspace BES.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `status === 'loading'` → ícone ⏳ + label `t('bes.statusPanel.checking')` (sem spinner animado) | RESSALVA |
| **empty/idle** | `status === 'idle'` → ícone 📁 + label + botão Recheck | OK |
| **error** | `status === 'error'` → ícone ❌ + label + mensagem de erro + lista de arquivos ausentes | OK |
| **success** | `status === 'valid'` → ícone ✅ + label + metadados | OK |

**Ressalva loading:** O estado `loading` usa apenas texto/ícone emoji sem animação visual de spinner — funcionalmente adequado mas menos polido que outros componentes.

### 2.12 TypographyPreview (`src/lib/components/typography/TypographyPreview.svelte`)

Preview simulado de página com detecção de órfãs/viúvas.

| Estado | Implementação | Status |
|--------|--------------|--------|
| **loading** | `{#if loading}` no `.preview-bar` → spinner SVG + `t('typography.verifying')` | RESSALVA |
| **empty** | `{#if !config}` → `<div class="preview-empty">` com texto | OK |
| **error** | `toast.error` no catch (exceto erro `SYS_050` que é silenciado por design) | RESSALVA |
| **success** | Página simulada com parágrafos + highlights de órfãs/viúvas | OK |

**Ressalva loading:** O spinner de loading aparece apenas na barra de caption — a página de preview é sempre renderizada com dados de amostra estáticos, independente do estado de loading. Funcionalmente não confunde o usuário, mas semanticamente o loading indicator é pouco proeminente.

**Ressalva error:** Erros IPC reais geram `toast.error`, mas erros `SYS_050` são silenciados por comentário de design — este comportamento intencional está documentado no código.

---

## 3. Tabela Consolidada

| Componente | Loading | Empty | Error | Success | Status Geral |
|------------|---------|-------|-------|---------|--------------|
| Dashboard (lista projetos) | ⚠ GAP | ✅ | ✅ | ✅ | RESSALVA |
| IllustrationGallery | ✅ | ✅ | ⚠ toast only | ✅ | RESSALVA |
| PageSpreadViewer | ✅ | ✅ | ✅ | ✅ | PASS |
| GenerationResults | ✅ | ✅ | ⚠ no inline | ✅ | RESSALVA |
| GenerationPanel | ✅ | ✅ | ✅ | ✅ | PASS |
| PreGenerationChecklist | ✅ | ✅ | ✅ | ✅ | PASS |
| TypographyPanel | ⚠ ambíguo | ⚠ ambíguo | ⚠ toast only | ✅ | RESSALVA |
| FontCatalog | ✅ | ✅ | ⚠ toast only | ✅ | RESSALVA |
| PreviewSidebar (chapters) | ⚠ GAP | ✅ | ⚠ GAP | ✅ | FAIL |
| PreviewSidebar (gallery) | ⚠ sem spinner | ✅ | ⚠ GAP | ✅ | FAIL |
| PreviewSidebar (projects) | N/A | ✅ | ⚠ GAP | ✅ | RESSALVA |
| CoverPreview | ✅ | ✅ | ⚠ toast only | ✅ | RESSALVA |
| BesStatusPanel | ⚠ sem spinner | ✅ | ✅ | ✅ | RESSALVA |
| TypographyPreview | ⚠ pouco proeminente | ✅ | ⚠ SYS_050 silenciado (intencional) | ✅ | RESSALVA |

Legenda: ✅ Implementado corretamente | ⚠ Implementado com ressalva | GAP Não implementado

---

## 4. Gaps Críticos

| # | Arquivo | Estado | Linha | Descrição |
|---|---------|--------|-------|-----------|
| S1 | `src/routes/+page.svelte` | loading | 8–26 | Nenhum skeleton/spinner exibido durante `$projectsStore.loading === true` — tela em branco antes dos projetos carregarem |
| S2 | `PreviewSidebar.svelte` | loading (chapters) | 60–70 | `chaptersLoading` existe no estado mas não há condicional `{#if chaptersLoading}` no template — sidebar de capítulos aparece imediatamente vazia |
| S3 | `PreviewSidebar.svelte` | error (chapters) | 60–70 | Erro de `loadChapters` apenas em `console.error` — usuário não sabe se os capítulos não foram carregados ou se o livro não tem capítulos |
| S4 | `PreviewSidebar.svelte` | error (gallery/projects) | 73–102 | Mesmo padrão de S3 para illustrações e projetos recentes |

---

## 5. Ressalvas Secundárias (não bloqueantes)

| # | Componente | Estado | Descrição |
|---|------------|--------|-----------|
| R1 | `TypographyPanel` | loading/empty | `panel-empty` usa chave `typography.loading` para estado sem config — semanticamente ambíguo |
| R2 | `IllustrationGallery` | error | Toast efêmero sem estado de erro persistente — galeria vazia e galeria com erro têm a mesma aparência após o toast fechar |
| R3 | `GenerationResults` | error | Sem estado de erro inline — depende inteiramente do `GenerationPanel` pai |
| R4 | `CoverPreview` | error | Toast efêmero — após fechar, componente volta ao empty state sem indicação de falha |
| R5 | `BesStatusPanel` | loading | Sem spinner animado — indicação textual apenas |
| R6 | `PreviewSidebar` (gallery) | loading | Texto simples em `panel-empty` sem spinner visual — inconsistente com outros loaders |

---

## 6. Componentes com 4 Estados Completos

Os seguintes componentes implementam todos os 4 estados corretamente:

- `PageSpreadViewer.svelte` — modelo de referência: skeleton pulsante + empty state + error com retry + success
- `GenerationPanel.svelte` — orquestrador com banners de erro inline e estados bem separados
- `PreGenerationChecklist.svelte` — estado binário pass/fail com representação visual clara

---

## 7. Verdict

**PASS COM RESSALVAS**

A maioria dos componentes data-driven implementa os 4 estados, mas com qualidade inconsistente no tratamento de erros (predominância de toasts efêmeros sem estado persistente) e gaps de loading em `PreviewSidebar` e na rota do Dashboard.

**Ações recomendadas (por prioridade):**

1. **S1 — Dashboard:** Adicionar bloco `{#if loading}` no template com skeleton de cards de projeto antes do `{#if error}` / `{:else if projects.length === 0}`
2. **S2 — PreviewSidebar (chapters):** Usar `chaptersLoading` no template para exibir indicador de loading durante `loadChapters`
3. **S3/S4 — PreviewSidebar (error states):** Adicionar estados de erro inline nos 3 painéis — `<div class="panel-error" role="alert">` com botão Retry
4. **R1 — TypographyPanel:** Corrigir chave i18n do empty state de `typography.loading` para `typography.noConfig` ou similar
5. **R2/R3/R4 — Toast efêmero:** Avaliar adição de banner de erro persistente nos componentes com maior impacto ao usuário (`IllustrationGallery`, `CoverPreview`)
