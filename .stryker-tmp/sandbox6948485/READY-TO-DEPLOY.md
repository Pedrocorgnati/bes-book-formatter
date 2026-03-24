# READY-TO-DEPLOY — BES Book Formatter

**Data:** 2026-03-22
**Auditor:** auto-flow execute / module-8-cross-rock-integration
**Versão auditada:** HEAD (workspace pós módulos 1-7)
**Baseado em:** MODULE-8-INTEGRATION-AUDIT.md + 11 relatórios de auditoria

---

## ✗ VEREDITO: NO-GO

> **O app NÃO está pronto para deploy.** Existem 3 bloqueadores P0/P1 que devem ser corrigidos antes de avançar para F9 (QA) ou deploy de qualquer tipo.

---

## Resumo Executivo

| Área | Resultado | Bloqueadores |
|------|-----------|-------------|
| Rotas e Navegação (TASK-1) | **FAIL** | P0: rota `/integration` ausente |
| IPC Commands (TASK-2) | PASS COM RESSALVAS | P2: `get_cover_config` import faltando |
| Shared Types (TASK-3) | PASS COM RESSALVAS | P3: Genre.Poetry divergência TS↔Rust |
| SQLite / Migrations (TASK-4) | PASS ✓ | — |
| UI Handlers (TASK-5) | PASS COM RESSALVAS | P2: 4 silent errors sem toast |
| UI States (TASK-6) | PASS COM RESSALVAS | P1: dashboard sem loading state, PreviewSidebar broken |
| E2E Flows (TASK-7) | PASS COM RESSALVAS | P1: CU-05 sem rota dedicada |
| Acessibilidade WCAG AA (TASK-8) | PASS COM RESSALVAS | P1: contraste toast warning (~3.0:1) |
| Performance / Build (TASK-9) | PASS COM RESSALVAS | P2: Typst sem cache PNG |
| Contratos Cross-Rock (TASK-10) | PASS COM RESSALVAS | P3: naming divergências C6, C10 |
| ECU (TASK-11) | PASS COM RESSALVAS | P3: 2 silent errors menores |

---

## Bloqueadores P0/P1 (impedem deploy)

### B01 — Rota `/project/[id]/integration` ausente [P0]
**Audit:** MODULE-8-ROUTES-AUDIT-REPORT.md — ISSUE-R01
**Impacto:** Rock-5 (BES Integration + CLI) não tem rota SvelteKit navegável. CU-05 está enterrado em `/project/[id]/settings?tab=integration`. Violação do critério DoD: "Zero rotas órfãs".
**Correção:** Criar `src/routes/project/[id]/integration/+page.svelte` com a UI de integração BES/CLI. Adicionar link nas tabs de `project/[id]/+layout.svelte` e no `Sidebar.svelte`.
**Estimativa:** ~3-4h

### B02 — Dashboard sem loading state [P1]
**Audit:** MODULE-8-STATES-AUDIT.md — Gap S1
**Impacto:** Na carga inicial, o dashboard exibe tela em branco antes dos projetos aparecerem. Violação: "Zero Estados Indefinidos".
**Correção:** Adicionar skeleton/spinner no `src/routes/+page.svelte` enquanto `$projectsStore.loading === true`.
**Estimativa:** ~1h

### B03 — `chaptersLoading` em PreviewSidebar não renderizado [P1]
**Audit:** MODULE-8-STATES-AUDIT.md — Gap S2/S3
**Impacto:** Sidebar de capítulos mostra conteúdo indistinguível entre "carregando", "vazio" e "erro". Violação: "Zero Estados Indefinidos".
**Correção:** Renderizar o estado `chaptersLoading` no template de `src/lib/components/preview/PreviewSidebar.svelte`. Adicionar empty/error states distintos.
**Estimativa:** ~1.5h

### B04 — Contraste toast warning abaixo de AA [P1]
**Audit:** MODULE-8-A11Y-AUDIT.md — B01
**Impacto:** `#FFFFFF` sobre `#D97706` = ~3.0:1 (mínimo WCAG AA: 4.5:1 para texto 14px). Violação WCAG 1.4.3.
**Correção:** Ajustar `--color-warning` para cor com contraste ≥ 4.5:1 (ex: `#92400E` sobre `#FEF3C7`, ratio ~6.1:1).
**Estimativa:** ~0.5h

