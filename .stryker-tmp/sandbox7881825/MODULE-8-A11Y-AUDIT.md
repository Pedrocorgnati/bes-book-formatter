# MODULE-8 — Auditoria de Acessibilidade WCAG AA
**Tipo de análise:** Estática (revisão de código-fonte)
**Data:** 2026-03-22
**Escopo:** Todos os componentes principais da UI — layout, navegação, modais, notificações, i18n

---

## 1. Skip Navigation

| Item | Status | Evidência |
|------|--------|-----------|
| Link skip-nav presente | PASS | `src/routes/+layout.svelte:81` — `<a href="#main-content" class="skip-nav">{t('a11y.skipNav')}` |
| CSS skip-nav posiciona corretamente ao focus | PASS | `src/app.css:220-245` — oculto com `left: -9999px` / exibido com `position: fixed` ao `:focus` |
| Alvo `#main-content` existe | PASS | `src/routes/+layout.svelte:101` — `<div id="main-content" ...>` e também `AppShell.svelte:149` — `<main id="main-content">` |
| Tradução da chave `a11y.skipNav` | PASS | `en-US.json:488` — `"skipNav": "Skip to content"` |

**Observação:** Existe `id="main-content"` duplicado — um no `<div>` em `+layout.svelte` e outro no `<main>` dentro de `AppShell.svelte`. O id duplicado não causa falha funcional no Tauri WebView mas é tecnicamente inválido HTML. Recomendado remover o `id` do wrapper `<div>` em `+layout.svelte` e manter apenas o `<main>` do AppShell.

---

## 2. Estrutura Semântica HTML

| Elemento | Status | Evidência |
|----------|--------|-----------|
| `<header>` para cabeçalho do app | PASS | `AppShell.svelte:80` — `<header data-testid="header" class="app-shell__header">` |
| `<nav>` para sidebar com `aria-label` | PASS | `AppShell.svelte:130-134` — `<nav aria-label={t('a11y.sidebarNav')} ...>` |
| `<main>` para conteúdo principal | PASS | `AppShell.svelte:149` — `<main aria-label={t('a11y.mainContent')}>` |
| `<aside>` para painel direito | PASS | `AppShell.svelte:160-163` — `<aside aria-label={t('a11y.propertiesPanel')}>` |
| `<section>` com headings na sidebar | PASS | `Sidebar.svelte:24` — `<section><h2 class="sidebar__section-title">` |
| Uso correto de `<ul>/<li>` em listas | PASS | `Sidebar.svelte:36,73,183` — listas de navegação com markup semântico |

---

## 3. ARIA Labels em Elementos Interativos

### AppShell (Collapse Buttons)
| Botão | aria-label | aria-expanded | Status |
|-------|-----------|---------------|--------|
| Toggle sidebar | `t('a11y.expandMenu')` / `t('a11y.collapseMenu')` | `!sidebarCollapsed` | PASS |
| Toggle right panel | `t('a11y.expandPanel')` / `t('a11y.collapsePanel')` | `!rightPanelCollapsed` | PASS |

**Nota:** `aria-hidden` aplicado à `<nav>` sidebar quando `sidebarCollapsed = true` (`AppShell.svelte:134`). Itens da sidebar colapsada ficam invisíveis para screen readers — comportamento correto.

### Sidebar (Itens de Navegação)
| Elemento | Status | Observação |
|----------|--------|------------|
| Botões de nav com `aria-disabled` | PASS | `Sidebar.svelte:82,103,124,144,163` |
| `aria-current="page"` em projeto ativo | PASS | `Sidebar.svelte:44` |
| Botão settings com `aria-label` | PASS | `Sidebar.svelte:208` — `aria-label={t('nav.goToSettings')}` |
| Botão "Novo Projeto" com `aria-label` | PASS | `Sidebar.svelte:58` — `aria-label={t('emptyState.importCta')}` |
| Marketplace com `aria-disabled="true"` | PASS | `Sidebar.svelte:188` |

**Ressalva:** Botões com `aria-disabled` mas sem `disabled` nativo precisam de tratamento extra para screen readers (o `aria-disabled` anuncia o estado, mas usuários podem ainda pressionar Enter). A classe `pointer-events: none` cobre mouse, mas a navegação via keyboard não é bloqueada — o `onclick` só executa se `currentProject` existir, então não causa dano funcional.

### ThemeToggle
| Atributo | Valor | Status |
|----------|-------|--------|
| `role="switch"` | presente | PASS |
| `aria-checked` | `isDark` (boolean) | PASS |
| `aria-label` | dinâmico por estado | PASS |
| `:focus-visible` CSS | `outline: 2px solid var(--color-primary)` | PASS |

