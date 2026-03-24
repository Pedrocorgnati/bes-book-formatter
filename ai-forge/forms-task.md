# Forms Audit — BES Book Formatter
> Gerado em: 2026-03-22
> Stack: SvelteKit 5 + Tauri (IPC, sem Server Actions)
> Workspace: `output/workspace/bes-book-formatter`

---

## Resumo do Audit

- **Arquivos com inputs:** 14
- **Total de inputs:** ~50 (text, number, range, color, checkbox, radio, select, textarea, file)
- **Biblioteca de validação:** Nenhuma (Svelte 5 `$derived` + toast)
- **Submissão:** IPC calls + store updates (sem `<form>` nativo)
- **Padrão de auto-save:** Debounce configurado (TIMING.DEBOUNCE_CONFIG_SAVE) ✅
- **`aria-invalid` + `aria-describedby`:** Parcial — apenas ImportWizard e uns poucos campos
- **Labels linkadas:** Quase todos os inputs têm `<label for="id">` ✅
- **fieldset/legend:** Apenas TypographyPanel (radio illustration mode) ✅

---

## Tasks

---

### T001 – CoverEditor: tablist/tabpanel sem ARIA completo
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/cover/CoverEditor.svelte`

**Descrição:** O `role="tablist"` tem botões com `aria-selected` mas sem `aria-controls`. O `role="tabpanel"` não tem `id` nem `aria-labelledby`. Leitores de tela não conseguem associar aba ao painel correspondente.

**Critérios de Aceite:**
- Cada `<button role="tab">` tem `id` único (ex: `tab-template`, `tab-text`, `tab-design`) e `aria-controls` apontando para o id do tabpanel
- O `<div role="tabpanel">` tem `id` (ex: `tabpanel-cover`) e `aria-labelledby` apontando para o id da aba ativa
- Tecla Tab move foco às abas; setas `←`/`→` navegam entre abas (aria-pattern de tablist)

**Estimativa:** 1h

**Status:** [x] COMPLETED

---

### T002 – AnnotationLayer: modal sem foco automático e aria-labelledby ausente
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/preview/AnnotationLayer.svelte`

**Descrição:** O diálogo de anotação usa `aria-label` no `role="dialog"` mas tem um `<h4>` de título — deve usar `aria-labelledby`. Ao abrir o modal, o foco não é movido para o dialog (o `tabindex="-1"` existe mas o `focus()` programático não é chamado). Além disso, o botão Save não tem `aria-busy={saving}`.

**Critérios de Aceite:**
- `<h4>` recebe `id="ann-modal-title"`
- `role="dialog"` usa `aria-labelledby="ann-modal-title"` (remover `aria-label` redundante)
- Ao abrir o modal (`showModal = true`), chama `tick()` + `dialogEl.focus()` via `$effect` ou `onMount`
- Botão Save tem `aria-busy={saving}`

**Estimativa:** 1h

**Status:** [x] COMPLETED

---

### T003 – AnnotationLayer: textarea sem counter e falha silenciosa no submit
**Tipo:** SEQUENTIAL
**Dependências:** T002
**Arquivos:**
- modificar: `src/lib/components/preview/AnnotationLayer.svelte`

**Descrição:** A textarea de conteúdo tem `maxlength="1000"` mas não exibe contador de caracteres. Além disso, `saveAnnotation()` tem `if (!newContent.trim() && newType === AnnotationType.COMMENT) return;` sem nenhum feedback ao usuário — a ação simplesmente falha silenciosamente.

**Critérios de Aceite:**
- Adicionar `<p id="ann-content-hint">` com `{newContent.length}/1000` caracteres, ligado via `aria-describedby="ann-content-hint"` na textarea
- Quando o conteúdo estiver vazio e o tipo for COMMENT, exibir `toast.error(t('preview.annotationContentRequired'))` antes de fazer `return`
- Chave i18n `preview.annotationContentRequired` adicionada nos arquivos de tradução

**Estimativa:** 45min

**Status:** [x] COMPLETED

---

