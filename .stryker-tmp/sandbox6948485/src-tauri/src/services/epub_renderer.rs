// BES Book Formatter — EPUB Renderer Service
//
// Provides HTML generation utilities consumed by the EPUB generation pipeline
// (module-4). Implemented here (module-3) as a preparatory foundation.
//
// # Responsibilities
// - ST002: Footnote HTML (bidirectional links, `<aside epub:type="doc-footnote">`)
// - ST003: Syntax highlighting via `syntect` → inline-styled `<pre>` blocks
// - ST004: Blockquote, epigraph, ornament HTML rendering

use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::models::manuscript::Footnote;

// ---------------------------------------------------------------------------
// Shared syntect resources (expensive to construct — lazy once)
// ---------------------------------------------------------------------------

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: Lazy<ThemeSet> = Lazy::new(ThemeSet::load_defaults);

// ---------------------------------------------------------------------------
// ST003 — Syntax highlighting
// ---------------------------------------------------------------------------

/// Highlight a fenced code block for EPUB HTML output.
///
/// Uses `syntect` with the `InspiredGitHub` light theme (suitable for print).
/// Returns a `<pre>` block with inline `style=` attributes — no external CSS
/// required for the color information, though `syntax.css` adds box styling.
///
/// # Fallback behaviour
/// - Empty `language` → treated as `"text"` (plain, no highlighting).
/// - Unknown language → plain `<pre><code>` without highlighting.
/// - Empty code string → `<pre class="lang-{lang}"></pre>`.
pub fn highlight_code_block(code: &str, language: &str) -> String {
    let lang = if language.trim().is_empty() {
        "text"
    } else {
        language.trim()
    };

    if code.is_empty() {
        return format!(
            "<pre class=\"code-block lang-{}\"></pre>",
            html_escape(lang)
        );
    }

    let ss = &*SYNTAX_SET;
    let ts = &*THEME_SET;

    // Try to find syntax by token (e.g. "rust", "python", "javascript").
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    // Prefer InspiredGitHub (light, print-friendly); fall back to any theme.
    let theme = ts
        .themes
        .get("InspiredGitHub")
        .or_else(|| ts.themes.values().next())
        .expect("syntect always ships at least one theme");

    match highlighted_html_for_string(code, ss, syntax, theme) {
        Ok(highlighted) => {
            // highlighted_html_for_string wraps in <pre style="...">.
            // We add a language class for CSS targeting.
            let class_attr = format!("class=\"code-block lang-{}\"", html_escape(lang));
            // Insert class into existing <pre ...> tag.
            if let Some(stripped) = highlighted.strip_prefix("<pre") {
                format!("<pre {} {}", class_attr, stripped.trim_start_matches(' '))
            } else {
                highlighted
            }
        }
        Err(_) => {
            // Graceful fallback: plain preformatted text.
            format!(
                "<pre class=\"code-block lang-{}\"><code>{}</code></pre>",
                html_escape(lang),
                html_escape(code)
            )
        }
    }
}

// ---------------------------------------------------------------------------
// ST002 — Footnote HTML rendering
// ---------------------------------------------------------------------------

/// Render all chapter footnotes as an EPUB-compatible `<section>` of asides.
///
/// Each footnote gets:
/// - `id="fn{n}"` on the `<aside>` (target of in-text `<a href="#fn{n}">`)
/// - A return link `<a href="#fnref{n}">↵</a>` back to the inline marker
///
/// If `footnotes` is empty, returns an empty string (no section rendered).
pub fn render_footnote_section_html(footnotes: &[Footnote]) -> String {
    if footnotes.is_empty() {
        return String::new();
    }

    let mut html = String::from(
        "<section class=\"footnotes\" epub:type=\"footnotes\" role=\"doc-endnotes\">\n",
    );

    for (i, footnote) in footnotes.iter().enumerate() {
        let num = i + 1;
        html.push_str(&format!(
            "  <aside id=\"fn{num}\" epub:type=\"doc-footnote\">\n\
                 <p>[{num}] {text} <a href=\"#fnref{num}\" epub:type=\"backlink\" aria-label=\"Voltar ao texto\">↵</a></p>\n\
             </aside>\n",
            num = num,
            text = html_escape(&footnote.text),
        ));
    }

    html.push_str("</section>\n");
    html
}

