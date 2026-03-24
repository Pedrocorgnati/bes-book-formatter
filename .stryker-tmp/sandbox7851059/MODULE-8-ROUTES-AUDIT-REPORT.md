# Routes Audit Report — Module 8

**Projeto:** BES Book Formatter
**Data:** 2026-03-22
**Auditor:** auto-flow execute (module-8 / TASK-1)
**Status:** FAIL

---

## Summary

| Métrica | Valor |
|---------|-------|
| Rotas encontradas (implementadas) | 10 |
| Rotas esperadas pelo spec | 9 |
| Correspondências exatas spec → impl | 4/9 |
| Links quebrados | 0 |
| Rotas órfãs (implementadas mas não linkadas) | 1 (`/project/[id]/settings`) |
| Rota faltando (spec, não implementada) | 1 (`/project/[id]/integration`) |
| Navigation guards (redirect sem projeto) | OK |
| Atalhos de teclado documentados | 7 |

**Verdict resumido:** FAIL — rota `/project/[id]/integration` (BES Integration UI, Rock-5) ausente como página dedicada; funcionalidade parcialmente embutida como aba em `/project/[id]/settings`.

---

## Route Inventory

### Rotas implementadas (src/routes/)

| # | Rota implementada | Arquivo | Descrição |
|---|-------------------|---------|-----------|
| 1 | `/` | `+page.svelte` | Dashboard de projetos |
| 2 | `/import` | `import/+page.svelte` | Wizard de importação |
| 3 | `/project/[id]` | `project/[id]/+page.svelte` | Editor de manuscrito / illustrations |
| 4 | `/project/[id]/typography` | `project/[id]/typography/+page.svelte` | Tipografia |
| 5 | `/project/[id]/illustrations` | `project/[id]/illustrations/+page.svelte` | Galeria de ilustrações |
| 6 | `/project/[id]/output` | `project/[id]/output/+page.svelte` | Geração de output |
| 7 | `/project/[id]/preview` | `project/[id]/preview/+page.svelte` | Preview live |
| 8 | `/project/[id]/cover` | `project/[id]/cover/+page.svelte` | Editor de capa |
| 9 | `/project/[id]/settings` | `project/[id]/settings/+page.svelte` | Settings por projeto (extra spec) |
| 10 | `/settings` | `settings/+page.svelte` | Preferências globais |

Especiais: `+error.svelte`, `+layout.svelte`, `+layout.ts`, `project/[id]/+layout.svelte`

### Rotas esperadas pelo spec vs implementação

| # | Rota spec | Rota impl | Status | Desvio |
|---|-----------|-----------|--------|--------|
| 1 | `/` | `/` | OK | — |
| 2 | `/projects` | `/` (dashboard) | DESVIO | Spec sugere lista separada; impl unificou no dashboard |
| 3 | `/projects/import` | `/import` | DESVIO | Prefixo `projects/` ausente; sem prefixo plural |
| 4 | `/projects/[id]/manuscript` | `/project/[id]` + `/project/[id]/illustrations` | DESVIO | Prefixo singular; seção renomeada de `manuscript` para root+illustrations |
| 5 | `/projects/[id]/typography` | `/project/[id]/typography` | DESVIO | Prefixo singular (`project` vs `projects`) |
| 6 | `/projects/[id]/output` | `/project/[id]/output` | DESVIO | Prefixo singular |
| 7 | `/projects/[id]/preview` | `/project/[id]/preview` | DESVIO | Prefixo singular |
| 8 | `/projects/[id]/cover` | `/project/[id]/cover` | DESVIO | Prefixo singular |
| 9 | `/projects/[id]/integration` | — | **FALTANDO** | Nenhum `+page.svelte` criado; funcionalidade embutida como aba em `/project/[id]/settings` |
| 10 | `/settings` | `/settings` | OK | — |

**Desvios classificados:**
- **Desvio cosmético (baixo risco):** Prefixo `project` vs `projects` — consistência interna mantida; app desktop Tauri não tem deep-link externo dependente deste prefixo.
- **Desvio funcional (médio risco):** `/projects` como página separada unificada no dashboard — cobertura funcional mantida.
- **Desvio de nomenclatura (baixo risco):** `manuscript` → `illustrations`/root — conteúdo coberto.
- **GAP BLOQUEANTE:** `/project/[id]/integration` ausente como rota dedicada — Rock-5 (BES Integration CLI) não tem UI própria navegável.

---

## Link Audit

### Links de entrada por rota

