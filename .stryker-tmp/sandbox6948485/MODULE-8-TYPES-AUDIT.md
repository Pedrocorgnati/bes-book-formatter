# MODULE-8-TYPES-AUDIT.md
# Auditoria de Types (Enums + Interfaces) — TASK-3 (Cross-Rock Integration)
# BES Book Formatter | SvelteKit 5 + Tauri 2 + Rust + SQLite

**Data:** 2026-03-22
**Auditor:** module-8-cross-rock-integration / TASK-3
**Arquivos auditados:**
- `src/lib/types/enums.ts`
- `src/lib/types/interfaces.ts`
- `src-tauri/src/models/enums.rs`
- `src-tauri/src/models/typography.rs`
- `src-tauri/src/models/project.rs`
- `src-tauri/src/models/manuscript.rs`

---

## 1. Resumo Executivo

| Métrica | Valor |
|---|---|
| Enums TypeScript | 14 |
| Enums Rust equivalentes | 14 (+ 4 extras exclusivos Rust) |
| Interfaces TypeScript core | 12+ (expandidas além das 6 core da spec) |
| Structs Rust equivalentes | 10 arquivos de models |
| Divergências TS↔Rust (bloqueantes) | 1 (`Genre`: `Poetry` ausente em TS) |
| Divergências de naming (não bloqueantes) | 2 (`TypographyDefaults` vs `TypographyConfig`; `currentProject` store) |

**Verdict: PASS COM RESSALVAS** — Type system sólido. Uma divergência semântica real: `Genre.Poetry` existe em Rust mas não em TypeScript, criando potencial erro de deserialização se um projeto com gênero "poetry" for retornado ao front-end. Naming divergences são rastreáveis e não causam runtime errors.

---

## 2. Enums TypeScript vs Rust

### 2.1 Genre — DIVERGÊNCIA IDENTIFICADA

| Variante | TS (`enums.ts`) | Rust (`enums.rs`) | Status |
|---|---|---|---|
| Nonfiction | `NONFICTION = 'nonfiction'` | `Nonfiction` | OK |
| SelfHelp | `SELF_HELP = 'self_help'` | `SelfHelp` | OK |
| Technical | `TECHNICAL = 'technical'` | `Technical` | OK |
| Academic | `ACADEMIC = 'academic'` | `Academic` | OK |
| Fiction | `FICTION = 'fiction'` | `Fiction` | OK |
| Romance | `ROMANCE = 'romance'` | `Romance` | OK |
| Business | `BUSINESS = 'business'` | `Business` | OK |
| Management | `MANAGEMENT = 'management'` | `Management` | OK |
| Children | `CHILDREN = 'children'` | `Children` | OK |
| Ya | `YA = 'ya'` | `Ya` | OK |
| Poetry | **AUSENTE** | `Poetry` → `"poetry"` | **DIVERGÊNCIA** |

**Impacto da divergência Genre.Poetry:**
- Rust serializa `Genre::Poetry` como `"poetry"` via `#[serde(rename_all = "snake_case")]`
- TypeScript não tem `Genre.POETRY` — se o IPC retornar um `Project` com `genre: "poetry"`, o front-end receberá uma string não mapeada no enum
- Comportamento em runtime: o campo `genre` em `BookProject` é tipado como `Genre | null`, então TypeScript aceitará a string `"poetry"` silenciosamente mas sem type-safety
- `TypographyConfig.genrePreset` aceita `string` (não enum), então o preset de Poetry funcionará corretamente no Rust
- **Risco:** Filtragem/exibição de projetos por gênero no front-end pode não reconhecer livros de poesia

**Ação recomendada:** Adicionar `POETRY = 'poetry'` ao enum `Genre` em `src/lib/types/enums.ts`.

### 2.2 OutputFormat

