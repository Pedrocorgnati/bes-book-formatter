# SQLite Schemas & Migrations Audit — Module 8

**Projeto:** BES Book Formatter
**Data:** 2026-03-22
**Auditor:** auto-flow execute (module-8 / TASK-4)
**Status:** PASS

---

## Summary

| Métrica | Valor |
|---------|-------|
| Tabelas encontradas | 9/9 |
| Migrations aplicadas | 7 (spec esperava 6) |
| Foreign Keys ON DELETE CASCADE | 6/6 |
| Índices presentes | Sim (múltiplos por tabela) |
| schema_version tracking | Parcial (M001-M003 registram; M004-M007 não registram) |
| Integridade referencial | OK |
| Migration extra (enhancement) | M002_add_error_state — válido |

**Verdict resumido:** PASS — todas as 9 tabelas existem com DDL correto e FKs válidas. 7 migrations ao invés de 6 (M002 é enhancement válido do estado `error` em illustrations). Ausência de `INSERT INTO schema_version` em M004-M007 é warning menor.

---

## Migration Inventory

| # | Arquivo | Descrição | Tabelas criadas/alteradas | schema_version INSERT |
|---|---------|-----------|--------------------------|----------------------|
| M001 | `M001_initial_schema.sql` | Schema inicial | `schema_version`, `projects`, `illustrations`, `user_preferences` | Sim |
| M002 | `M002_add_error_state.sql` | Enhancement: adiciona estado `error` em illustrations via recreação | `illustrations` (alterada) | Sim |
| M003 | `M003_typography_config.sql` | Configurações tipográficas | `typography_configs` | Sim |
| M004 | `M004_generation_results.sql` | Resultados de geração de output | `generation_results` | **Não** |
| M005 | `M005_annotations.sql` | Anotações de preview + trigger updated_at | `annotations` | **Não** |
| M006 | `M006_bes_document_cache.sql` | Cache de documentos BES com TTL e SHA-256 | `bes_document_cache` | **Não** |
| M007 | `M007_cover_configs.sql` | Configuração de capa (template, cores, dimensões) | `cover_configs` | **Não** |

**Total:** 7 migrations | **Desvio do spec:** Spec referenciava 6 migrations (V001-V006); implementação tem 7 (M001-M007) devido ao M002 extra (enhancement de ilustrações).

**Naming:** Spec usa prefixo `V` (V001-V006); implementação usa prefixo `M` (M001-M007). Desvio apenas de nomenclatura, sem impacto funcional.

---

## Schema Inventory (9 tabelas)

### 1. schema_version

| Atributo | Valor |
|----------|-------|
| Migration | M001 |
| PK | `version INTEGER AUTOINCREMENT` |
| Colunas | `version`, `migration_name`, `applied_at` |
| FK | Nenhuma |
| Índices | (PK implícito) |
| Status | OK |

### 2. projects

| Atributo | Valor |
|----------|-------|
| Migration | M001 |
| PK | `id TEXT NOT NULL` |
| Colunas relevantes | `name`, `bes_root_path UNIQUE`, `genre`, `language DEFAULT 'pt-BR'`, `completeness_score`, `chapter_count`, `illustration_count`, `manuscript_root`, `output_dir` |
| FK | Nenhuma (tabela raiz) |
| Índices | `idx_projects_last_opened`, `idx_projects_genre` |
| CHECK | `completeness_score IN [0.0, 1.0]`, `completeness_level IN ('blocking', 'warning', 'normal')` |
| Status | OK |

### 3. illustrations

| Atributo | Valor |
|----------|-------|
| Migration | M001 + M002 (enhancement) |
| PK | `id TEXT NOT NULL` |
| Colunas relevantes | `project_id`, `placeholder_name`, `state`, `image_path`, `validated_dpi`, `alt_text`, `color_space` |
| FK | `project_id REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_illustrations_project_id`, `idx_illustrations_project_state`, `uq_illustrations_project_placeholder` (UNIQUE) |
| CHECK | `state IN ('pending', 'imported', 'linked', 'error')` — `error` adicionado por M002 |
| Status | OK |

**Nota M002:** Migration utiliza estratégia de recreação de tabela (CREATE TABLE illustrations_new → INSERT → DROP → RENAME) para adicionar `error` ao CHECK constraint. Padrão correto para SQLite que não suporta ALTER COLUMN.

### 4. user_preferences

