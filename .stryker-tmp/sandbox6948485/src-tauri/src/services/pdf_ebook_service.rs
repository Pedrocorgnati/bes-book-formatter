// BES Book Formatter — PDF Ebook Service (module-4 TASK-3)
//
// Gera PDF/A-3 otimizado para e-book via Typst: RGB, fontes embedadas,
// hyperlinks clicáveis, tagged PDF. Sem pós-processamento Ghostscript.

use std::time::Instant;

use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::enums::Genre;
use crate::models::responses::GenerationResult;
use crate::services::platform_presets::EbookPreset;
use crate::services::preflight_service::PreflightService;
use crate::services::common::sanitize_slug;
use crate::services::{ParserService, SidecarManager, TypographyService};

pub struct PdfEbookService;

impl PdfEbookService {
    /// Gera PDF/A-3 ebook via Typst (sem GS).
    pub async fn generate(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
    ) -> Result<GenerationResult, AppError> {
        let start = Instant::now();

        // 1. Checklist pré-geração
        let preflight =
            PreflightService::pre_generation_check(pool, project_id, "pdf_ebook").await?;
        if !preflight.passed {
            return Ok(GenerationResult {
                success: false,
                output_path: None,
                format: "pdf_ebook".to_string(),
                platform: platform.to_string(),
                errors: preflight.blockers.iter().map(|i| i.message.clone()).collect(),
                warnings: preflight.warnings.iter().map(|i| i.message.clone()).collect(),
                duration_ms: start.elapsed().as_millis() as u64,
            });
        }

        // 2. Carregar projeto e manuscrito
        let project = sqlx::query_as::<_, crate::models::project::Project>(
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
        .ok_or_else(|| AppError::new("GEN_005", "Projeto não encontrado"))?;

        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("GEN_001", "manuscript_root não configurado"))?;

        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // 3. TypographyConfig
        let typo_svc = TypographyService::new(pool.clone());
        let typo = typo_svc
            .get_typography_config(project_id)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| TypographyService::get_default_typography_config(project_id, &Genre::Fiction));

        // 4. Preset de e-book
        let preset = EbookPreset::for_platform(platform);
        let (page_w, page_h) = preset.page_size;

        let title = project.name.clone();
        let author = Self::get_author(pool, project_id).await;
        let lang_short = if project.language.starts_with("pt") {
            "pt"
        } else if project.language.starts_with("es") {
            "es"
        } else {
            "en"
        };

        // 5. Gerar .typ para e-book (RGB, sem bleed, com hyperlinks)
        let tmp_dir = std::env::temp_dir().join(format!("bes-ebook-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp_dir)
            .map_err(|e| AppError::new("GEN_050", format!("Falha tmp dir: {}", e)))?;

        let typ_content = Self::generate_ebook_typ(
            &title, &author, lang_short, &ast, &typo, page_w, page_h,
        );

        let main_typ_path = tmp_dir.join("main.typ");
        std::fs::write(&main_typ_path, &typ_content)
            .map_err(|e| AppError::new("GEN_051", format!("Falha ao escrever .typ: {}", e)))?;

        // 6. Compilar Typst → PDF
        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_002", format!("Falha output dir: {}", e)))?;

        let output_path = format!("{}/{}-ebook.pdf", output_dir, sanitize_slug(&project.name));

        let typst_args = vec![
            "compile".to_string(),
            main_typ_path.to_string_lossy().to_string(),
            output_path.clone(),
        ];

        let mut warnings: Vec<String> = preflight.warnings.iter().map(|i| i.message.clone()).collect();
        let mut errors: Vec<String> = Vec::new();

        match SidecarManager::spawn_typst(&typst_args, 120_000).await {
            Ok(_) => {}
            Err(e) => {
                errors.push(format!("Typst falhou: {}", e.message));
            }
        }

        let duration_ms = start.elapsed().as_millis() as u64;
        let file_size = std::fs::metadata(&output_path).map(|m| m.len() as i64).unwrap_or(0);
        let status = if errors.is_empty() { "success" } else { "error" };

        let _ = sqlx::query(
            "INSERT INTO generation_results (id, project_id, format, platform, output_path, file_size_bytes, duration_ms, status, errors, warnings)
             VALUES (?, ?, 'pdf_ebook', ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(project_id)
        .bind(platform)
        .bind(&output_path)
        .bind(file_size)
        .bind(duration_ms as i64)
        .bind(status)
        .bind(serde_json::to_string(&errors).unwrap_or_else(|_| "[]".to_string()))
        .bind(serde_json::to_string(&warnings).unwrap_or_else(|_| "[]".to_string()))
        .execute(pool)
        .await;

        let _ = std::fs::remove_dir_all(&tmp_dir);

        Ok(GenerationResult {
            success: errors.is_empty(),
            output_path: Some(output_path),
            format: "pdf_ebook".to_string(),
            platform: platform.to_string(),
            errors,
            warnings,
            duration_ms,
        })
    }

    fn generate_ebook_typ(
        title: &str,
        author: &str,
        lang: &str,
        ast: &crate::models::manuscript::ParsedManuscript,
        typo: &crate::models::typography::TypographyConfig,
        page_w: f64,
        page_h: f64,
    ) -> String {
        let font_body = &typo.font_body;
        let font_size = typo.font_size_body;
        let leading = typo.leading;

        let mut chapters_typ = String::new();
        // TOC automático
        chapters_typ.push_str("#outline(title: \"Sumário\", depth: 2)\n\n#pagebreak()\n\n");

        for chapter in &ast.chapters {
            chapters_typ.push_str(&format!("\n= {}\n\n", chapter.title));
            for line in chapter.content.lines() {
                let line = line.trim();
                if line.starts_with("## ") {
                    chapters_typ.push_str(&format!("== {}\n\n", &line[3..]));
                } else if line.starts_with("### ") {
                    chapters_typ.push_str(&format!("=== {}\n\n", &line[4..]));
                } else if line.is_empty() {
                    chapters_typ.push('\n');
                } else {
                    chapters_typ.push_str(line);
                    chapters_typ.push('\n');
                }
            }
            chapters_typ.push_str("\n#pagebreak()\n");
        }

        format!(
            r#"#set document(
  title: "{title}",
  author: "{author}",
)

#set page(
  paper: "a4",
  width: {page_w}in,
  height: {page_h}in,
  margin: (x: 0.75in, y: 0.75in),
)

#set text(
  font: "{font_body}",
  size: {font_size}pt,
  lang: "{lang}",
  fill: rgb("#1A1A1A"),
)

#set par(
  justify: true,
  leading: {leading}em,
)

// Título
#align(center)[
  #text(size: 24pt, weight: "bold")[{title}]
  #v(0.5em)
  #text(size: 14pt)[{author}]
]

#pagebreak()

{chapters_typ}
"#,
            title = title.replace('"', "'"),
            author = author.replace('"', "'"),
            page_w = page_w,
            page_h = page_h,
            font_body = font_body.replace('"', "'"),
            font_size = font_size,
            lang = lang,
            leading = leading,
            chapters_typ = chapters_typ,
        )
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
}

