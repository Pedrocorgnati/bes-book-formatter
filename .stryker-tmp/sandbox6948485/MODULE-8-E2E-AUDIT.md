# MODULE-8 — E2E User Flows Audit (TASK-7)
**Projeto:** BES Book Formatter (SvelteKit 5 + Tauri 2 + Rust)
**Data:** 2026-03-22
**Auditor:** SystemForge Integration Auditor

---

## Sumário Executivo

| Fluxo | CU | Status | Verdict |
|-------|----|--------|---------|
| Import BES Project | CU-01 | Completo | PASS |
| Configure Typography | CU-02 | Completo | PASS |
| Generate EPUB (KDP) | CU-03 | Completo | PASS |
| Preview Live | CU-04 | Completo | PASS |
| Export to BES + CLI | CU-05 | Parcial | WARN |

**Verdict Geral: PASS com ressalvas** — 4/5 fluxos completos; CU-05 operacional mas sem rota dedicada `/integration`.

---

## CU-01 — Import BES Project

**Rota:** `/import` → `+page.svelte` → `ImportWizard.svelte`
**IPC commands usados:**
- `select_directory` — abre dialog nativo (Tauri plugin dialog)
- `read_book_config` — analisa `bes-format.yaml` no diretório selecionado
- `import_project` — cria registro no SQLite e retorna `BookProject`
- `write_bes_format` — (condicional) salva `bes-format.yaml` atualizado se nome/genre mudou

**Steps do fluxo:**

| # | Step | Implementado | Detalhe |
|---|------|-------------|---------|
| 1 | Abre página `/import` | Sim | Rota existe em `src/routes/import/+page.svelte` |
| 2 | Clica "Selecionar Pasta" | Sim | `handleSelectFolder` → `ipc('select_directory')` |
| 3 | Sistema mostra path selecionado | Sim | `wizard-selected-path` com `<code>` |
| 4 | Avança para análise | Sim | `analyseFolder()` → `read_book_config` |
| 5 | Exibe spinner durante análise | Sim | `analyzing` state + `.wizard-step__analyzing` |
| 6 | Detecta BES válido, exibe summary | Sim | `wizard-config-summary` com título e gênero |
| 7 | Erro: pasta sem BES | Sim | `wizard-bes-error` com role="alert" |
| 8 | Auto-advance para step 3 se sem warnings | Sim | `if (!res.error && res.warnings.length === 0) step = 3` |
| 9 | Edita nome/gênero no step 3 | Sim | `projectName` input + `GenrePicker` |
| 10 | Importa e redireciona | Sim | `goto('/project/${project.id}')` após sucesso |
| 11 | Erro de import → toast | Sim | `toast.error(res.error ?? t('wizard.importError'))` |
| 12 | Botão desabilitado durante import | Sim | `disabled={!canImport}` |

**Happy path:** Completo e robusto (3 steps, step indicator visual, back/next).
**Error handling:** Presente em todos os steps — toast para erros de IPC, alerta inline para pasta inválida.
**Duração esperada:** 30–60s (depende do tamanho do manuscrito).
**Verdict: PASS**

---

## CU-02 — Configure Typography

**Rota:** `/project/[id]/typography` → `+page.svelte`
**IPC commands usados:**
- `get_typography_config` — carrega config atual no `onMount`
- `set_typography_config` — salva alterações (debounced 500ms)
- `list_fonts` — lista fontes do projeto
- `upload_font` — faz upload de fonte customizada
- `delete_custom_font` — remove fonte customizada
- `validate_illustration_dpi` — valida DPI de ilustrações
- `list_illustrations` — lista ilustrações
- `detect_orphans_widows` — detecta problemas tipográficos

**Steps do fluxo:**

| # | Step | Implementado | Detalhe |
|---|------|-------------|---------|
| 1 | Navega para aba Tipografia | Sim | Tab "typography" no `+layout.svelte` |
| 2 | Carrega config existente | Sim | `ipcGetTypographyConfig` no `onMount`, `typographyStore.set(config)` |
| 3 | Exibe erro de carregamento | Sim | `toast.error(t('typography.loadError'))` no catch |
| 4 | Altera parâmetros tipográficos | Sim | `TypographyPanel` com debounced save |
| 5 | Auto-save com feedback | Sim | `saveTimer` 500ms → `toast.success` / `toast.error` |
| 6 | Configura formato de página | Sim | `PageConfigPanel` componente presente |
| 7 | Upload de fonte customizada | Sim | `FontUploader` → `upload_font` IPC |
| 8 | Catálogo de fontes | Sim | `FontCatalog` com método `refresh()` |
| 9 | Preview tipográfico em tempo real | Sim | `TypographyPreview` no painel principal |
| 10 | Estado sem projeto | Sim | `EmptyState` com CTA de volta ao dashboard |

