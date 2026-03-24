use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::enums::{
    BookLanguage, ChapterStartPage, DropCapStyle, Genre, OrnamentStyle,
};

/// Typography configuration for a project.
/// Maps to the `typography_configs` SQLite table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypographyConfig {
    pub id: String,
    pub project_id: String,
    pub font_body: String,
    pub font_heading: String,
    pub font_code: Option<String>,
    pub font_size_body: f64,
    pub font_size_h1: f64,
    pub font_size_h2: f64,
    pub font_size_h3: f64,
    pub font_size_h4: f64,
    pub leading: f64,
    pub paragraph_indent: f64,
    pub tracking: f64,
    pub kerning: bool,
    pub justification: bool,
    pub hyphenation: bool,
    pub hyphenation_language: String,
    pub orphan_control: u8,
    pub widow_control: u8,
    pub drop_cap_style: String,
    pub ornament_style: String,
    pub baseline_grid: f64,
    pub genre_preset: String,
    pub custom_overrides: serde_json::Value,
    pub page_width: f64,
    pub page_height: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_inner: f64,
    pub margin_outer: f64,
    pub chapter_start: String,
    pub illustration_missing_mode: String,
    pub created_at: String,
    pub updated_at: String,
}

impl TypographyConfig {
    /// Create a default TypographyConfig for a given project and genre.
    pub fn default_for_genre(project_id: &str, genre: &Genre) -> Self {
        let preset = GenrePreset::from_genre(genre);
        let lang = match genre {
            Genre::Technical | Genre::Academic => BookLanguage::EnUs,
            _ => BookLanguage::PtBr,
        };

        Self {
            id: Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            font_body: preset.font_body.to_string(),
            font_heading: preset.font_heading.to_string(),
            font_code: preset.font_code.map(|s| s.to_string()),
            font_size_body: preset.size_body,
            font_size_h1: preset.size_body * 2.0,
            font_size_h2: preset.size_body * 1.6,
            font_size_h3: preset.size_body * 1.3,
            font_size_h4: preset.size_body * 1.1,
            leading: preset.leading,
            paragraph_indent: 1.5,
            tracking: 0.0,
            kerning: true,
            justification: true,
            hyphenation: true,
            hyphenation_language: lang.as_str().to_string(),
            orphan_control: 2,
            widow_control: 2,
            drop_cap_style: DropCapStyle::None.as_str().to_string(),
            ornament_style: OrnamentStyle::None.as_str().to_string(),
            baseline_grid: preset.size_body * 1.2,
            genre_preset: genre.as_str().to_string(),
            custom_overrides: serde_json::json!({}),
            page_width: preset.page_width,
            page_height: preset.page_height,
            margin_top: 0.75,
            margin_bottom: 0.75,
            margin_inner: 1.0,
            margin_outer: 0.75,
            chapter_start: ChapterStartPage::Odd.as_str().to_string(),
            illustration_missing_mode: "placeholder_visual".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Validate that all fields are within acceptable ranges.
    /// Returns a list of validation errors (empty = valid).
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if self.font_body.is_empty() {
            errors.push(ValidationError::required("fontBody"));
        }
        if self.font_heading.is_empty() {
            errors.push(ValidationError::required("fontHeading"));
        }
        if !(8.0..=48.0).contains(&self.font_size_body) {
            errors.push(ValidationError::range(
                "fontSizeBody",
                "Tamanho de fonte fora do intervalo: 8pt a 48pt",
            ));
        }
        if !(0.8..=3.0).contains(&self.leading) {
            errors.push(ValidationError::range(
                "leading",
                "Leading fora do intervalo: 0.8 a 3.0",
            ));
        }
        if !(0.0..=5.0).contains(&self.paragraph_indent) {
            errors.push(ValidationError::range(
                "paragraphIndent",
                "Indentação fora do intervalo: 0 a 5em",
            ));
        }
        if self.margin_top < 0.0 || self.margin_bottom < 0.0
            || self.margin_inner < 0.0 || self.margin_outer < 0.0
        {
            errors.push(ValidationError::range(
                "margins",
                "Margens devem ser ≥ 0",
            ));
        }

        // Validate genre_preset is a known value
        let valid_genres = [
            "nonfiction", "self_help", "technical", "academic",
            "fiction", "romance", "business", "management", "children", "ya",
            "poetry",
        ];
        if !valid_genres.contains(&self.genre_preset.as_str()) {
            errors.push(ValidationError::invalid_format(
                "genrePreset",
                &format!("Gênero inválido: use {}", valid_genres.join(", ")),
            ));
        }

        errors
    }
}

/// Validation error codes (VAL_001-003).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub field: String,
    pub message: String,
}

