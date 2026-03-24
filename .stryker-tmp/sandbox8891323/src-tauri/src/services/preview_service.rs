use std::path::{Path, PathBuf};
use std::time::Instant;

use sha2::{Digest, Sha256};
use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{PreviewPageResponse, PageImage};
use crate::services::parser_service::ParserService;
use crate::services::typography_service::TypographyService;
use crate::repositories::ProjectRepository;

// ---------------------------------------------------------------------------
// File-system cache for .typ compilation artifacts
// ---------------------------------------------------------------------------

fn compute_cache_key(project_id: &str, config_json: &str, manuscript_json: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(project_id.as_bytes());
    hasher.update(config_json.as_bytes());
    hasher.update(manuscript_json.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };
        result.push(ALPHABET[(b0 >> 2) as usize] as char);
        result.push(ALPHABET[((b0 & 3) << 4 | b1 >> 4) as usize] as char);
        result.push(if chunk.len() > 1 {
            ALPHABET[((b1 & 0xf) << 2 | b2 >> 6) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            ALPHABET[(b2 & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    result
}

// ---------------------------------------------------------------------------
// Typst document generator
// ---------------------------------------------------------------------------

fn inches_to_typst(in_val: f64) -> String {
    format!("{:.3}in", in_val)
}

fn pt_to_typst(pt_val: f64) -> String {
    format!("{:.1}pt", pt_val)
}

/// Escapes Markdown content for safe inclusion in a Typst document.
fn escape_typst_content(content: &str) -> String {
    // Convert basic Markdown to Typst syntax
    let mut result = String::with_capacity(content.len() + 64);
    for line in content.lines() {
        // Headings: # → = , ## → == , ### → ===
        if let Some(rest) = line.strip_prefix("### ") {
            result.push_str(&format!("=== {}\n", escape_typst_text(rest)));
        } else if let Some(rest) = line.strip_prefix("## ") {
            result.push_str(&format!("== {}\n", escape_typst_text(rest)));
        } else if let Some(rest) = line.strip_prefix("# ") {
            result.push_str(&format!("= {}\n", escape_typst_text(rest)));
        } else if line.starts_with("---") || line.starts_with("***") {
            // Ornament separator
            result.push_str("#align(center)[⁂]\n");
        } else if line.trim_start().starts_with("> ") {
            // Blockquote
            let quote = line.trim_start().trim_start_matches("> ");
            result.push_str(&format!("#quote[{}]\n", escape_typst_text(quote)));
        } else {
            result.push_str(&escape_typst_inline(line));
            result.push('\n');
        }
    }
    result
}

fn escape_typst_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('#', "\\#")
        .replace('@', "\\@")
        .replace('$', "\\$")
        .replace('<', "\\<")
        .replace('>', "\\>")
}

fn escape_typst_inline(text: &str) -> String {
    // Handle bold **text** → *text* (Typst bold)
    let mut s = escape_typst_text(text);
    // Replace **text** with #strong[text]
    s = regex_replace_bold(&s);
    // Replace *text* with #emph[text]
    s = regex_replace_italic(&s);
    s
}

fn regex_replace_bold(text: &str) -> String {
    // Simple state-machine replacement for **...**
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '*' {
            if chars.peek() == Some(&'*') {
                chars.next();
                // Collect until next **
                let mut inner = String::new();
                let mut closed = false;
                while let Some(ic) = chars.next() {
                    if ic == '*' && chars.peek() == Some(&'*') {
                        chars.next();
                        closed = true;
                        break;
                    }
                    inner.push(ic);
                }
                if closed {
                    result.push_str(&format!("#strong[{}]", inner));
                } else {
                    result.push_str("**");
                    result.push_str(&inner);
                }
            } else {
                result.push('*');
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn regex_replace_italic(text: &str) -> String {
    // Simple state-machine for *...*
    let mut result = String::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'*'
            && (i == 0 || bytes[i - 1] != b'*')
            && (i + 1 < bytes.len() && bytes[i + 1] != b'*')
        {
            let start = i + 1;
            let mut j = start;
            let mut found = false;
            while j < bytes.len() {
                if bytes[j] == b'*' {
                    found = true;
                    break;
                }
                j += 1;
            }
            if found {
                let inner = &text[start..j];
                result.push_str(&format!("#emph[{}]", inner));
                i = j + 1;
            } else {
                result.push(bytes[i] as char);
                i += 1;
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Typst source builder
// ---------------------------------------------------------------------------

use crate::models::manuscript::ParsedManuscript;
use crate::models::typography::TypographyConfig;

pub fn build_typst_document(manuscript: &ParsedManuscript, config: &TypographyConfig) -> String {
    let mut doc = String::new();

    // Page setup
    doc.push_str(&format!(
        "#set page(\n  width: {},\n  height: {},\n  margin: (top: {}, bottom: {}, inside: {}, outside: {}),\n  numbering: \"1\",\n)\n\n",
        inches_to_typst(config.page_width),
        inches_to_typst(config.page_height),
        inches_to_typst(config.margin_top),
        inches_to_typst(config.margin_bottom),
        inches_to_typst(config.margin_inner),
        inches_to_typst(config.margin_outer),
    ));

    // Text setup
    let lang = if config.hyphenation_language.starts_with("pt") { "pt" } else { "en" };
    doc.push_str(&format!(
        "#set text(font: \"{}\", size: {}, lang: \"{}\", hyphenate: {})\n\n",
        config.font_body,
        pt_to_typst(config.font_size_body),
        lang,
        if config.hyphenation { "true" } else { "false" },
    ));

    // Paragraph setup
    doc.push_str(&format!(
        "#set par(\n  justify: {},\n  leading: {}em,\n  first-line-indent: {}pt,\n)\n\n",
        if config.justification { "true" } else { "false" },
        config.leading,
        config.paragraph_indent,
    ));

    // Heading styles
    doc.push_str(&format!(
        "#show heading.where(level: 1): it => [\n  #pagebreak(weak: true)\n  #v(2cm)\n  #align(center)[#text(font: \"{}\", size: {})[#it.body]]\n  #v(1cm)\n]\n\n",
        config.font_heading,
        pt_to_typst(config.font_size_h1),
    ));

    doc.push_str(&format!(
        "#show heading.where(level: 2): it => [\n  #v(0.8cm)\n  #text(font: \"{}\", size: {})[#it.body]\n  #v(0.3cm)\n]\n\n",
        config.font_heading,
        pt_to_typst(config.font_size_h2),
    ));

    // Front matter
    for chapter in &manuscript.front_matter {
        if !chapter.content.trim().is_empty() {
            doc.push_str(&escape_typst_content(&chapter.content));
            doc.push_str("\n#pagebreak()\n\n");
        }
    }

    // Main chapters
    for chapter in &manuscript.chapters {
        doc.push_str("= ");
        doc.push_str(&escape_typst_text(&chapter.title));
        doc.push('\n');
        doc.push('\n');
        doc.push_str(&escape_typst_content(&chapter.content));
        doc.push_str("\n#pagebreak()\n\n");
    }

    // Back matter
    for chapter in &manuscript.back_matter {
        if !chapter.content.trim().is_empty() {
            doc.push_str(&escape_typst_content(&chapter.content));
            doc.push_str("\n#pagebreak()\n\n");
        }
    }

    doc
}

// ---------------------------------------------------------------------------
// PreviewService
// ---------------------------------------------------------------------------

pub struct PreviewService {
    pool: SqlitePool,
}

impl PreviewService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Render a page (or spread) from the current manuscript.
    /// Returns PreviewPageResponse with PNG base64 image(s).
    pub async fn render_page(
        &self,
        project_id: &str,
        page: u32,
        zoom: f32,
        spread: bool,
    ) -> Result<PreviewPageResponse, AppError> {
        if page < 1 {
            return Err(AppError::val_out_of_range("page", &page.to_string(), "1", "∞"));
        }
        let valid_zooms = [0.5f32, 0.75, 1.0, 1.25, 1.5, 0.0]; // 0.0 = fit
        if !valid_zooms.contains(&zoom) {
            return Err(AppError::val_out_of_range(
                "zoom",
                &zoom.to_string(),
                "0.5",
                "1.5 (or 0.0 for fit)",
            ));
        }

        let start = Instant::now();

        // Resolve project → manuscript root
        let project_repo = ProjectRepository::new(self.pool.clone());
        let project = project_repo
            .find_by_id(project_id)
            .await?
            .ok_or_else(|| AppError::project_not_found(project_id))?;

        let manuscript_root = project
            .manuscript_root
            .as_deref()
            .ok_or_else(|| AppError::new("PREVIEW_001", "Project has no manuscript_root set"))?;

        // Parse manuscript
        let manuscript = ParserService::parse_manuscript(project_id, manuscript_root).await?;

        // Get typography config
        let typo_svc = TypographyService::new(self.pool.clone());
        let config = typo_svc.get_typography_config(project_id).await?;

        // Compute cache key
        let config_json = serde_json::to_string(&config).unwrap_or_default();
        let manuscript_json = serde_json::to_string(&manuscript.chapters.iter().map(|c| &c.content).collect::<Vec<_>>()).unwrap_or_default();
        let cache_key = compute_cache_key(project_id, &config_json, &manuscript_json);

        // Temp directory for this project's preview artifacts
        let temp_dir = std::env::temp_dir()
            .join("bes-preview")
            .join(project_id);
        tokio::fs::create_dir_all(&temp_dir).await?;

        let typ_path = temp_dir.join("main.typ");
        let hash_path = temp_dir.join("cache.hash");

        // Check if .typ file is still valid
        let need_regen = match tokio::fs::read_to_string(&hash_path).await {
            Ok(cached_hash) => cached_hash.trim() != cache_key,
            Err(_) => true,
        };

        if need_regen {
            let typst_doc = build_typst_document(&manuscript, &config);
            tokio::fs::write(&typ_path, &typst_doc).await?;
            tokio::fs::write(&hash_path, &cache_key).await?;
        }

        // DPI based on zoom (96 base DPI at 100%)
        let effective_zoom = if zoom == 0.0 { 1.0 } else { zoom };
        let dpi = (96.0 * effective_zoom) as u32;

        // Output pattern: temp_dir/output-{n}.png
        let output_prefix = temp_dir.join("output.png");

        // Compile with typst
        let compile_args: Vec<String> = vec![
            "compile".to_string(),
            "--format".to_string(),
            "png".to_string(),
            "--ppi".to_string(),
            dpi.to_string(),
            typ_path.to_string_lossy().to_string(),
            output_prefix.to_string_lossy().to_string(),
        ];

        // Remove old output files before recompile if AST changed
        if need_regen {
            if let Ok(mut entries) = tokio::fs::read_dir(&temp_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    if name_str.starts_with("output-") && name_str.ends_with(".png") {
                        let _ = tokio::fs::remove_file(entry.path()).await;
                    }
                }
            }
        }

        let (_stdout, stderr) =
            crate::services::SidecarManager::spawn_typst(&compile_args, 10_000).await?;

        if !stderr.is_empty() && stderr.contains("error") {
            log::warn!("[PREVIEW] typst warnings: {}", stderr);
        }

        // Count generated pages
        let total_pages = Self::count_pages(&temp_dir).await;

        if total_pages == 0 {
            return Err(AppError::new("PREVIEW_002", "Typst produced no PNG output"));
        }

        if page > total_pages {
            return Err(AppError::val_out_of_range(
                "page",
                &page.to_string(),
                "1",
                &total_pages.to_string(),
            ));
        }

        // Load page(s)
        let pages_to_load = if spread && page < total_pages {
            vec![page, page + 1]
        } else {
            vec![page]
        };

        let mut page_images = Vec::new();
        for p in pages_to_load {
            let img = Self::load_page_image(&temp_dir, p, total_pages).await?;
            page_images.push(img);
        }

        let render_ms = start.elapsed().as_millis() as u64;
        Ok(PreviewPageResponse {
            pages: page_images,
            total_pages,
            render_ms,
        })
    }

    /// Get total page count without rendering (uses cached count if available).
    pub async fn get_page_count(&self, project_id: &str) -> Result<u32, AppError> {
        let temp_dir = std::env::temp_dir()
            .join("bes-preview")
            .join(project_id);

        if temp_dir.exists() {
            let count = Self::count_pages(&temp_dir).await;
            if count > 0 {
                return Ok(count);
            }
        }

        // No cached pages — render page 1 to populate cache
        let response = self.render_page(project_id, 1, 1.0, false).await?;
        Ok(response.total_pages)
    }

    /// Invalidate the .typ cache for a project (called when manuscript or typography changes).
    pub async fn invalidate_cache(&self, project_id: &str) {
        let hash_path = std::env::temp_dir()
            .join("bes-preview")
            .join(project_id)
            .join("cache.hash");
        let _ = tokio::fs::remove_file(&hash_path).await;
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    async fn count_pages(temp_dir: &Path) -> u32 {
        let mut count = 0u32;
        if let Ok(mut entries) = tokio::fs::read_dir(temp_dir).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let name = entry.file_name();
                let name_str = name.to_string_lossy();
                if name_str.starts_with("output-") && name_str.ends_with(".png") {
                    count += 1;
                }
            }
        }
        // Also check for single-page output (output.png without number suffix)
        if count == 0 && temp_dir.join("output.png").exists() {
            count = 1;
        }
        count
    }

    async fn load_page_image(
        temp_dir: &Path,
        page: u32,
        total_pages: u32,
    ) -> Result<PageImage, AppError> {
        // Typst generates: output-0001.png, output-0002.png ... for multi-page
        // or output.png for single page
        let png_path = if total_pages == 1 {
            let single = temp_dir.join("output.png");
            if single.exists() {
                single
            } else {
                temp_dir.join("output-0001.png")
            }
        } else {
            temp_dir.join(format!("output-{:04}.png", page))
        };

        let bytes = tokio::fs::read(&png_path).await.map_err(|e| {
            AppError::new(
                "PREVIEW_003",
                format!("Cannot read page {} PNG ({}): {}", page, png_path.display(), e),
            )
        })?;

        // Get image dimensions from PNG header
        let (width_px, height_px) = read_png_dimensions(&bytes).unwrap_or((0, 0));

        Ok(PageImage {
            page_number: page,
            image_base64: base64_encode(&bytes),
            width_px,
            height_px,
        })
    }
}

/// Read PNG image dimensions from the IHDR chunk (bytes 16–23).
fn read_png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    // PNG signature: 8 bytes, IHDR length: 4 bytes, IHDR type: 4 bytes, then width+height
    if data.len() < 24 {
        return None;
    }
    // Check PNG magic bytes
    if &data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((width, height))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_hello() {
        // "Man" → "TWFu"
        assert_eq!(base64_encode(b"Man"), "TWFu");
        // "Ma" → "TWE="
        assert_eq!(base64_encode(b"Ma"), "TWE=");
        // "M" → "TQ=="
        assert_eq!(base64_encode(b"M"), "TQ==");
    }

    #[test]
    fn test_inches_to_typst() {
        assert_eq!(inches_to_typst(6.0), "6.000in");
        assert_eq!(inches_to_typst(1.5), "1.500in");
    }

    #[test]
    fn test_compute_cache_key_deterministic() {
        let k1 = compute_cache_key("proj-1", "{config}", "{manuscript}");
        let k2 = compute_cache_key("proj-1", "{config}", "{manuscript}");
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_compute_cache_key_different_for_different_inputs() {
        let k1 = compute_cache_key("proj-1", "{config-a}", "{manuscript}");
        let k2 = compute_cache_key("proj-1", "{config-b}", "{manuscript}");
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_read_png_dimensions_invalid_data() {
        assert_eq!(read_png_dimensions(b"not a png"), None);
    }

    #[test]
    fn test_escape_typst_text_no_change() {
        assert_eq!(escape_typst_text("hello world"), "hello world");
    }

    #[test]
    fn test_escape_typst_text_special_chars() {
        let escaped = escape_typst_text("cost: #5 total");
        assert!(escaped.contains("\\#"));
    }
}