**Happy path:** Completo — layout 2-colunas (config + preview).
**Error handling:** Toast em carregamento e save; loading state explícito.
**Save pattern:** Auto-save debounced (500ms) com `toast.success` confirmando persistência. Sem botão "Save" manual — comportamento intencional.
**Duração esperada:** 1–3min (interação livre).
**Verdict: PASS**

---

## CU-03 — Generate EPUB (KDP)

**Rota:** `/project/[id]/output` → `GenerationPanel.svelte`
**IPC commands usados:**
- `run_preflight` — verifica pré-condições antes de gerar
- `generate_epub` — gera EPUB3 para plataforma alvo (KDP/Ingram/generic)
- `generate_pdf_print` — (opcional na mesma sessão)
- `generate_pdf_ebook` — (opcional na mesma sessão)
- `generate_docx` — (opcional na mesma sessão)
- `get_generation_results` — histórico de gerações
- `cancel_generation` — cancela geração em andamento

**Steps do fluxo:**

| # | Step | Implementado | Detalhe |
|---|------|-------------|---------|
| 1 | Navega para aba Output | Sim | Tab "output" no `+layout.svelte` |
| 2 | Seleciona formato (EPUB3) | Sim | `FormatSelector` → `currentSelection` |
| 3 | Seleciona plataforma (KDP) | Sim | `FormatSelector` inclui campo platform |
| 4 | Executa preflight | Sim | `runPreflight()` → `ipcRunPreflight` → `PreGenerationChecklist` |
| 5 | Preflight falha → toast + block | Sim | `generationStore.setError` + `toastStore.error` |
| 6 | Preflight passa → habilita generate | Sim | `state.preflight?.passed` condiciona botão |
| 7 | Inicia geração | Sim | `startGeneration()` → loop por `formats` |
| 8 | Exibe progresso | Sim | `GenerationProgress` com `onCancel` |
| 9 | Geração concluída → toast success | Sim | `toastStore.success($t('generation.done'))` |
| 10 | Erro de geração → toast por formato | Sim | `toastStore.error('Erro ao gerar ${format}: ${e}')` |
| 11 | Banner de erro no painel | Sim | `.gen-panel__error` com role="alert" |
| 12 | Histórico de gerações | Sim | `GenerationResults` carregado no `$effect` |
| 13 | Cancela geração | Sim | `handleCancel()` → `generationStore.reset()` |
| 14 | Estado sem projeto | Sim | `EmptyState` com CTA |

**Happy path:** Completo — fluxo FormatSelector → Preflight → Generate → Results.
**Error handling:** Toast por formato que falhou + banner de erro global + store de erro.
**Duração esperada:** 2–5min (preflight + geração, depende do manuscrito).
**Verdict: PASS**

---

## CU-04 — Preview Live

**Rota:** `/project/[id]/preview` → `+page.svelte`
**IPC commands usados:**
- `render_preview` / `render_preview_page` — renderiza páginas
- `get_page_count` — total de páginas
- `navigate_to_page` / `set_zoom_level` / `toggle_spread_view` — controles de visualização
- `toggle_distraction_free` — modo imersivo
- `add_annotation` / `get_annotations` / `delete_annotation` — anotações
- `detect_orphans_widows` — detecção de problemas tipográficos

**Steps do fluxo:**

