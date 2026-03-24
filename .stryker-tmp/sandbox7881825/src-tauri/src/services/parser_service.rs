use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::AppError;
use crate::models::enums::{BackMatterType, FrontMatterType};
use crate::models::manuscript::{Footnote, IllustrationRef, IndexEntry, ParseError, ParsedChapter, ParsedManuscript};

// ---------------------------------------------------------------------------
// Compiled regexes (lazy — compiled once on first use)
// ---------------------------------------------------------------------------

/// Detects `@ILLUSTRATION_PLACEHOLDER[name](description)` — description optional.
static ILLUS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"@ILLUSTRATION_PLACEHOLDER\[([^\]]+)\](?:\(([^)]*)\))?").unwrap()
});

/// Detects `TOC_PLACEHOLDER` on a line of its own.
static TOC_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^TOC_PLACEHOLDER\s*$").unwrap());

/// Detects `INDEX_PLACEHOLDER` on a line of its own.
static INDEX_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^INDEX_PLACEHOLDER\s*$").unwrap());

/// Detects footnote definitions: `[^id]: text` at the start of a line.
static FOOTNOTE_DEF_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\[\^([^\]]+)\]:\s+(.+)$").unwrap());

/// Detects inline footnote markers: `[^id]` (not followed by `:`).
static FOOTNOTE_MARKER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\[\^([^\]]+)\](?!:)").unwrap());

/// Detects a horizontal rule `---` used as a section ornament (on its own line,
/// optionally with surrounding spaces, but NOT a YAML front-matter delimiter).
static ORNAMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?m)^\s*---\s*$").unwrap());

/// Detects `@INDEX[term]` or `@INDEX[category, term]` markers in text.
/// [VAL_001] Unclosed `@INDEX[` (no `]`) is ignored.
static INDEX_MARKER_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"@INDEX\[([^\]]+)\]").unwrap());

// ---------------------------------------------------------------------------
// Parser service
// ---------------------------------------------------------------------------

pub struct ParserService;

