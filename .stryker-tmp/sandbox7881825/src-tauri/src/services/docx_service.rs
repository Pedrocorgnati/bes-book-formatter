// BES Book Formatter — DOCX Service (module-4 TASK-4 ST001)
//
// Gera DOCX com estilos Word nomeados via ZIP + raw OOXML.
// Fallback opcional: pandoc como sidecar se disponível.

use std::io::Write;
use std::time::Instant;

use sqlx::SqlitePool;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use crate::error::AppError;
use crate::models::enums::Genre;
use crate::models::responses::GenerationResult;
use crate::services::common::sanitize_slug;
use crate::services::{ParserService, SidecarManager, TypographyService};

pub struct DocxService;

impl DocxService {
    /// Gera DOCX com estilos Word nomeados a partir do ManuscriptAST.
    pub async fn generate(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
    ) -> Result<GenerationResult, AppError> {
        let start = Instant::now();

        // Tentar pandoc primeiro (qualidade superior)
        let pandoc_status = SidecarManager::check_sidecar("pandoc").await;
        if pandoc_status.available {
            return Self::generate_via_pandoc(pool, project_id, platform, start).await;
        }

        // Fallback: DOCX via OOXML raw
        Self::generate_via_ooxml(pool, project_id, platform, start).await
    }

    // ── Pandoc pathway ─────────────────────────────────────────────────────

