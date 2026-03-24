# MODULE-8-HANDLERS-AUDIT.md
**Projeto:** BES Book Formatter (SvelteKit 5 + Tauri 2)
**Data:** 2026-03-22
**Escopo:** TASK-5 — Validação de handlers em elementos interativos (botões, links, forms, IPC calls)
**Auditor:** SystemForge (automated review)

---

## 1. Metodologia

- Revisão de todos os `<button>`, `<a>`, `<input>`, `<select>`, `<form>` e chamadas `invoke()` / `ipc()` em rotas e componentes
- Critérios: Svelte 5 usa `onclick` (não `on:click`); handlers vazios `() => {}` são gap; `invoke()` sem `try/catch` é gap
- Cobertura: 10 rotas + 33 componentes Svelte analisados

---

## 2. Inventário por Tela

### 2.1 Dashboard (`src/routes/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `<a href="/import">` | Link | N/A — navegação nativa | OK |
| `<a href="/project/{id}">` | Link (loop) | N/A — navegação nativa | OK |
| `EmptyState onCta` | Prop callback | `() => goto('/import')` | OK |

Nenhum `<button>` direto nesta rota. Todos os elementos interativos navegam via `<a>` ou prop callback com destino explícito.

### 2.2 Import (`src/routes/import/+page.svelte`)

Delega inteiramente para `ImportWizard`. Ver seção de componentes.

### 2.3 Project Editor (`src/routes/project/[id]/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |
| Placeholder `<!-- TODO: Implementar backend -->` | — | N/A (placeholder sem interatividade) | INFO |

Rota em estado de placeholder. Nenhum handler ausente — não há elementos interativos além do EmptyState.

### 2.4 Typography (`src/routes/project/[id]/typography/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |
| `onMount` IPC `ipcGetTypographyConfig` | IPC call | `try/catch` com `toast.error` | OK |

Toda interatividade delegada a sub-componentes (`TypographyPanel`, `FontUploader`, `FontCatalog`, `PageConfigPanel`, `TypographyPreview`).

### 2.5 Illustrations (`src/routes/project/[id]/illustrations/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |

Toda interatividade delegada a `IllustrationGallery`.

### 2.6 Output (`src/routes/project/[id]/output/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |
| `$t` sem `$` prefix no `<script>` | Bug menor | `$t('nav.output')` correto no head, `$t(...)` no HTML — OK | OK |

Toda interatividade delegada a `GenerationPanel`.

### 2.7 Preview (`src/routes/project/[id]/preview/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |
| `svelte:window onkeydown` | Keyboard handler | `handleKeydown` — implementado | OK |
| `PreviewSidebar onNavigate`, `onToggleCollapse` | Props | Arrow functions | OK |
| `PreviewToolbar onNavigate`, `onZoomChange`, `onSpreadToggle`, `onRulerToggle`, `onAnnotationsToggle`, `onTypoHighlightsToggle`, `onDetectTypoIssues` | Props | Arrow functions | OK |
| `PageSpreadViewer onRendered`, `onNavigate` | Props | `handleRendered`, arrow | OK |
| `PreviewRightPanel onToggleCollapse`, `onNavigate`, `onTypoIssuesDetected` | Props | Arrow functions | OK |
| `detectTypoIssuesFromToolbar` (IPC) | invoke call | `try/catch` + `console.error` | RESSALVA |

**Gap L59 (`preview/+page.svelte`):** `detectTypoIssuesFromToolbar` captura exceções mas faz apenas `console.error` — sem feedback ao usuário via toast.

### 2.8 Cover (`src/routes/project/[id]/cover/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `EmptyState onCta` | Prop callback | `() => goto('/')` | OK |
| `onMount` IPC `ipcGetCoverTemplates` + `ipcGetCoverConfig` | IPC call | `try/catch` com `toast.error` | OK |
| `CoverEditor onSaved`, `onPreviewRequest` | Props | `handleConfigSaved`, `() => { previewTrigger++; }` | OK |
| `SpineCalculator onSpineCalculated` | Prop | `handleSpineCalculated` | OK |

### 2.9 Project Settings (`src/routes/project/[id]/settings/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `<button>` tab "integration" | button | `onclick={() => switchTab('integration')}` | OK |
| `<button>` tab "progress" | button | `onclick={() => switchTab('progress')}` | OK |

### 2.10 Global Settings (`src/routes/settings/+page.svelte`)