| Atributo | Valor |
|----------|-------|
| Migration | M001 |
| PK | `id INTEGER AUTOINCREMENT` |
| Colunas relevantes | `theme`, `language`, `font_size_offset`, `sidebar_collapsed`, `last_project_id` |
| FK | Nenhuma |
| Índices | (PK implícito) |
| Status | OK |

### 5. typography_configs

| Atributo | Valor |
|----------|-------|
| Migration | M003 |
| PK | `id TEXT DEFAULT (lower(hex(randomblob(16))))` |
| Colunas relevantes | `project_id UNIQUE`, `font_body`, `font_heading`, `font_size_body`, `line_height`, `chapter_start`, `hyphenation`, `page_width_mm`, `page_height_mm` |
| FK | `project_id REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_typography_project_id` |
| CHECK | `chapter_start IN ('recto', 'verso', 'any')` |
| Status | OK |

### 6. generation_results

| Atributo | Valor |
|----------|-------|
| Migration | M004 |
| PK | `id TEXT NOT NULL` |
| Colunas relevantes | `project_id`, `format`, `platform`, `output_path`, `file_size_bytes`, `duration_ms`, `status DEFAULT 'pending'`, `errors JSON`, `warnings JSON` |
| FK | `project_id REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_generation_results_project_id`, `idx_generation_results_created_at` (project_id, created_at DESC) |
| CHECK | `status IN ('success', 'error', 'pending')` |
| Status | OK |

### 7. annotations

| Atributo | Valor |
|----------|-------|
| Migration | M005 |
| PK | `id TEXT DEFAULT (lower(hex(randomblob(16))))` |
| Colunas relevantes | `project_id`, `page_number`, `x_percent`, `y_percent`, `annotation_type`, `color`, `content`, `created_at`, `updated_at` |
| FK | `project_id REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_annotations_project_page` (project_id, page_number) |
| Trigger | `annotations_updated_at` — AFTER UPDATE define `updated_at = datetime('now')` |
| CHECK | `page_number >= 1`, `x_percent BETWEEN 0 AND 100`, `y_percent BETWEEN 0 AND 100`, `annotation_type IN ('comment', 'highlight', 'flag')` |
| Status | OK |

### 8. bes_document_cache

| Atributo | Valor |
|----------|-------|
| Migration | M006 |
| PK | `id TEXT DEFAULT (lower(hex(randomblob(16))))` |
| Colunas relevantes | `project_id`, `document_type`, `content`, `parsed_json`, `file_path`, `file_hash SHA-256`, `cached_at` |
| FK | `fk_bdc_project: project_id REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_bdc_project_type` (project_id, document_type), `idx_bdc_project` |
| UNIQUE | `uq_bdc_entry (project_id, document_type)` |
| CHECK | `ck_bdc_document_type IN ('bdd', 'book_architecture', 'metadata', 'editorial_progress')` |
| Status | OK |

### 9. cover_configs

| Atributo | Valor |
|----------|-------|
| Migration | M007 |
| PK | `id TEXT DEFAULT (lower(hex(randomblob(16))))` |
| Colunas relevantes | `project_id UNIQUE`, `template_id`, `genre`, `platform`, `title_override`, `author_override`, `back_cover_text`, `primary_color`, `secondary_color`, `font_title`, `font_author`, `cover_image_path`, `page_count`, `spine_width_mm`, `paper_type` |
| FK | `project_id UNIQUE REFERENCES projects(id) ON DELETE CASCADE` |
| Índices | `idx_cover_configs_project` |
| CHECK | `platform IN ('amazon-kdp', 'ingram', 'generic')`, `paper_type IN ('white', 'cream')`, `page_count >= 0`, `spine_width_mm >= 0.0` |
| Status | OK |

---

## Foreign Key Validation

| Tabela filha | Coluna FK | Tabela pai | ON DELETE | Status |
|---|---|---|---|---|
| illustrations | project_id | projects | CASCADE | OK |
| typography_configs | project_id | projects | CASCADE | OK |
| generation_results | project_id | projects | CASCADE | OK |
| annotations | project_id | projects | CASCADE | OK |
| bes_document_cache | project_id | projects | CASCADE | OK |
| cover_configs | project_id | projects | CASCADE | OK |

**Total FKs CASCADE:** 6/6 — todas as tabelas filhas deletam registros ao deletar o projeto pai.

---

## Index Coverage