    async fn generate_via_pandoc(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
        start: Instant,
    ) -> Result<GenerationResult, AppError> {
        let project = Self::load_project(pool, project_id).await?;
        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;

        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // Consolida todo o conteúdo em um único Markdown
        let mut combined_md = format!("# {}\n\n", project.name);
        for chapter in &ast.chapters {
            combined_md.push_str(&format!("# {}\n\n{}\n\n", chapter.title, chapter.content));
        }

        let tmp_dir = std::env::temp_dir().join(format!("bes-docx-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp_dir)
            .map_err(|e| AppError::new("GEN_060", format!("Falha tmp dir: {}", e)))?;

        let md_path = tmp_dir.join("manuscript.md");
        std::fs::write(&md_path, &combined_md)
            .map_err(|e| AppError::new("GEN_061", format!("Falha ao escrever MD: {}", e)))?;

        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_002", format!("Falha output dir: {}", e)))?;

        let output_path = format!("{}/{}.docx", output_dir, sanitize_slug(&project.name));

        // SEC-009: argumentos como array
        let args = vec![
            md_path.to_string_lossy().to_string(),
            "-o".to_string(),
            output_path.clone(),
            "--reference-doc=reference.docx".to_string(), // template com estilos Word
        ];

        let mut warnings: Vec<String> = Vec::new();
        let mut errors: Vec<String> = Vec::new();

        match SidecarManager::spawn_pandoc(&args, 60_000).await {
            Ok(_) => {}
            Err(e) => {
                errors.push(format!("Pandoc falhou: {}", e.message));
            }
        }

        let _ = std::fs::remove_dir_all(&tmp_dir);
        let duration_ms = start.elapsed().as_millis() as u64;

        Self::save_result(pool, project_id, platform, &output_path, duration_ms, &errors, &warnings).await;

        Ok(GenerationResult {
            success: errors.is_empty(),
            output_path: Some(output_path),
            format: "docx".to_string(),
            platform: platform.to_string(),
            errors,
            warnings,
            duration_ms,
        })
    }

    // ── OOXML raw pathway ──────────────────────────────────────────────────

    async fn generate_via_ooxml(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
        start: Instant,
    ) -> Result<GenerationResult, AppError> {
        let project = Self::load_project(pool, project_id).await?;
        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;

        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // Gerar document.xml com estilos Word nomeados
        let document_xml = Self::build_document_xml(&project.name, &ast);

        // Empacotar como DOCX (ZIP)
        let docx_bytes = Self::build_docx_zip(&document_xml)?;

        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_002", format!("Falha output dir: {}", e)))?;

        let output_path = format!("{}/{}.docx", output_dir, sanitize_slug(&project.name));
        std::fs::write(&output_path, &docx_bytes)
            .map_err(|e| AppError::new("GEN_063", format!("Falha ao salvar DOCX: {}", e)))?;

        let duration_ms = start.elapsed().as_millis() as u64;
        let warnings: Vec<String> = vec!["DOCX gerado via OOXML raw (pandoc não disponível)".to_string()];
        let errors: Vec<String> = Vec::new();

        Self::save_result(pool, project_id, platform, &output_path, duration_ms, &errors, &warnings).await;

        Ok(GenerationResult {
            success: true,
            output_path: Some(output_path),
            format: "docx".to_string(),
            platform: platform.to_string(),
            errors,
            warnings,
            duration_ms,
        })
    }

    fn build_document_xml(
        title: &str,
        ast: &crate::models::manuscript::ParsedManuscript,
    ) -> String {
        let mut body = String::new();

        // Título
        body.push_str(&Self::para_xml(title, "Title"));

        for chapter in &ast.chapters {
            body.push_str(&Self::para_xml(&chapter.title, "Heading1"));
            for line in chapter.content.lines() {
                let line = line.trim();
                if line.starts_with("## ") {
                    body.push_str(&Self::para_xml(&line[3..], "Heading2"));
                } else if line.starts_with("### ") {
                    body.push_str(&Self::para_xml(&line[4..], "Heading3"));
                } else if line.starts_with("> ") {
                    body.push_str(&Self::para_xml(&line[2..], "Quote"));
                } else if line.starts_with("    ") || line.starts_with('\t') {
                    body.push_str(&Self::para_xml(line.trim(), "CodeBlock"));
                } else if !line.is_empty() {
                    body.push_str(&Self::para_xml(line, "Normal"));
                }
            }
        }

        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:wpc="http://schemas.microsoft.com/office/word/2010/wordprocessingCanvas"
            xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
            xmlns:o="urn:schemas-microsoft-com:office:office"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
            xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"
            xmlns:v="urn:schemas-microsoft-com:vml"
            xmlns:wp14="http://schemas.microsoft.com/office/word/2010/wordprocessingDrawing"
            xmlns:wp="http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing"
            xmlns:w10="urn:schemas-microsoft-com:office:word"
            xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
            xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
            xmlns:wpg="http://schemas.microsoft.com/office/word/2010/wordprocessingGroup"
            xmlns:wpi="http://schemas.microsoft.com/office/word/2010/wordprocessingInk"
            xmlns:wne="http://schemas.microsoft.com/office/word/2006/wordml"
            xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape"
            mc:Ignorable="w14 wp14">
  <w:body>
{}  </w:body>
</w:document>"#,
            body
        )
    }

    fn para_xml(text: &str, style: &str) -> String {
        let escaped = text
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;");

        format!(
            "    <w:p>\n      <w:pPr><w:pStyle w:val=\"{}\"/></w:pPr>\n      <w:r><w:t xml:space=\"preserve\">{}</w:t></w:r>\n    </w:p>\n",
            style, escaped
        )
    }

    fn build_docx_zip(document_xml: &str) -> Result<Vec<u8>, AppError> {
        let buf = std::io::Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(buf);
        let deflated = SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // [Content_Types].xml
        zip.start_file("[Content_Types].xml", deflated)
            .map_err(|e| AppError::new("GEN_070", format!("ZIP: {}", e)))?;
        zip.write_all(CONTENT_TYPES.as_bytes())
            .map_err(|e| AppError::new("GEN_071", format!("ZIP write: {}", e)))?;

        // _rels/.rels
        zip.start_file("_rels/.rels", deflated)
            .map_err(|e| AppError::new("GEN_070", format!("ZIP: {}", e)))?;
        zip.write_all(RELS.as_bytes())
            .map_err(|e| AppError::new("GEN_071", format!("ZIP write: {}", e)))?;

        // word/_rels/document.xml.rels
        zip.start_file("word/_rels/document.xml.rels", deflated)
            .map_err(|e| AppError::new("GEN_070", format!("ZIP: {}", e)))?;
        zip.write_all(DOC_RELS.as_bytes())
            .map_err(|e| AppError::new("GEN_071", format!("ZIP write: {}", e)))?;

        // word/document.xml
        zip.start_file("word/document.xml", deflated)
            .map_err(|e| AppError::new("GEN_070", format!("ZIP: {}", e)))?;
        zip.write_all(document_xml.as_bytes())
            .map_err(|e| AppError::new("GEN_071", format!("ZIP write: {}", e)))?;

        // word/styles.xml
        zip.start_file("word/styles.xml", deflated)
            .map_err(|e| AppError::new("GEN_070", format!("ZIP: {}", e)))?;
        zip.write_all(STYLES_XML.as_bytes())
            .map_err(|e| AppError::new("GEN_071", format!("ZIP write: {}", e)))?;

        let cursor = zip.finish()
            .map_err(|e| AppError::new("GEN_072", format!("ZIP finish: {}", e)))?;

        Ok(cursor.into_inner())
    }

    // ── Shared helpers ─────────────────────────────────────────────────────

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
        .map_err(|e| AppError::new("GEN_004", format!("DB: {}", e)))?
        .ok_or_else(|| AppError::new("GEN_005", "Projeto não encontrado"))
    }