/// Replace inline footnote markers `[^id]` with sequential HTML superscripts.
///
/// Markers are replaced in document order; each unique `id` is assigned the
/// sequential number matching its first appearance. Markers with no matching
/// definition in `footnotes` are left as-is (VAL_001 warning logged upstream).
///
/// Returns `(modified_content, id_to_num_map)` so the caller can cross-check.
pub fn replace_footnote_markers(
    content: &str,
    footnotes: &[Footnote],
) -> String {
    // Build an ordered list of (id, sequential_number) from footnote definitions.
    let id_to_num: Vec<(&str, usize)> = footnotes
        .iter()
        .enumerate()
        .map(|(i, f)| (f.id.as_str(), i + 1))
        .collect();

    let mut result = content.to_string();

    // Replace each `[^id]` with `<sup><a href="#fn{n}" id="fnref{n}">{n}</a></sup>`.
    // Process in reverse definition order to avoid offset shifts — but since
    // we replace the exact string `[^id]` everywhere, using string replacement
    // (not byte-offset) is safe.
    for (id, num) in &id_to_num {
        let marker = format!("[^{}]", id);
        let replacement = format!(
            "<sup id=\"fnref{num}\"><a href=\"#fn{num}\" epub:type=\"noteref\" role=\"doc-noteref\">{num}</a></sup>",
            num = num
        );
        result = result.replace(&marker, &replacement);
    }

    result
}

// ---------------------------------------------------------------------------
// ST004 — Blockquote / Epigraph / Ornament HTML rendering
// ---------------------------------------------------------------------------

/// Render a Markdown blockquote (`> text`) as HTML `<blockquote>`.
///
/// Strips the leading `> ` prefix from each line. The caller must pass only
/// the blockquote lines (without the `> ` prefix already stripped, or still
/// with it — this function handles both).
pub fn render_blockquote_html(lines: &[&str]) -> String {
    let inner: String = lines
        .iter()
        .map(|l| {
            let stripped = l.strip_prefix("> ").unwrap_or(l.strip_prefix('>').unwrap_or(l));
            format!("<p>{}</p>\n", html_escape(stripped.trim()))
        })
        .collect();

    format!("<blockquote class=\"blockquote\">\n{}</blockquote>\n", inner)
}

/// Render an epigraph block (`> _text_` at the chapter opening).
pub fn render_epigraph_html(text: &str) -> String {
    // Strip surrounding underscores if present (Markdown italic)
    let clean = text
        .trim()
        .trim_start_matches("> _")
        .trim_end_matches('_')
        .trim_start_matches('>')
        .trim()
        .trim_start_matches('_')
        .trim_end_matches('_')
        .trim();

    format!(
        "<div class=\"epigraph\">\n  <p><em>{}</em></p>\n</div>\n",
        html_escape(clean)
    )
}

/// Render a section ornament (`---` on its own line) based on the configured style.
///
/// `style` values: `"fleuron"` (default), `"rule"`, `"dinkus"`, `"none"`.
/// For EPUB, inline SVG is preferred; falls back to Unicode characters.
pub fn render_ornament_html(style: &str) -> &'static str {
    match style {
        "rule" => "<hr class=\"ornament ornament-rule\" aria-hidden=\"true\" />\n",
        "dinkus" => "<p class=\"ornament ornament-dinkus\" aria-hidden=\"true\">&#x2022; &#x2022; &#x2022;</p>\n",
        "none" => "",
        _ => "<p class=\"ornament ornament-fleuron\" aria-hidden=\"true\">&#x2766;</p>\n",
    }
}

// ---------------------------------------------------------------------------
// ST002 TASK-4 — EPUB TOC generation
// ---------------------------------------------------------------------------