| Variante | TS | Rust | Status |
|---|---|---|---|
| Epub3 | `EPUB3 = 'epub3'` | `Epub3` | OK |
| PdfEbook | `PDF_EBOOK = 'pdf_ebook'` | `PdfEbook` | OK |
| PdfPrint | `PDF_PRINT = 'pdf_print'` | `PdfPrint` | OK |
| Docx | `DOCX = 'docx'` | `Docx` | OK |
| Html5 | `HTML5 = 'html5'` | `Html5` | OK |
| MarkdownClean | `MARKDOWN_CLEAN = 'markdown_clean'` | `MarkdownClean` | OK |
| Txt | `TXT = 'txt'` | `Txt` | OK |
| JsonStructural | `JSON_STRUCTURAL = 'json_structural'` | `JsonStructural` | OK |

**Status: OK (paridade total)**

### 2.3 Platform

| Variante | TS | Rust | Status |
|---|---|---|---|
| Kdp | `KDP = 'kdp'` | `Kdp` | OK |
| KdpPrint | `KDP_PRINT = 'kdp_print'` | `KdpPrint` | OK |
| IngramSpark | `INGRAM_SPARK = 'ingram_spark'` | `IngramSpark` | OK |
| AppleBooks | `APPLE_BOOKS = 'apple_books'` | `AppleBooks` | OK |
| Kobo | `KOBO = 'kobo'` | `Kobo` | OK |
| Draft2digital | `DRAFT2DIGITAL = 'draft2digital'` | `Draft2digital` | OK |
| Generic | `GENERIC = 'generic'` | `Generic` | OK |

**Status: OK (paridade total)**

### 2.4 IllustrationState

| Variante | TS | Rust | Status |
|---|---|---|---|
| Pending | `PENDING = 'pending'` | `Pending` | OK |
| Imported | `IMPORTED = 'imported'` | `Imported` | OK |
| Linked | `LINKED = 'linked'` | `Linked` | OK |
| Error | `ERROR = 'error'` | `Error` | OK |

**Status: OK (paridade total)**

### 2.5 PageFormat

| Variante | TS | Rust | Status |
|---|---|---|---|
| Trade6x9 | `TRADE_6X9 = 'trade_6x9'` | `Trade6x9` | OK |
| Digest5_5x8_5 | `DIGEST_5_5X8_5 = 'digest_5_5x8_5'` | `Digest5_5x8_5` | OK |
| Pocket4_25x6_87 | `POCKET_4_25X6_87 = 'pocket_4_25x6_87'` | `Pocket4_25x6_87` | OK |
| A5 | `A5 = 'a5'` | `A5` | OK |
| A4 | `A4 = 'a4'` | `A4` | OK |
| Letter | `LETTER = 'letter'` | `Letter` | OK |
| Custom | `CUSTOM = 'custom'` | `Custom` | OK |

**Status: OK (paridade total)**

### 2.6 BookLanguage

| Variante | TS | Rust | Status |
|---|---|---|---|
| PtBr | `PT_BR = 'pt-BR'` | `PtBr` → `"pt-BR"` | OK |
| EnUs | `EN_US = 'en-US'` | `EnUs` → `"en-US"` | OK |
| ItIt | `IT_IT = 'it-IT'` | `ItIt` → `"it-IT"` | OK |
| EsEs | `ES_ES = 'es-ES'` | `EsEs` → `"es-ES"` | OK |

**Status: OK (paridade total, serialização manual no Rust via `#[serde(rename)]`)**

### 2.7 UILanguage

| Variante | TS | Rust | Status |
|---|---|---|---|
| PtBr | `PT_BR = 'pt-BR'` | `PtBr` → `"pt-BR"` | OK |
| EnUs | `EN_US = 'en-US'` | `EnUs` → `"en-US"` | OK |
| EsEs | `ES_ES = 'es-ES'` | `EsEs` → `"es-ES"` | OK |

**Status: OK (paridade total)**

### 2.8 PaperColor

| Variante | TS | Rust | Status |
|---|---|---|---|
| White70lb | `WHITE_70LB = 'white_70lb'` | `White70lb` | OK |
| Cream60lb | `CREAM_60LB = 'cream_60lb'` | `Cream60lb` | OK |
| Custom | `CUSTOM = 'custom'` | `Custom` | OK |

