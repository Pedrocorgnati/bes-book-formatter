# DB Migration Report

**Projeto:** bes-book-formatter
**ORM/Ferramenta:** SQLx 0.8 (Rust) + MigrationService customizado
**Database:** SQLite (WAL mode, `foreign_keys = ON`)
**Path do banco:** `{app_local_data_dir}/bes-book-formatter.db`
**Data:** 2026-03-22
**Data-Integrity-Decision:** não disponível
**ERROR-CATALOG:** utilizado — enums validados via CHECK constraints (SQLite não suporta tipos nativos)

---

## Migrations Registradas

| # | Arquivo | Operação | Tabelas Afetadas | Tipo | Reversível |
|---|---------|----------|-----------------|------|------------|
| 1 | `M001_initial_schema.sql` | CREATE TABLE (4x) + INSERT defaults | `projects`, `illustrations`, `user_preferences`, `schema_version` | additive | Sim |

---

## Status do Delta

**Delta detectado:** ZERO — nenhuma migration adicional necessária.

### Análise: M002 e M003 são obsoletas

O LLD (§2.3) planeja três migrations:

| ID | Nome | Campos planejados | Status |
|----|------|-------------------|--------|
| M001 | initial_schema | Tabelas base | ✅ Aplicada |
| M002 | add_rock1_fields | `completeness_score`, `completeness_level`, `chapter_count`, `illustration_count`, `manuscript_root` em `projects` | ✅ Supersedida — campos já presentes em M001 |
| M003 | add_rock2_fields | `width_px`, `height_px`, `color_space` em `illustrations` | ✅ Supersedida — campos já presentes em M001 |

**M001 foi gerada como migration "fat"** — inclui proativamente todos os campos dos rocks 1, 2 e 3. Em um banco novo, isso é correto e mais eficiente do que três migrations em sequência. M002 e M003 **não devem ser geradas** pois `ALTER TABLE ADD COLUMN` falharia com "duplicate column name" no SQLite.

---

## Cobertura do ERD (LLD §2)

### Tabela: `projects` (16 campos)

| Campo | Tipo | Constraints | Presente em M001 |
|-------|------|-------------|-----------------|
| id | TEXT | PK, NOT NULL | ✅ |
| name | TEXT | NOT NULL | ✅ |
| bes_root_path | TEXT | NOT NULL, UNIQUE | ✅ |
| book_config_path | TEXT | nullable | ✅ |
| genre | TEXT | nullable | ✅ |
| language | TEXT | DEFAULT 'pt-BR' | ✅ |
| config_version | TEXT | nullable | ✅ |
| last_opened | DATETIME | nullable | ✅ |
| format_file_path | TEXT | nullable | ✅ |
| created_at | DATETIME | NOT NULL, DEFAULT | ✅ |
| updated_at | DATETIME | NOT NULL, DEFAULT | ✅ |
| completeness_score | REAL | nullable, CHECK(0.0–1.0) | ✅ |
| completeness_level | TEXT | nullable, CHECK enum | ✅ |
| chapter_count | INTEGER | nullable | ✅ |
| illustration_count | INTEGER | nullable | ✅ |
| manuscript_root | TEXT | nullable | ✅ |
| output_dir | TEXT | nullable | ✅ |

### Tabela: `illustrations` (13 campos)

| Campo | Tipo | Constraints | Presente em M001 |
|-------|------|-------------|-----------------|
| id | TEXT | PK, NOT NULL | ✅ |
| project_id | TEXT | NOT NULL, FK ON DELETE CASCADE | ✅ |
| placeholder_name | TEXT | NOT NULL | ✅ |
| description | TEXT | nullable | ✅ |
| state | TEXT | NOT NULL, DEFAULT 'pending', CHECK enum | ✅ |
| image_path | TEXT | nullable | ✅ |
| validated_dpi | INTEGER | nullable | ✅ |
| alt_text | TEXT | nullable | ✅ |
| width_px | INTEGER | nullable | ✅ |
| height_px | INTEGER | nullable | ✅ |
| color_space | TEXT | nullable, CHECK('srgb'\|'cmyk') | ✅ |
| created_at | DATETIME | NOT NULL, DEFAULT | ✅ |
| updated_at | DATETIME | NOT NULL, DEFAULT | ✅ |

### Tabela: `user_preferences` (3 campos)

| Campo | Tipo | Constraints | Presente em M001 |
|-------|------|-------------|-----------------|
| key | TEXT | PK, NOT NULL | ✅ |
| value | TEXT | NOT NULL | ✅ |
| updated_at | DATETIME | NOT NULL, DEFAULT | ✅ |

Seed padrão inserido via `INSERT OR IGNORE`: `theme='light'`, `ui_language='pt-BR'`.

### Tabela: `schema_version` (3 campos)

| Campo | Tipo | Constraints | Presente em M001 |
|-------|------|-------------|-----------------|
| version | INTEGER | PK, AUTOINCREMENT | ✅ |
| migration_name | TEXT | NOT NULL | ✅ |
| applied_at | DATETIME | NOT NULL, DEFAULT | ✅ |