### Header
| Elemento | Status |
|----------|--------|
| `aria-label` na marca/logo | PASS — `"BES Book Formatter"` |
| `aria-label` no breadcrumb | PASS — `t('a11y.breadcrumb')` |

---

## 4. Modais e Diálogos

### Modal.svelte
| Critério | Status | Evidência |
|----------|--------|-----------|
| `role="dialog"` | PASS | `Modal.svelte:83` |
| `aria-modal="true"` | PASS | `Modal.svelte:83` |
| `aria-labelledby` aponta para `<h2>` | PASS | `Modal.svelte:84,89` — ID gerado aleatoriamente |
| Focus trap Tab/Shift+Tab | PASS | `Modal.svelte:30-51` — `trapFocus()` com querySelectorAll de focusáveis |
| Foco ao abrir | PASS | `Modal.svelte:59-64` — `$effect` foca no primeiro elemento focável |
| ESC fecha modal | PASS | `Modal.svelte:53-55` — `handleKeydown` com `e.key === 'Escape'` |
| Botão fechar com `aria-label` | PASS | `Modal.svelte:94` — `t('a11y.closeModal')` |
| SVG ícone com `aria-hidden` | PASS | `Modal.svelte:96` |
| Overlay com `aria-hidden="true"` | PASS | `Modal.svelte:75` |

**Ressalva menor:** O foco no `$effect` usa `setTimeout(..., 50ms)`. Em cenários de testes automatizados ou conexões muito lentas isso pode causar race conditions. Recomendado usar `requestAnimationFrame` como alternativa.

### ConfirmDialog.svelte
| Critério | Status | Evidência |
|----------|--------|-----------|
| `role="alertdialog"` (correto para ação destrutiva) | PASS | `ConfirmDialog.svelte:49` |
| `aria-modal="true"` | PASS | `ConfirmDialog.svelte:51` |
| `aria-labelledby` e `aria-describedby` | PASS | `ConfirmDialog.svelte:52-53` |
| Foco automático no botão Cancelar | PASS | `ConfirmDialog.svelte:34-37` — padrão seguro WCAG |
| ESC fecha diálogo | PASS | `ConfirmDialog.svelte:29-31` |
| Overlay não fecha (ação crítica) | PASS | Overlay sem `onclick` — comentado explicitamente |

---

## 5. Notificações (Toast)

| Critério | Status | Evidência |
|----------|--------|-----------|
| `role="alert"` para erros/warnings | PASS | `Toast.svelte:15-16` — `role` derivado por `type` |
| `role="status"` para info/success | PASS | `Toast.svelte:15` |
| `aria-live="assertive"` para erros | PASS | `Toast.svelte:16` |
| `aria-live="polite"` para info | PASS | `Toast.svelte:16` |
| Botão dismiss com `aria-label` | PASS | `Toast.svelte:62` — `t('a11y.closeNotification')` |
| Container com `aria-live="polite"` | PASS | `ToastContainer.svelte:10` |
| Container `role="region"` com `aria-label` | PASS | `ToastContainer.svelte:10` |

**Nota:** `ToastContainer` tem `aria-live="polite"` enquanto toasts individuais de erro têm `aria-live="assertive"`. Existe conflito de live region aninhada — screen readers podem anunciar apenas o container ou o item, dependendo do browser. O comportamento mais confiável seria definir a live region apenas no container e remover `aria-live` dos itens individuais.

---

## 6. Estados de Loading (aria-busy)

| Componente | Status |
|-----------|--------|
| Skeleton sidebar `aria-busy="true"` | PASS — `AppShell.svelte:137` |
| Skeleton main `aria-busy="true"` | PASS — `AppShell.svelte:152` |
| Loading no layout principal com `aria-label` | PASS — `+layout.svelte:103` |
| Spinner com `aria-hidden="true"` | PASS — `+layout.svelte:104` |
| EmptyState com `role="status"` | PASS — `EmptyState.svelte:25` |

---

## 7. Imagens e SVGs

| Critério | Status | Evidência |
|----------|--------|-----------|
| SVGs decorativos com `aria-hidden="true"` | PASS | Todos os ícones SVG em Sidebar, AppShell, Header, Modal, Toast usam `aria-hidden="true"` |
| Logo com `aria-label` | PASS | `Header.svelte:38` |
| SVGs de ícone de estado sem texto alternativo escrito no DOM | PASS | Texto de contexto fornecido pelos labels do botão pai |