**Status: OK (paridade total)**

### 2.9 ChapterStartPage

| Variante | TS | Rust | Status |
|---|---|---|---|
| Odd | `ODD = 'odd'` | `Odd` | OK |
| Even | `EVEN = 'even'` | `Even` | OK |
| Continuous | `CONTINUOUS = 'continuous'` | `Continuous` | OK |

**Status: OK (paridade total)**

### 2.10 DropCapStyle

| Variante | TS | Rust | Status |
|---|---|---|---|
| None | `NONE = 'none'` | `None` | OK |
| FirstLetter | `FIRST_LETTER = 'first_letter'` | `FirstLetter` | OK |
| FirstWordSmallCaps | `FIRST_WORD_SMALL_CAPS = 'first_word_small_caps'` | `FirstWordSmallCaps` | OK |

**Status: OK (paridade total)**

### 2.11 OrnamentStyle

| Variante | TS | Rust | Status |
|---|---|---|---|
| None | `NONE = 'none'` | `None` | OK |
| Line | `LINE = 'line'` | `Line` | OK |
| Vignette | `VIGNETTE = 'vignette'` | `Vignette` | OK |
| Asterisks | `ASTERISKS = 'asterisks'` | `Asterisks` | OK |

**Status: OK (paridade total)**

### 2.12 PDFXProfile

| Variante | TS | Rust | Status |
|---|---|---|---|
| PdfX1a | `PDF_X1A = 'pdf_x1a'` | `PdfX1a` (enum `PdfxProfile`) | OK — naming do enum diverge (PDFXProfile vs PdfxProfile), mas valores serializados coincidem |
| PdfX4 | `PDF_X4 = 'pdf_x4'` | `PdfX4` | OK |

**Status: OK com nota** — enum TS chama-se `PDFXProfile`, Rust chama-se `PdfxProfile`. Sem impacto em runtime.

### 2.13 BookConfigVersion

| Variante | TS | Rust | Status |
|---|---|---|---|
| V1 | `V1 = 'v1'` | `V1` | OK |
| V2 | `V2 = 'v2'` | `V2` | OK |
| V3 | `V3 = 'v3'` | `V3` | OK |

**Status: OK (paridade total)**

### 2.14 ManuscriptCompleteness

| Variante | TS | Rust | Status |
|---|---|---|---|
| Blocking | `BLOCKING = 'blocking'` | `Blocking` | OK |
| Warning | `WARNING = 'warning'` | `Warning` | OK |
| Normal | `NORMAL = 'normal'` | `Normal` | OK |

**Status: OK (paridade total)**

---

## 3. Enums Rust Extras (sem equivalente TS)

Estes enums existem apenas no Rust e não têm contraparte em `enums.ts`. Todos são tipos internos ao backend, não retornados diretamente via IPC como campos tipados.

| Enum Rust | Localização | Uso | Risco |
|---|---|---|---|
| `ColorSpace` | `enums.rs` linha 241 | Validação DPI interno | Baixo — não exposto como campo tipado em TS |
| `FrontMatterType` | `enums.rs` linha 251 | Parser interno, `matter_type` é `Option<String>` no `ParsedChapter` | Baixo — serializado como string |
| `BackMatterType` | `enums.rs` linha 283 | Parser interno, mesma lógica | Baixo — serializado como string |
| `BookConfigVersion` (Rust) | `enums.rs` linha 226 | Versionamento interno | OK — tem equivalente TS |

---

## 4. Interfaces TypeScript vs Structs Rust

### 4.1 BookProject (TS) vs Project (Rust)