| Elemento | Tipo | Handler | Status |
|----------|------|---------|--------|
| `<input type="radio">` tema light | radio | `onchange={() => handleThemeChange('light')}` | OK |
| `<input type="radio">` tema dark | radio | `onchange={() => handleThemeChange('dark')}` | OK |
| `<select>` idioma | select | `onchange={handleLanguageChange}` | OK |
| `<input type="checkbox">` analytics | checkbox | `onchange={handleAnalyticsChange}` | OK |

---

## 3. Inventário por Componente

### 3.1 `ImportWizard.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Selecionar pasta | 159 | `onclick={handleSelectFolder}` | OK |
| `<button>` Voltar | 253 | `onclick={handleBack}` | OK |
| `<button>` Próximo | 264 | `onclick={handleNext}` | OK |
| `<button>` Importar | 275 | `onclick={handleImport}` | OK |
| IPC `ipc('select_directory')` | 29 | `try/catch` com `toast.error` | OK |
| IPC `ipc('read_book_config')` | 62 | `try/catch` com `configError` | OK |
| IPC `ipc('import_project')` | 91 | `try/catch` com `toast.error` | OK |
| IPC `ipc('write_bes_format')` | 109 | Sem `try/catch` próprio — dentro do bloco `try` do `handleImport` | OK |

### 3.2 `IllustrationGallery.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` filtros (5x) | 238 | `onclick={() => (activeFilter = filter.key)}` | OK |
| `<button>` Skip alt-text | 179 | `onclick={skipAltText}` | OK |
| `<button>` Salvar alt-text | 185 | `onclick={saveAltText}` | OK |
| `<button>` Fechar modal dropzone | 218 | `onclick={closeDropzoneModal}` | OK |
| IPC `ipcListIllustrations` | 61 | `try/catch` com `toast.error` | OK |
| IPC `ipcUpdateIllustrationAltText` | 90 | `try/catch` com `toast.error` | OK |

### 3.3 `GenerationPanel.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Nova Geração | 124 | `onclick={() => (showNewGeneration = true)}` | OK |
| `<button>` Cancelar fluxo | 128 | `onclick={() => { showNewGeneration = false; generationStore.reset(); }}` | OK |
| `<button>` Run Preflight | 159 | `onclick={runPreflight}` | OK |
| IPC `ipcRunPreflight` | 41 | `try/catch` com `generationStore.setError` + `toastStore.error` | OK |
| IPC `ipcGenerateEpub/PdfPrint/PdfEbook/Docx` | 63–72 | `try/catch` por formato com `toastStore.error` | OK |
| IPC `ipcGetGenerationResults` ($effect) | 109 | `.then().catch()` com `toastStore.error` | OK |
| IPC `ipcGetGenerationResults` (refresh) | 92 | Sem `try/catch` — dentro de `startGeneration` que já tem try/catch | OK |

### 3.4 `GenerationResults.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Abrir pasta | 98 | `onclick={() => openFolder(item.outputPath)}` | OK |
| `<button>` Regenerar | 106 | `onclick={() => onRegenerate?.(item.format, item.platform)}` | OK |
| IPC `invoke('plugin:shell|open')` | 43 | `try/catch` com `toastStore.error` | OK |

### 3.5 `TypographyPanel.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<select>` fonte body | 121 | `onchange={onFontBodyChange}` | OK |
| `<input type="number">` font-size | 139 | `onchange={onFontSizeChange}` | OK |
| `<input type="range">` leading | 163 | `oninput={onLeadingChange}` | OK |
| `<input type="checkbox">` justification | 187 | `onchange={onJustificationToggle}` | OK |
| `<input type="checkbox">` hyphenation | 205 | `onchange={onHyphenationToggle}` | OK |
| `<input type="number">` orphan-control | 221 | `onchange={onOrphanControlChange}` | OK |
| `<input type="number">` widow-control | 236 | `onchange={onWidowControlChange}` | OK |
| `<input type="radio">` illustration mode (3x) | 267 | `onchange={onIllustrationMissingModeChange}` | OK |
| IPC `ipcSetTypographyConfig` (debounced) | 29 | `try/catch` com `toast.error` | OK |

### 3.6 `FontCatalog.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` selecionar fonte (bundled + custom) | 100, 129 | `onclick={() => selectFont(font.name)}` | OK |
| `<button>` deletar fonte | 138 | `onclick={() => (confirmDelete = font.name)}` | OK |
| `<button>` confirmar delete | 168 | `onclick={() => deleteFont(confirmDelete!)}` | OK |
| `<button>` cancelar delete | 165 | `onclick={() => (confirmDelete = null)}` | OK |
| IPC `ipcListFonts` | 31 | `try/catch` com `toast.error` | OK |
| IPC `ipcSetTypographyConfig` (selectFont) | 43 | `try/catch` com `toast.error` | OK |
| IPC `ipcDeleteCustomFont` | 61 | `try/catch` com `toast.error` | OK |