| # | Step | Implementado | Detalhe |
|---|------|-------------|---------|
| 1 | Navega para aba Preview | Sim | Tab "preview" no `+layout.svelte` |
| 2 | Layout 3 painéis | Sim | `grid-template-columns: 260px 1fr 280px` |
| 3 | Sidebar com miniaturas/navegação | Sim | `PreviewSidebar` com `onNavigate` |
| 4 | Toolbar de controles | Sim | `PreviewToolbar` (zoom, spread, ruler, annotations) |
| 5 | Visualizador de páginas | Sim | `PageSpreadViewer` com suporte a spread mode |
| 6 | Painel direito de config | Sim | `PreviewRightPanel` |
| 7 | Detecção de orphans/widows | Sim | `detectTypoIssuesFromToolbar()` + highlights visuais |
| 8 | Distraction-free mode | Sim | `DistractionFreeMode` overlay + `:global(body.distraction-free)` CSS |
| 9 | Atalhos de teclado (Cmd+B, Cmd+R) | Sim | `handleKeydown` global |
| 10 | Collapse de painéis | Sim | `sidebarCollapsed` / `rightPanelCollapsed` states |
| 11 | Responsividade (<1280px) | Sim | Media query com auto-collapse |
| 12 | Erro de detecção tipográfica | Parcial | `console.error` — sem toast ao usuário |
| 13 | Estado sem projeto | Sim | `EmptyState` com CTA |

**Happy path:** Completo — layout 3-painéis funcional, navegação, zoom, spread.
**Gap identificado:** `detectTypoIssuesFromToolbar` captura erros apenas em `console.error`, sem toast ao usuário. Falha silenciosa.
**Duração esperada:** <1min para carregar e navegar.
**Verdict: PASS com ressalva** (falha silenciosa no detect-orphans)

---

## CU-05 — Export to BES + CLI

**Rota esperada:** `/project/[id]/integration` — **AUSENTE**
**Rota real:** `/project/[id]/settings` com aba `?tab=integration` (default)

**Componentes BES Integration:**
- `BesStatusPanel.svelte` — valida workspace, exibe status e metadata
- `EditorialProgressBar.svelte` — sincroniza progresso F1-F12 via `sync_editorial_progress`

**CLI standalone:** `src-tauri/src/cli/` — binário `bes-format` compilado com `--features cli`
- Subcomandos: `generate`, `check`, `illustrations`, `status`
- Exige `cargo build --features cli --bin bes-format`
- **Limitação documentada:** Geração CLI retorna `"status": "pending"` com mensagem explícita de que requer Tauri GUI para serviços de renderização (Typst/Ghostscript)

**IPC commands usados:**
- `validate_bes_workspace` — valida estrutura do projeto BES
- `get_bes_metadata` — recupera metadados do `bes-format.yaml`
- `read_bes_docs` — lê documentos BES
- `invalidate_bes_cache` — invalida cache
- `sync_editorial_progress` — sincroniza progresso editorial F1-F12
- `update_editorial_f10` — atualiza F10 após geração

**Steps do fluxo:**

| # | Step | Implementado | Detalhe |
|---|------|-------------|---------|
| 1 | Navega para aba Settings | Sim | Tab "settings" no `+layout.svelte` |
| 2 | Seleciona aba Integration | Sim | `?tab=integration` (padrão) |
| 3 | Valida workspace BES automaticamente | Sim | `$effect` em `BesStatusPanel` ao montar |
| 4 | Exibe status (valid/warning/error) | Sim | Ícones e cores por status |
| 5 | Exibe metadados do livro | Sim | `bes-metadata-display` dl/dt/dd |
| 6 | Recheck manual | Sim | Botão "Recheck" |
| 7 | Sincroniza progresso editorial | Sim | Aba "progress" → `EditorialProgressBar` |
| 8 | Grid de fases F1-F12 | Sim | `epb__grid` com 4 colunas |
| 9 | CLI: generate/check/illustrations/status | Sim | `src-tauri/src/cli/` completo |
| 10 | CLI: geração completa | Parcial | Limitada — requer Tauri GUI para renderização |
| 11 | Rota `/integration` dedicada | Ausente | Funcionalidade em `/settings?tab=integration` |
| 12 | Export direto para pasta BES via UI | Ausente | Não implementado na UI; disponível apenas via CLI |

**Gaps identificados:**
1. **Rota `/project/[id]/integration` ausente** — a funcionalidade existe em `/settings?tab=integration`, mas não há aba "Integration" dedicada no nav bar do projeto. O label no `+layout.svelte` é "Settings", que agrupa integração e progresso.
2. **CLI generate é parcial** — o handler retorna `"status": "pending"` com mensagem explicando que a geração real requer Tauri GUI. Fluxo de geração standalone não funciona de ponta a ponta.
3. **Sem CTA de "Export to BES folder"** na UI — o fluxo de exportação de volta ao workspace BES não está exposto na interface gráfica.

