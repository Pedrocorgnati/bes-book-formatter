# TypeScript Audit — BES Book Formatter
Stack: SvelteKit + Tauri (TypeScript frontend)
Auditado em: 2026-03-22

## Resultado tsc --noEmit: PASSOU ✅

---

### T001 – Adicionar noUncheckedIndexedAccess ao tsconfig
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `tsconfig.json`

**Descrição:** O `tsconfig.json` tem `strict: true` mas não habilita `noUncheckedIndexedAccess`. Isso permite acesso a arrays/objetos indexados sem verificação de `undefined`, o que pode gerar runtime errors silenciosos.
**Critérios de Aceite:** `noUncheckedIndexedAccess: true` no tsconfig; `tsc --noEmit` continua passando.
**Estimativa:** 0.5h
**Status:** COMPLETED

---

### T002 – Tipar acesso ao path de File da API Tauri (eliminar `as any`)
**Tipo:** SEQUENTIAL
**Dependências:** T001
**Arquivos:**
- modificar: `src/lib/components/illustrations/IllustrationDropzone.svelte`

**Descrição:** `(file as any).path` em linha 97 usa `as any` para acessar a propriedade `.path` que o Tauri injeta no objeto `File` da WebView. Deve ser tipado via interface local `TauriFile extends File { path: string }`.
**Critérios de Aceite:** Nenhum `as any` no arquivo; eslint-disable removido; `tsc --noEmit` passa.
**Estimativa:** 0.5h
**Status:** COMPLETED

---

### T003 – Substituir mocks `as any` nos testes i18n por `vi.mocked()`
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/i18n/engine.test.ts`

**Descrição:** `(localStorage.getItem as any).mockReturnValue(...)` em linhas 69 e 75. Vitest expõe `vi.mocked()` para mocks tipados, eliminando o `as any`.
**Critérios de Aceite:** Sem `as any` no arquivo de teste; testes continuam passando.
**Estimativa:** 0.5h
**Status:** COMPLETED

---

### T004 – Promover string com comentário de union para union type literal
**Tipo:** SEQUENTIAL
**Dependências:** T001
**Arquivos:**
- modificar: `src/lib/types/interfaces.ts`

**Descrição:** Quatro campos usam `string` com comentário indicando os valores válidos:
- `TypoIssue.issueType: string // "orphan" | "widow"` — deve ser `'orphan' | 'widow'`
- `LayoutIssue.issueType: string // "orphan" | "widow" | "short_page"` — deve ser `'orphan' | 'widow' | 'short_page'`
- `StoredGenerationResult.status: string // 'success' | 'error' | 'cancelled'` — deve ser `'success' | 'error' | 'cancelled'`
- `GenOptions.format/platform`, `FormatSelection.platform` — devem usar `OutputFormat` / `Platform` onde possível (campos vindos do DB SQLite têm limitação — manter `string` documentado)

**Critérios de Aceite:** Union types aplicados; `tsc --noEmit` passa; sem regressão nos componentes.
**Estimativa:** 1h
**Status:** COMPLETED

---

### T005 – Substituir non-null assertions `result.data!` por narrowing explícito
**Tipo:** SEQUENTIAL
**Dependências:** none
**Arquivos:**
- modificar: `src/lib/ipc/illustrations.ts`
- modificar: `src/lib/ipc/bes.ts`
- modificar: `src/lib/ipc/parser.ts`
- modificar: `src/lib/ipc/generation.ts`
- modificar: `src/lib/ipc/typography.ts`
- modificar: `src/lib/ipc/preview.ts`

**Descrição:** 18 ocorrências de `result.data!` no padrão:
```ts
if (result.error) throw new Error(result.error);
return result.data!; // data ainda é T | null aqui
```
Após o guard de `result.error`, `data` pode ainda ser `null` (ex: resposta `{ data: null, error: null }`). O `!` suprime o erro mas não garante segurança. Deve-se adicionar um segundo guard ou extrair helper.
**Critérios de Aceite:** Sem `!` não documentados; helper `unwrapResponse<T>()` criado ou guard local adicionado; `tsc --noEmit` passa.
**Estimativa:** 1.5h
**Status:** COMPLETED
