# Anti-Hacking Fingerprint — BES Book Formatter
Data: 2026-03-22

## Stack

| Componente | Tecnologia | Versão |
|------------|------------|--------|
| Framework Desktop | Tauri | 2.x |
| Frontend | SvelteKit + Svelte 5 | ^2.9.0 |
| Backend | Rust (Tauri commands) | 2021 edition |
| Banco de Dados | SQLite via sqlx | 0.8.x |
| ORM | sqlx (raw queries parametrizadas) | 0.8.x |
| Deploy | GitHub Releases (binário desktop) | — |
| Auth | Nenhuma (app local single-user) | — |
| Pagamentos | Nenhum | — |

## Dependências Rust Relevantes

| Crate | Versão | Função |
|-------|--------|--------|
| tauri | 2 | Runtime desktop |
| tauri-plugin-sql | 2 | SQLite bridge |
| tauri-plugin-shell | 2 | Execução de sidecars |
| tauri-plugin-fs | 2 | Acesso ao filesystem |
| sqlx | 0.8 | Queries SQLite |
| comrak | 0.26 | Parser Markdown |
| syntect | 5 | Syntax highlighting |
| image | 0.25 | Processamento de imagens |
| zip | 2 | Arquivo ZIP |
| sha2 | 0.10 | Hashing |
| regex | 1 | Expressões regulares |

## Sidecars Externos

| Sidecar | Versão | Uso |
|---------|--------|-----|
| typst | bundled | Compilação PDF/Preview |
| ghostscript | bundled | PDF/X conversion |
| epubcheck (java) | bundled | Validação EPUB |
| pandoc | opcional | Exportação DOCX |

## Superfície de Ataque IPC (Tauri Commands)

**Total de comandos registrados:** ~50 IPC handlers

Categorias:
- System: `init_database`, `get_preferences`, `set_preference`, `check_sidecar`
- Projects: `get_projects`, `get_project`, `import_project`, `update_project`, `delete_project`
- Parser: `parse_manuscript`, `read_book_config`, `select_directory`, `write_bes_format`, `get_illustrations`, `calculate_completeness`, `run_content_checklist`
- Generation: `generate_epub`, `generate_html`, `generate_pdf`, `generate_pdf_print`, `generate_pdf_ebook`, `generate_docx`, `get_generation_results`, `run_epubcheck`, `run_preflight`, `cancel_generation`
- Preview: `render_preview`, `render_preview_page`, `get_page_count`, `navigate_to_page`, `set_zoom_level`, `toggle_spread_view`, `toggle_distraction_free`, `detect_orphans_widows`, `add_annotation`, `get_annotations`, `delete_annotation`
- Typography: `get_typography_config`, `set_typography_config`, `generate_toc`, `validate_illustration_dpi`, `process_illustration`, `list_illustrations`, `update_illustration_alt_text`
- Fonts: `select_font_file`, `list_fonts`, `upload_font`, `delete_custom_font`
- BES Integration: `validate_bes_workspace`, `read_bes_docs`, `get_bes_metadata`, `invalidate_bes_cache`, `sync_editorial_progress`, `update_editorial_f10`
- Cover: `get_cover_config`, `calculate_spine_width`, `save_cover_config`, `generate_cover_pdf`, `get_cover_templates`, `export_cover_image`, `render_mockup_3d`

## Capabilities Tauri (Permissões)

```
core:default, sql:default, dialog:default, store:default
shell:allow-open          → abre URLs no browser
shell:allow-execute       → executa sidecars (typst, gs, epubcheck) APENAS
fs:allow-read-file        → $APPDATA/** e $RESOURCE/**
fs:allow-write-file       → $APPDATA/** APENAS
fs:allow-read-dir         → $APPDATA/**
```

## CSP Configurada

```
default-src 'self'
script-src 'self'
style-src 'self' 'unsafe-inline'     ← NOTA: unsafe-inline para styles
img-src 'self' asset: tauri: data:
font-src 'self' asset:
connect-src 'self' ipc: tauri:
```

## Observações de Arquitetura

- **Single-user local**: sem rede, sem multi-tenant, sem autenticação
- **Fonte de dados primária**: arquivos do manuscrito (filesystem local controlado pelo usuário)
- **Ameaça principal**: arquivos de manuscrito/imagem maliciosos
- **Cargo.lock**: **NÃO COMMITADO** — impossível auditar dependências transitivas exatas
- **npm audit (devDeps)**: 19 vulnerabilidades (6 high) — todas em devDependencies
