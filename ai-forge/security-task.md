# Security Tasks — BES Book Formatter

> Gerado por: /nextjs:security (adaptado para SvelteKit + Rust/Tauri)
> Data: 2026-03-22
> Risco Geral: BAIXO

---

## T001 - Path Traversal em upload_font (leitura de arquivo arbitrário)

**Severidade:** MÉDIA
**OWASP:** A01 (Broken Access Control), A03 (Injection — Path Traversal)
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src-tauri/src/services/font_service.rs`

**Descrição:**
`FontService::upload_font` recebe `file_path: &str` do frontend via IPC (`invoke('upload_font', { filePath })`)
e executa `fs::copy(source, &dest_path)` sem canonicalizar o path de origem.
Um JS malicioso (ou extensão de navegador comprometida) que controle o argumento `filePath`
pode copiar arquivos arbitrários do sistema para dentro de `$APPDATA/fonts/{project_id}/`,
efetivamente exfiltrando conteúdo via o próprio mecanismo de "listagem de fontes".

Evidência: `src-tauri/src/services/font_service.rs:35-78`

**Critérios de Aceite:**
- [ ] `source.canonicalize()` chamado antes de `fs::copy`
- [ ] Verificar que o path canonicalizado não aponta para diretório protegido (ex: `/etc`, `C:\Windows`)
- [ ] Teste: `upload_font` rejeita `../../../etc/passwd` com erro FS_001
- [ ] Teste: `upload_font` rejeita symlinks que apontam para fora do diretório permitido

**Estimativa:** 1h

---

## T002 - shell:allow-execute sem scope de binários autorizados

**Severidade:** MÉDIA
**OWASP:** A05 (Security Misconfiguration)
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src-tauri/capabilities/default.json`

**Descrição:**
A capability `"shell:allow-execute"` é concedida de forma genérica, sem definir o escopo
de binários permitidos. Conforme documentação Tauri v2, sem um scope declarado o plugin
permite executar qualquer programa do sistema via `invoke('shell_execute', ...)` a partir
do renderer. Embora o CSP vigente mitigue XSS, a superfície de ataque permanece aberta
caso o renderer seja comprometido.

O correto é substituir pela permissão específica do `tauri-plugin-shell` com scope
limitando os binários autorizados (typst, gs, java, pandoc — os sidecars reais).

Evidência: `src-tauri/capabilities/default.json:8` — `"shell:allow-execute"`

**Critérios de Aceite:**
- [ ] `"shell:allow-execute"` substituído por scope com `allow` listando apenas os binários usados
- [ ] Teste: invoke para binário não listado é rejeitado pelo Tauri runtime
- [ ] CI não quebra após a mudança (sidecars continuam funcionando)

**Estimativa:** 1h

---

## T003 - Atualizar devDependências vulneráveis (@stryker-mutator)

**Severidade:** BAIXA
**OWASP:** A06 (Vulnerable and Outdated Components)
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `package.json`, `package-lock.json`

**Descrição:**
`npm audit` reporta 19 vulnerabilidades (6 high, 7 moderate, 6 low).
As high são todas em `@stryker-mutator/*` (prototype pollution + command injection via glob CLI).
São exclusivamente devDependencies — não afetam o bundle de produção Tauri.
Atualizar para `@stryker-mutator/core@9.6.0` resolve todas as high.

Evidência: `npm audit` — GHSA-9j5q-479x-43g2, GHSA-5j98-mcp5-4vw2

**Critérios de Aceite:**
- [ ] `npm audit --omit=dev` reporta 0 high/critical
- [ ] `npm run test:mutation` ainda executa após upgrade
- [ ] `package-lock.json` atualizado e commitado

**Estimativa:** 30min

---

## T004 - console.error com objetos de erro completos nos componentes Svelte

**Severidade:** BAIXA
**OWASP:** A09 (Security Logging and Monitoring Failures)
**Tipo:** PARALLEL-GROUP-1
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/components/preview/AnnotationLayer.svelte`
- modificar: `src/lib/components/preview/PreviewRightPanel.svelte`
- modificar: `src/lib/components/illustrations/IllustrationGallery.svelte`
- modificar: `src/lib/components/generation/GenerationPanel.svelte`
- modificar: `src/lib/components/typography/TypographyPanel.svelte`
- modificar: `src/routes/project/[id]/preview/+page.svelte`

**Descrição:**
`console.error('[Component] error:', e)` expõe o objeto de erro completo no DevTools do
Tauri. Em produção, os DevTools são acessíveis por padrão (`devtools: true` no build de dev).
O objeto `e` pode conter paths internos, conteúdo de manuscrito parcial, ou stack traces
que facilitam engenharia reversa.

Padronizar para `console.error('[Component] error:', e instanceof Error ? e.message : String(e))`
para não expor stacks/objetos em produção.

Evidência: `src/lib/components/preview/AnnotationLayer.svelte:50,93,106`

**Critérios de Aceite:**
- [ ] Todos os `console.error(..., e)` trocados por `console.error(..., e instanceof Error ? e.message : String(e))`
- [ ] DevTools em produção não expõe stack traces de libs internas
- [ ] Funcionamento dos componentes preservado (testes existentes passam)

**Estimativa:** 1h

---

## Resumo

| Task | Severidade | OWASP | Estimativa |
|------|-----------|-------|-----------|
| T001 | MÉDIA | A01, A03 | 1h |
| T002 | MÉDIA | A05 | 1h |
| T003 | BAIXA | A06 | 30min |
| T004 | BAIXA | A09 | 1h |
| **Total** | | | **3h 30min** |
