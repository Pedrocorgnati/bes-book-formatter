# Error Handling — Task List
**Projeto:** BES Book Formatter (SvelteKit + Tauri + Rust/Axum)
**Stack:** SvelteKit (SPA mode), Tauri IPC, SQLite
**Data:** 2026-03-22
**Nota:** Projeto não usa Next.js; padrões equivalentes SvelteKit aplicados.

---

## Tasks Executadas

### T001 – GenerationPanel: try/catch ausente em ipcGetGenerationResults ✅ COMPLETED
**Arquivo:** `src/lib/components/generation/GenerationPanel.svelte`

**Problema:** Na função `startGeneration()`, o call `ipcGetGenerationResults(projectId)` após o loop de geração não tinha try/catch. Erro de IPC crasharia silenciosamente sem feedback e sem fechar o painel de nova geração.

**Correção aplicada:**
```typescript
// Refresh history
try {
  const history = await ipcGetGenerationResults(projectId);
  generationStore.setHistory(history);
} catch (err) {
  console.error('[GenerationPanel] history refresh error:', err);
  toastStore.error(t('generation.historyLoadError'));
} finally {
  showNewGeneration = false;
}
```

**Benefícios:**
- `showNewGeneration = false` agora no `finally` — executa mesmo em caso de erro
- Toast de erro exibido ao usuário com mensagem localizada
- Log com contexto de componente para debugging

---

## Tasks Canceladas (Falsos Positivos — Rotas São Thin Wrappers)

### T002 – /import ❌ FALSE POSITIVE
`ImportWizard.svelte` já tem tratamento completo: try/catch em `handleSelectFolder`, `analyseFolder`, `handleImport`; loading state, configError state, toast.error, `finally` com reset de `importing`.

### T003 – /illustrations ❌ FALSE POSITIVE
`IllustrationGallery.svelte` (componente filho) já tem try/catch completo com toast.error e log com prefix `[IllustrationGallery]`.

### T004 – /integration ❌ FALSE POSITIVE
`BesStatusPanel` e `EditorialProgressBar` (componentes filhos) gerenciam error handling internamente. Página apenas passa props.

### T005 – /output ❌ FALSE POSITIVE
`GenerationPanel.svelte` (componente filho) gerencia todo o error handling. Página apenas faz guard de `!project`.

### T006 – /settings ❌ FALSE POSITIVE
`BesStatusPanel` e `EditorialProgressBar` (componentes filhos) gerenciam error handling internamente.

### T007 – Toast aria-live ❌ JÁ IMPLEMENTADO
`ToastContainer.svelte` já tem `aria-live="polite"` e `aria-atomic="false"`.
`Toast.svelte` já diferencia: erros/warnings usam `role="alert"` + `aria-live="assertive"`, success/info usam `role="status"` + `aria-live="polite"`.

---

## Tasks Pendentes (Infra / Melhorias)

### T008 – Sentry: implementar integração de error tracking
**Prioridade:** Alta (pré-lançamento)
**Arquivo:** `src/lib/utils/analytics.ts`

`analytics.ts` tem TODOs para Sentry mas não implementado. Em produção (desktop via GitHub Releases), erros não rastreados impossibilitam debug de issues dos usuários.

**Critérios de Aceite:**
- `Sentry.init()` chamado condicionalmente (só se user consentiu via analytics opt-in)
- `logger.ts` centralizado: `logger.error(msg, context)`, `logger.warn(msg)`, `logger.info(msg)`
- Contexto capturado: projectId, rota, action, mensagem de erro
- Stack trace não exposto na UI

**Estimativa:** 2h

---

### T009 – Substituir console.error por logger centralizado
**Prioridade:** Média (após T008)
**Dependências:** T008
**Arquivos com console.error:**
- `src/routes/project/[id]/preview/+page.svelte`
- `src/lib/components/typography/TypographyPanel.svelte`
- `src/lib/components/illustrations/IllustrationGallery.svelte`
- `src/lib/components/preview/AnnotationLayer.svelte`
- `src/lib/components/preview/PreviewRightPanel.svelte`

**Critérios de Aceite:**
- Todos os `console.error` substituídos por `logger.error(msg, { component, action, error })`
- Zero `console.error` no código de produção

**Estimativa:** 30min

---

### T010 – ErrorPage.svelte: adicionar CTA de "Tentar Novamente"
**Prioridade:** Baixa
**Arquivo:** `src/lib/components/ui/ErrorPage.svelte`

Componente exibe erro mas sem ações contextuais de recovery.

**Critérios de Aceite:**
- Prop `onRetry?: () => void` — botão "Tentar novamente" quando fornecida
- Prop `supportText?: string` — texto de sugestão contextual
- Botão "Voltar ao início" sempre presente

**Estimativa:** 20min

---

## Resumo de Achados (Strengths)

O projeto tem uma base sólida de error handling:
- ✅ Padrão consistente: `try/catch` → `state update` → `toast.error`
- ✅ Zero catch vazios
- ✅ Loading states com skeletons e aria labels
- ✅ Empty states com CTA em todas as listagens
- ✅ Degradação graciosa (localStorage, analytics, init DB)
- ✅ Toast system com acessibilidade correta (role/aria-live por tipo)
- ✅ IPC wrapper com propagação correta de erros
- ✅ Stores com campos error/loading centralizados