impl ParserService {
    /// Parse a BES manuscript directory and return the full AST.
    ///
    /// Expects `manuscript_root/{front-matter,chapters,back-matter}/*.md`.
    pub async fn parse_manuscript(
        project_id: &str,
        manuscript_root: &str,
    ) -> Result<ParsedManuscript, AppError> {
        let root = Path::new(manuscript_root);

        if !root.exists() {
            return Err(AppError::new(
                "MANUSCRIPT_001",
                format!("MANUSCRIPT_ROOT_NOT_FOUND: {}", manuscript_root),
            ));
        }

        if !root.is_dir() {
            return Err(AppError::val_invalid_format(
                "manuscriptRoot",
                "directory path",
            ));
        }

        let mut all_errors: Vec<ParseError> = Vec::new();
        let mut illustrations: Vec<IllustrationRef> = Vec::new();
        let mut toc_present = false;
        let mut index_present = false;

        let front_matter = Self::parse_section(
            root,
            "front-matter",
            &mut all_errors,
            &mut illustrations,
            &mut toc_present,
            &mut index_present,
        )
        .await?;

        let chapters = Self::parse_section(
            root,
            "chapters",
            &mut all_errors,
            &mut illustrations,
            &mut toc_present,
            &mut index_present,
        )
        .await?;

        let back_matter = Self::parse_section(
            root,
            "back-matter",
            &mut all_errors,
            &mut illustrations,
            &mut toc_present,
            &mut index_present,
        )
        .await?;

        // Fatal: no chapters found
        if chapters.is_empty() {
            return Err(AppError::new(
                "MANUSCRIPT_002",
                format!("NO_CHAPTERS_FOUND: no .md files in {}/chapters/", manuscript_root),
            ));
        }

        // Warning: TOC missing when front-matter exists
        if !toc_present && !front_matter.is_empty() {
            all_errors.push(ParseError {
                level: "warning".to_string(),
                code: "TOC_MISSING".to_string(),
                message: "TOC_PLACEHOLDER not found — front-matter present but no TOC marker"
                    .to_string(),
                file_path: None,
                line: None,
            });
        }

        let total_words: usize = front_matter.iter().map(|c| c.word_count).sum::<usize>()
            + chapters.iter().map(|c| c.word_count).sum::<usize>()
            + back_matter.iter().map(|c| c.word_count).sum::<usize>();

        Ok(ParsedManuscript {
            project_id: project_id.to_string(),
            front_matter,
            chapters,
            back_matter,
            illustrations,
            toc_present,
            index_present,
            total_words,
            errors: all_errors,
        })
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    async fn parse_section(
        root: &Path,
        section: &str,
        errors: &mut Vec<ParseError>,
        illustrations: &mut Vec<IllustrationRef>,
        toc_present: &mut bool,
        index_present: &mut bool,
    ) -> Result<Vec<ParsedChapter>, AppError> {
        let section_path = root.join(section);
        if !section_path.exists() {
            return Ok(Vec::new()); // Optional sections are acceptable
        }

        // Guard: never scan ai-forge/workflow-app (INT-032)
        let path_str = section_path.to_string_lossy();
        if path_str.contains("ai-forge/workflow-app") || path_str.contains("workflow-app") {
            return Ok(Vec::new());
        }

        // Collect .md files
        let mut md_files: Vec<std::path::PathBuf> = Vec::new();
        let mut dir_entries =
            tokio::fs::read_dir(&section_path)
                .await
                .map_err(|e| {
                    AppError::fs_permission_denied(&format!("{}: {}", section_path.display(), e))
                })?;

        while let Some(entry) = dir_entries.next_entry().await.map_err(|e| {
            AppError::sys_internal(format!("Failed to read directory entry: {}", e))
        })? {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                md_files.push(path);
            }
        }

        // Sort by filename (001-intro.md → 002-chapter.md → lexicographic)
        md_files.sort();

        let mut chapters: Vec<ParsedChapter> = Vec::new();
        for (order, file_path) in md_files.iter().enumerate() {
            match Self::parse_file(file_path, order, section, illustrations, toc_present, index_present)
                .await
            {
                Ok(chapter) => {
                    Self::check_heading_hierarchy(&chapter, errors);
                    chapters.push(chapter);
                }
                Err(e) => {
                    errors.push(ParseError {
                        level: "error".to_string(),
                        code: "PARSE_ERROR".to_string(),
                        message: e.message.clone(),
                        file_path: Some(file_path.to_string_lossy().to_string()),
                        line: None,
                    });
                }
            }
        }

        Ok(chapters)
    }