| Tabela | Índices | Cobertura |
|--------|---------|-----------|
| schema_version | PK implícito | OK |
| projects | `idx_projects_last_opened`, `idx_projects_genre` | OK |
| illustrations | `idx_illustrations_project_id`, `idx_illustrations_project_state`, `uq_illustrations_project_placeholder` | OK |
| user_preferences | PK implícito | OK |
| typography_configs | `idx_typography_project_id` | OK |
| generation_results | `idx_generation_results_project_id`, `idx_generation_results_created_at` | OK |
| annotations | `idx_annotations_project_page` | OK |
| bes_document_cache | `idx_bdc_project_type`, `idx_bdc_project` | OK |
| cover_configs | `idx_cover_configs_project` | OK |

---

## Warnings Identificados

### WARN-DB01 — M004-M007 sem INSERT INTO schema_version [WARNING]

- **Severidade:** P2 — Aviso (não-bloqueante)
- **Detalhe:** Migrations M001, M002 e M003 registram sua aplicação na tabela `schema_version`. As migrations M004 (`generation_results`), M005 (`annotations`), M006 (`bes_document_cache`) e M007 (`cover_configs`) **não inserem** registro em `schema_version`.
- **Impacto:** Impossível determinar programaticamente quais migrations foram aplicadas após M003. PRAGMA `user_version` ou contagem de tabelas pode ser usada como alternativa, mas é menos confiável.
- **Mitigação:** Adicionar `INSERT INTO schema_version (migration_name) VALUES ('M004_generation_results');` (e equivalentes) ao final de M004-M007.

### WARN-DB02 — M002 usa recreação de tabela (DDL migration workaround) [INFORMACIONAL]

- **Severidade:** P3 — Informacional
- **Detalhe:** SQLite não suporta `ALTER TABLE ... MODIFY COLUMN`. M002 recria `illustrations` via pattern CREATE NEW → INSERT SELECT → DROP OLD → RENAME. Padrão correto mas requer que dados existentes sejam preservados (INSERT INTO illustrations_new SELECT * FROM illustrations).
- **Status:** Aceitável e documentado.

### WARN-DB03 — M007 não tem INSERT INTO schema_version [incluído em WARN-DB01]

Já coberto acima.

---

## Gaps vs Spec

| Critério de aceite (TASK-4) | Status |
|-----------------------------|--------|
| 9/9 tabelas criadas com DDL correto | OK |
| 6/6 migrations aplicam sequencialmente sem erro | OK (7/7 na impl; extra M002 válido) |
| PRAGMA integrity_check → "ok" (inferido por análise estática) | OK (FKs e CHECKs corretos) |
| Foreign keys enforced (ON DELETE CASCADE) | OK (6/6) |
| Índices presentes para performance | OK |
| Seed data carregado | Não verificado (fora do escopo desta análise estática) |

**Spec referenciava "6 migrations V001-V006":** A implementação tem 7 migrations (M001-M007). O extra é M002 que adicionou o estado `error` ao enum de illustrations — enhancement válido e necessário (Rock-3, module-3). Não é um gap mas uma expansão documentada.

---

## Verdict

**STATUS: PASS**

**Justificativa:** Todas as 9 tabelas SQLite estão implementadas com DDL correto, tipos apropriados e CHECK constraints. As 6 Foreign Keys ON DELETE CASCADE garantem integridade referencial ao deletar projetos. Os índices cobrem os padrões de acesso esperados. A migration extra (M002) é um enhancement válido e não representa gap. O prefixo M vs V é puramente cosmético.

**Warnings não-bloqueantes:**
1. **WARN-DB01:** M004-M007 não registram em `schema_version` — dificulta rastreamento de migrations aplicadas (P2)
2. **WARN-DB02:** M002 usa recreação de tabela — padrão correto para SQLite, sem risco (P3)

**Critérios de aceite do TASK-4:**
- [x] 9/9 tabelas criadas
- [x] Migrations aplicáveis sequencialmente (7 migrations válidas)
- [x] Integridade referencial (6 FK CASCADE)
- [x] Índices presentes
- [ ] schema_version tracking completo — **WARNING** (M004-M007 não registram)
- [ ] Seed data — não verificado nesta auditoria estática

**Ação recomendada:** Adicionar `INSERT INTO schema_version` em M004-M007 antes de deploy para rastreabilidade completa. Não bloqueia aprovação desta TASK.