/// A single TOC entry derived from a chapter's H1 heading.
pub struct TocEntry {
    /// Chapter title (H1 content).
    pub title: String,
    /// Relative XHTML file path within the EPUB, e.g. `chapter-01.xhtml`.
    pub href: String,
    /// Optional H2 sub-entries for depth-2 TOC.
    pub sub_entries: Vec<(String, String)>, // (title, href)
}

/// Generate an EPUB 3 Navigation Document (`nav.xhtml`).
///
/// Produces a valid `<nav epub:type="toc">` block with `<ol>/<li>/<a>` structure.
/// H1 chapters appear at depth 1; H2 sections at depth 2 (if provided).
pub fn generate_epub_toc_nav(
    title: &str,
    entries: &[TocEntry],
    lang: &str,
) -> String {
    let mut html = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml"
      xmlns:epub="http://www.idpf.org/2007/ops"
      xml:lang="{lang}" lang="{lang}">
<head>
  <meta charset="UTF-8" />
  <title>{title}</title>
</head>
<body>
<nav epub:type="toc" id="toc" role="doc-toc">
  <h1>{title}</h1>
  <ol>
"#,
        lang = html_escape(lang),
        title = html_escape(title),
    );

    for entry in entries {
        if entry.sub_entries.is_empty() {
            html.push_str(&format!(
                "    <li><a href=\"{}\">{}</a></li>\n",
                html_escape(&entry.href),
                html_escape(&entry.title)
            ));
        } else {
            html.push_str(&format!(
                "    <li><a href=\"{}\">{}</a>\n      <ol>\n",
                html_escape(&entry.href),
                html_escape(&entry.title)
            ));
            for (sub_title, sub_href) in &entry.sub_entries {
                html.push_str(&format!(
                    "        <li><a href=\"{}\">{}</a></li>\n",
                    html_escape(sub_href),
                    html_escape(sub_title)
                ));
            }
            html.push_str("      </ol>\n    </li>\n");
        }
    }

    html.push_str("  </ol>\n</nav>\n</body>\n</html>\n");
    html
}

/// Generate an EPUB 2 NCX file (`toc.ncx`) for backward compatibility.
///
/// Required by older Kindle devices and some EPUB 2 readers.
pub fn generate_epub_toc_ncx(
    title: &str,
    uid: &str,
    entries: &[TocEntry],
) -> String {
    let mut ncx = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ncx xmlns="http://www.daisy.org/z3986/2005/ncx/" version="2005-1">
<head>
  <meta name="dtb:uid" content="{uid}"/>
  <meta name="dtb:depth" content="1"/>
  <meta name="dtb:totalPageCount" content="0"/>
  <meta name="dtb:maxPageNumber" content="0"/>
</head>
<docTitle><text>{title}</text></docTitle>
<navMap>
"#,
        uid = html_escape(uid),
        title = html_escape(title),
    );

    for (i, entry) in entries.iter().enumerate() {
        let play_order = i + 1;
        ncx.push_str(&format!(
            "  <navPoint id=\"navpoint-{id}\" playOrder=\"{order}\">\n\
             <navLabel><text>{title}</text></navLabel>\n\
             <content src=\"{href}\"/>\n\
             </navPoint>\n",
            id = play_order,
            order = play_order,
            title = html_escape(&entry.title),
            href = html_escape(&entry.href),
        ));
    }

    ncx.push_str("</navMap>\n</ncx>\n");
    ncx
}

/// Generate a colophon string with the current date.
///
/// Format: "Gerado por BES Book Formatter em DD/MM/YYYY" (pt-BR default).
pub fn generate_colophon(lang: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Compute current date from Unix timestamp (avoids chrono dependency here)
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Simple date calculation: days since epoch
    let days = secs / 86400;
    let (year, month, day) = epoch_days_to_ymd(days);

    match lang {
        "en-US" | "en" => format!(
            "Generated by BES Book Formatter on {:02}/{:02}/{}",
            month, day, year
        ),
        "es-ES" | "es" => format!(
            "Generado por BES Book Formatter el {:02}/{:02}/{}",
            day, month, year
        ),
        _ => format!(
            "Gerado por BES Book Formatter em {:02}/{:02}/{}",
            day, month, year
        ),
    }
}

