use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard API response wrapper matching the frontend ApiResponse<T> interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: Option<T>,
    pub error: Option<String>,
    pub warnings: Vec<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
            warnings: vec![],
        }
    }

    pub fn ok_with_warnings(data: T, warnings: Vec<String>) -> Self {
        Self {
            data: Some(data),
            error: None,
            warnings,
        }
    }

    pub fn err(message: impl Into<String>) -> Self {
        Self {
            data: None,
            error: Some(message.into()),
            warnings: vec![],
        }
    }
}

/// Sidecar availability status.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SidecarStatus {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub error: Option<String>,
}

/// Result of database initialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitResult {
    pub preferences: HashMap<String, String>,
    pub sidecars: Vec<SidecarStatus>,
    pub migrations_applied: Vec<String>,
}

/// Manuscript completeness result (Rock-1).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletenessResult {
    pub score: f64,
    pub level: String,
    pub total_chapters: i32,
    pub chapters_with_content: i32,
    pub empty_chapters: Vec<String>,
    pub placeholder_count: i32,
    pub warnings: Vec<String>,
}

/// Content checklist result (Rock-1).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistResult {
    pub passed: bool,
    pub blockers: Vec<ChecklistItem>,
    pub warnings: Vec<ChecklistItem>,
    pub info: Vec<ChecklistItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: String,
    pub message: String,
    pub files: Option<Vec<String>>,
}

/// Manuscript AST (Rock-1, simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManuscriptAst {
    pub chapters: Vec<ChapterNode>,
    pub front_matter: Vec<SectionNode>,
    pub back_matter: Vec<SectionNode>,
    pub illustrations: Vec<IllustrationPlaceholder>,
    pub metadata: ManuscriptMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterNode {
    pub slug: String,
    pub title: String,
    pub word_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SectionNode {
    pub slug: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IllustrationPlaceholder {
    pub name: String,
    pub description: String,
    pub chapter_slug: String,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManuscriptMetadata {
    pub total_words: i32,
    pub total_chapters: i32,
    pub total_illustrations: i32,
}

/// Generation result (Rock-3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub format: String,
    pub platform: String,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub duration_ms: u64,
}

/// EPUB validation result (Rock-3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub info: Vec<String>,
}

/// Preflight check result (Rock-3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreflightResult {
    pub passed: bool,
    pub blockers: Vec<ChecklistItem>,
    pub warnings: Vec<ChecklistItem>,
}

/// Preview render result (Rock-4, legacy SVG).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewResult {
    pub page_number: u32,
    pub svg_content: String,
    pub duration_ms: u64,
    pub layout_issues: Option<Vec<LayoutIssue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutIssue {
    pub issue_type: String,
    pub page: u32,
    pub description: String,
}

/// Multi-page preview response with PNG images (Rock-4, module-5).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewPageResponse {
    pub pages: Vec<PageImage>,
    pub total_pages: u32,
    pub render_ms: u64,
}

/// A single rendered page image (PNG base64).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageImage {
    pub page_number: u32,
    pub image_base64: String,
    pub width_px: u32,
    pub height_px: u32,
}

/// Annotation entry (module-5 TASK-3).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub id: String,
    pub project_id: String,
    pub page_number: u32,
    pub x_percent: f64,
    pub y_percent: f64,
    pub annotation_type: String, // "comment" | "highlight" | "flag"
    pub color: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Typographic issue (orphan/widow) for preview overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypoIssuePreview {
    pub issue_type: String,   // "orphan" | "widow"
    pub page_number: u32,
    pub line_text: String,
    pub line_y_percent: f64,
    pub severity: String,     // "error" | "warning"
}

/// BES structure verification result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructureReport {
    pub valid: bool,
    pub book_config_found: bool,
    pub book_config_path: Option<String>,
    pub manuscript_root: Option<String>,
    pub warnings: Vec<String>,
}

/// Page dimensions response (matches TS PageDimensions).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDimensionsResponse {
    pub width_inches: f64,
    pub height_inches: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_inner: f64,
    pub margin_outer: f64,
}

/// Typography defaults response (matches TS TypographyDefaults).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypographyDefaultsResponse {
    pub body_font: String,
    pub heading_font: String,
    pub code_font: Option<String>,
    pub body_size_pt: f64,
    pub line_height: f64,
}

/// Book config read from filesystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookConfig {
    pub version: Option<String>,
    pub title: String,
    pub author: String,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub manuscript_root: Option<String>,
    pub outline_root: Option<String>,
    pub output_dir: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub isbn: Option<String>,
    pub page_dimensions: Option<PageDimensionsResponse>,
    pub typography: Option<TypographyDefaultsResponse>,
}