---

## Issues P2/P3 (recomendadas antes do QA)

| ID | Descrição | Audit | Estimativa |
|----|-----------|-------|------------|
| P2-01 | `get_cover_config` registrado mas não importado em lib.rs | IPC-AUDIT | 0.5h |
| P2-02 | `detectTypoIssuesFromToolbar` captura erro só em console.error | HANDLERS-AUDIT | 0.5h |
| P2-03 | `FontUploader.svelte` — `select_font_file` sem try/catch | HANDLERS-AUDIT | 0.5h |
| P2-04 | `PreviewSidebar` load errors sem toast visual | HANDLERS-AUDIT | 0.5h |
| P2-05 | Typst preview recompila a cada página sem cache de PNG | PERFORMANCE-AUDIT | 4-6h |
| P2-06 | M004-M007 não inserem em schema_version | SQLITE-AUDIT | 0.5h |
| P3-01 | Genre.Poetry em Rust sem equivalente TS | TYPES-AUDIT | 0.5h |
| P3-02 | C6 naming: TypographyDefaults vs TypographyConfig | CONTRACTS-AUDIT | 1h |
| P3-03 | C10 naming: currentProject store → projectsStore.current | CONTRACTS-AUDIT | 0.5h |
| P3-04 | `id="main-content"` duplicado (WCAG) | A11Y-AUDIT | 0.5h |
| P3-05 | `catch {}` vazio no guard de project/[id]/+layout.svelte | ROUTES-AUDIT | 0.5h |

---

## Definition of Done — Checklist Final

| Critério | Status |
|---------|--------|
| Zero rotas órfãs (9 rotas com handlers) | ✗ FAIL — /integration ausente |
| Zero IPC commands sem implementação (39 testados) | ⚠️ PASS COM RESSALVAS — 59 registrados, get_cover_config import |
| Zero componentes com estados indefinidos | ✗ FAIL — dashboard + PreviewSidebar |
| 11 contratos cross-rock válidos e sem divergências | ⚠️ PASS COM RESSALVAS — C6/C10 naming |
| 9 tabelas SQLite com schemas e migrations corretos | ✅ PASS |
| WCAG AA: 0 erros críticos (Axe-core) | ✗ FAIL — contraste warning |
| Preview Typst live < 500ms (P95) | ⚠️ PROVÁVEL FAIL — sem cache PNG |
| App boot < 3s | ✅ PASS (provável) |
| 5 fluxos E2E completos | ⚠️ PASS COM RESSALVAS — CU-05 sem rota dedicada |
| App lançável (ECU) | ✅ PASS COM RESSALVAS |

**Score: 3/10 PASS, 4/10 PASS COM RESSALVAS, 3/10 FAIL**

---

## Plano de Remediação

Para atingir GO, corrigir nesta ordem:

1. **B01** — Criar rota `/integration` (~3-4h) → re-auditar TASK-1 e TASK-7
2. **B02** — Dashboard loading state (~1h) → re-auditar TASK-6
3. **B03** — PreviewSidebar states (~1.5h) → re-auditar TASK-6
4. **B04** — Toast warning contrast (~0.5h) → re-auditar TASK-8
5. **P2-01** — get_cover_config import (~0.5h) → re-auditar TASK-2
6. P2-02 a P2-06 — Consolidar em batch (~7h)

**Estimativa total para GO:** ~13-15h de desenvolvimento

Ver `MODULE-8-REMEDIATION-TASKS.md` para tasks detalhadas.

---

## Próximos Passos

```
1. Executar MODULE-8-REMEDIATION-TASKS.md (bloqueadores P0/P1 primeiro)
2. Re-auditar TASK-1 (rotas), TASK-6 (estados), TASK-8 (a11y)
3. Re-executar TASK-12 com novo veredito
4. Se GO: prosseguir para F9 (/qa:prep → /qa:trace → /qa:report)
```

---

*Gerado por auto-flow execute — module-8-cross-rock-integration — 2026-03-22*
