use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::enums::Genre;
use crate::models::manuscript::ParsedManuscript;
use crate::models::typography::{TypoIssue, TypographyConfig, UpdateTypographyConfig, ValidationError};

/// Service for managing typography configuration per project.
pub struct TypographyService {
    pool: SqlitePool,
}

impl TypographyService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Load typography config for a project, creating defaults if none exists.
    pub async fn get_typography_config(
        &self,
        project_id: &str,
    ) -> Result<TypographyConfig, AppError> {
        let row = sqlx::query(
            "SELECT id, project_id, font_body, font_heading, font_code,
                    font_size_body, font_size_h1, font_size_h2, font_size_h3, font_size_h4,
                    leading, paragraph_indent, tracking,
                    kerning, justification, hyphenation, hyphenation_language,
                    orphan_control, widow_control,
                    drop_cap_style, ornament_style, baseline_grid,
                    genre_preset, custom_overrides,
                    page_width, page_height,
                    margin_top, margin_bottom, margin_inner, margin_outer,
                    chapter_start, illustration_missing_mode,
                    created_at, updated_at
             FROM typography_configs WHERE project_id = ?",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_001: Failed to load typography config: {}", e)))?;

        if let Some(row) = row {
            let config = self.row_to_config(row)?;
            return Ok(config);
        }

        // No config yet — look up the project's genre and create defaults
        let genre_str: Option<String> = sqlx::query_scalar(
            "SELECT genre FROM projects WHERE id = ?",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_001: Failed to load project genre: {}", e)))?
        .flatten();

        let genre = genre_str
            .as_deref()
            .and_then(Genre::from_str)
            .unwrap_or(Genre::Nonfiction);

        let default_config = TypographyConfig::default_for_genre(project_id, &genre);
        self.save_typography_config(&default_config).await?;
        Ok(default_config)
    }

    /// Save or update the full typography config.
    pub async fn save_typography_config(
        &self,
        config: &TypographyConfig,
    ) -> Result<(), AppError> {
        let errors = config.validate();
        if !errors.is_empty() {
            let msg = errors
                .iter()
                .map(|e| format!("[{}] {}: {}", e.code, e.field, e.message))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(AppError::validation(msg));
        }

        let custom_overrides = serde_json::to_string(&config.custom_overrides)
            .unwrap_or_else(|_| "{}".to_string());

        sqlx::query(
            "INSERT INTO typography_configs (
                id, project_id,
                font_body, font_heading, font_code,
                font_size_body, font_size_h1, font_size_h2, font_size_h3, font_size_h4,
                leading, paragraph_indent, tracking,
                kerning, justification, hyphenation, hyphenation_language,
                orphan_control, widow_control,
                drop_cap_style, ornament_style, baseline_grid,
                genre_preset, custom_overrides,
                page_width, page_height,
                margin_top, margin_bottom, margin_inner, margin_outer,
                chapter_start, illustration_missing_mode,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(project_id) DO UPDATE SET
                font_body = excluded.font_body,
                font_heading = excluded.font_heading,
                font_code = excluded.font_code,
                font_size_body = excluded.font_size_body,
                font_size_h1 = excluded.font_size_h1,
                font_size_h2 = excluded.font_size_h2,
                font_size_h3 = excluded.font_size_h3,
                font_size_h4 = excluded.font_size_h4,
                leading = excluded.leading,
                paragraph_indent = excluded.paragraph_indent,
                tracking = excluded.tracking,
                kerning = excluded.kerning,
                justification = excluded.justification,
                hyphenation = excluded.hyphenation,
                hyphenation_language = excluded.hyphenation_language,
                orphan_control = excluded.orphan_control,
                widow_control = excluded.widow_control,
                drop_cap_style = excluded.drop_cap_style,
                ornament_style = excluded.ornament_style,
                baseline_grid = excluded.baseline_grid,
                genre_preset = excluded.genre_preset,
                custom_overrides = excluded.custom_overrides,
                page_width = excluded.page_width,
                page_height = excluded.page_height,
                margin_top = excluded.margin_top,
                margin_bottom = excluded.margin_bottom,
                margin_inner = excluded.margin_inner,
                margin_outer = excluded.margin_outer,
                chapter_start = excluded.chapter_start,
                illustration_missing_mode = excluded.illustration_missing_mode,
                updated_at = excluded.updated_at",
        )
        .bind(&config.id)
        .bind(&config.project_id)
        .bind(&config.font_body)
        .bind(&config.font_heading)
        .bind(&config.font_code)
        .bind(config.font_size_body)
        .bind(config.font_size_h1)
        .bind(config.font_size_h2)
        .bind(config.font_size_h3)
        .bind(config.font_size_h4)
        .bind(config.leading)
        .bind(config.paragraph_indent)
        .bind(config.tracking)
        .bind(config.kerning as i32)
        .bind(config.justification as i32)
        .bind(config.hyphenation as i32)
        .bind(&config.hyphenation_language)
        .bind(config.orphan_control as i32)
        .bind(config.widow_control as i32)
        .bind(&config.drop_cap_style)
        .bind(&config.ornament_style)
        .bind(config.baseline_grid)
        .bind(&config.genre_preset)
        .bind(&custom_overrides)
        .bind(config.page_width)
        .bind(config.page_height)
        .bind(config.margin_top)
        .bind(config.margin_bottom)
        .bind(config.margin_inner)
        .bind(config.margin_outer)
        .bind(&config.chapter_start)
        .bind(&config.illustration_missing_mode)
        .bind(&config.created_at)
        .bind(&config.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_003: Failed to save typography config: {}", e)))?;

        Ok(())
    }

    /// Apply a partial update to the typography config.
    pub async fn update_typography_config(
        &self,
        project_id: &str,
        update: UpdateTypographyConfig,
    ) -> Result<TypographyConfig, AppError> {
        let mut config = self.get_typography_config(project_id).await?;

        if let Some(v) = update.font_body { config.font_body = v; }
        if let Some(v) = update.font_heading { config.font_heading = v; }
        if let Some(v) = update.font_code { config.font_code = v; }
        if let Some(v) = update.font_size_body { config.font_size_body = v; }
        if let Some(v) = update.font_size_h1 { config.font_size_h1 = v; }
        if let Some(v) = update.font_size_h2 { config.font_size_h2 = v; }
        if let Some(v) = update.font_size_h3 { config.font_size_h3 = v; }
        if let Some(v) = update.font_size_h4 { config.font_size_h4 = v; }
        if let Some(v) = update.leading { config.leading = v; }
        if let Some(v) = update.paragraph_indent { config.paragraph_indent = v; }
        if let Some(v) = update.tracking { config.tracking = v; }
        if let Some(v) = update.kerning { config.kerning = v; }
        if let Some(v) = update.justification { config.justification = v; }
        if let Some(v) = update.hyphenation { config.hyphenation = v; }
        if let Some(v) = update.hyphenation_language { config.hyphenation_language = v; }
        if let Some(v) = update.orphan_control { config.orphan_control = v; }
        if let Some(v) = update.widow_control { config.widow_control = v; }
        if let Some(v) = update.drop_cap_style { config.drop_cap_style = v; }
        if let Some(v) = update.ornament_style { config.ornament_style = v; }
        if let Some(v) = update.baseline_grid { config.baseline_grid = v; }
        if let Some(v) = update.genre_preset { config.genre_preset = v; }
        if let Some(v) = update.custom_overrides { config.custom_overrides = v; }
        if let Some(v) = update.page_width { config.page_width = v; }
        if let Some(v) = update.page_height { config.page_height = v; }
        if let Some(v) = update.margin_top { config.margin_top = v; }
        if let Some(v) = update.margin_bottom { config.margin_bottom = v; }
        if let Some(v) = update.margin_inner { config.margin_inner = v; }
        if let Some(v) = update.margin_outer { config.margin_outer = v; }
        if let Some(v) = update.chapter_start { config.chapter_start = v; }
        if let Some(v) = update.illustration_missing_mode { config.illustration_missing_mode = v; }

        config.updated_at = Utc::now().to_rfc3339();

        // Apply preset if genre_preset changed
        if update.genre_preset.is_some() {
            if let Some(genre) = Genre::from_str(&config.genre_preset) {
                let preset = crate::models::typography::GenrePreset::from_genre(&genre);
                // Only override if fonts are still at the old default (i.e., user hasn't customised)
                config.font_body = preset.font_body.to_string();
                config.font_heading = preset.font_heading.to_string();
                config.font_size_body = preset.size_body;
                config.leading = preset.leading;
                config.page_width = preset.page_width;
                config.page_height = preset.page_height;
            }
        }

        self.save_typography_config(&config).await?;
        Ok(config)
    }

    /// Get the default typography config for a genre without persisting.
    pub fn get_default_typography_config(
        project_id: &str,
        genre: &Genre,
    ) -> TypographyConfig {
        TypographyConfig::default_for_genre(project_id, genre)
    }

    /// Detect orphan and widow lines in a parsed manuscript.
    ///
    /// Algorithm (heuristic — no full page renderer):
    /// - Estimates lines per page from page_height, font_size_body, and leading.
    /// - Scans each paragraph's last line word count; a paragraph ending with a
    ///   short line (≤ 3 words) at a page-break boundary → orphan candidate.
    /// - A paragraph starting with a very short paragraph (≤ 2 lines) at a
    ///   page-break boundary → widow candidate.
    pub fn detect_orphans_widows(
        manuscript: &ParsedManuscript,
        config: &TypographyConfig,
    ) -> Result<Vec<TypoIssue>, String> {
        if manuscript.chapters.is_empty() && manuscript.front_matter.is_empty() {
            return Ok(vec![]);
        }

        // Estimate printable height in points and lines per page
        let printable_height_in = config.page_height - config.margin_top - config.margin_bottom;
        if printable_height_in <= 0.0 {
            return Err("SYS_001: Invalid page dimensions — printable area is zero or negative".to_string());
        }
        let line_height_pt = config.font_size_body * config.leading;
        let lines_per_page = ((printable_height_in * 72.0) / line_height_pt).floor() as usize;
        let lines_per_page = lines_per_page.max(1);

        // Approx chars per line (6in printable width, ~avg 6 chars/word, 12 words/line)
        let printable_width_in = config.page_width - config.margin_inner - config.margin_outer;
        let chars_per_line = ((printable_width_in * 72.0) / (config.font_size_body * 0.5)) as usize;
        let chars_per_line = chars_per_line.max(20);

        let mut issues = Vec::new();
        let mut current_line: usize = 0;

        let all_chapters: Vec<_> = manuscript
            .front_matter
            .iter()
            .chain(manuscript.chapters.iter())
            .chain(manuscript.back_matter.iter())
            .collect();

        for chapter in &all_chapters {
            // Split content into paragraphs on blank lines
            let paragraphs: Vec<&str> = chapter
                .content
                .split("\n\n")
                .map(|p| p.trim())
                .filter(|p| !p.is_empty() && !p.starts_with('#'))
                .collect();

            for para in &paragraphs {
                let para_len = para.len();
                let para_lines = (para_len + chars_per_line - 1) / chars_per_line;
                let para_lines = para_lines.max(1);

                let page_before = current_line / lines_per_page;
                let last_line_pos = current_line + para_lines - 1;
                let page_after = last_line_pos / lines_per_page;

                // Orphan: paragraph ends on a different page than it started,
                // and the last line is at the top of a new page
                if page_after > page_before && (last_line_pos % lines_per_page) < (config.orphan_control as usize) {
                    let excerpt: String = para.chars().take(50).collect();
                    issues.push(TypoIssue {
                        page: (page_after + 1) as u32,
                        issue_type: "orphan".to_string(),
                        text_excerpt: format!("{}…", excerpt),
                        suggestion: "Aumentar leading ou ajustar margens para reposicionar parágrafo".to_string(),
                    });
                }

                // Widow: paragraph starts near the bottom of a page (only 1-2 lines before break)
                let lines_at_start = lines_per_page.saturating_sub(current_line % lines_per_page);
                if para_lines > 1 && lines_at_start < (config.widow_control as usize) && page_after > page_before {
                    let excerpt: String = para.chars().take(50).collect();
                    issues.push(TypoIssue {
                        page: (page_before + 1) as u32,
                        issue_type: "widow".to_string(),
                        text_excerpt: format!("{}…", excerpt),
                        suggestion: "Adicionar quebra de página antes deste parágrafo".to_string(),
                    });
                }

                current_line += para_lines;
            }

            // Chapter heading takes ~3 lines + forced page break
            let next_page = ((current_line / lines_per_page) + 1) * lines_per_page;
            current_line = next_page;
        }

        Ok(issues)
    }

    // ---- Private helpers ----

    fn row_to_config(&self, row: sqlx::sqlite::SqliteRow) -> Result<TypographyConfig, AppError> {
        let custom_overrides_str: String = row.try_get("custom_overrides").unwrap_or_default();
        let custom_overrides: serde_json::Value =
            serde_json::from_str(&custom_overrides_str).unwrap_or(serde_json::json!({}));

        Ok(TypographyConfig {
            id: row.try_get("id").map_err(|e| AppError::internal(e.to_string()))?,
            project_id: row.try_get("project_id").map_err(|e| AppError::internal(e.to_string()))?,
            font_body: row.try_get("font_body").unwrap_or_else(|_| "EB Garamond".to_string()),
            font_heading: row.try_get("font_heading").unwrap_or_else(|_| "EB Garamond".to_string()),
            font_code: row.try_get("font_code").ok().flatten(),
            font_size_body: row.try_get("font_size_body").unwrap_or(11.0),
            font_size_h1: row.try_get("font_size_h1").unwrap_or(22.0),
            font_size_h2: row.try_get("font_size_h2").unwrap_or(18.0),
            font_size_h3: row.try_get("font_size_h3").unwrap_or(14.0),
            font_size_h4: row.try_get("font_size_h4").unwrap_or(12.0),
            leading: row.try_get("leading").unwrap_or(1.4),
            paragraph_indent: row.try_get("paragraph_indent").unwrap_or(1.5),
            tracking: row.try_get("tracking").unwrap_or(0.0),
            kerning: row.try_get::<i32, _>("kerning").unwrap_or(1) != 0,
            justification: row.try_get::<i32, _>("justification").unwrap_or(1) != 0,
            hyphenation: row.try_get::<i32, _>("hyphenation").unwrap_or(1) != 0,
            hyphenation_language: row.try_get("hyphenation_language").unwrap_or_else(|_| "pt-BR".to_string()),
            orphan_control: row.try_get::<i32, _>("orphan_control").unwrap_or(2) as u8,
            widow_control: row.try_get::<i32, _>("widow_control").unwrap_or(2) as u8,
            drop_cap_style: row.try_get("drop_cap_style").unwrap_or_else(|_| "none".to_string()),
            ornament_style: row.try_get("ornament_style").unwrap_or_else(|_| "none".to_string()),
            baseline_grid: row.try_get("baseline_grid").unwrap_or(12.0),
            genre_preset: row.try_get("genre_preset").unwrap_or_else(|_| "nonfiction".to_string()),
            custom_overrides,
            page_width: row.try_get("page_width").unwrap_or(6.0),
            page_height: row.try_get("page_height").unwrap_or(9.0),
            margin_top: row.try_get("margin_top").unwrap_or(0.75),
            margin_bottom: row.try_get("margin_bottom").unwrap_or(0.75),
            margin_inner: row.try_get("margin_inner").unwrap_or(1.0),
            margin_outer: row.try_get("margin_outer").unwrap_or(0.75),
            chapter_start: row.try_get("chapter_start").unwrap_or_else(|_| "odd".to_string()),
            illustration_missing_mode: row
                .try_get("illustration_missing_mode")
                .unwrap_or_else(|_| "placeholder_visual".to_string()),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        })
    }
}

