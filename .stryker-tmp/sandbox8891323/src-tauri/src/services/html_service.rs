// BES Book Formatter — HTML5 Service (module-4 TASK-4 ST002)
//
// Gera HTML5 responsivo de alta qualidade a partir do ManuscriptAST.
// CSS embedado com tipografia completa e media query de impressão.

use std::time::Instant;

use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::enums::Genre;
use crate::models::responses::GenerationResult;
use crate::services::common::sanitize_slug;
use crate::services::{BookConfigService, ParserService, TypographyService};

pub struct HtmlService;

impl HtmlService {
    /// Gera HTML5 responsivo como arquivo único `{slug}.html`.
    pub async fn generate(pool: &SqlitePool, project_id: &str, platform: &str) -> Result<GenerationResult, AppError> {
        let start = Instant::now();

        // 1. Carregar projeto
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
        .map_err(|e| AppError::new("GEN_100", format!("DB error: {}", e)))?
        .ok_or_else(|| AppError::new("GEN_101", format!("Projeto não encontrado: {}", project_id)))?;

        // 2. Carregar ManuscriptAST
        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("GEN_102", "manuscript_root não configurado".to_string()))?;
        let ast = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // 3. Carregar tipografia
        let typo_svc = TypographyService::new(pool.clone());
        let typo = typo_svc
            .get_typography_config(project_id)
            .await
            .unwrap_or_else(|_| TypographyService::get_default_typography_config(project_id, &Genre::Fiction));

        // 4. Carregar autor
        let author = Self::get_author(pool, project_id).await;
        let title = &project.name;
        let slug = sanitize_slug(title);

        // 5. Gerar HTML
        let html = Self::generate_html(title, &author, &ast, &typo);

        // 6. Salvar
        let output_dir = project
            .output_dir
            .as_deref()
            .map(|d| std::path::PathBuf::from(d))
            .unwrap_or_else(|| {
                std::path::PathBuf::from(&project.bes_root_path).join("output")
            });
        std::fs::create_dir_all(&output_dir)
            .map_err(|e| AppError::new("GEN_103", format!("Falha ao criar diretório: {}", e)))?;

        let output_path = output_dir.join(format!("{}.html", slug));
        std::fs::write(&output_path, html.as_bytes())
            .map_err(|e| AppError::new("GEN_104", format!("Falha ao escrever HTML: {}", e)))?;

        let file_size = std::fs::metadata(&output_path).map(|m| m.len()).ok();
        let duration_ms = start.elapsed().as_millis() as u64;

        // 7. Salvar resultado no DB
        let result_id = uuid::Uuid::new_v4().to_string();
        let path_str = output_path.to_string_lossy().to_string();
        sqlx::query(
            "INSERT INTO generation_results (id, project_id, format, platform, output_path, file_size_bytes, duration_ms, status, errors, warnings)
             VALUES (?, ?, 'html', ?, ?, ?, ?, 'success', '[]', '[]')",
        )
        .bind(&result_id)
        .bind(project_id)
        .bind(platform)
        .bind(&path_str)
        .bind(file_size.map(|s| s as i64))
        .bind(duration_ms as i64)
        .execute(pool)
        .await
        .ok();

        Ok(GenerationResult {
            success: true,
            output_path: Some(path_str),
            format: "html".to_string(),
            platform: platform.to_string(),
            errors: vec![],
            warnings: vec![],
            duration_ms,
        })
    }

    fn generate_html(
        title: &str,
        author: &str,
        ast: &crate::models::manuscript::ParsedManuscript,
        typo: &crate::models::typography::TypographyConfig,
    ) -> String {
        use comrak::{markdown_to_html, ComrakOptions};
        use crate::services::epub_renderer::html_escape;

        let css = Self::generate_css(typo);

        let mut body = String::new();

        // Title page
        body.push_str(&format!(
            "<header class=\"title-page\">\n<h1 class=\"book-title\">{}</h1>\n<p class=\"book-author\">{}</p>\n</header>\n",
            html_escape(title),
            html_escape(author),
        ));

        // Chapters
        let opts = ComrakOptions::default();
        let all_chapters: Vec<_> = ast
            .front_matter
            .iter()
            .chain(ast.chapters.iter())
            .chain(ast.back_matter.iter())
            .collect();

        for chapter in &all_chapters {
            let html_content = markdown_to_html(&chapter.content, &opts);
            body.push_str(&format!(
                "<section class=\"chapter\" id=\"ch-{}\">\n{}</section>\n",
                chapter.order,
                html_content,
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<meta name="author" content="{}">
<style>
{}
</style>
</head>
<body>
{}
</body>
</html>"#,
            html_escape(title),
            html_escape(author),
            css,
            body,
        )
    }

    fn generate_css(typo: &crate::models::typography::TypographyConfig) -> String {
        format!(
            r#"
/* BES Book Formatter — Generated HTML5 */
:root {{
  --font-body: '{body_font}', Georgia, serif;
  --font-heading: '{heading_font}', 'Georgia', serif;
  --font-size-body: {body_size}pt;
  --line-height: {leading};
  --color-text: #1a1a1a;
  --color-bg: #ffffff;
  --max-width: 700px;
}}

*, *::before, *::after {{ box-sizing: border-box; }}

html {{ font-size: 16px; }}

body {{
  font-family: var(--font-body);
  font-size: var(--font-size-body);
  line-height: var(--line-height);
  color: var(--color-text);
  background: var(--color-bg);
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 2rem 1.5rem;
}}

h1, h2, h3, h4 {{
  font-family: var(--font-heading);
  line-height: 1.3;
  margin: 2em 0 0.5em;
}}

p {{ margin: 0 0 1em; text-indent: {indent}em; }}
p:first-of-type {{ text-indent: 0; }}

blockquote {{
  border-left: 3px solid #ccc;
  padding-left: 1em;
  margin-left: 0;
  color: #555;
  font-style: italic;
}}

pre, code {{
  font-family: 'Fira Code', 'Courier New', monospace;
  background: #f5f5f5;
  border-radius: 4px;
}}

pre {{ padding: 1em; overflow-x: auto; }}
code {{ padding: 0.1em 0.3em; }}

img {{ max-width: 100%; height: auto; display: block; margin: 1.5em auto; }}

.title-page {{ text-align: center; padding: 4em 0; border-bottom: 1px solid #eee; margin-bottom: 3em; }}
.book-title {{ font-size: 2.5em; margin: 0 0 0.5em; }}
.book-author {{ font-size: 1.2em; color: #555; margin: 0; }}

.chapter {{ margin-bottom: 3em; padding-top: 2em; border-top: 1px solid #eee; }}
.chapter:first-of-type {{ border-top: none; }}

@media (max-width: 600px) {{
  body {{ padding: 1rem; }}
  .book-title {{ font-size: 1.8em; }}
}}

@media print {{
  body {{ max-width: none; padding: 2cm; }}
  .title-page {{ page-break-after: always; }}
  .chapter {{ page-break-before: always; }}
}}
"#,
            body_font = typo.font_body,
            heading_font = typo.font_heading,
            body_size = typo.font_size_body,
            leading = typo.leading,
            indent = typo.paragraph_indent,
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
            if let Ok(cfg) = BookConfigService::read_book_config(&root).await {
                return cfg.author;
            }
        }
        "Autor Desconhecido".to_string()
    }
}