impl ValidationError {
    fn required(field: &str) -> Self {
        Self {
            code: "VAL_001".to_string(),
            field: field.to_string(),
            message: format!("Campo obrigatório não preenchido: {}", field),
        }
    }

    fn invalid_format(field: &str, message: &str) -> Self {
        Self {
            code: "VAL_002".to_string(),
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    fn range(field: &str, message: &str) -> Self {
        Self {
            code: "VAL_003".to_string(),
            field: field.to_string(),
            message: message.to_string(),
        }
    }
}

/// Typography preset values for a given genre.
pub struct GenrePreset {
    pub font_body: &'static str,
    pub font_heading: &'static str,
    pub font_code: Option<&'static str>,
    pub size_body: f64,
    pub leading: f64,
    pub page_width: f64,
    pub page_height: f64,
}

impl GenrePreset {
    pub fn from_genre(genre: &Genre) -> Self {
        match genre {
            Genre::Fiction | Genre::Romance => Self {
                font_body: "EB Garamond",
                font_heading: "EB Garamond",
                font_code: None,
                size_body: 11.0,
                leading: 1.4,
                page_width: 5.5,
                page_height: 8.5,
            },
            Genre::Nonfiction | Genre::SelfHelp | Genre::Business | Genre::Management => Self {
                font_body: "Source Serif 4",
                font_heading: "Source Serif 4",
                font_code: None,
                size_body: 11.0,
                leading: 1.45,
                page_width: 6.0,
                page_height: 9.0,
            },
            Genre::Technical | Genre::Academic => Self {
                font_body: "Source Serif 4",
                font_heading: "Source Serif 4",
                font_code: Some("JetBrains Mono"),
                size_body: 10.0,
                leading: 1.5,
                page_width: 7.0,
                page_height: 10.0,
            },
            Genre::Children | Genre::Ya => Self {
                font_body: "Source Serif 4",
                font_heading: "Source Serif 4",
                font_code: None,
                size_body: 14.0,
                leading: 1.6,
                page_width: 8.5,
                page_height: 8.5,
            },
            Genre::Poetry => Self {
                font_body: "EB Garamond",
                font_heading: "EB Garamond",
                font_code: None,
                size_body: 12.0,
                leading: 1.6,
                page_width: 5.5,
                page_height: 8.5,
            },
        }
    }
}

/// DTO for updating typography config (partial update allowed).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTypographyConfig {
    pub font_body: Option<String>,
    pub font_heading: Option<String>,
    pub font_code: Option<Option<String>>,
    pub font_size_body: Option<f64>,
    pub font_size_h1: Option<f64>,
    pub font_size_h2: Option<f64>,
    pub font_size_h3: Option<f64>,
    pub font_size_h4: Option<f64>,
    pub leading: Option<f64>,
    pub paragraph_indent: Option<f64>,
    pub tracking: Option<f64>,
    pub kerning: Option<bool>,
    pub justification: Option<bool>,
    pub hyphenation: Option<bool>,
    pub hyphenation_language: Option<String>,
    pub orphan_control: Option<u8>,
    pub widow_control: Option<u8>,
    pub drop_cap_style: Option<String>,
    pub ornament_style: Option<String>,
    pub baseline_grid: Option<f64>,
    pub genre_preset: Option<String>,
    pub custom_overrides: Option<serde_json::Value>,
    pub page_width: Option<f64>,
    pub page_height: Option<f64>,
    pub margin_top: Option<f64>,
    pub margin_bottom: Option<f64>,
    pub margin_inner: Option<f64>,
    pub margin_outer: Option<f64>,
    pub chapter_start: Option<String>,
    pub illustration_missing_mode: Option<String>,
}

/// A typographic issue (orphan or widow line) detected in the manuscript layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypoIssue {
    /// Estimated page number where the issue occurs (1-based).
    pub page: u32,
    /// "orphan" (short last line at top of page) or "widow" (short first line at bottom).
    pub issue_type: String,
    /// First ~50 chars of the problematic paragraph text.
    pub text_excerpt: String,
    /// Human-readable suggestion for fixing the issue.
    pub suggestion: String,
}

/// Result of DPI validation for an illustration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DpiValidation {
    pub dpi: u32,
    pub adequate: bool,
    pub warning: Option<String>,
}