**Duração esperada:** 1–2min para validar + sincronizar.
**Verdict: WARN** — integração BES visível e funcional na Settings; CLI implementado com limitação de geração documentada; sem rota dedicada e sem export UI.

---

## Tabela de IPC Commands Verificados

| Command | Registrado em lib.rs | Chamado pelo frontend | Fluxo |
|---------|---------------------|-----------------------|-------|
| `select_directory` | Sim | Sim (ImportWizard) | CU-01 |
| `read_book_config` | Sim | Sim (ImportWizard) | CU-01 |
| `import_project` | Sim | Sim (ImportWizard, projects.ts) | CU-01 |
| `write_bes_format` | Sim | Sim (ImportWizard) | CU-01 |
| `get_typography_config` | Sim | Sim (typography.ts) | CU-02 |
| `set_typography_config` | Sim | Sim (typography.ts) | CU-02 |
| `list_fonts` | Sim | Sim (typography.ts) | CU-02 |
| `upload_font` | Sim | Sim (typography.ts) | CU-02 |
| `delete_custom_font` | Sim | Sim (typography.ts) | CU-02 |
| `run_preflight` | Sim | Sim (generation.ts) | CU-03 |
| `generate_epub` | Sim | Sim (generation.ts) | CU-03 |
| `generate_pdf_print` | Sim | Sim (generation.ts) | CU-03 |
| `generate_pdf_ebook` | Sim | Sim (generation.ts) | CU-03 |
| `generate_docx` | Sim | Sim (generation.ts) | CU-03 |
| `get_generation_results` | Sim | Sim (generation.ts) | CU-03 |
| `cancel_generation` | Sim | Sim (generation.ts) | CU-03 |
| `render_preview` | Sim | Sim (preview page) | CU-04 |
| `detect_orphans_widows` | Sim | Sim (preview page) | CU-04 |
| `navigate_to_page` | Sim | Sim (preview components) | CU-04 |
| `toggle_distraction_free` | Sim | Sim (DistractionFreeMode) | CU-04 |
| `validate_bes_workspace` | Sim | Sim (bes.ts) | CU-05 |
| `get_bes_metadata` | Sim | Sim (bes.ts) | CU-05 |
| `sync_editorial_progress` | Sim | Sim (bes.ts) | CU-05 |
| `update_editorial_f10` | Sim | Sim (bes.ts) | CU-05 |

---

## Gaps Consolidados

| Prioridade | Gap | Fluxo | Impacto |
|-----------|-----|-------|---------|
| P2 | Erro em `detectTypoIssuesFromToolbar` é silencioso (`console.error` sem toast) | CU-04 | UX — usuário não sabe que detecção falhou |
| P2 | CLI `generate` não executa geração real (requer Tauri GUI) | CU-05 | Funcional — CLI standalone incompleto para geração |
| P3 | Rota `/project/[id]/integration` ausente no nav | CU-05 | Discoverability — feature enterrada em Settings |
| P3 | Sem UI para "Export to BES folder" | CU-05 | Feature ausente na interface gráfica |
| P3 | Editor principal (`/project/[id]`) é placeholder com TODO | Todos | Base do fluxo de edição não implementada |

---

## Verdict por Fluxo

| CU | Verdict | Justificativa |
|----|---------|---------------|
| CU-01 | PASS | Wizard 3-steps completo, todos IPC corretos, error handling em todos os pontos |
| CU-02 | PASS | Load + auto-save debounced + toast, FontUploader/Catalog operacionais |
| CU-03 | PASS | Preflight gate + geração multi-formato + histórico + cancel |
| CU-04 | PASS (ressalva) | Layout 3-painéis completo; detect-orphans tem falha silenciosa |
| CU-05 | WARN | BES status/sync funcional em Settings; CLI generate limitado; rota dedicada ausente |

## Verdict Geral: PASS com ressalvas

4 de 5 fluxos totalmente operacionais. CU-05 requer atenção para entrega completa do requisito de "Export to BES + CLI standalone".