### T004 – ImportWizard: genre picker sem label semântica
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/project/ImportWizard.svelte`
- modificar: `src/lib/components/project/GenrePicker.svelte` (se necessário)

**Descrição:** O rótulo do genre picker é um `<p class="wizard-step__field-label">` — não é uma `<label>` HTML nem tem `id` associado ao grupo de botões. O GenrePicker (`role="radiogroup"` ou equivalente) não tem `aria-labelledby` apontando para o rótulo.

**Critérios de Aceite:**
- Substituir `<p>` por `<p id="genre-picker-label">` (ou `<label>` se aplicável)
- GenrePicker aceita prop `aria-labelledby` ou `labelId` e aplica `aria-labelledby` no container do grupo
- Verificar que GenrePicker.svelte expõe `role="radiogroup"` ou `role="group"` no container

**Estimativa:** 45min

**Status:** [ ] SKIPPED — GenrePicker já tem `role="radiogroup" aria-label` correto; label semântico adicional não necessário

---

### T005 – TypographyPanel/PageConfigPanel: validação silenciosa sem feedback
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/typography/TypographyPanel.svelte`
- modificar: `src/lib/components/typography/PageConfigPanel.svelte`

**Descrição:** Vários handlers rejeitam valores fora do range sem qualquer feedback:
- `onFontSizeChange`: `if (isNaN(val) || val < 8 || val > 48) return;` — silencioso
- `onOrphanControlChange` / `onWidowControlChange`: mesma rejeição silenciosa

O usuário digita um valor inválido, o campo não reverte e nenhuma mensagem aparece.

**Critérios de Aceite:**
- Em cada handler de rejeição, adicionar `toast.error(t('...'))` com mensagem contextual (ex: `t('typography.fontSizeRange', { min: 8, max: 48 })`)
- Após rejeição, o input deve reverter ao valor atual do store (`e.currentTarget.value = config.fontSizeBody`)
- PageConfigPanel: mesma lógica para width/height com limites documentados

**Estimativa:** 1h

**Status:** [x] COMPLETED (apenas TypographyPanel — PageConfigPanel já tinha toast)

---

### T006 – CoverEditor: hex color inputs sem ID e label ambígua
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/cover/CoverEditor.svelte`

**Descrição:** Cada cor tem dois inputs: `type="color"` (com `id` e `<label for>`) e `type="text"` (hex, com `aria-label` mas sem `id`). Para leitores de tela, o campo de texto hex parece desconectado. Além disso, o hex text não valida formato antes de aplicar (permite strings inválidas no bind).

**Critérios de Aceite:**
- Hex text input de cor primária recebe `id="input-primary-color-hex"` e `aria-label` atualizado (já existe, confirmar)
- Adicionar validação inline: ao sair do campo (`onblur`), verificar se o valor bate com `/#[0-9A-Fa-f]{6}/.test(val)`; se não, reverter ao valor anterior e mostrar `toast.error(t('cover.editor.invalidHexColor'))`
- Garantir que `type="color"` e `type="text"` estejam dentro de um `<div role="group" aria-labelledby="label-primary-color">` semântico com `<label id="label-primary-color">`

**Estimativa:** 1h

**Status:** [x] COMPLETED

---

## Checklist Final

- [ ] Todos os inputs dentro de `<form>` ou com padrão explícito de submissão documentado
- [ ] Formulários complexos com biblioteca de validação (n/a — padrão IPC aceito para Tauri)
- [ ] Inputs acessíveis (label/for, aria-describedby, aria-invalid): parcial → T001–T004
- [ ] Schemas de validação com feedback visível: parcial → T003, T005
- [ ] Loading states e feedback no botão submit: ✅ (salvo AnnotationLayer → T002)
- [ ] Inputs especiais (file, color): ✅ IllustrationDropzone / CoverEditor
- [ ] Character counters: IllustrationGallery ✅ / AnnotationLayer → T003
- [ ] Acessibilidade (fieldset/legend): TypographyPanel ✅ / outros → T001, T004
- [ ] Focus management em modais: → T002
