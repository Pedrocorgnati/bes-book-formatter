# Accessibility Task List — BES Book Formatter (pós-MODULE-8)

**Data:** 2026-03-22
**Auditoria base:** MODULE-8-A11Y-AUDIT.md
**Status B01 + R01:** CORRIGIDOS na release anterior

---

### T001 — Remover aria-live aninhado: Toast/ToastContainer

**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/ui/Toast.svelte`
- modificar: `src/lib/components/ui/ToastContainer.svelte`

**Descrição:**
`ToastContainer` já declara `aria-live="polite"` + `role="region"` no container.
`Toast` individual também declara `aria-live` (polite ou assertive por tipo).
Live regions aninhadas causam comportamento não-determinístico em screen readers — alguns anunciam o container, outros o item. A solução correta é:
1. Manter `aria-live` APENAS no ToastContainer
2. Remover `aria-live` e `role` dos itens Toast individuais
3. Para suporte a mensagens assertivas (erros), adicionar um segundo container `aria-live="assertive"` para toasts de erro/warning

**WCAG:** 4.1.3 (Status Messages)

**Critérios de Aceite:**
- [ ] Toast.svelte sem `aria-live` individual
- [ ] ToastContainer com dois containers: polite (info/success) e assertive (error/warning)
- [ ] Screen reader anuncia corretamente em ambos os tipos

**Estimativa:** 20min

---

### T002 — Sidebar: disabled items inacessíveis por Tab quando desabilitados

**Tipo:** SEQUENTIAL
**Dependências:** T001
**Arquivos:**
- modificar: `src/lib/components/layout/Sidebar.svelte`

**Descrição:**
6 botões de navegação usam `aria-disabled={!currentProject}` + CSS `pointer-events: none`.
O CSS bloqueia mouse mas o Tab ainda alcança esses botões quando desabilitados.
A correção é adicionar `disabled={!currentProject || undefined}` nos botões que dependem de projeto ativo.
Para o botão Marketplace (sempre desabilitado), adicionar `disabled`.

**WCAG:** 2.1.1 (Keyboard)

**Critérios de Aceite:**
- [ ] Tab não foca botões desabilitados de navegação do projeto
- [ ] Botões habilitados continuam acessíveis por teclado
- [ ] `aria-disabled` mantido para screen reader announcements

**Estimativa:** 15min

---

### T003 — lang dinâmico em app.html / layout

**Tipo:** SEQUENTIAL
**Dependências:** T002
**Arquivos:**
- modificar: `src/routes/+layout.svelte`

**Descrição:**
`app.html` tem `lang="pt-BR"` hardcoded. O app suporta 3 locales (pt-BR, en-US, es-ES).
Ao mudar o locale via LanguageSelector, o `<html lang>` não é atualizado.
Correção: subscrever ao store `locale` no layout e atualizar `document.documentElement.lang`
quando o locale mudar.

**WCAG:** 3.1.1 (Language of Page)

**Critérios de Aceite:**
- [ ] `document.documentElement.lang` reflete locale ativo
- [ ] Muda imediatamente ao trocar idioma
- [ ] Valor padrão é `pt-BR` (mantido do app.html como fallback)

**Estimativa:** 10min

---

### T004 — Modal: substituir setTimeout por requestAnimationFrame

**Tipo:** PARALLEL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/ui/Modal.svelte`

**Descrição:**
`$effect` usa `setTimeout(..., 50ms)` para focar no primeiro elemento após modal abrir.
Em testes automatizados ou ambientes lentos isso pode causar race conditions.
Substituir por `requestAnimationFrame` para garantir que o foco ocorre após o browser
ter calculado o layout do modal.

**WCAG:** 2.4.3 (Focus Order) — qualidade

**Critérios de Aceite:**
- [ ] `requestAnimationFrame` substituindo `setTimeout`
- [ ] Foco ainda ocorre no primeiro elemento focável ao abrir o modal
- [ ] ESC fecha, focus trap funciona

**Estimativa:** 5min