---

## Índices e Integridade Referencial

| Índice | Tabela | Campos | Tipo | Presente |
|--------|--------|--------|------|---------|
| `idx_projects_last_opened` | projects | `last_opened DESC` | Simples | ✅ |
| `idx_projects_genre` | projects | `genre` | Simples | ✅ |
| `idx_illustrations_project_id` | illustrations | `project_id` | Simples | ✅ |
| `idx_illustrations_project_state` | illustrations | `project_id, state` | Composto | ✅ |
| `uq_illustrations_project_placeholder` | illustrations | `project_id, placeholder_name` | UNIQUE | ✅ |

**Foreign Keys:** `illustrations.project_id → projects.id ON DELETE CASCADE` ✅

---

## Checklist de Segurança — M001

| Item | Status | Observação |
|------|--------|-----------|
| Tem rollback documentado? | ✅ | Ver seção Rollback abaixo |
| Usa IF NOT EXISTS? | ✅ | Todas as CREATE TABLE usam IF NOT EXISTS |
| Colunas NOT NULL com DEFAULT? | ✅ | Apenas `id`, `name` e paths não têm DEFAULT (obrigatórios no insert) |
| DROP de coluna sem backup? | N/A | Nenhum DROP nesta migration |
| FKs com ON DELETE explícito? | ✅ | `ON DELETE CASCADE` em `illustrations.project_id` |
| Índices criados para FKs? | ✅ | `idx_illustrations_project_id` cobre a FK |
| Ordem de criação respeita dependências? | ✅ | `projects` criada antes de `illustrations` |
| Dados sensíveis com tipo correto? | ✅ | Nenhum dado sensível — apenas paths de filesystem |

**Resultado:** 8/8 itens ok ✅

---

## Infraestrutura de Migration (MigrationService)

O `MigrationService` em `src-tauri/src/services/migration_service.rs` gerencia as migrations:

- **Registro:** `MIGRATIONS` array com `version`, `name` e SQL embutido via `include_str!`
- **Controle:** versão atual via `MAX(version)` na tabela `schema_version`
- **Execução:** split por `;`, statements executados individualmente com rollback em erro
- **Verificação:** `PRAGMA integrity_check` após aplicação

**Ação necessária:** Nenhuma. O MIGRATIONS array já referencia M001 corretamente.

**Para adicionar migrations futuras:**

```rust
// Em src-tauri/src/services/migration_service.rs
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "M001_initial_schema",
        sql: include_str!("../../migrations/M001_initial_schema.sql"),
    },
    // Adicionar próximas migrations aqui em ordem crescente de version
    // Migration {
    //     version: 2,
    //     name: "M002_nome_descritivo",
    //     sql: include_str!("../../migrations/M002_nome_descritivo.sql"),
    // },
];
```

---

## Rollback de M001 (se necessário)

Como o banco é SQLite local (`app_local_data_dir`), o rollback consiste em deletar o arquivo de banco:

```bash
# macOS
rm ~/Library/Application\ Support/com.bes.book-formatter/bes-book-formatter.db

# Windows
del %APPDATA%\com.bes.book-formatter\bes-book-formatter.db

# Linux
rm ~/.local/share/com.bes.book-formatter/bes-book-formatter.db
```

O banco é recriado automaticamente na próxima inicialização do app com `MigrationService.apply_pending()`.

**Projetos importados serão perdidos** (os arquivos BES no filesystem permanecem intactos — apenas os metadados SQLite são perdidos). O usuário pode reimportar os projetos via `import_project` command.

---

## Ordem de Execução (para novos bancos)

Para instalação limpa, M001 é aplicada automaticamente pelo `MigrationService` no boot do app. Nenhuma ação manual é necessária.

```
App init → create_pool() → MigrationService::new() → apply_pending() → M001 aplicada → banco pronto
```

---

## Checklist Pré-Deploy

- [x] M001 revisada e validada contra LLD §2
- [x] Todos os campos do ERD cobertos
- [x] CHECK constraints validam enums (IllustrationState, ManuscriptCompleteness, ColorSpace)
- [x] Índices criados para todas as FKs e queries frequentes
- [x] MigrationService registrado no MIGRATIONS array
- [x] WAL mode habilitado para concorrência leitura/escrita
- [x] foreign_keys=ON habilitado no pool
- [x] INSERT OR IGNORE para seeds idempotentes
- [ ] Testar em banco novo em staging (primeira execução)
- [ ] Testar `PRAGMA integrity_check` após migration
- [ ] Confirmar que `schema_version` registra `version=1` após boot

---

## Próximos Passos Sugeridos

1. `/seed-data-create .claude/projects/bes-book-formatter.json` — popular banco com dados de teste
2. `/integration-test-create .claude/projects/bes-book-formatter.json` — testar repositories contra banco real