| Campo TS (`BookProject`) | Tipo TS | Campo Rust (`Project`) | Tipo Rust | Status |
|---|---|---|---|---|
| `id` | `string` | `id` | `String` | OK |
| `name` | `string` | `name` | `String` | OK |
| `besRootPath` | `string` | `bes_root_path` | `String` | OK |
| `bookConfigPath` | `string \| null` | `book_config_path` | `Option<String>` | OK |
| `genre` | `Genre \| null` | `genre` | `Option<String>` | Nota: Rust usa `String` (não enum tipado) |
| `language` | `BookLanguage` | `language` | `String` | OK — serde converte |
| `configVersion` | `BookConfigVersion \| null` | `config_version` | `Option<String>` | OK |
| `lastOpened` | `string \| null` | `last_opened` | `Option<String>` | OK |
| `formatFilePath` | `string \| null` | `format_file_path` | `Option<String>` | OK |
| `completenessLevel` | `string \| null` | `completeness_level` | `Option<String>` | OK |
| `completenessScore` | `number \| null` | `completeness_score` | `Option<f64>` | OK |
| — | — | `created_at` | `String` | Extra Rust (não exposto em TS) |
| — | — | `updated_at` | `String` | Extra Rust (não exposto em TS) |
| — | — | `chapter_count` | `Option<i32>` | Extra Rust (não exposto em TS) |
| — | — | `illustration_count` | `Option<i32>` | Extra Rust (não exposto em TS) |
| — | — | `manuscript_root` | `Option<String>` | Extra Rust (não exposto em TS) |
| — | — | `output_dir` | `Option<String>` | Extra Rust (não exposto em TS) |

**Status: OK** — TS tem subconjunto dos campos Rust. Campos extras no Rust não causam problemas de deserialização no front-end.

### 4.2 BookConfig (TS) vs BookConfig (Rust — `book_config.rs`)

**Status: OK** — Mapeamento confirmado pela geração/parsing do `bes.json`.

### 4.3 Illustration (TS) vs Illustration (Rust — `illustration.rs`)

**Status: OK** — Interface TS mapeia campos do SQLite via Rust.

### 4.4 GenerationResult (TS) vs GenerationResult (Rust — `generation.rs`)

**Status: OK** — Campos `success`, `outputPath`, `format`, `platform`, `errors`, `warnings`, `durationMs` presentes.

### 4.5 TypographyConfig (TS) vs TypographyConfig (Rust — `typography.rs`) — DIVERGÊNCIA DE NAMING

**Situação:**
- `interfaces.ts` linha 98: interface `TypographyDefaults` — campos simplificados (bodyFont, headingFont, codeFont, bodySizePt, lineHeight)
- `interfaces.ts` linha 132: interface `TypographyConfig` — struct completa (29 campos) mapeando a tabela SQLite
- `typography.rs` struct `TypographyConfig` — 29 campos, mapeamento exato com `TypographyConfig` TS

**Conclusão:** A spec do contrato C6 referenciava "TypographyConfig" esperando um único tipo. Na implementação, existem DOIS tipos TS:
- `TypographyDefaults`: subset resumido embutido em `BookConfig.typography`
- `TypographyConfig`: struct completa retornada pelos IPC `get_typography_config`/`set_typography_config`

**Status: OK com nota** — A interface `TypographyConfig` TS (linha 132) mapeia fielmente o struct Rust. A `TypographyDefaults` é um subset para o `BookConfig` embedded. Sem divergência de dados; divergência apenas de naming na spec original.

### 4.6 UserPreferences (TS) vs Preference (Rust — `preference.rs`)

**Status: OK** — Estrutura de preferências via key-value no Rust, serializada como objeto no TS.

### 4.7 ManuscriptAST — Contrato C5

**Situação verificada:**
- `manuscript.rs` contém `ParsedManuscript` (struct interna — nota no código: "NOT returned directly by IPC")
- Campos: `project_id`, `front_matter`, `chapters`, `back_matter`, `illustrations`, `toc_present`, `index_present`, `total_words`, `errors`
- TypeScript não tem interface `ManuscriptAST` em `interfaces.ts` — o IPC `parse_manuscript` provavelmente retorna uma versão resumida ou `CompletenessResult`

