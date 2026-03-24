// BES Book Formatter — PDF Print Service (module-4 TASK-2)
//
// Pipeline em dois estágios: Typst → PDF/A base, depois Ghostscript → PDF/X-1a ou PDF/X-4.
// Guarda conformidade SEC-009: argumentos passados como array, nunca interpolados em shell string.

use std::time::Instant;

use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::enums::Genre;
use crate::models::responses::{GenerationResult, PreflightResult};
use crate::services::platform_presets::PrintPreset;
use crate::services::preflight_service::PreflightService;
use crate::services::common::sanitize_slug;
use crate::repositories::CoverConfigRepository;
use crate::services::{ParserService, SidecarManager, TypographyService};
use crate::services::cover_service;

pub struct PdfPrintService;

impl PdfPrintService {
    /// Gera PDF/X-1a (KDP) ou PDF/X-4 (IngramSpark) via Typst + Ghostscript.
    pub async fn generate(
        pool: &SqlitePool,
        project_id: &str,
        platform: &str,
        pdfx_profile: &str,
    ) -> Result<GenerationResult, AppError> {
        let start = Instant::now();

        // 1. Checklist pré-geração
        let preflight = PreflightService::pre_generation_check(pool, project_id, "pdf_print").await?;
        if !preflight.passed {
            let errors: Vec<String> = preflight.blockers.iter().map(|i| i.message.clone()).collect();
            return Ok(GenerationResult {
                success: false,
                output_path: None,
                format: "pdf_print".to_string(),
                platform: platform.to_string(),
                errors,
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
        .map_err(|e| AppError::new("GEN_004", format!("DB error: {}", e)))?
        .ok_or_else(|| AppError::new("GEN_005", format!("Projeto não encontrado: {}", project_id)))?;

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
            .unwrap_or_else(|| TypographyService::get_default_typography_config(project_id, &Genre::Fiction));

        // 4. Preset de plataforma
        let preset = PrintPreset::for_platform(platform);

        // 5. Gerar arquivo .typ em diretório temporário
        let tmp_dir = std::env::temp_dir().join(format!("bes-fmt-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp_dir)
            .map_err(|e| AppError::new("GEN_040", format!("Falha ao criar tmp dir: {}", e)))?;

        let title = project.name.clone();
        let author = Self::get_author(pool, project_id).await;
        let lang_short = if project.language.starts_with("pt") {
            "pt"
        } else if project.language.starts_with("es") {
            "es"
        } else {
            "en"
        };

        let typ_content = Self::generate_typ_file(&title, &author, lang_short, &ast, &typo, &preset);
        let main_typ_path = tmp_dir.join("main.typ");
        std::fs::write(&main_typ_path, &typ_content)
            .map_err(|e| AppError::new("GEN_041", format!("Falha ao escrever .typ: {}", e)))?;

        // Copiar templates Typst para tmp_dir
        Self::copy_typst_assets(&tmp_dir)?;

        // 6. Compilar com Typst → PDF/A base
        let pdf_base_path = tmp_dir.join("output-base.pdf");
        let typst_args = vec![
            "compile".to_string(),
            main_typ_path.to_string_lossy().to_string(),
            pdf_base_path.to_string_lossy().to_string(),
        ];

        SidecarManager::spawn_typst(&typst_args, 120_000).await.map_err(|e| {
            AppError::new("GEN_042", format!("Typst falhou: {}", e.message))
        })?;

        // 7. Pós-processar com Ghostscript → PDF/X
        let output_dir = format!("output/books/{}", sanitize_slug(&project.name));
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_002", format!("Falha ao criar output dir: {}", e)))?;

        let output_path = format!("{}/{}-print-{}.pdf", output_dir, sanitize_slug(&project.name), platform);

        let gs_args = Self::build_gs_args(
            &pdf_base_path.to_string_lossy(),
            &output_path,
            pdfx_profile,
        );

        let gs_result = SidecarManager::spawn_ghostscript(&gs_args, 120_000).await;

        let mut warnings: Vec<String> = preflight.warnings.iter().map(|i| i.message.clone()).collect();
        let mut errors: Vec<String> = Vec::new();

        match gs_result {
            Ok(_) => {}
            Err(e) => {
                // GS falhou — tentar usar o PDF/A base diretamente
                warnings.push(format!("Ghostscript falhou, usando PDF/A base: {}", e.message));
                std::fs::copy(&pdf_base_path, &output_path)
                    .map_err(|e| AppError::new("GEN_043", format!("Falha ao copiar PDF: {}", e)))?;
            }
        }

        // 7b. Merge cover PDF se cover config existir
        let cover_repo = CoverConfigRepository::new(pool.clone());
        if let Ok(Some(cover_config)) = cover_repo.get_by_project(project_id).await {
            let cover_title = cover_config.title_override.as_deref().unwrap_or(&title);
            let cover_author = cover_config.author_override.as_deref().unwrap_or(&author);
            let front_width = cover_service::front_cover_width_mm(&cover_config.platform);
            let height_mm = 228.6;

            let typst_cover = cover_service::build_cover_typst(
                &cover_config, cover_title, cover_author, front_width, height_mm,
            );

            let cover_pdf_path = tmp_dir.join("cover.pdf");
            match cover_service::compile_typst_to_pdf(&typst_cover, &cover_pdf_path).await {
                Ok(()) => {
                    let body_only = tmp_dir.join("body-only.pdf");
                    let _ = std::fs::rename(&output_path, &body_only);

                    match cover_service::merge_cover_with_body(
                        &cover_pdf_path,
                        &body_only,
                        std::path::Path::new(&output_path),
                    ).await {
                        Ok(()) => {
                            log::info!("[PdfPrintService] Cover merged into final PDF");
                        }
                        Err(e) => {
                            // SIDECAR_001: Ghostscript merge falhou — usa body sem capa
                            warnings.push(format!("Cover merge skipped: {}", e));
                            let _ = std::fs::rename(&body_only, &output_path);
                        }
                    }
                }
                Err(e) => {
                    warnings.push(format!("Cover PDF generation skipped: {}", e));
                }
            }
        }

        // 8. Validar número de páginas (alerta se não divisível por 4)
        // Heurística: word_count total / 250 palavras por página
        let total_words = ast.total_words;
        let approx_pages = (total_words / 250).max(1);
        if approx_pages % 4 != 0 {
            warnings.push(format!(
                "~{} páginas estimadas — considere adicionar páginas de guarda (verso branco) para divisibilidade por 4",
                approx_pages
            ));
        }

        // 9. Pós-validação PDF/X
        let pdf_preflight = PreflightService::validate_pdf(&output_path, pdfx_profile).await?;
        for item in &pdf_preflight.warnings {
            warnings.push(item.message.clone());
        }
        for item in &pdf_preflight.blockers {
            errors.push(item.message.clone());
        }

        // 10. Persistir resultado no SQLite
        let duration_ms = start.elapsed().as_millis() as u64;
        let file_size = std::fs::metadata(&output_path).map(|m| m.len() as i64).unwrap_or(0);
        let status = if errors.is_empty() { "success" } else { "error" };

        let _ = sqlx::query(
            "INSERT INTO generation_results (id, project_id, format, platform, output_path, file_size_bytes, duration_ms, status, errors, warnings)
             VALUES (?, ?, 'pdf_print', ?, ?, ?, ?, ?, ?, ?)",
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

        // Limpar arquivos temporários
        let _ = std::fs::remove_dir_all(&tmp_dir);

        Ok(GenerationResult {
            success: errors.is_empty(),
            output_path: Some(output_path),
            format: "pdf_print".to_string(),
            platform: platform.to_string(),
            errors,
            warnings,
            duration_ms,
        })
    }

    // ── Private helpers ────────────────────────────────────────────────────

    fn build_gs_args(input: &str, output: &str, pdfx_profile: &str) -> Vec<String> {
        let dpdx = match pdfx_profile {
            "pdf_x4" => "-dPDFX=4",
            _ => "-dPDFX=true",
        };

        // SEC-009: argumentos como array, nunca concatenados em shell string
        vec![
            "-dBATCH".to_string(),
            "-dNOPAUSE".to_string(),
            "-sDEVICE=pdfwrite".to_string(),
            dpdx.to_string(),
            "-sColorConversionStrategy=CMYK".to_string(),
            "-sProcessColorModel=DeviceCMYK".to_string(),
            format!("-sOutputFile={}", output),
            input.to_string(),
        ]
    }

    fn generate_typ_file(
        title: &str,
        author: &str,
        lang: &str,
        ast: &crate::models::manuscript::ParsedManuscript,
        typo: &crate::models::typography::TypographyConfig,
        preset: &PrintPreset,
    ) -> String {
        let (page_w, page_h) = preset.default_page_size;
        let font_body = &typo.font_body;
        let font_size = typo.font_size_body;
        let leading = typo.leading;
        let justification = if typo.justification { "true" } else { "false" };
        let hyphenation = if typo.hyphenation { "true" } else { "false" };
        let margin_top = typo.margin_top;
        let margin_bottom = typo.margin_bottom;
        let margin_inner = typo.margin_inner;
        let margin_outer = typo.margin_outer;

        // Determinar qual preset de gênero usar
        let genre_preset = typo.genre_preset.as_str();
        let preset_file = match genre_preset {
            "fiction" | "romance" => "fiction",
            "poetry" => "poetry",
            "technical" | "academic" => "technical",
            "children" | "ya" => "children",
            _ => "non-fiction",
        };

        // Construir conteúdo dos capítulos como Typst
        let mut chapters_typ = String::new();
        for chapter in &ast.chapters {
            chapters_typ.push_str(&format!("\n= {}\n\n", chapter.title));
            // Converter Markdown básico para Typst
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
            r#"#import "typst-templates/presets/{preset_file}.typ": *

#set document(
  title: "{title}",
  author: "{author}",
)

#set page(
  width: {page_w}in,
  height: {page_h}in,
  margin: (
    top: {margin_top}in,
    bottom: {margin_bottom}in,
    inside: {margin_inner}in,
    outside: {margin_outer}in,
  ),
)

#set text(
  font: "{font_body}",
  size: {font_size}pt,
  lang: "{lang}",
  hyphenate: {hyphenation},
)

#set par(
  justify: {justification},
  leading: {leading}em,
)

{chapters_typ}
"#,
            preset_file = preset_file,
            title = title.replace('"', "'"),
            author = author.replace('"', "'"),
            page_w = page_w,
            page_h = page_h,
            margin_top = margin_top,
            margin_bottom = margin_bottom,
            margin_inner = margin_inner,
            margin_outer = margin_outer,
            font_body = font_body.replace('"', "'"),
            font_size = font_size,
            lang = lang,
            hyphenation = hyphenation,
            justification = justification,
            leading = leading,
            chapters_typ = chapters_typ,
        )
    }

    fn copy_typst_assets(tmp_dir: &std::path::Path) -> Result<(), AppError> {
        // Em produção, os templates são copiados do bundle do app.
        // Para agora, criamos uma referência simbólica ou copiamos o base.typ.
        let dest = tmp_dir.join("typst-templates");
        std::fs::create_dir_all(&dest)
            .map_err(|e| AppError::new("GEN_044", format!("Falha ao criar dir typst: {}", e)))?;

        // O path real dos assets é resolvido em tempo de execução pelo Tauri
        // via app.path().resource_dir(). Para o skeleton, apenas logamos.
        log::info!("[PdfPrintService] Typst assets devem ser copiados para: {:?}", dest);
        Ok(())
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