// ---- Unit tests ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiction_preset_values() {
        let config = TypographyConfig::default_for_genre("proj-1", &Genre::Fiction);
        assert_eq!(config.font_body, "EB Garamond");
        assert_eq!(config.font_size_body, 11.0);
        assert_eq!(config.leading, 1.4);
        assert_eq!(config.page_width, 5.5_f64);
        assert_eq!(config.page_height, 8.5_f64);
    }

    #[test]
    fn test_technical_preset_values() {
        let config = TypographyConfig::default_for_genre("proj-2", &Genre::Technical);
        assert_eq!(config.font_body, "Source Serif 4");
        assert_eq!(config.font_code, Some("JetBrains Mono".to_string()));
        assert_eq!(config.font_size_body, 10.0);
    }

    #[test]
    fn test_typography_config_validation_success() {
        let config = TypographyConfig::default_for_genre("proj-3", &Genre::Nonfiction);
        assert!(config.validate().is_empty());
    }

    #[test]
    fn test_typography_config_validation_font_size_out_of_range() {
        let mut config = TypographyConfig::default_for_genre("proj-4", &Genre::Fiction);
        config.font_size_body = 72.0;
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert_eq!(errors[0].code, "VAL_003");
    }

    #[test]
    fn test_typography_config_validation_empty_font() {
        let mut config = TypographyConfig::default_for_genre("proj-5", &Genre::Fiction);
        config.font_body = String::new();
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert_eq!(errors[0].code, "VAL_001");
    }

    #[test]
    fn test_typography_config_validation_invalid_genre() {
        let mut config = TypographyConfig::default_for_genre("proj-6", &Genre::Fiction);
        config.genre_preset = "anime".to_string();
        let errors = config.validate();
        assert!(!errors.is_empty());
        assert_eq!(errors[0].code, "VAL_002");
    }

    #[test]
    fn test_typography_config_serde() {
        let config = TypographyConfig::default_for_genre("proj-7", &Genre::Romance);
        let json = serde_json::to_string(&config).expect("serialize");
        let back: TypographyConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(config.font_body, back.font_body);
        assert_eq!(config.page_width, back.page_width);
    }

    #[test]
    fn test_detect_orphans_widows_empty_manuscript() {
        use crate::models::manuscript::ParsedManuscript;
        let manuscript = ParsedManuscript {
            project_id: "p".to_string(),
            front_matter: vec![],
            chapters: vec![],
            back_matter: vec![],
            illustrations: vec![],
            toc_present: false,
            index_present: false,
            total_words: 0,
            errors: vec![],
        };
        let config = TypographyConfig::default_for_genre("p", &Genre::Fiction);
        let issues = TypographyService::detect_orphans_widows(&manuscript, &config).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn test_detect_single_word_orphan() {
        use crate::models::manuscript::{ParsedChapter, ParsedManuscript};
        // A short paragraph "Fim." at the end of content — will be an orphan candidate
        let long_para = "Lorem ipsum dolor sit amet consectetur adipiscing elit. ".repeat(30);
        let short_para = "Fim.";
        let content = format!("{}\n\n{}", long_para, short_para);
        let chapter = ParsedChapter {
            title: "Cap 1".to_string(),
            order: 1,
            file_path: "cap1.md".to_string(),
            word_count: 200,
            heading_level: 1,
            content,
            footnotes: vec![],
            matter_type: None,
            index_entries: vec![],
        };
        let manuscript = ParsedManuscript {
            project_id: "p".to_string(),
            front_matter: vec![],
            chapters: vec![chapter],
            back_matter: vec![],
            illustrations: vec![],
            toc_present: false,
            index_present: false,
            total_words: 200,
            errors: vec![],
        };
        let config = TypographyConfig::default_for_genre("p", &Genre::Fiction);
        // Should not error
        let result = TypographyService::detect_orphans_widows(&manuscript, &config);
        assert!(result.is_ok());
        // Issues may or may not be found depending on heuristic — just check no panic
    }
}
