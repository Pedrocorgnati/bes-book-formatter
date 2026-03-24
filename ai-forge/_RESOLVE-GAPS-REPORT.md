# Resolve Gaps - Relatório de Resolução

**Data:** 2026-03-22
**Fonte:** `ai-forge/styling-task.md`
**Duplicados removidos:** 0
**Total de gaps:** 3

---

## Decisões

### Corrigidos (automatico): 3

| # | Gap | Tipo | Sev | Antes | Depois |
|---|-----|------|-----|-------|--------|
| T001 | Tokens errados nas BES components (dark mode quebrado) | CODIGO | CRITICO | `var(--border,#e2e8f0)` / `var(--surface,#fff)` / `var(--text-muted,...)` | `var(--color-border)` / `var(--color-surface)` / `var(--color-text-muted)` |
| G001+G002 | Font-sizes, spacings e durations hardcoded → tokens | CODIGO | MEDIO | 74 font-sizes + 86 spacings + ~15 transitions raw | ~39 font-sizes + ~75 spacings + ~19 transitions substituídos por tokens |
| G003 | app.css sem @layer | CODIGO | BAIXO | Regras misturadas sem camadas | `@layer base { }` + `@layer utilities { }` + keyframes externos |

---

## Detalhes de G001+G002

### Arquivos modificados

| Arquivo | font-size | spacing | transition |
|---------|-----------|---------|------------|
| `src/lib/components/bes/EditorialProgressBar.svelte` | — | — | 2 (já em T001) |
| `src/lib/components/bes/BesStatusPanel.svelte` | — | — | 1 (já em T001) |
| `src/lib/components/cover/CoverPreview.svelte` | 0 | 0 | 1 |
| `src/lib/components/generation/FormatSelector.svelte` | 0 | 0 | 1 |
| `src/lib/components/generation/GenerationProgress.svelte` | 0 | 0 | 1 |
| `src/lib/components/preview/DistractionFreeMode.svelte` | 0 | 0 | 1 |
| `src/lib/components/illustrations/IllustrationCard.svelte` | 2 | 5 | 2 |
| `src/lib/components/illustrations/IllustrationDropzone.svelte` | 5 | 8 | 2 |
| `src/lib/components/illustrations/IllustrationGallery.svelte` | 8 | 10 | 2 |
| `src/lib/components/typography/FontCatalog.svelte` | 4 | 10 | 4 |
| `src/lib/components/typography/FontUploader.svelte` | 2 | 3 | 1 |
| `src/lib/components/typography/GenrePresetPicker.svelte` | 2 | 4 | 1 |
| `src/lib/components/typography/PageConfigPanel.svelte` | 4 | 11 | 1 |
| `src/lib/components/typography/TypographyPanel.svelte` | 7 | 11 | 2 |
| `src/lib/components/typography/TypographyPreview.svelte` | 3 | 5 | 0 |
| `src/routes/project/[id]/illustrations/+page.svelte` | 1 | 3 | 0 |
| `src/routes/project/[id]/typography/+page.svelte` | 1 | 5 | 0 |

**Totais:** ~39 font-sizes · ~75 spacings · ~19 transitions

### Valores sem token (mantidos como estão)

| Valor | Contexto | Motivo |
|-------|---------|--------|
| `0.8125rem` | font-size (botões, labels) | Sem token entre --text-xs e --text-sm |
| `0.6875rem` | font-size (badges) | Sem token entre --text-xs e menor |
| `0.9375rem` | font-size (títulos de panel) | Sem token entre --text-base e --text-lg |
| `0.625rem` | font-size / padding | Sem token |
| `0.375rem` | border-radius | Sem token entre --radius-sm e --radius-md |
| `0.4375rem` | padding | Sem token |
| `0.125rem` | padding | Sem token |
| `1.25rem` | padding / font-size | Sem token (entre --text-xl e --text-2xl para font-size) |
| `0.2s` | transition | Sem token |

---

## Detalhes de G003 (app.css)

- `@layer base { }`: linhas 1–222 (font-faces, :root, dark theme, reset, body, reduced-motion, :focus-visible)
- `@layer utilities { }`: linhas 223–307 (skip-nav, scrollbar, sr-only)
- `@keyframes`: fora de layers (skeleton-pulse, spin, fade-in, fade-out, scale-in)

---

## Verificação de Build

- `svelte-check`: 21 erros TypeScript **pré-existentes** (LayoutIssue, TypoIssue, OutputFormat — unrelated to CSS)
- Nenhum erro novo introduzido pelas mudanças de styling
