use serde::{Deserialize, Serialize};

/// Detailed internal AST of a parsed BES manuscript.
/// Used by parser_service and completeness_service — NOT returned directly by IPC.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedManuscript {
    pub project_id: String,
    pub front_matter: Vec<ParsedChapter>,
    pub chapters: Vec<ParsedChapter>,
    pub back_matter: Vec<ParsedChapter>,
    pub illustrations: Vec<IllustrationRef>,
    pub toc_present: bool,
    pub index_present: bool,
    pub total_words: usize,
    pub errors: Vec<ParseError>,
}

/// A single parsed chapter/section file.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedChapter {
    pub title: String,
    pub order: usize,
    pub file_path: String,
    pub word_count: usize,
    pub heading_level: u8,
    /// Raw Markdown content (used for heading hierarchy checks and re-scan).
    pub content: String,
    /// Footnote definitions extracted from this chapter (e.g. `[^1]: Nota aqui`).
    pub footnotes: Vec<Footnote>,
    /// Front/back matter type as a string (e.g. "half_title", "references").
    /// `None` for regular chapters.
    pub matter_type: Option<String>,
    /// `@INDEX[term]` markers found in this chapter.
    /// Pages are populated by the generation pipeline (module-4); empty in module-3.
    pub index_entries: Vec<IndexEntry>,
}

/// An `@INDEX[term]` or `@INDEX[category, term]` marker found in manuscript text.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexEntry {
    /// Primary index term (e.g. "Maçã").
    pub term: String,
    /// Sub-terms for hierarchical entries (e.g. `@INDEX[Fruta, Maçã]` → `["Maçã"]`).
    pub subterms: Vec<String>,
    /// Page numbers where this term appears. Populated during two-pass generation.
    pub pages: Vec<u32>,
}

/// A footnote definition extracted from a chapter (e.g. `[^1]: Nota importante`).
/// Markers in the body (`[^1]`) reference these by id.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Footnote {
    /// Footnote identifier (e.g. "1", "note-a").
    pub id: String,
    /// Full text of the footnote definition.
    pub text: String,
    /// Byte offset of the definition line in the chapter content.
    pub position_in_chapter: usize,
}

/// An `@ILLUSTRATION_PLACEHOLDER[name](description)` found in the manuscript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IllustrationRef {
    /// Placeholder name (e.g. "hero-battle").
    pub name: String,
    /// Human-readable description from the placeholder syntax.
    pub description: String,
    /// Surrounding text for context (~1 paragraph).
    pub context: String,
    pub line_number: usize,
    pub file_path: String,
}

/// A non-fatal warning or fatal error emitted during parsing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseError {
    /// "error" for fatal, "warning" for non-blocking issues.
    pub level: String,
    /// Error code (e.g. "MISSING_FILE", "HEADING_HIERARCHY").
    pub code: String,
    pub message: String,
    pub file_path: Option<String>,
    pub line: Option<usize>,
}
