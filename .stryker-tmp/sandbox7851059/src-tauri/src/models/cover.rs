// BES Book Formatter — Cover models (module-7-cover-design)

use serde::{Deserialize, Serialize};

/// Persisted cover configuration (maps to cover_configs table).
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CoverConfig {
    pub id: String,
    pub project_id: String,
    pub template_id: String,
    pub genre: String,
    pub platform: String,
    pub title_override: Option<String>,
    pub subtitle: Option<String>,
    pub author_override: Option<String>,
    pub back_cover_text: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_title: String,
    pub font_author: String,
    pub cover_image_path: Option<String>,
    pub cover_image_original: Option<String>,
    pub cover_image_dpi: Option<i64>,
    pub page_count: i64,
    pub spine_width_mm: f64,
    pub paper_type: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Input DTO for saving/updating cover config.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverConfigInput {
    pub project_id: String,
    pub template_id: Option<String>,
    pub genre: Option<String>,
    pub platform: Option<String>,
    pub title_override: Option<String>,
    pub subtitle: Option<String>,
    pub author_override: Option<String>,
    pub back_cover_text: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub font_title: Option<String>,
    pub font_author: Option<String>,
    pub cover_image_path: Option<String>,
    pub page_count: Option<i64>,
    pub paper_type: Option<String>,
}

/// Built-in cover template (embutido no binário Rust).
/// Note: `typst_template` is intentionally excluded from frontend response — too large.
/// Frontend only uses: id, genre, name, description, primaryColor, secondaryColor, tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverTemplate {
    pub id: String,
    pub genre: String,
    pub name: String,
    pub description: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub tags: Vec<String>,
    pub typst_template: String,
}

/// Result of a cover PDF generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverGenerationResult {
    pub output_path: String,
    pub spine_width_mm: f64,
    pub platform: String,
    pub warnings: Vec<String>,
}

/// Spine width calculation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpineWidthResult {
    pub spine_width_mm: f64,
    pub spine_width_inches: f64,
    pub page_count: u32,
    pub platform: String,
    pub paper_type: String,
}