    async fn save_result(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
        output_path: &str,
        duration_ms: u64,
        errors: &[String],
        warnings: &[String],
    ) {
        let file_size = std::fs::metadata(output_path).map(|m| m.len() as i64).unwrap_or(0);
        let status = if errors.is_empty() { "success" } else { "error" };
        let _ = sqlx::query(
            "INSERT INTO generation_results (id, project_id, format, platform, output_path, file_size_bytes, duration_ms, status, errors, warnings)
             VALUES (?, ?, 'docx', ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(platform)
        .bind(output_path)
        .bind(file_size)
        .bind(duration_ms as i64)
        .bind(status)
        .bind(serde_json::to_string(errors).unwrap_or_else(|_| "[]".to_string()))
        .bind(serde_json::to_string(warnings).unwrap_or_else(|_| "[]".to_string()))
        .execute(pool)
        .await;
    }
}

// ── Exportações simples (TASK-4 ST003) ─────────────────────────────────────

pub struct SimpleExportService;

impl SimpleExportService {
    /// Exporta Markdown limpo consolidado (sem frontmatter técnico).
    pub async fn export_markdown(
        pool: &SqlitePool,
        project_id: &str,
    ) -> Result<String, AppError> {
        let project = DocxService::load_project(pool, project_id).await?;
        let manuscript_root = project.manuscript_root.as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;
        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        let mut output = String::new();
        for chapter in &ast.chapters {
            output.push_str(&format!("# {}\n\n{}\n\n", chapter.title, chapter.content));
        }

        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir).ok();
        let path = format!("{}/{}-consolidated.md", output_dir, sanitize_slug(&project.name));
        std::fs::write(&path, &output).ok();

        Ok(path)
    }

    /// Exporta TXT plano com contagem de palavras.
    pub async fn export_txt(pool: &SqlitePool, project_id: &str) -> Result<String, AppError> {
        let project = DocxService::load_project(pool, project_id).await?;
        let manuscript_root = project.manuscript_root.as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;
        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        let mut plain = String::new();
        let mut total_words = 0usize;

        for chapter in &ast.chapters {
            plain.push_str(&format!("{}\n{}\n\n", chapter.title, "=".repeat(chapter.title.len())));
            // Strip Markdown formatting: remove # headers, ** *, etc.
            for line in chapter.content.lines() {
                let stripped = strip_markdown(line);
                plain.push_str(&stripped);
                plain.push('\n');
                total_words += stripped.split_whitespace().count();
            }
            plain.push('\n');
        }

        plain.push_str(&format!("\nTotal: {} palavras\n", total_words));

        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir).ok();
        let path = format!("{}/{}.txt", output_dir, sanitize_slug(&project.name));
        std::fs::write(&path, &plain).ok();

        Ok(path)
    }

    /// Exporta JSON estrutural (metadados + capítulos).
    pub async fn export_json(pool: &SqlitePool, project_id: &str) -> Result<String, AppError> {
        let project = DocxService::load_project(pool, project_id).await?;
        let manuscript_root = project.manuscript_root.as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;
        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        let chapters_json: Vec<serde_json::Value> = ast.chapters.iter().map(|c| {
            serde_json::json!({
                "order": c.order,
                "title": c.title,
                "word_count": c.word_count,
                "content": c.content,
            })
        }).collect();

        let json = serde_json::json!({
            "title": project.name,
            "language": project.language,
            "genre": project.genre,
            "chapters": chapters_json,
            "total_word_count": ast.total_words,
        });

        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir).ok();
        let path = format!("{}/{}-structure.json", output_dir, sanitize_slug(&project.name));
        std::fs::write(&path, serde_json::to_string_pretty(&json).unwrap_or_default()).ok();

        Ok(path)
    }
}

// ── Constantes OOXML ────────────────────────────────────────────────────────

const CONTENT_TYPES: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
  <Default Extension="xml" ContentType="application/xml"/>
  <Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
  <Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/>
</Types>"#;

const RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/>
</Relationships>"#;

const DOC_RELS: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
</Relationships>"#;

const STYLES_XML: &str = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" w:latentStyleCount="371">
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Title">
    <w:name w:val="Title"/>
    <w:basedOn w:val="Normal"/>
    <w:rPr><w:b/><w:sz w:val="48"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Heading1">
    <w:name w:val="heading 1"/>
    <w:basedOn w:val="Normal"/>
    <w:pPr><w:numPr/><w:outlineLvl w:val="0"/></w:pPr>
    <w:rPr><w:b/><w:sz w:val="32"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Heading2">
    <w:name w:val="heading 2"/>
    <w:basedOn w:val="Normal"/>
    <w:pPr><w:outlineLvl w:val="1"/></w:pPr>
    <w:rPr><w:b/><w:sz w:val="28"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Heading3">
    <w:name w:val="heading 3"/>
    <w:basedOn w:val="Normal"/>
    <w:pPr><w:outlineLvl w:val="2"/></w:pPr>
    <w:rPr><w:b/><w:sz w:val="24"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Quote">
    <w:name w:val="Quote"/>
    <w:basedOn w:val="Normal"/>
    <w:pPr><w:ind w:left="720" w:right="720"/></w:pPr>
    <w:rPr><w:i/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="CodeBlock">
    <w:name w:val="CodeBlock"/>
    <w:basedOn w:val="Normal"/>
    <w:rPr><w:rFonts w:ascii="Courier New" w:hAnsi="Courier New"/><w:sz w:val="18"/></w:rPr>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Caption">
    <w:name w:val="caption"/>
    <w:basedOn w:val="Normal"/>
    <w:rPr><w:i/><w:sz w:val="18"/></w:rPr>
  </w:style>
</w:styles>"#;

fn strip_markdown(line: &str) -> String {
    let line = line.trim_start_matches('#').trim();
    let line = line.trim_start_matches('*').trim_end_matches('*').trim();
    let line = line.trim_start_matches('_').trim_end_matches('_').trim();
    let line = line.trim_start_matches("> ").trim();
    line.to_string()
}
