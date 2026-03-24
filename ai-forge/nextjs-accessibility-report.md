# Accessibility Report — BES Book Formatter (SvelteKit + Tauri)
**Data:** 2026-03-22
**Escopo:** Código-fonte completo — 58 componentes Svelte + rotas SvelteKit

---

## Issues Pré-existentes (MODULE-8) — Status

| ID | Componente | Descrição | Status |
|----|-----------|-----------|--------|
| B01 | `Toast.svelte` | Contraste `#FFFFFF/#D97706` (WCAG 1.4.3) | **JÁ CORRIGIDO** — `color: #422006` (~6.8:1) |
| R01 | `+layout.svelte` + `AppShell` | `id="main-content"` duplicado | **JÁ CORRIGIDO** — removido do div wrapper |

## Issues Corrigidos Nesta Sessão

### T001 — aria-live aninhado (WCAG 4.1.3)
**Antes:** `ToastContainer` com `aria-live="polite"` + cada `Toast` com `aria-live` próprio
**Depois:** Dois containers separados (polite para info/success, assertive para error/warning). `Toast` individual sem `aria-live`.
**Arquivos:** `ToastContainer.svelte`, `Toast.svelte`

### T002 — Sidebar disabled keyboard (WCAG 2.1.1)
**Antes:** 6 botões de projeto + 1 botão Marketplace com `aria-disabled` mas sem `disabled` nativo — Tab os alcançava
**Depois:** `disabled={!currentProject || undefined}` em todos os 6 botões de projeto; `disabled` no Marketplace. CSS expandido para cobrir `:disabled` nativo.
**Arquivo:** `Sidebar.svelte`

### T003 — lang dinâmico (WCAG 3.1.1)
**Antes:** `<html lang="pt-BR">` hardcoded em `app.html`
**Depois:** `$effect(() => { document.documentElement.lang = $locale; })` em `+layout.svelte` — atualiza automaticamente ao trocar idioma
**Arquivo:** `+layout.svelte`

### T004 — Modal focus (qualidade)
**Antes:** `setTimeout(() => focusable?.focus(), 50)` — frágil em testes automatizados
**Depois:** `requestAnimationFrame(() => focusable?.focus())` — sincronizado com ciclo de layout do browser
**Arquivo:** `Modal.svelte`

---

## Conformidade WCAG 2.1 AA — Status Final

| Critério | Status | Notas |
|---------|--------|-------|
| 1.1.1 Non-text Content | ✅ PASS | SVGs decorativos com aria-hidden; logos com aria-label |
| 1.3.1 Info and Relationships | ✅ PASS | header, nav, main, aside, section com semântica correta |
| 1.3.2 Meaningful Sequence | ✅ PASS | Ordem DOM reflete ordem visual |
| 1.4.1 Use of Color | ✅ PASS | Cores não são único meio de informação |
| 1.4.3 Contrast (Minimum) | ✅ PASS | Toast warning corrigido para #422006 (~6.8:1) |
| 1.4.4 Resize Text | ✅ PASS | Tokens em rem/var, sem px fixo em textos |
| 1.4.10 Reflow | ✅ PASS | Layout grid responsivo |
| 1.4.11 Non-text Contrast | ✅ PASS | Bordas e ícones com contraste adequado |
| 2.1.1 Keyboard | ✅ PASS | Sidebar disabled corrigido; todos interativos acessíveis |
| 2.1.2 No Keyboard Trap | ✅ PASS | Focus trap correto (Tab+Shift+Tab) em Modal |
| 2.3.3 Animation from Interactions | ✅ PASS | `prefers-reduced-motion` global em app.css:206-209 |
| 2.4.1 Bypass Blocks | ✅ PASS | Skip-nav implementado e funcional |
| 2.4.2 Page Titled | ✅ PASS | `<svelte:head><title>` em todas as rotas |
| 2.4.3 Focus Order | ✅ PASS | Ordem lógica; ConfirmDialog foca em Cancelar |
| 2.4.6 Headings and Labels | ✅ PASS | Labels descritivos via i18n |
| 2.4.7 Focus Visible | ✅ PASS | `:focus-visible` global 2px solid var(--color-primary) |
| 3.1.1 Language of Page | ✅ PASS | `document.documentElement.lang` dinâmico via locale store |
| 3.3.1 Error Identification | ✅ PASS | Toast error com role="alert" |
| 4.1.2 Name, Role, Value | ✅ PASS | role, aria-label, aria-expanded, aria-checked, aria-modal |
| 4.1.3 Status Messages | ✅ PASS | Containers separados para polite/assertive |

---

## Arquivos Modificados

- `src/lib/components/ui/Toast.svelte` — removido aria-live individual
- `src/lib/components/ui/ToastContainer.svelte` — dois containers polite/assertive
- `src/lib/components/layout/Sidebar.svelte` — disabled nativo nos 7 botões desabilitados
- `src/routes/+layout.svelte` — lang dinâmico via $locale
- `src/lib/components/ui/Modal.svelte` — requestAnimationFrame no focus
