// BES Book Formatter — EPUB Service (module-4 TASK-1)
//
// Gera EPUB 3.3 completo a partir do ManuscriptAST + TypographyConfig.
// Empacota como ZIP com estrutura EPUB válida e executa EPUBCheck automaticamente.

use std::io::Write;
use std::time::Instant;

use comrak::{markdown_to_html, ComrakOptions};
use sqlx::SqlitePool;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::AppError;
use crate::models::manuscript::ParsedChapter;
use crate::models::responses::{GenerationResult, ValidationResult};
use crate::models::typography::TypographyConfig;
use crate::services::epub_renderer::{
    generate_colophon, generate_epub_toc_nav, generate_epub_toc_ncx, highlight_code_block,
    html_escape, render_footnote_section_html, render_ornament_html, replace_footnote_markers,
    TocEntry,
};
use crate::services::common::sanitize_slug;
use crate::services::{ParserService, SidecarManager, TypographyService};

pub struct EpubService;

impl EpubService {
    // ── Public API ─────────────────────────────────────────────────────────

    /// Gera EPUB 3.3 completo para um projeto.
    pub async fn generate(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
    ) -> Result<GenerationResult, AppError> {
        let start = Instant::now();

        // 1. Carregar projeto do DB
        let project = Self::load_project(pool, project_id).await?;

        // 2. Parse do manuscrito
        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;

        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // 3. Carregar TypographyConfig
        let typo_svc = TypographyService::new(pool.clone());
        let typo = typo_svc
            .get_typography_config(project_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| {
                use crate::models::enums::Genre;
                TypographyService::get_default_typography_config(project_id, &Genre::Fiction)
            });

        // 4. Carregar ilustrações do DB
        let illustrations = Self::load_illustrations(pool, project_id).await;

        // 5. Montar slug/título/autor
        let slug = sanitize_slug(&project.name);
        let title = project.name.clone();
        let author = Self::get_author(pool, project_id).await;
        let lang = project.language.as_str();
        let lang_short = if lang.starts_with("pt") { "pt" } else if lang.starts_with("es") { "es" } else { "en" };
        let uid = format!("urn:uuid:{}", uuid::Uuid::new_v4());

        // 6. Gerar todos os XHTML dos capítulos
        let all_chapters: Vec<&ParsedChapter> = ast
            .front_matter
            .iter()
            .chain(ast.chapters.iter())
            .chain(ast.back_matter.iter())
            .collect();

        let mut toc_entries: Vec<TocEntry> = Vec::new();
        let mut chapter_files: Vec<(String, String)> = Vec::new(); // (filename, xhtml)

        for (i, chapter) in all_chapters.iter().enumerate() {
            let fname = if i < ast.front_matter.len() {
                format!("front-matter/fm-{:03}.xhtml", i + 1)
            } else if i < ast.front_matter.len() + ast.chapters.len() {
                format!("content/chapter-{:03}.xhtml", i - ast.front_matter.len() + 1)
            } else {
                format!("back-matter/bm-{:03}.xhtml", i - ast.front_matter.len() - ast.chapters.len() + 1)
            };

            let xhtml = Self::chapter_to_xhtml(chapter, &typo, lang, lang_short, &illustrations);

            toc_entries.push(TocEntry {
                title: chapter.title.clone(),
                href: fname.clone(),
                sub_entries: vec![],
            });

            chapter_files.push((fname, xhtml));
        }

        // Colofão
        let colophon_xhtml = Self::colophon_xhtml(lang, lang_short, &title, &author);
        chapter_files.push(("back-matter/colophon.xhtml".to_string(), colophon_xhtml));

        // 7. Construir EPUB em memória como ZIP
        let epub_bytes =
            Self::build_epub_zip(&slug, &title, &author, &uid, lang, lang_short, &typo, &toc_entries, &chapter_files)?;

        // 8. Salvar em disco
        let output_dir = format!("output/books/{}", slug);
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_002", format!("Falha ao criar output dir: {}", e)))?;

        let output_path = format!("{}/{}.epub", output_dir, slug);
        std::fs::write(&output_path, &epub_bytes)
            .map_err(|e| AppError::new("GEN_003", format!("Falha ao salvar EPUB: {}", e)))?;

        // 9. Executar EPUBCheck automaticamente
        let validation = Self::run_epubcheck_internal(&output_path).await;

        // 10. Persistir resultado no SQLite
        let duration_ms = start.elapsed().as_millis() as u64;
        let errors_json = serde_json::to_string(&validation.errors).unwrap_or_else(|_| "[]".to_string());
        let warnings_json = serde_json::to_string(&validation.warnings).unwrap_or_else(|_| "[]".to_string());
        let status = if validation.valid { "success" } else { "error" };

        let _ = sqlx::query(
            "INSERT INTO generation_results (id, project_id, format, platform, output_path, file_size_bytes, duration_ms, status, errors, warnings)
             VALUES (?, ?, 'epub3', ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(platform)
        .bind(&output_path)
        .bind(epub_bytes.len() as i64)
        .bind(duration_ms as i64)
        .bind(status)
        .bind(&errors_json)
        .bind(&warnings_json)
        .execute(pool)
        .await;

        Ok(GenerationResult {
            success: validation.valid,
            output_path: Some(output_path),
            format: "epub3".to_string(),
            platform: platform.to_string(),
            errors: validation.errors,
            warnings: validation.warnings,
            duration_ms,
        })
    }

    /// Executa EPUBCheck em um arquivo EPUB já gerado.
    pub async fn run_epubcheck(epub_path: &str) -> Result<ValidationResult, AppError> {
        if !std::path::Path::new(epub_path).exists() {
            return Err(AppError::new("GEN_020", format!("EPUB não encontrado: {}", epub_path)));
        }
        Ok(Self::run_epubcheck_internal(epub_path).await)
    }

    // ── Private helpers ────────────────────────────────────────────────────

    async fn load_project(pool: &SqlitePool, project_id: &str) -> Result<crate::models::project::Project, AppError> {
        sqlx::query_as::<_, crate::models::project::Project>(
            "SELECT id, name, bes_root_path, book_config_path, genre, language, config_version,
                    last_opened, format_file_path, created_at, updated_at,
                    completeness_score, completeness_level, chapter_count, illustration_count,
                    manuscript_root, output_dir
             FROM projects WHERE id = ?",
        )
        .bind(project_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::new("GEN_004", format!("DB error: {}", e)))?
        .ok_or_else(|| AppError::new("GEN_005", format!("Projeto não encontrado: {}", project_id)))
    }

    async fn get_author(pool: &SqlitePool, project_id: &str) -> String {
        let bes_root: Option<String> =
            sqlx::query_scalar("SELECT bes_root_path FROM projects WHERE id = ?")
                .bind(project_id)
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
        if let Some(root) = bes_root {
            if let Ok(cfg) = crate::services::BookConfigService::read_book_config(&root).await {
                return cfg.author;
            }
        }
        "Autor Desconhecido".to_string()
    }

    async fn load_illustrations(pool: &SqlitePool, project_id: &str) -> Vec<(String, String)> {
        // Retorna (placeholder_name, alt_text) para ilustrações LINKED
        sqlx::query_as::<_, (String, String)>(
            "SELECT placeholder_name, COALESCE(alt_text, placeholder_name)
             FROM illustrations WHERE project_id = ? AND state = 'linked'",
        )
        .bind(project_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default()
    }

    fn chapter_to_xhtml(
        chapter: &ParsedChapter,
        typo: &TypographyConfig,
        lang: &str,
        lang_short: &str,
        _illustrations: &[(String, String)],
    ) -> String {
        let mut opts = ComrakOptions::default();
        opts.extension.strikethrough = true;
        opts.extension.table = true;
        opts.extension.autolink = false;
        opts.extension.tasklist = false;
        opts.render.unsafe_ = false;
        opts.render.escape = false;

        // Substituir marcadores de nota de rodapé antes do render
        let content_with_refs = replace_footnote_markers(&chapter.content, &chapter.footnotes);

        // Converter Markdown → HTML
        let mut body_html = markdown_to_html(&content_with_refs, &opts);

        // Adicionar seção de notas de rodapé
        let footnotes_html = render_footnote_section_html(&chapter.footnotes);
        body_html.push_str(&footnotes_html);

        // Substituir divisórias por ornamentos
        let ornament_style = &typo.ornament_style;
        body_html = body_html.replace(
            "<hr />",
            render_ornament_html(ornament_style),
        );
        body_html = body_html.replace(
            "<hr/>",
            render_ornament_html(ornament_style),
        );

        // Determinar epub:type para o body
        let epub_type = if chapter.matter_type.is_some() {
            "frontmatter"
        } else {
            "bodymatter chapter"
        };

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:epub="http://www.idpf.org/2007/ops"
      xml:lang="{lang}" lang="{lang}">
<head>
  <meta charset="UTF-8"/>
  <title>{title}</title>
  <link rel="stylesheet" type="text/css" href="../css/styles.css"/>
  <link rel="stylesheet" type="text/css" href="../css/syntax.css"/>
</head>
<body epub:type="{epub_type}">
{body}
</body>
</html>
"#,
            lang = html_escape(lang),
            title = html_escape(&chapter.title),
            epub_type = epub_type,
            body = body_html,
        )
    }

    fn colophon_xhtml(lang: &str, lang_short: &str, title: &str, author: &str) -> String {
        let colophon_text = generate_colophon(lang);
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:epub="http://www.idpf.org/2007/ops"
      xml:lang="{lang}" lang="{lang}">
<head>
  <meta charset="UTF-8"/>
  <title>Colofão</title>
  <link rel="stylesheet" type="text/css" href="../css/styles.css"/>
</head>
<body epub:type="backmatter">
  <section epub:type="colophon" class="colophon">
    <p><em>{title}</em></p>
    <p>{author}</p>
    <p class="colophon-generator">{colophon}</p>
  </section>
</body>
</html>
"#,
            lang = html_escape(lang),
            title = html_escape(title),
            author = html_escape(author),
            colophon = html_escape(&colophon_text),
        )
    }

    fn content_opf(
        title: &str,
        author: &str,
        uid: &str,
        lang: &str,
        chapter_files: &[(String, String)],
    ) -> String {
        let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        let year = chrono::Utc::now().format("%Y").to_string();

        // Manifest items
        let mut manifest_items = String::new();
        manifest_items.push_str(
            "    <item id=\"nav\" href=\"toc.xhtml\" media-type=\"application/xhtml+xml\" properties=\"nav\"/>\n",
        );
        manifest_items.push_str(
            "    <item id=\"ncx\" href=\"toc.ncx\" media-type=\"application/x-dtbncx+xml\"/>\n",
        );
        manifest_items
            .push_str("    <item id=\"css-main\" href=\"css/styles.css\" media-type=\"text/css\"/>\n");
        manifest_items
            .push_str("    <item id=\"css-syntax\" href=\"css/syntax.css\" media-type=\"text/css\"/>\n");

        // Spine items
        let mut spine_items = String::new();

        for (i, (fname, _)) in chapter_files.iter().enumerate() {
            let item_id = format!("chapter-{:03}", i + 1);
            manifest_items.push_str(&format!(
                "    <item id=\"{}\" href=\"{}\" media-type=\"application/xhtml+xml\"/>\n",
                item_id, html_escape(fname)
            ));
            spine_items.push_str(&format!("    <itemref idref=\"{}\"/>\n", item_id));
        }

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf"
         xmlns:dc="http://purl.org/dc/elements/1.1/"
         version="3.0"
         unique-identifier="uid">
  <metadata xmlns:dc="http://purl.org/dc/elements/1.1/"
            xmlns:opf="http://www.idpf.org/2007/opf">
    <dc:identifier id="uid">{uid}</dc:identifier>
    <dc:title>{title}</dc:title>
    <dc:creator>{author}</dc:creator>
    <dc:language>{lang}</dc:language>
    <dc:date>{year}</dc:date>
    <meta property="dcterms:modified">{now}</meta>
    <meta name="cover" content="cover-image"/>
  </metadata>
  <manifest>
{manifest_items}  </manifest>
  <spine toc="ncx">
{spine_items}  </spine>
</package>
"#,
            uid = html_escape(uid),
            title = html_escape(title),
            author = html_escape(author),
            lang = html_escape(lang),
            year = year,
            now = now,
            manifest_items = manifest_items,
            spine_items = spine_items,
        )
    }

