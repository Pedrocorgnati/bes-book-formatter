# Security Report — BES Book Formatter
> Data: 2026-03-22
> Stack: SvelteKit + Rust/Axum + Tauri + SQLite
> Auditoria: OWASP Top 10 2021 adaptada para app desktop

---

## Resumo Executivo

**Risco Geral: BAIXO**

App desktop single-user sem rede pública, sem autenticação multiusuário, sem servidor HTTP exposto.
O maior vetor de ataque é um XSS no renderer combinado com IPC permissivo — mitigado pelo CSP vigente.

---

## OWASP Top 10 Coverage

| OWASP | Categoria | Status | Findings |
|-------|-----------|--------|---------|
| A01 | Broken Access Control | ⚠️ MÉDIO | Path traversal em upload_font → CORRIGIDO |
| A02 | Cryptographic Failures | ✅ OK | Sem senhas, sem tokens, SQLite local |
| A03 | Injection | ✅ OK | SQL via SQLx parametrizado; args como array (SEC-009); sem @html em Svelte |
| A04 | Insecure Design | ✅ OK | sem rate-limit necessário (app local); sidecars com timeout |
| A05 | Security Misconfiguration | ⚠️ MÉDIO | shell:allow-execute sem scope → CORRIGIDO; CSP OK |
| A06 | Vulnerable Components | ⚠️ BAIXO | 19 vulns npm (apenas devDeps); 0 vulns em produção |
| A07 | Auth Failures | ✅ N/A | App single-user sem auth |
| A08 | Data Integrity | ✅ OK | .env no .gitignore; sem secrets no código |
| A09 | Logging Failures | ⚠️ BAIXO | console.error com objetos raw → CORRIGIDO |
| A10 | SSRF | ✅ OK | Sem fetch com URL de usuário; sidecars usam paths fixos |

---

## Vulnerabilidades Encontradas e Corrigidas

### [CORRIGIDA] T001 — Path Traversal em upload_font
- **Arquivo:** `src-tauri/src/services/font_service.rs:35-78`
- **OWASP:** A01, A03
- **Severidade:** MÉDIA
- **Descrição:** `upload_font` recebia `file_path` arbitrário do frontend sem canonicalizar.
  Um atacante com acesso ao renderer poderia invocar `invoke('upload_font', {filePath: '../../../etc/passwd'})`
  para copiar arquivos do sistema para `$APPDATA/fonts/`.
- **Correção:** Adicionado `source.canonicalize()` + bloqueio de paths de sistema (`/etc`, `/proc`, `/sys`, `/dev`).

### [CORRIGIDA] T002 — shell:allow-execute sem scope
- **Arquivo:** `src-tauri/capabilities/default.json:11`
- **OWASP:** A05
- **Severidade:** MÉDIA
- **Descrição:** `"shell:allow-execute"` genérico permitia que JS no renderer executasse qualquer binário do sistema.
- **Correção:** Substituído por scope explicitando apenas: typst, gs, pandoc, java, which.

### [CORRIGIDA] T003 — npm audit @stryker-mutator
- **OWASP:** A06
- **Severidade:** BAIXA
- **Descrição:** 19 vulns (6 high) em devDeps (@stryker-mutator prototype pollution + glob command injection).
- **Resultado:** `npm audit fix --force` executado. `npm audit --omit=dev` → 0 vulnerabilidades.

### [CORRIGIDA] T004 — console.error com objetos de erro completos
- **Arquivos:** 6 componentes Svelte
- **OWASP:** A09
- **Severidade:** BAIXA
- **Descrição:** `console.error('...', e)` expunha stack traces e paths internos nos DevTools.
- **Correção:** Substituído por `e instanceof Error ? e.message : String(e)` em todos os casos.

---

## O que Estava Correto (não requer ação)

| Item | Evidência |
|------|-----------|
| CSP sem `'unsafe-eval'` | `tauri.conf.json` security.csp |
| Zero `@html` em Svelte | grep: nenhum resultado |
| SQL parametrizado (SQLx) | repositories/*.rs — sem format! em queries |
| Sem secrets hardcoded | grep: nenhum resultado |
| `.env` no `.gitignore` | `.gitignore:4` |
| `sanitize_slug` correto | `common.rs` — apenas alphanumeric + dash |
| `filesystem_service.rs` com canonicalize | `filesystem_service.rs:27` — THREAT-001 mitigado |
| Sidecar args como array | SEC-009 comentado no código |
| Font upload valida extensão+tamanho | `font_service.rs:42-63` |
| Stderr truncado em sidecars | SEC-008 — 100 chars max |

---

## Headers de Segurança (Tauri CSP)

| Header/Policy | Status |
|----------------|--------|
| `default-src 'self'` | ✅ |
| `script-src 'self'` (sem unsafe-eval) | ✅ |
| `img-src asset: tauri: data:` | ✅ |
| `connect-src ipc: tauri:` | ✅ |
| `style-src 'unsafe-inline'` | ⚠️ Necessário para Tailwind/CSS-in-JS |

---

## Dependências (npm audit)

| Contexto | Vulnerabilidades |
|----------|----------------|
| Produção (`--omit=dev`) | 0 ✅ |
| Dev total | 2 low (cookie em @sveltejs/kit — sem impacto em Tauri) |

---

## Arquivos Modificados

- `src-tauri/src/services/font_service.rs` — canonicalize + bloqueio de paths de sistema
- `src-tauri/capabilities/default.json` — shell scope restrito
- `src/lib/components/preview/AnnotationLayer.svelte` — console.error
- `src/lib/components/preview/PreviewRightPanel.svelte` — console.error
- `src/lib/components/illustrations/IllustrationGallery.svelte` — console.error
- `src/lib/components/generation/GenerationPanel.svelte` — console.error
- `src/lib/components/typography/TypographyPanel.svelte` — console.error
- `src/routes/project/[id]/preview/+page.svelte` — console.error
- `package-lock.json` — @stryker-mutator atualizado