| Rota destino | Origem(s) | Válido |
|---|---|---|
| `/` | `project/[id]/+layout.svelte` (goto ao fechar projeto), `goto('/')` no guard | OK |
| `/import` | `+page.svelte` (`href`, `goto`), `Sidebar.svelte` (`goto`), `AppShell.svelte` (`bes:navigate` Ctrl+I) | OK |
| `/project/[id]` | `+page.svelte` (`href="/project/{project.id}"`), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/typography` | `project/[id]/+layout.svelte` (tab), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/illustrations` | `project/[id]/+layout.svelte` (tab), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/output` | `project/[id]/+layout.svelte` (tab), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/preview` | `project/[id]/+layout.svelte` (tab), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/cover` | `project/[id]/+layout.svelte` (tab), `Sidebar.svelte` (`goto`) | OK |
| `/project/[id]/settings` | `project/[id]/+layout.svelte` (tab) | OK (mas rota extra-spec) |
| `/settings` | `Sidebar.svelte` (`goto`), `AppShell.svelte` (`bes:navigate` Ctrl+,) | OK |

**Links quebrados encontrados:** 0

### Links que referenciam rotas inexistentes

| Origem | Link referenciado | Status |
|--------|-------------------|--------|
| Nenhum | — | OK |

Nenhum link no codebase aponta para `/project/[id]/integration`, `/projects/*` ou qualquer rota não-existente.

---

## Orphan Check

| Rota | Tem `+page.svelte` | Tem link de entrada | Status |
|------|--------------------|---------------------|--------|
| `/` | Sim | Sim (redirect de guards) | OK |
| `/import` | Sim | Sim (3 origens) | OK |
| `/project/[id]` | Sim | Sim (dashboard + sidebar) | OK |
| `/project/[id]/typography` | Sim | Sim (tabs + sidebar) | OK |
| `/project/[id]/illustrations` | Sim | Sim (tabs + sidebar) | OK |
| `/project/[id]/output` | Sim | Sim (tabs + sidebar) | OK |
| `/project/[id]/preview` | Sim | Sim (tabs + sidebar) | OK |
| `/project/[id]/cover` | Sim | Sim (tabs + sidebar) | OK |
| `/project/[id]/settings` | Sim | Sim (tab) | **ORFAO PARCIAL** — linkada apenas pela tab, não no sidebar; não especificada no spec original |
| `/settings` | Sim | Sim (sidebar + Ctrl+,) | OK |
| `/project/[id]/integration` | **Não** | Não | **FALTANDO** |

---

## Navigation Guards

| Cenário | Implementação | Arquivo | Status |
|---------|---------------|---------|--------|
| Acesso a `/project/[id]/*` sem projeto encontrado | `goto('/')` após `ipcGetProject()` retornar null | `project/[id]/+layout.svelte:onMount` | OK |
| Acesso a `/project/[id]/*` sem `projectId` | `goto('/')` se `!projectId` | `project/[id]/+layout.svelte:onMount` | OK |
| Erro no carregamento do projeto (catch) | `// TODO: Implementar backend` — sem redirect | `project/[id]/+layout.svelte:69` | **WARNING** — catch vazio sem fallback |
| Redirecionamento do guard é reativo | `$derived($page.params.id)` + onMount | `project/[id]/+layout.svelte` | OK |

**Nota:** O bloco `catch {}` no onMount de `project/[id]/+layout.svelte` (linha ~69) tem comentário `TODO: Implementar backend` sem redirect de fallback. Em ambiente de produção, um erro de IPC não redirecionaria o usuário, deixando-o em tela indefinida.

---

## Keyboard Shortcuts

### AppShell (`src/lib/components/layout/AppShell.svelte`)

| Atalho | Ação | Contexto |
|--------|------|---------|
| `Ctrl/Cmd + B` | Toggle sidebar esquerda | Global (todas as rotas) |
| `Ctrl/Cmd + ,` | Navegar para `/settings` | Global |
| `Ctrl/Cmd + I` | Navegar para `/import` | Global |

### Preview (`src/routes/project/[id]/preview/+page.svelte`)

| Atalho | Ação | Contexto |
|--------|------|---------|
| `Ctrl/Cmd + B` | Toggle sidebar (local, sobrescreve AppShell) | `/project/[id]/preview` |
| `Ctrl/Cmd + R` | Toggle painel direito | `/project/[id]/preview` |

### DistractionFreeMode (`src/lib/components/preview/DistractionFreeMode.svelte`)

| Atalho | Ação | Contexto |
|--------|------|---------|
| `F11` | Toggle distraction-free mode | `/project/[id]/preview` |
| `Escape` | Sair do distraction-free mode | `/project/[id]/preview` (quando ativo) |

### PageSpreadViewer (`src/lib/components/preview/PageSpreadViewer.svelte`)

| Atalho | Ação | Contexto |
|--------|------|---------|
| `ArrowLeft` | Página anterior | Componente de preview |
| `ArrowRight` | Próxima página | Componente de preview |

**Infraestrutura:** `src/lib/utils/keyboard.ts` — utilitário centralizado `registerShortcuts()` com guard para INPUT/TEXTAREA/SELECT.

---

## Issues Detalhados

### ISSUE-R01 — Rota /project/[id]/integration ausente [BLOQUEANTE]

- **Severidade:** P0 — Bloqueante
- **Spec:** TASK-1 escopo linha 37: `/projects/[id]/integration` — integração BES e CLI (Rock-5)
- **Implementação:** Nenhum arquivo `src/routes/project/[id]/integration/+page.svelte` existe
- **Workaround encontrado:** `src/routes/project/[id]/settings/+page.svelte` contém aba `integration` (type `SettingsTab = 'integration' | 'progress'`) com `data-testid="settings-integration-panel"` — funcionalidade embutida em settings
- **Impacto:** Não é possível navegar diretamente para a UI de integração BES/CLI; a tab dentro de settings não é uma rota SvelteKit independente; deep-links e navegação por atalho para integração são impossíveis

### ISSUE-R02 — Prefixo /project vs /projects [COSMÉTICO]

- **Severidade:** P3 — Informacional
- **Spec:** Usa prefixo plural `/projects/[id]/*`
- **Implementação:** Usa prefixo singular `/project/[id]/*`
- **Impacto:** Sem impacto funcional em app desktop Tauri (sem deep-links externos); consistência interna mantida
- **Decisão recomendada:** Registrar como ADR de decisão de nomenclatura

### ISSUE-R03 — /projects como rota separada ausente [BAIXO]

- **Severidade:** P3 — Informacional
- **Spec:** `/projects` — lista de projetos como página dedicada
- **Implementação:** Dashboard `/` combina lista de projetos + CTA de importação
- **Impacto:** Funcionalidade equivalente; UX simplificada aceitável

### ISSUE-R04 — /projects/[id]/manuscript renomeado [COSMÉTICO]

- **Severidade:** P3 — Informacional
- **Spec:** `/projects/[id]/manuscript` — editor de manuscrito
- **Implementação:** `/project/[id]` (root do projeto) + `/project/[id]/illustrations` separado
- **Impacto:** Funcionalidade distribuída; cobertura mantida

### ISSUE-R05 — catch vazio sem redirect no guard [WARNING]

- **Severidade:** P2 — Aviso
- **Arquivo:** `src/routes/project/[id]/+layout.svelte` linha ~69
- **Problema:** Bloco `catch {}` com `TODO: Implementar backend` sem `goto('/')` de fallback
- **Impacto:** Erro de IPC pode deixar usuário em tela indefinida (sem loading, sem error state)

---

## Verdict

**STATUS: FAIL**

**Justificativa:** A rota `/project/[id]/integration` — dedicada à UI de integração BES/CLI (Rock-5) — está ausente como página SvelteKit independente. A funcionalidade está parcialmente presente como aba dentro de `/project/[id]/settings`, mas isso não satisfaz o critério de aceite "9 rotas SvelteKit inventariadas com handlers" pois integration não é uma rota navegável.

**Issues bloqueantes:**
1. **ISSUE-R01:** `/project/[id]/integration` — rota faltando (P0)

**Issues não-bloqueantes (a corrigir antes de deploy):**
2. **ISSUE-R05:** catch vazio em `project/[id]/+layout.svelte` — estado de erro indefinido (P2)

**Issues informativos (não bloqueam):**
3. **ISSUE-R02:** Prefixo singular vs plural (P3)
4. **ISSUE-R03:** /projects como rota separada (P3)
5. **ISSUE-R04:** manuscript vs illustrations+root (P3)

**Próximos passos:**
- Criar `src/routes/project/[id]/integration/+page.svelte` com a UI de integração BES/CLI
- Adicionar link na `project/[id]/+layout.svelte` (tabs) e no `Sidebar.svelte`
- Adicionar fallback no catch do guard (`goto('/')`)
- Re-executar TASK-1 após correções
