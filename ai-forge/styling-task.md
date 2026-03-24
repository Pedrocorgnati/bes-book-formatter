# Styling Task List — BES Book Formatter
**Auditoria:** `/nextjs:styling` adaptado para SvelteKit + Tauri
**Data:** 2026-03-22
**Stack:** SvelteKit 5 + Svelte scoped `<style>` + CSS Custom Properties

---

## Resumo dos Gaps

| # | Severidade | Problema | Arquivos |
|---|-----------|---------|---------|
| T001 | 🔴 CRITICAL | Nomes de tokens errados → dark mode quebrado nos componentes BES | EditorialProgressBar, BesStatusPanel |
| T002 | 🟡 MEDIUM | Font-sizes e spacings hardcoded em vez de tokens (74 + 86 ocorrências) | ~30 componentes |
| T003 | 🟡 MEDIUM | Durations de transição hardcoded em vez de `var(--duration-*)` | ~12 componentes |
| T004 | 🟢 LOW | `app.css` sem `@layer` — regras base/utilities misturadas | app.css |

---

## Tasks

### T001 — Corrigir nomes de tokens errados nos componentes BES
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/bes/EditorialProgressBar.svelte`
- modificar: `src/lib/components/bes/BesStatusPanel.svelte`

**Descrição:**
Estes dois componentes usam nomes de variáveis CSS **inexistentes** no design system:

| Token usado (errado) | Token correto (app.css) | Fallback hardcoded |
|---|---|---|
| `var(--border, #e2e8f0)` | `var(--color-border)` | `#e2e8f0` |
| `var(--surface, #fff)` | `var(--color-surface)` | `#fff` |
| `var(--surface-hover, #f8fafc)` | `var(--color-surface-hover)` | `#f8fafc` |
| `var(--text-muted, #64748b)` | `var(--color-text-muted)` | `#64748b` |

Como as variáveis não existem, o CSS usa os fallbacks — que são cores do modo claro. Isso faz com que esses componentes **não respondam ao dark mode** mesmo com `[data-theme="dark"]` aplicado.

**Critérios de Aceite:**
- [ ] `var(--border, ...)` substituído por `var(--color-border)` em ambos os arquivos
- [ ] `var(--surface, ...)` substituído por `var(--color-surface)`
- [ ] `var(--surface-hover, ...)` substituído por `var(--color-surface-hover)`
- [ ] `var(--text-muted, ...)` substituído por `var(--color-text-muted)`
- [ ] Verificar visualmente que dark mode aplica corretamente nesses componentes

**Estimativa:** 0.5h
**Status:** [x] COMPLETED

---

### T002 — Substituir font-sizes e spacings hardcoded por tokens
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/bes/EditorialProgressBar.svelte`
- modificar: `src/lib/components/bes/BesStatusPanel.svelte`
- modificar: `src/lib/components/illustrations/IllustrationDropzone.svelte`
- modificar: `src/lib/components/illustrations/IllustrationCard.svelte`
- modificar: (demais componentes com hardcodes detectados)

**Descrição:**
O `app.css` define uma escala tipográfica (`--text-xs`, `--text-sm`, `--text-base`, etc.) e escala de espaçamento (`--space-1` a `--space-16`), mas os blocos `<style>` dos componentes usam valores raw:

Exemplos:
- `font-size: 0.875rem` → `var(--text-sm)`
- `font-size: 0.75rem` → `var(--text-xs)`
- `font-size: 0.8125rem` → sem token equivalente (entre xs e sm) — criar `--text-xs-plus: 0.8125rem` ou arredondar para `var(--text-sm)`
- `font-size: 0.6875rem` → usar `var(--text-xs)` (0.75rem) ou criar token específico
- `gap: 0.5rem` → `var(--space-2)`
- `padding: 1rem` → `var(--space-4)`

> **Nota:** Valores não cobertos pela escala (0.8125rem, 0.9375rem, 0.625rem) são usados para micro-ajustes visuais em componentes BES. Pode-se criar tokens adicionais em `app.css` ou aceitar os valores raw para esses casos específicos.

**Critérios de Aceite:**
- [ ] `font-size: 0.875rem` → `var(--text-sm)` (todos os arquivos)
- [ ] `font-size: 0.75rem` → `var(--text-xs)` (todos os arquivos)
- [ ] `gap: 0.5rem` → `var(--space-2)`, `padding: 1rem` → `var(--space-4)`, etc. nos arquivos BES
- [ ] Decisão documentada sobre valores fora da escala (0.8125rem, 0.6875rem)

**Estimativa:** 2h
**Status:** [x] COMPLETED

---

### T003 — Substituir durations hardcoded por tokens
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/bes/EditorialProgressBar.svelte`
- modificar: `src/lib/components/bes/BesStatusPanel.svelte`
- modificar: `src/lib/components/illustrations/IllustrationDropzone.svelte`
- modificar: `src/lib/components/illustrations/IllustrationCard.svelte`
- modificar: `src/lib/components/illustrations/IllustrationGallery.svelte`
- modificar: `src/lib/components/cover/CoverPreview.svelte`
- modificar: `src/lib/components/generation/FormatSelector.svelte`
- modificar: `src/lib/components/generation/GenerationProgress.svelte`
- modificar: `src/lib/components/preview/DistractionFreeMode.svelte`

**Descrição:**
O design system define tokens de duration (`--duration-fast: 150ms`, `--duration-normal: 300ms`, `--duration-slow: 500ms`), mas a maioria dos componentes usa valores raw:

| Valor raw | Token correto |
|---|---|
| `0.15s` / `150ms` | `var(--duration-fast)` |
| `0.1s` / `100ms` | `var(--duration-fast)` (mais próximo) |
| `0.3s` / `300ms` | `var(--duration-normal)` |
| `0.4s` / `400ms` | `var(--duration-normal)` (mais próximo) |
| `0.5s` / `500ms` | `var(--duration-slow)` |

**Critérios de Aceite:**
- [ ] `transition: background 0.15s` → `transition: background var(--duration-fast)` (8 arquivos)
- [ ] `transition: transform 0.4s ease` → `var(--duration-normal)` em CoverPreview
- [ ] `transition: width 0.3s ease` → `var(--duration-normal)` em GenerationProgress
- [ ] Todos os `transition` com valores raw substituídos

**Estimativa:** 1h
**Status:** [x] COMPLETED

---

### T004 — Organizar app.css com @layer
**Tipo:** SEQUENTIAL
**Dependências:** T001, T002, T003
**Arquivos:**
- modificar: `src/app.css`

**Descrição:**
O `app.css` (309 linhas) mistura `@font-face`, variáveis, reset, utilitários e keyframes sem separação por camadas. Adicionar `@layer` melhora especificidade previsível e facilita overrides futuros:

```css
@layer base {
  /* @font-face, :root, [data-theme="dark"], *, html/body */
}
@layer utilities {
  /* .sr-only, .skip-nav, ::-webkit-scrollbar */
}
/* @keyframes fora de layers (sem suporte em @layer ainda) */
```

**Critérios de Aceite:**
- [ ] `@layer base` engloba reset, variáveis e tipografia
- [ ] `@layer utilities` engloba `.sr-only`, `.skip-nav` e scrollbar
- [ ] Build sem erros após reorganização
- [ ] Sem regressão visual (verificar dark mode, focus styles)

**Estimativa:** 0.5h
**Status:** [x] COMPLETED