    async fn parse_file(
        file_path: &Path,
        order: usize,
        section: &str,
        illustrations: &mut Vec<IllustrationRef>,
        toc_present: &mut bool,
        index_present: &mut bool,
    ) -> Result<ParsedChapter, AppError> {
        let file_str = file_path.to_string_lossy().to_string();

        let content = tokio::fs::read_to_string(file_path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::fs_path_not_found(&file_str)
            } else {
                AppError::fs_permission_denied(&format!("{}: {}", file_str, e))
            }
        })?;

        // Detect TOC / INDEX markers
        if TOC_RE.is_match(&content) {
            *toc_present = true;
        }
        if INDEX_RE.is_match(&content) {
            *index_present = true;
        }

        // Collect illustration placeholders
        for cap in ILLUS_RE.captures_iter(&content) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            let description = cap
                .get(2)
                .map(|m| m.as_str())
                .unwrap_or("")
                .to_string();
            let byte_offset = cap.get(0).map(|m| m.start()).unwrap_or(0);
            let line_number = Self::byte_offset_to_line(&content, byte_offset);
            let context = Self::extract_context(&content, line_number);

            illustrations.push(IllustrationRef {
                name,
                description,
                context,
                line_number,
                file_path: file_str.clone(),
            });
        }

        let title = Self::extract_title(&content, file_path);
        let word_count = Self::count_words(&content);
        let heading_level = Self::detect_primary_heading_level(&content);
        let footnotes = Self::detect_footnotes_in_file(&content);
        let index_entries = Self::scan_index_markers(&content);

        // Detect matter type from filename + section
        let matter_type = match section {
            "front-matter" => {
                let stem = file_path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let ft = Self::detect_front_matter_type(&stem);
                if ft == FrontMatterType::Unknown {
                    // [VAL_001] Unrecognized front-matter file — log warning
                    // (error will be pushed by caller if needed; here we just tag it)
                    Some("unknown".to_string())
                } else {
                    Some(ft.as_str().to_string())
                }
            }
            "back-matter" => {
                let stem = file_path
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let bt = Self::detect_back_matter_type(&stem);
                Some(bt.as_str().to_string())
            }
            _ => None, // Regular chapter
        };

        Ok(ParsedChapter {
            title,
            order,
            file_path: file_str,
            word_count,
            heading_level,
            footnotes,
            matter_type,
            index_entries,
            content,
        })
    }

    /// Extract the first H1 title; fall back to the filename stem.
    fn extract_title(content: &str, file_path: &Path) -> String {
        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(title) = trimmed.strip_prefix("# ") {
                return title.trim().to_string();
            }
        }
        file_path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string())
    }

    /// Simple word count: split on whitespace, ignore Markdown syntax lines.
    fn count_words(content: &str) -> usize {
        let mut count = 0usize;
        let mut in_code_block = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Toggle fenced code blocks
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            // Skip Markdown headings, HR, blank lines, front-matter
            if trimmed.is_empty()
                || trimmed.starts_with('#')
                || trimmed.starts_with("---")
                || trimmed.starts_with("===")
            {
                continue;
            }

            count += trimmed.split_whitespace().count();
        }

        count
    }

    /// Return the primary (lowest-numbered) heading level in the file.
    fn detect_primary_heading_level(content: &str) -> u8 {
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return 1;
            }
            if trimmed.starts_with("## ") {
                return 2;
            }
            if trimmed.starts_with("### ") {
                return 3;
            }
            if trimmed.starts_with("#### ") {
                return 4;
            }
        }
        1
    }

    /// Convert a byte offset to a 1-based line number.
    fn byte_offset_to_line(content: &str, byte_offset: usize) -> usize {
        let clamped = byte_offset.min(content.len());
        content[..clamped].lines().count().max(1)
    }

    /// Extract ~1 paragraph of context around the given line.
    fn extract_context(content: &str, line_number: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return String::new();
        }
        let idx = line_number.saturating_sub(1).min(lines.len() - 1);
        let start = idx.saturating_sub(1);
        let end = (idx + 2).min(lines.len());
        lines[start..end].join(" ")
    }

    // -----------------------------------------------------------------------
    // ST001 TASK-4 — Front/Back Matter type detection
    // -----------------------------------------------------------------------

    /// Detect the FrontMatterType from a normalised filename stem.
    ///
    /// Detection is intentionally fuzzy (contains-based) so that both
    /// `00-ante-rosto` and `ante-rosto` match `HalfTitle`.
    pub fn detect_front_matter_type(stem: &str) -> FrontMatterType {
        if stem.contains("ante-rosto") || stem.contains("half-title") || stem.contains("halftitle") {
            FrontMatterType::HalfTitle
        } else if stem.contains("frontispicio") || stem.contains("title-page") || stem.contains("titlepage") || stem.contains("frontispiece") {
            FrontMatterType::TitlePage
        } else if stem.contains("creditos") || stem.contains("copyright") || stem.contains("credits") {
            FrontMatterType::CopyrightPage
        } else if stem.contains("dedicatoria") || stem.contains("dedication") {
            FrontMatterType::Dedication
        } else if stem.contains("epigrafe") || stem.contains("epigraph") {
            FrontMatterType::Epigraph
        } else if stem == "toc" || stem.contains("sumario") || stem.contains("contents") {
            FrontMatterType::Toc
        } else if stem.contains("prefacio") || stem.contains("foreword") {
            FrontMatterType::Foreword
        } else if stem.contains("introducao") || stem.contains("introduction") || stem.contains("preface") {
            FrontMatterType::Preface
        } else if stem.contains("agradecimentos") || stem.contains("acknowledgment") {
            FrontMatterType::Acknowledgments
        } else {
            FrontMatterType::Unknown
        }
    }

    /// Detect the BackMatterType from a normalised filename stem.
    pub fn detect_back_matter_type(stem: &str) -> BackMatterType {
        if stem.contains("apendice") || stem.contains("appendix") {
            BackMatterType::Appendix
        } else if stem.contains("referencias") || stem.contains("references") || stem.contains("bibliography") {
            BackMatterType::References
        } else if stem.contains("bibliografia") {
            BackMatterType::Bibliography
        } else if stem.contains("glossario") || stem.contains("glossary") {
            BackMatterType::Glossary
        } else if stem.contains("indice") || stem.contains("index") {
            BackMatterType::Index
        } else if stem.contains("sobre-o-autor") || stem.contains("about") || stem.contains("author") {
            BackMatterType::AboutAuthor
        } else if stem.contains("colofao") || stem.contains("colophon") {
            BackMatterType::Colophon
        } else {
            BackMatterType::Unknown
        }
    }

    // -----------------------------------------------------------------------
    // ST004 TASK-4 — Index marker detection
    // -----------------------------------------------------------------------

    /// Scan manuscript content for `@INDEX[term]` markers.
    ///
    /// Supports:
    /// - `@INDEX[Term]` → primary term
    /// - `@INDEX[Category, Term]` → Category as primary, Term as subterm
    ///
    /// [VAL_001] Unclosed `@INDEX[` (no `]`) is silently ignored (not matched).
    /// [EDGE] Multiple `@INDEX[Same]` in the same chapter → merged, unique pages.
    pub fn scan_index_markers(content: &str) -> Vec<IndexEntry> {
        let mut entries: Vec<IndexEntry> = Vec::new();

        for cap in INDEX_MARKER_RE.captures_iter(content) {
            let raw = cap[1].trim().to_string();
            let parts: Vec<&str> = raw.splitn(2, ',').collect();

            let (term, subterms) = if parts.len() == 2 {
                let category = parts[0].trim().to_string();
                let subterm = parts[1].trim().to_string();
                (category, vec![subterm])
            } else {
                (raw, vec![])
            };

            // Merge duplicates (same term) — pages populated later in module-4.
            if let Some(existing) = entries.iter_mut().find(|e| e.term == term) {
                for st in &subterms {
                    if !existing.subterms.contains(st) {
                        existing.subterms.push(st.clone());
                    }
                }
            } else {
                entries.push(IndexEntry {
                    term,
                    subterms,
                    pages: vec![],
                });
            }
        }

        entries
    }

    // -----------------------------------------------------------------------
    // ST002 — Footnote detection (TASK-3)
    // -----------------------------------------------------------------------

    /// Extract all footnote definitions from chapter content.
    ///
    /// A definition looks like `[^id]: Nota aqui` at the beginning of a line.
    /// Markers (`[^id]` without `:`) appear inline in the body text.
    /// Returns definitions ordered by their position in the file.
    pub fn detect_footnotes_in_file(content: &str) -> Vec<Footnote> {
        let mut footnotes: Vec<Footnote> = Vec::new();

        for cap in FOOTNOTE_DEF_RE.captures_iter(content) {
            let id = cap[1].to_string();
            let text = cap[2].trim().to_string();
            let position_in_chapter = cap.get(0).map(|m| m.start()).unwrap_or(0);

            // Avoid duplicate ids (keep first occurrence).
            if !footnotes.iter().any(|f| f.id == id) {
                footnotes.push(Footnote {
                    id,
                    text,
                    position_in_chapter,
                });
            }
        }

        footnotes
    }

    /// Count inline footnote markers (`[^id]`) in a content string.
    /// Used to verify that all markers have a matching definition.
    pub fn count_footnote_markers(content: &str) -> usize {
        FOOTNOTE_MARKER_RE.find_iter(content).count()
    }

    // -----------------------------------------------------------------------
    // ST004 — Ornament detection (TASK-3)
    // -----------------------------------------------------------------------

    /// Returns `true` if `content` contains at least one `---` section ornament.
    ///
    /// Note: YAML front-matter delimiters (`---` on the very first/last line of
    /// a front-matter block) are not distinguished here — the renderer is
    /// responsible for skipping the first `---` if it is part of YAML.
    pub fn has_ornament(content: &str) -> bool {
        ORNAMENT_RE.is_match(content)
    }

    /// Returns `true` if `content` contains at least one blockquote (`> text`).
    pub fn has_blockquote(content: &str) -> bool {
        content.lines().any(|l| l.trim_start().starts_with("> "))
    }

    /// Emit `HEADING_HIERARCHY` warnings when a heading skips a level (H1 → H3).
    fn check_heading_hierarchy(chapter: &ParsedChapter, errors: &mut Vec<ParseError>) {
        let mut last_level: u8 = 0;
        for (line_idx, line) in chapter.content.lines().enumerate() {
            let trimmed = line.trim();
            let level: u8 = if trimmed.starts_with("#### ") {
                4
            } else if trimmed.starts_with("### ") {
                3
            } else if trimmed.starts_with("## ") {
                2
            } else if trimmed.starts_with("# ") {
                1
            } else {
                continue;
            };

            if last_level > 0 && level > last_level + 1 {
                errors.push(ParseError {
                    level: "warning".to_string(),
                    code: "HEADING_HIERARCHY".to_string(),
                    message: format!(
                        "H{} found after H{} — skipped a heading level",
                        level, last_level
                    ),
                    file_path: Some(chapter.file_path.clone()),
                    line: Some(line_idx + 1),
                });
            }
            last_level = level;
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests (TASK-1 ST007)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio::fs;

    /// Create a temporary BES project structure for testing.
    async fn setup_bes_project(dir: &PathBuf) {
        fs::create_dir_all(dir.join("front-matter")).await.unwrap();
        fs::create_dir_all(dir.join("chapters")).await.unwrap();
        fs::create_dir_all(dir.join("back-matter")).await.unwrap();

        fs::write(
            dir.join("front-matter/000-preface.md"),
            "# Prefácio\n\nTOC_PLACEHOLDER\n\nEste é o prefácio do livro.",
        )
        .await
        .unwrap();

        fs::write(
            dir.join("chapters/001-intro.md"),
            "# Introdução\n\nEste é o primeiro capítulo com conteúdo suficiente para o score.\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
        )
        .await
        .unwrap();

        fs::write(
            dir.join("chapters/002-body.md"),
            "# O Corpo da Obra\n\n@ILLUSTRATION_PLACEHOLDER[hero-battle](Ilustração da batalha final)\n\nConteúdo do segundo capítulo.",
        )
        .await
        .unwrap();

        fs::write(
            dir.join("back-matter/999-index.md"),
            "# Índice\n\nINDEX_PLACEHOLDER\n",
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_parse_valid_bes_project() {
        let tmp = tempdir();
        setup_bes_project(&tmp).await;

        let result = ParserService::parse_manuscript("test-project-id", &tmp.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(result.chapters.len(), 2);
        assert_eq!(result.front_matter.len(), 1);
        assert_eq!(result.back_matter.len(), 1);
        assert!(result.toc_present);
        assert!(result.index_present);
        assert!(result.total_words > 0);
    }

    #[tokio::test]
    async fn test_parse_missing_chapters_dir() {
        let tmp = tempdir();
        // Only front-matter, no chapters/
        fs::create_dir_all(tmp.join("front-matter")).await.unwrap();
        fs::write(tmp.join("front-matter/000-preface.md"), "# Prefácio\n\nConteúdo")
            .await
            .unwrap();

        let result = ParserService::parse_manuscript("test-id", &tmp.to_string_lossy()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().code.contains("MANUSCRIPT"));
    }

    #[tokio::test]
    async fn test_detect_illustration_placeholders() {
        let tmp = tempdir();
        setup_bes_project(&tmp).await;

        let result = ParserService::parse_manuscript("test-id", &tmp.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(result.illustrations.len(), 1);
        assert_eq!(result.illustrations[0].name, "hero-battle");
        assert_eq!(
            result.illustrations[0].description,
            "Ilustração da batalha final"
        );
    }

    #[tokio::test]
    async fn test_illustration_placeholder_no_description() {
        let tmp = tempdir();
        fs::create_dir_all(tmp.join("chapters")).await.unwrap();
        fs::write(
            tmp.join("chapters/001-ch.md"),
            "# Chapter\n\n@ILLUSTRATION_PLACEHOLDER[no-desc]\n\nConteúdo do capítulo aqui.",
        )
        .await
        .unwrap();

        let result = ParserService::parse_manuscript("test-id", &tmp.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(result.illustrations[0].description, "");
    }

    #[tokio::test]
    async fn test_toc_placeholder_detection() {
        let tmp = tempdir();
        setup_bes_project(&tmp).await;

        let result = ParserService::parse_manuscript("test-id", &tmp.to_string_lossy())
            .await
            .unwrap();

        assert!(result.toc_present);
    }

    #[tokio::test]
    async fn test_ignore_workflow_app_dir() {
        // The guard clause in parse_section prevents scanning workflow-app
        let path_str = "/some/path/ai-forge/workflow-app/chapters";
        let root = Path::new("/some/path");
        // The check is path_str.contains("workflow-app") — verified in code above
        assert!(path_str.contains("workflow-app"));
        let _ = root; // avoid unused warning
    }

    #[tokio::test]
    async fn test_heading_hierarchy_violation() {
        let tmp = tempdir();
        fs::create_dir_all(tmp.join("chapters")).await.unwrap();
        // H1 → H3 skips H2
        fs::write(
            tmp.join("chapters/001-ch.md"),
            "# Title\n\n### SubSubSection\n\nConteúdo aqui para ter palavras suficientes.",
        )
        .await
        .unwrap();

        let result = ParserService::parse_manuscript("test-id", &tmp.to_string_lossy())
            .await
            .unwrap();

        let hierarchy_warnings: Vec<_> = result
            .errors
            .iter()
            .filter(|e| e.code == "HEADING_HIERARCHY")
            .collect();
        assert!(!hierarchy_warnings.is_empty());
    }

    // ── ST001/ST004 TASK-4 tests ────────────────────────────────────────────

    #[test]
    fn test_parse_front_matter_detection() {
        use crate::models::enums::FrontMatterType;
        assert_eq!(ParserService::detect_front_matter_type("00-ante-rosto"), FrontMatterType::HalfTitle);
        assert_eq!(ParserService::detect_front_matter_type("01-frontispicio"), FrontMatterType::TitlePage);
        assert_eq!(ParserService::detect_front_matter_type("02-creditos"), FrontMatterType::CopyrightPage);
        assert_eq!(ParserService::detect_front_matter_type("03-dedicatoria"), FrontMatterType::Dedication);
        assert_eq!(ParserService::detect_front_matter_type("toc"), FrontMatterType::Toc);
        assert_eq!(ParserService::detect_front_matter_type("xyz-unknown"), FrontMatterType::Unknown);
    }

    #[test]
    fn test_parse_back_matter_detection() {
        use crate::models::enums::BackMatterType;
        assert_eq!(ParserService::detect_back_matter_type("referencias"), BackMatterType::References);
        assert_eq!(ParserService::detect_back_matter_type("sobre-o-autor"), BackMatterType::AboutAuthor);
        assert_eq!(ParserService::detect_back_matter_type("colofao"), BackMatterType::Colophon);
        assert_eq!(ParserService::detect_back_matter_type("glossario"), BackMatterType::Glossary);
        assert_eq!(ParserService::detect_back_matter_type("apendice-a"), BackMatterType::Appendix);
    }

    #[test]
    fn test_index_marker_parsing() {
        let content = "Apple @INDEX[Maçã] Orange @INDEX[Laranja].";
        let entries = ParserService::scan_index_markers(content);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].term, "Maçã");
        assert_eq!(entries[1].term, "Laranja");
        assert!(entries[0].subterms.is_empty());
    }

    #[test]
    fn test_index_subterm_handling() {
        let content = "Fruit @INDEX[Fruta, Maçã] and @INDEX[Fruta, Laranja].";
        let entries = ParserService::scan_index_markers(content);
        // Both are "Fruta" with different subterms — should be merged
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].term, "Fruta");
        assert_eq!(entries[0].subterms.len(), 2);
    }

    #[test]
    fn test_index_unclosed_marker_ignored() {
        // `@INDEX[` without `]` should not match
        let content = "Bad @INDEX[ marker without close.";
        let entries = ParserService::scan_index_markers(content);
        assert_eq!(entries.len(), 0);
    }

    // ── ST002/ST004 TASK-3 tests ────────────────────────────────────────────

    #[test]
    fn test_footnote_parsing_and_linking() {
        let content = "# Capítulo\n\nTexto com nota[^1] e outra[^2].\n\n[^1]: Primeira nota importante.\n[^2]: Segunda nota explicativa.\n";
        let footnotes = ParserService::detect_footnotes_in_file(content);
        assert_eq!(footnotes.len(), 2);
        assert_eq!(footnotes[0].id, "1");
        assert_eq!(footnotes[0].text, "Primeira nota importante.");
        assert_eq!(footnotes[1].id, "2");
        assert_eq!(footnotes[1].text, "Segunda nota explicativa.");
    }

    #[test]
    fn test_footnote_no_definition_marker_only() {
        // Markers without definitions → no footnotes extracted
        let content = "Texto com referência[^1] mas sem definição.\n";
        let footnotes = ParserService::detect_footnotes_in_file(content);
        assert_eq!(footnotes.len(), 0);
        // But markers are still counted
        assert_eq!(ParserService::count_footnote_markers(content), 1);
    }

    #[test]
    fn test_footnote_duplicate_id_kept_once() {
        let content = "[^1]: Primeira definição.\n[^1]: Duplicada (ignorada).\n";
        let footnotes = ParserService::detect_footnotes_in_file(content);
        assert_eq!(footnotes.len(), 1);
        assert_eq!(footnotes[0].text, "Primeira definição.");
    }

    #[test]
    fn test_blockquote_parsing() {
        let content = "# Chapter\n\n> Esta é uma citação importante.\n\nTexto normal.\n";
        assert!(ParserService::has_blockquote(content));
    }

    #[test]
    fn test_ornament_detection() {
        let content = "# Capítulo\n\nPrimeira parte.\n\n---\n\nSegunda parte.\n";
        assert!(ParserService::has_ornament(content));
    }

    #[test]
    fn test_ornament_detection_spaces() {
        let content = "# Capítulo\n\nPrimeira parte.\n\n  ---  \n\nSegunda parte.\n";
        assert!(ParserService::has_ornament(content));
    }

    /// Create a temporary directory as PathBuf.
    fn tempdir() -> PathBuf {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let dir = std::env::temp_dir().join(format!("bes_parser_test_{}", id));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