### 3.7 `FontUploader.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Adicionar fonte | 46 | `onclick={handleSelectFont}` | OK |
| IPC `ipc('select_font_file')` | 21 | Sem `try/catch` para o `ipc()` call | GAP |
| IPC `ipcUploadFont` | 26 | `try/catch` com `toast.error` diferenciado | OK |

**Gap L17-21 (`FontUploader.svelte`):** `ipc('select_font_file')` chamado sem `try/catch`. Se o IPC falhar (ex: canal Tauri indisponível), a exceção não tratada pode crashar o componente silenciosamente — sem feedback ao usuário.

### 3.8 `PreviewToolbar.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` página anterior | 95 | `onclick={() => onNavigate(currentPage - 1)}` | OK |
| `<button>` página seguinte | 121 | `onclick={() => onNavigate(currentPage + 1)}` | OK |
| `<input type="number">` página | 107 | `onblur={handlePageInputBlur}` + `onkeydown={handlePageInputKeydown}` | OK |
| `<select>` zoom | 137 | `onchange={(e) => onZoomChange(...)}` | OK |
| `<input type="checkbox">` spread | 155 | `onchange={(e) => onSpreadToggle(...)}` | OK |
| `<input type="checkbox">` ruler | 163 | `onchange={(e) => onRulerToggle(...)}` | OK |
| `<input type="checkbox">` annotations | 175 | `onchange={(e) => onAnnotationsToggle?.(...)}` | OK |
| `<button>` detectar tipografia | 187 | `onclick={onDetectTypoIssues}` | OK |

### 3.9 `PreviewSidebar.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` collapse/expand | 119 | `onclick={onToggleCollapse}` | OK |
| `<button>` tabs (chapters/gallery/projects) | 131, 143, 153 | `onclick={() => (activeTab = ...)}` | OK |
| `<button>` capítulo (por item) | 190 | `onclick={() => onNavigate(i + 1)}` | OK |
| `<button>` projeto recente (por item) | 243 | `onclick={() => openProject(proj.id)}` | OK |
| `<button>` collapsed tabs (3x) | 268, 273, 279 | `onclick={() => { activeTab = ...; onToggleCollapse(); }}` | OK |
| IPC `invoke('parse_manuscript')` | 64 | `try/catch` com `console.error` | RESSALVA |
| IPC `invoke('list_illustrations')` | 77 | `try/catch` com `console.error` | RESSALVA |
| IPC `invoke('get_project')` | 98 | `try/catch` com `console.error` | RESSALVA |

**Gap L60-84 (`PreviewSidebar.svelte`):** `loadChapters`, `loadIllustrations` e `openProject` capturam erros mas apenas fazem `console.error` — sem nenhum feedback visual ao usuário. Se a carga de capítulos falhar, o painel ficará vazio sem indicação de erro.

### 3.10 `PageSpreadViewer.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `svelte:window onkeydown` | 237 | `handleKeydown` — navegação por setas | OK |
| `<button>` Retry | 266 | `onclick={retry}` | OK |
| IPC `invoke('render_preview_page')` | 98 | `try/catch` com `error = String(e)` | OK |
| IPC `invoke('render_preview_page')` (background) | 146 | `try/catch` silencioso (prefetch, não crítico) | OK |

### 3.11 `CoverEditor.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` tabs (template/text/design) | 76, 82, 88 | `onclick={() => (activeSection = ...)}` | OK |
| `<select>` plataforma | 114 | `bind:value={platform}` | OK (two-way binding) |
| `<select>` papel | 122 | `bind:value={paperType}` | OK |
| `<input>` title override | 135 | `bind:value={titleOverride}` | OK |
| `<input>` subtitle | 145 | `bind:value={subtitle}` | OK |
| `<input>` author | 155 | `bind:value={authorOverride}` | OK |
| `<textarea>` back cover | 167 | `bind:value={backCoverText}` | OK |
| `<input type="color">` cor primária | 188 | `bind:value={primaryColor}` | OK |
| `<input type="color">` cor secundária | 208 | `bind:value={secondaryColor}` | OK |
| `<button>` Visualizar | 253 | `onclick={onPreviewRequest}` | OK |
| `<button>` Salvar | 259 | `onclick={handleSave}` | OK |
| IPC `ipcSaveCoverConfig` | 60 | `try/catch` com `toast.error` | OK |

