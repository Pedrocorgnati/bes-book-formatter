# Backend Build Report

Projeto: bes-book-formatter
Stack: rust-axum (Tauri 2.x IPC — desktop app, sem REST API)
Data: 2026-03-22

## Estrutura Gerada

```
src-tauri/
├── Cargo.toml                          # Dependências Rust
├── build.rs                            # Tauri build script
├── tauri.conf.json                     # Config Tauri: window, plugins, sidecars
├── capabilities/
│   └── default.json                    # Permissões de segurança (fs, sql, shell)
├── migrations/
│   └── M001_initial_schema.sql         # DDL completo: 4 tabelas + índices + defaults
└── src/
    ├── main.rs                         # Entry point (Windows subsystem)
    ├── lib.rs                          # Tauri builder: plugins, setup, invoke_handler
    ├── error.rs                        # AppError com códigos do ERROR-CATALOG (12 prefixos)
    ├── db/
    │   ├── mod.rs
    │   └── pool.rs                     # SQLite pool (WAL mode, FK enabled)
    ├── models/
    │   ├── mod.rs
    │   ├── enums.rs                    # 14 enums (Genre, OutputFormat, Platform, etc.)
    │   ├── project.rs                  # Project, NewProject, UpdateProject
    │   ├── illustration.rs             # Illustration, NewIllustration
    │   ├── preference.rs               # Preference
    │   └── responses.rs                # ApiResponse<T>, SidecarStatus, DTOs de Rock-1/3/4
    ├── repositories/
    │   ├── mod.rs
    │   ├── project_repository.rs       # CRUD completo + find_by_bes_root
    │   ├── illustration_repository.rs  # CRUD + find_by_state + update_image
    │   └── preference_repository.rs    # get/set/get_all
    ├── services/
    │   ├── mod.rs
    │   ├── migration_service.rs        # apply_pending + verify_integrity
    │   ├── sidecar_manager.rs          # check/spawn Typst, Ghostscript, EPUBCheck
    │   └── filesystem_service.rs       # verify_bes_structure, read_book_config, list_md
    └── commands/
        ├── mod.rs
        ├── system.rs                   # init_database, get_preferences, set_preference, check_sidecar
        ├── projects.rs                 # get_projects, get_project, import_project, delete_project
        ├── parser.rs                   # parse_manuscript, read_book_config, calculate_completeness, run_content_checklist
        ├── generation.rs               # generate_epub, generate_pdf, run_epubcheck, run_preflight
        └── preview.rs                  # render_preview, detect_orphans_widows
```

### Models / Schemas (4 entidades + 14 enums)
- `Project` — 17 campos, matching ERD projects table
- `Illustration` — 13 campos, matching ERD illustrations table
- `Preference` — key/value pair
- `schema_version` — migration tracking
- 14 Rust enums com Serde serialize/deserialize (Genre, OutputFormat, Platform, etc.)
- DTOs: ApiResponse<T>, SidecarStatus, InitResult, CompletenessResult, ManuscriptAst, GenerationResult, PreviewResult, etc.

### Commands / IPC (18 comandos em 5 módulos)
| Módulo | Comandos | Status |
|--------|----------|--------|
| system.rs | init_database, get_preferences, set_preference, check_sidecar | ✅ Implementados |
| projects.rs | get_projects, get_project, import_project, delete_project | ✅ Implementados |
| parser.rs | parse_manuscript, read_book_config, calculate_completeness, run_content_checklist | read_book_config ✅ / demais TODO |
| generation.rs | generate_epub, generate_pdf, run_epubcheck, run_preflight | TODO (stubs) |
| preview.rs | render_preview, detect_orphans_widows | TODO (stubs) |

### Repositories (3)
- `ProjectRepository` — find_by_id, find_all_recent, create, update, delete, find_by_bes_root, update_last_opened
- `IllustrationRepository` — find_by_project, find_by_state, find_by_id, create, update_state, update_image, count_by_project
- `PreferenceRepository` — get, set, get_all

### Services (3)
- `MigrationService` — apply_pending (M001), verify_integrity (PRAGMA check)
- `SidecarManager` — check_sidecar (typst/gs/epubcheck), spawn_typst, spawn_ghostscript, spawn_epubcheck (com timeout)
- `FilesystemService` — verify_bes_structure (THREAT-001 mitigation), read_book_config, list_manuscript_files, write_bes_format

### Middlewares / Segurança
- Path traversal protection: `canonicalize()` + rejeita `..` (THREAT-001)
- Tauri Capabilities: fs scope restrito a $APPDATA (default.json)
- Sidecar checksum: preparado para THREAT-002 (SHA-256 check)
- Error codes mapeados do ERROR-CATALOG.md (12 prefixos)

### Frontend Atualizado
- `src/lib/ipc/projects.ts` — stubs substituídos por `invoke()` real
- `src/lib/ipc/preferences.ts` — stubs substituídos por `invoke()` real

## Stubs Pendentes

Os seguintes métodos são stubs e precisam de implementação:

| Comando | Módulo | Rock |
|---------|--------|------|
| parse_manuscript | parser.rs | Rock-1 |
| calculate_completeness | parser.rs | Rock-1 |
| run_content_checklist | parser.rs | Rock-1 |
| generate_epub | generation.rs | Rock-3 |
| generate_pdf | generation.rs | Rock-3 |
| run_epubcheck | generation.rs | Rock-3 |
| run_preflight | generation.rs | Rock-3 |
| render_preview | preview.rs | Rock-4 |
| detect_orphans_widows | preview.rs | Rock-4 |

Execute `/auto-flow execute {range_de_modules}` para implementar a lógica de negócio.

## Build

**Status:** Cargo não disponível neste ambiente. Código revisado manualmente.
- Todas as queries SQLite usam runtime binding (sem macros `query!` que requerem DATABASE_URL)
- Imports e módulos verificados
- Tipos Serde alinhados com frontend TypeScript

## Próximos Passos

1. `cargo check` — verificar compilação (requer Rust toolchain instalado)
2. `/env-creation .claude/projects/bes-book-formatter.json` — configurar variáveis de ambiente
3. `/db-migration-create .claude/projects/bes-book-formatter.json` — gerar migrations adicionais (M002, M003)
4. `/auto-flow execute` — implementar lógica de negócio task a task