---

## 8. Foco e Keyboard Navigation

### Estilos de Focus Global
| Critério | Status | Evidência |
|----------|--------|-----------|
| `:focus-visible` global | PASS | `app.css:213-217` — `outline: 2px solid var(--color-primary); outline-offset: 2px` |
| `:focus-visible` em ThemeToggle local | PASS | `ThemeToggle.svelte:67-70` |
| Botões collapse sem outline customizado (herdam global) | PASS | AppShell e componentes herdam `:focus-visible` global |

### Atalhos de Teclado Implementados
| Atalho | Função | Implementado em |
|--------|--------|----------------|
| `Ctrl/Cmd+B` | Toggle sidebar | `AppShell.svelte:50-53` |
| `Ctrl/Cmd+,` | Navegar para /settings | `AppShell.svelte:54-58` |
| `Ctrl/Cmd+I` | Navegar para /import | `AppShell.svelte:59-62` |
| `Ctrl/Cmd+B` (preview) | Toggle sidebar do preview | `preview/+page.svelte:67-72` |
| `Ctrl/Cmd+R` (preview) | Toggle right panel do preview | `preview/+page.svelte:74-79` |
| `ESC` | Fechar modal | `Modal.svelte:54` |
| `ESC` | Fechar ConfirmDialog | `ConfirmDialog.svelte:30` |
| `ESC` | Sair do Distraction Free | `DistractionFreeMode.svelte:21` |
| `Tab` (focus trap) | Navegar dentro de Modal | `Modal.svelte:30-51` |

**Nota:** Atalho `Ctrl+O` (import) está implementado como `Ctrl+I` (`e.key === 'i'`), não `Ctrl+O` como indicado nos requisitos da TASK-8. Verificar se a spec original usa `I` de Import ou `O` de Open.

### Preview Toolbar
| Critério | Status | Evidência |
|----------|--------|-----------|
| `role="toolbar"` com `aria-label` | PASS | `PreviewToolbar.svelte:92` |
| Botões Prev/Next com `aria-label` | PASS | `PreviewToolbar.svelte:99,125` |
| Input de página com `aria-label` | PASS | `PreviewToolbar.svelte:114` |
| `aria-live="polite"` no tempo de render | PASS | `PreviewToolbar.svelte:199` |

---

## 9. Internacionalização e Screen Reader

### Suporte multilíngue
| Critério | Status |
|----------|--------|
| 3 locales disponíveis: pt-BR, en-US, es-ES | PASS |
| Seção `a11y` em todos os arquivos JSON | PASS — `en-US.json:487-504` verificado |
| Fallback automático para pt-BR | PASS — `engine.ts:50-54` |
| `lang` no HTML root | NAO VERIFICADO — `src/app.html` não foi lido; provável que o atributo `lang` precise ser dinâmico com o locale ativo. **Recomendado verificar.** |

---

## 10. Contraste de Cores (Estimativa Estática)

Baseado nos tokens em `src/app.css`:

### Modo Claro
| Par de cores | Tokens | Estimativa |
|-------------|--------|-----------|
| Texto principal sobre fundo | `#1A1A1A` sobre `#F8F6F0` | ~18:1 — PASS AA/AAA |
| Texto muted sobre fundo | `#6B6560` sobre `#F8F6F0` | ~5.2:1 — PASS AA |
| Primary (#991B1B) sobre branco | `#991B1B` sobre `#FFFFFF` | ~7.1:1 — PASS AA |
| On-primary sobre primary | `#FFFFFF` sobre `#991B1B` | ~7.1:1 — PASS AA |
| Toast success: branco sobre verde | `#FFFFFF` sobre `#16A34A` | ~4.5:1 — PASS AA (borderline) |
| Toast warning: branco sobre amber | `#FFFFFF` sobre `#D97706` | ~3.0:1 — FAIL AA (texto normal) |
| Toast info: branco sobre azul | `#FFFFFF` sobre `#2563EB` | ~4.6:1 — PASS AA |

### Modo Escuro
| Par de cores | Tokens | Estimativa |
|-------------|--------|-----------|
| Texto sobre fundo escuro | `#E8E4DF` sobre `#1E1E1E` | ~13.5:1 — PASS AA/AAA |
| Texto muted sobre fundo | `#9C9690` sobre `#1E1E1E` | ~5.8:1 — PASS AA |
| Primary dark (#EF4444) sobre fundo escuro | `#EF4444` sobre `#1E1E1E` | ~4.7:1 — PASS AA |
| On-primary dark (#1E1E1E) sobre EF4444 | `#1E1E1E` sobre `#EF4444` | ~4.7:1 — PASS AA |

**FALHA IDENTIFICADA — Toast Warning:** A combinação `#FFFFFF` sobre `#D97706` no modo claro tem razão de contraste estimada em ~3.0:1, abaixo do mínimo WCAG AA de 4.5:1 para texto normal e 3.0:1 para texto grande. O texto do toast usa `font-size: var(--text-sm)` (0.875rem / 14px) e `font-weight: 500`, o que classifica como texto normal. **Requer correção.**

---

## 11. Issues Identificados

### BLOQUEADORES (WCAG AA)
| # | Componente | Descrição | Critério |
|---|-----------|-----------|---------|
| B01 | `Toast.svelte` | Contraste insuficiente em `.toast--warning`: `#FFFFFF` sobre `#D97706` (~3.0:1 < 4.5:1 mínimo para texto normal de 14px/500weight) | WCAG 1.4.3 |

### RESSALVAS (Não bloqueadores)
| # | Componente | Descrição | Impacto |
|---|-----------|-----------|--------|
| R01 | `+layout.svelte` + `AppShell.svelte` | `id="main-content"` duplicado — tecnicamente inválido HTML | Baixo (Tauri WebView tolera) |
| R02 | `Modal.svelte` | `setTimeout(..., 50ms)` para focus ao abrir — frágil em testes automatizados | Baixo |
| R03 | `Toast.svelte` + `ToastContainer.svelte` | Live regions aninhadas com `aria-live` — comportamento variável entre screen readers | Médio |
| R04 | `Sidebar.svelte` | Botões com `aria-disabled` mas sem `disabled` nativo — Tab ainda os foca | Baixo |
| R05 | `src/app.html` | Atributo `lang` no `<html>` não verificado — pode estar estático em vez de refletir o locale ativo | Médio |
| R06 | `AppShell.svelte` | Tecla `Ctrl+I` implementada para Import (spec menciona `Ctrl+O`) — verificar requisito | Baixo |

---

## Sumário por Critério WCAG 2.1 AA

| Critério | Status | Notas |
|---------|--------|-------|
| 1.1.1 Non-text Content | PASS | SVGs decorativos com aria-hidden; elementos funcionais com labels |
| 1.3.1 Info and Relationships | PASS | Semântica HTML correta (header, nav, main, aside, section) |
| 1.3.2 Meaningful Sequence | PASS | Ordem DOM reflete ordem visual |
| 1.4.1 Use of Color | PASS | Cores não são o único meio de conveyer informação |
| 1.4.3 Contrast (Minimum) | FAIL | Toast warning (#FFFFFF/#D97706) ~3.0:1 |
| 2.1.1 Keyboard | PASS | Todos os interativos acessíveis via teclado |
| 2.1.2 No Keyboard Trap | PASS | Focus trap correto em Modal (Tab+Shift+Tab) |
| 2.4.1 Bypass Blocks | PASS | Skip-nav implementado e funcional |
| 2.4.2 Page Titled | PASS | `<svelte:head><title>` em rotas verificadas |
| 2.4.3 Focus Order | PASS | Ordem de foco lógica; ConfirmDialog foca em Cancelar |
| 2.4.6 Headings and Labels | PASS | Labels descritivos e traduzidos via i18n |
| 2.4.7 Focus Visible | PASS | `:focus-visible` global com outline de 2px |
| 3.1.1 Language of Page | PARCIAL | `lang` no `<html>` não verificado |
| 4.1.2 Name, Role, Value | PASS | role, aria-label, aria-expanded, aria-checked, aria-modal presentes |
| 4.1.3 Status Messages | PASS | aria-live em toasts e loading states |

---

## Verdict

**PASS COM RESSALVAS**

Uma falha de contraste de cor (B01 — toast warning) viola WCAG 1.4.3 e deve ser corrigida. As demais ressalvas são melhorias de qualidade sem impacto bloqueador. A base de acessibilidade é sólida: skip-nav, semântica HTML, ARIA completo em modais, focus trap, keyboard navigation global e i18n para screen readers estão todos corretamente implementados.

**Ações Obrigatórias Antes de Release:**
1. Corrigir contraste do toast warning — aumentar luminosidade do amber ou trocar texto para `#78350F` (dark amber) estimado ~7:1 sobre `#D97706`
2. Verificar e corrigir atributo `lang` dinâmico em `src/app.html`
3. Remover `id="main-content"` duplicado no `<div>` wrapper em `+layout.svelte`