### 3.12 `CoverPreview.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Regenerar preview | 94 | `onclick={generatePreview}` | OK |
| `<button>` Gerar Preview (empty state) | 113 | `onclick={generatePreview}` | OK |
| `<select>` formato export | 132 | `bind:value={exportFormat}` | OK |
| `<select>` resolução | 139 | `bind:value={exportResolution}` | OK |
| `<button>` Exportar capa | 147 | `onclick={handleExport}` | OK |
| IPC `ipcGenerateCoverPdf` | 35 | `try/catch` com `toast.error` | OK |
| IPC `ipcExportCoverImage` | 51 | `try/catch` com `toast.error` | OK |

### 3.13 `BesStatusPanel.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` Recheck (idle) | 90 | `onclick={validateWorkspace}` | OK |
| `<button>` Recheck (refresh) | 142 | `onclick={validateWorkspace}` | OK |
| IPC `ipcValidateBesWorkspace` | 40 | `try/catch` com `status = 'error'` | OK |
| IPC `ipcGetBesMetadata` | 64 | `try/catch` silencioso (best-effort, documentado) | OK |

### 3.14 `PreGenerationChecklist.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `<button>` toggle blockers | 47 | `onclick={() => (expandedBlockers = !expandedBlockers)}` | OK |
| `<button>` toggle warnings | 79 | `onclick={() => (expandedWarnings = !expandedWarnings)}` | OK |
| `<button>` Recheck | 101 | `onclick={onRecheck}` | OK |
| `<button>` Gerar | 104 | `onclick={onGenerate}` | OK |

### 3.15 `DistractionFreeMode.svelte`

| Elemento | Linha | Handler | Status |
|----------|-------|---------|--------|
| `svelte:window onkeydown` | 40 | `handleKeydown` | OK |
| IPC `invoke('toggle_distraction_free')` | 11 | Sem `try/catch` — await direto sem tratamento | GAP |

**Gap L10-12 (`DistractionFreeMode.svelte`):** `toggle()` chama `await invoke('toggle_distraction_free', ...)` sem `try/catch`. Se o IPC falhar (ex: modo de desenvolvimento, Tauri não registrado), a exceção se propaga silenciosamente para o handler de teclado sem feedback ao usuário.

---

## 4. Resumo de Gaps

| # | Arquivo | Linha | Tipo | Descrição |
|---|---------|-------|------|-----------|
| G1 | `preview/+page.svelte` | 51–61 | IPC sem toast | `detectTypoIssuesFromToolbar` captura exceção mas só faz `console.error` — usuário não recebe feedback de falha |
| G2 | `FontUploader.svelte` | 17–21 | IPC sem try/catch | `ipc('select_font_file')` sem tratamento de erro — exceção pode propagar silenciosamente |
| G3 | `PreviewSidebar.svelte` | 60–84 | IPC sem feedback visual | `loadChapters`, `loadIllustrations`, `openProject` com `console.error` apenas — painéis ficam vazios sem indicação de erro |
| G4 | `DistractionFreeMode.svelte` | 10–12 | IPC sem try/catch | `invoke('toggle_distraction_free')` sem proteção — pode crashar handler de teclado |

---

## 5. Conformidade Svelte 5

Todos os handlers inspecionados usam a sintaxe correta Svelte 5:
- `onclick` (não `on:click`) — CONFORME em todos os arquivos
- `onchange` (não `on:change`) — CONFORME
- `oninput` (não `on:input`) — CONFORME
- `onblur`/`onkeydown` (não `on:blur`/`on:keydown`) — CONFORME

Nenhum handler vazio `() => {}` encontrado. Não há botões sem `onclick`.

---

## 6. Verdict

**PASS COM RESSALVAS**

A grande maioria dos handlers está corretamente implementada com tratamento de erro via toast. Os 4 gaps identificados são de baixa-a-média severidade (não bloqueiam funcionalidades críticas, mas degradam a experiência ao ocultar erros do usuário).

**Ações recomendadas (por prioridade):**
1. **G4** — `DistractionFreeMode`: adicionar `try/catch` no `invoke` (risco de crash silencioso no handler de teclado)
2. **G2** — `FontUploader`: adicionar `try/catch` no `ipc('select_font_file')`
3. **G1** — `preview/+page.svelte`: adicionar `toast.error` no catch de `detectTypoIssuesFromToolbar`
4. **G3** — `PreviewSidebar`: adicionar indicação visual de erro (estado de erro nos painéis de capítulos/ilustrações/projetos)