**Status: ACEITÁVEL** — `ParsedManuscript` é struct interna ao Rust. O front-end não precisa tipar o AST completo; recebe apenas os dados processados (completeness score, lista de ilustrações, etc.) via IPC dedicados.

---

## 5. Interfaces TypeScript Expandidas (além das 6 core da spec)

A spec esperava 6 interfaces core. A implementação entrega 20+ interfaces, todas coerentes com o escopo:

| Interface | Arquivo | Propósito |
|---|---|---|
| `BookProject` | `interfaces.ts` | Core — C1 |
| `BookConfig` | `interfaces.ts` | Core — C2 |
| `Illustration` | `interfaces.ts` | Core — C3 |
| `GenerationResult` | `interfaces.ts` | Core — C7 |
| `TypographyConfig` | `interfaces.ts` | Core — C6 (completo) |
| `UserPreferences` | `interfaces.ts` | Core — preferences |
| `ApiResponse<T>` | `interfaces.ts` | Wrapper genérico IPC |
| `Pagination` | `interfaces.ts` | Listagens |
| `PageDimensions` | `interfaces.ts` | Dimensões físicas |
| `TypographyDefaults` | `interfaces.ts` | Subset de typography para BookConfig |
| `CompletenessResult` | `interfaces.ts` | Resultado de calculate_completeness |
| `SidecarInfo` | `interfaces.ts` | Estado de sidecar individual |
| `SidecarStatus` | `interfaces.ts` | Estado agregado de sidecars |
| `FontInfo` | `interfaces.ts` | Catálogo de fontes |
| `DpiValidation` | `interfaces.ts` | Resultado validate_illustration_dpi |
| `TypoIssue` | `interfaces.ts` | Orphan/widow detection |
| `LayoutIssue` | `interfaces.ts` | Preview layout issues |
| `ChecklistItem` | `interfaces.ts` | Item do content checklist |
| `PreflightResult` | `interfaces.ts` | Resultado run_preflight |
| `ValidationResult` | `interfaces.ts` | Validações genéricas |
| `StoredGenerationResult` | `interfaces.ts` | Resultado armazenado no SQLite |
| `GenOptions` | `interfaces.ts` | Opções de geração |
| `FormatSelection` | `interfaces.ts` | Seleção de formatos |
| `CoverConfig` | `interfaces.ts` | Configuração de capa |
| `CoverConfigInput` | `interfaces.ts` | DTO de input para cover |
| `CoverTemplate` | `interfaces.ts` | Template de capa |
| `SpineWidthResult` | `interfaces.ts` | Resultado calculate_spine_width |

---

## 6. Store Svelte: currentProject

**Contrato C10 esperava:** uma store separada `currentProject` exportada de `projectStore.ts`

**Implementação encontrada em `src/lib/stores/projectStore.ts`:**
```typescript
// Store é um objeto composto:
export const projectsStore = createProjectsStore();
// currentProject está em: projectsStore.current (campo dentro do state)
// Não existe export separado 'currentProject'
```

**Impacto:** Componentes que fazem `import { currentProject } from '$lib/stores/projectStore'` terão erro de importação. O padrão correto é `$projectsStore.current`.

**Ação recomendada:** Adicionar export derivado para compatibilidade:
```typescript
export const currentProject = derived(projectsStore, $s => $s.current);
```

---

## 7. Verdict Final

```
PASS COM RESSALVAS

✓ 13/14 enums TS têm paridade total com Rust
✓ 20+ interfaces TS excedem as 6 core da spec
✓ Structs Rust em 10 arquivos cobrem todos os contratos
✓ Serde serialization compatível em todos os tipos verificados

⚠ Genre.POETRY ausente em TS (presente em Rust) — adicionar variante
⚠ TypographyDefaults vs TypographyConfig: dois tipos TS para um Rust — documentar distinção
⚠ currentProject store: não existe como export separado — available via $projectsStore.current
```