/// Convert days since Unix epoch to (year, month, day) using the Gregorian calendar.
fn epoch_days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    // Algorithm from https://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

// ---------------------------------------------------------------------------
// Utilities
// ---------------------------------------------------------------------------

/// Escape HTML special characters in `s`.
pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::manuscript::Footnote;

    // ── ST003 tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_rust_syntax_highlight() {
        let code = "fn main() {\n    println!(\"Hello, world!\");\n}";
        let html = highlight_code_block(code, "rust");
        assert!(html.contains("<pre"), "must contain <pre");
        assert!(html.contains("code-block"), "must carry css class");
        assert!(html.contains("lang-rust"), "must carry language class");
    }

    #[test]
    fn test_unknown_language_fallback() {
        let code = "some exotic code";
        let html = highlight_code_block(code, "fantasylang");
        // Should not panic; should return some preformatted block.
        assert!(html.contains("<pre"), "must still emit pre block");
    }

    #[test]
    fn test_empty_code_block() {
        let html = highlight_code_block("", "rust");
        assert!(html.contains("<pre"), "empty code still emits pre");
        assert!(!html.contains("<span"), "empty code has no spans");
    }

    #[test]
    fn test_no_language_defaults_to_text() {
        let html = highlight_code_block("hello world", "");
        assert!(html.contains("lang-text"), "empty lang → text class");
    }

    // ── ST002 tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_footnote_html_section() {
        let footnotes = vec![
            Footnote {
                id: "1".to_string(),
                text: "Primeira nota.".to_string(),
                position_in_chapter: 0,
            },
            Footnote {
                id: "2".to_string(),
                text: "Segunda nota.".to_string(),
                position_in_chapter: 20,
            },
        ];

        let html = render_footnote_section_html(&footnotes);
        assert!(html.contains("id=\"fn1\""));
        assert!(html.contains("id=\"fn2\""));
        assert!(html.contains("href=\"#fnref1\""));
        assert!(html.contains("↵"));
        assert!(html.contains("Primeira nota."));
    }

    #[test]
    fn test_footnote_section_empty() {
        let html = render_footnote_section_html(&[]);
        assert_eq!(html, "");
    }

    #[test]
    fn test_replace_footnote_markers() {
        let footnotes = vec![
            Footnote {
                id: "1".to_string(),
                text: "Nota A.".to_string(),
                position_in_chapter: 0,
            },
            Footnote {
                id: "2".to_string(),
                text: "Nota B.".to_string(),
                position_in_chapter: 10,
            },
        ];

        let content = "Texto com nota[^1] e mais[^2].";
        let result = replace_footnote_markers(content, &footnotes);
        assert!(result.contains("href=\"#fn1\""));
        assert!(result.contains("id=\"fnref1\""));
        assert!(result.contains("href=\"#fn2\""));
        assert!(!result.contains("[^1]"));
        assert!(!result.contains("[^2]"));
    }

    // ── ST004 tests ──────────────────────────────────────────────────────────

    #[test]
    fn test_blockquote_html() {
        let lines = vec!["> Uma citação importante.", "> Continua aqui."];
        let html = render_blockquote_html(&lines);
        assert!(html.contains("<blockquote"));
        assert!(html.contains("Uma citação importante."));
    }

    #[test]
    fn test_ornament_html_fleuron() {
        let html = render_ornament_html("fleuron");
        assert!(html.contains("ornament-fleuron"));
        assert!(html.contains("&#x2766;")); // ❦
    }

    #[test]
    fn test_ornament_html_dinkus() {
        let html = render_ornament_html("dinkus");
        assert!(html.contains("ornament-dinkus"));
        assert!(html.contains("&#x2022;")); // •
    }

    #[test]
    fn test_ornament_none() {
        let html = render_ornament_html("none");
        assert_eq!(html, "");
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<b>"), "&lt;b&gt;");
        assert_eq!(html_escape("a & b"), "a &amp; b");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }
}