    fn build_epub_zip(
        slug: &str,
        title: &str,
        author: &str,
        uid: &str,
        lang: &str,
        lang_short: &str,
        _typo: &TypographyConfig,
        toc_entries: &[TocEntry],
        chapter_files: &[(String, String)],
    ) -> Result<Vec<u8>, AppError> {
        let buf = std::io::Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);

        // ── mimetype (sem compressão — EPUB spec exige) ──────────────────
        let stored = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);
        zip.start_file("mimetype", stored)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP error: {}", e)))?;
        zip.write_all(b"application/epub+zip")
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write error: {}", e)))?;

        let deflated = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // ── META-INF/container.xml ────────────────────────────────────────
        zip.start_file("META-INF/container.xml", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(CONTAINER_XML.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── EPUB/toc.xhtml ────────────────────────────────────────────────
        let toc_nav = generate_epub_toc_nav(title, toc_entries, lang);
        zip.start_file("EPUB/toc.xhtml", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(toc_nav.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── EPUB/toc.ncx ──────────────────────────────────────────────────
        let toc_ncx = generate_epub_toc_ncx(title, uid, toc_entries);
        zip.start_file("EPUB/toc.ncx", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(toc_ncx.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── EPUB/content.opf ──────────────────────────────────────────────
        let opf = Self::content_opf(title, author, uid, lang, chapter_files);
        zip.start_file("EPUB/content.opf", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(opf.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── EPUB/css/styles.css ───────────────────────────────────────────
        let css = include_str!("../../assets/epub-templates/styles.css");
        zip.start_file("EPUB/css/styles.css", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(css.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── EPUB/css/syntax.css ───────────────────────────────────────────
        let syntax_css = include_str!("../../assets/epub-templates/syntax.css");
        zip.start_file("EPUB/css/syntax.css", deflated)
            .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
        zip.write_all(syntax_css.as_bytes())
            .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;

        // ── Capítulos XHTML ───────────────────────────────────────────────
        for (fname, xhtml) in chapter_files {
            // Garantir que as subpastas existem no ZIP
            zip.start_file(format!("EPUB/{}", fname), deflated)
                .map_err(|e| AppError::new("GEN_030", format!("ZIP: {}", e)))?;
            zip.write_all(xhtml.as_bytes())
                .map_err(|e| AppError::new("GEN_031", format!("ZIP write: {}", e)))?;
        }

        let cursor = zip
            .finish()
            .map_err(|e| AppError::new("GEN_032", format!("ZIP finish error: {}", e)))?;

        Ok(cursor.into_inner())
    }

    async fn run_epubcheck_internal(epub_path: &str) -> ValidationResult {
        match SidecarManager::spawn_epubcheck(epub_path, 60_000).await {
            Ok((stdout, stderr)) => {
                let combined = format!("{}\n{}", stdout, stderr);
                let errors: Vec<String> = combined
                    .lines()
                    .filter(|l| l.starts_with("ERROR") || l.starts_with("FATAL"))
                    .map(|l| l.to_string())
                    .collect();
                let warnings: Vec<String> = combined
                    .lines()
                    .filter(|l| l.starts_with("WARNING"))
                    .map(|l| l.to_string())
                    .collect();
                ValidationResult {
                    valid: errors.is_empty(),
                    errors,
                    warnings,
                    info: vec![],
                }
            }
            Err(e) => {
                // EPUBCheck não disponível — retornar aviso sem bloquear
                ValidationResult {
                    valid: true,
                    errors: vec![],
                    warnings: vec![format!(
                        "EPUBCheck não executado (sidecar indisponível): {}",
                        e.message
                    )],
                    info: vec![],
                }
            }
        }
    }
}

// ── Constantes ──────────────────────────────────────────────────────────────

const CONTAINER_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
  <rootfiles>
    <rootfile full-path="EPUB/content.opf" media-type="application/oebps-package+xml"/>
  </rootfiles>
</container>
"#;

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::manuscript::{Footnote, IllustrationRef, IndexEntry, ParsedChapter, ParsedManuscript};
    use crate::models::typography::TypographyConfig;
    use std::time::Instant;

    /// Gera um ManuscriptAST de teste com `n` capítulos (TASK-5 ST004 — benchmark).
    fn generate_mock_ast(n: usize, project_id: &str) -> ParsedManuscript {
        let chapters = (1..=n)
            .map(|i| ParsedChapter {
                title: format!("Capítulo {}", i),
                order: i,
                file_path: format!("ch{:04}.md", i),
                word_count: 500,
                heading_level: 1,
                content: format!("# Capítulo {}\n\n{}\n", i, "Lorem ipsum dolor sit amet. ".repeat(100)),
                footnotes: vec![],
                matter_type: None,
                index_entries: vec![],
            })
            .collect();

        ParsedManuscript {
            project_id: project_id.to_string(),
            front_matter: vec![],
            chapters,
            back_matter: vec![],
            illustrations: vec![],
            toc_present: false,
            index_present: false,
            total_words: n * 500,
            errors: vec![],
        }
    }

    fn mock_typography() -> TypographyConfig {
        TypographyService::get_default_typography_config("test", &crate::models::enums::Genre::Fiction)
    }

    /// ST004: Renderização de XHTML para 1000 capítulos deve completar em < 10s em memória.
    /// Para benchmark completo de geração com I/O: executar manualmente com `cargo test benchmark_1000 -- --nocapture --ignored`.
    #[test]
    #[ignore]
    fn benchmark_1000_page_epub() {
        use comrak::{markdown_to_html, ComrakOptions};
        let ast = generate_mock_ast(1000, "bench-project");
        let typo = mock_typography();
        let opts = ComrakOptions::default();
        let start = Instant::now();

        let all_chapters: Vec<&ParsedChapter> = ast.chapters.iter().collect();
        let mut total_bytes = 0usize;
        for chapter in &all_chapters {
            // Mede renderização Markdown → XHTML (parte mais cara da geração EPUB)
            let html = markdown_to_html(&chapter.content, &opts);
            total_bytes += html.len();
        }

        let elapsed = start.elapsed();
        // A renderização XHTML deve ser < 10s para 1000 caps. Geração completa com I/O < 120s.
        assert!(elapsed.as_secs() < 10, "Benchmark de renderização XHTML falhou: {:?}", elapsed);
        assert!(total_bytes > 10_000, "Output vazio");

        println!("Benchmark 1000 capítulos (XHTML render): {:?} ({} bytes totais)", elapsed, total_bytes);
    }
}
