use crate::models::{enums::ManuscriptCompleteness, ParsedManuscript};

/// Calculates manuscript completeness score and classification.
pub struct CompletenessService;

impl CompletenessService {
    /// Compute a 0.0–1.0 completeness score and classify it.
    ///
    /// Scoring weights (must sum to 1.0):
    ///  - Chapters with content > 100 words: 0.50
    ///  - Front-matter present:              0.20
    ///  - Illustrations linked (at parse time: 0 illus = full score): 0.15
    ///  - Book config was readable (always 0.15 if we reached here):  0.15
    ///
    /// Classification:
    ///  - `< 0.80`  → `Blocking` (generation blocked)
    ///  - `0.80–0.95` → `Warning` (generation allowed with warning)
    ///  - `>= 0.95` → `Normal`
    pub fn calculate(ast: &ParsedManuscript) -> (f64, ManuscriptCompleteness) {
        let total_chapters = ast.chapters.len().max(1);
        let chapters_with_content = ast
            .chapters
            .iter()
            .filter(|c| c.word_count > 100)
            .count();

        let mut score: f64 = 0.0;

        // Weight 1: proportion of chapters with content (0.50)
        score += (chapters_with_content as f64 / total_chapters as f64) * 0.50;

        // Weight 2: front-matter present (0.20)
        score += if !ast.front_matter.is_empty() { 0.20 } else { 0.0 };

        // Weight 3: illustration placeholders linked.
        // At initial parse, no illustrations are linked yet.
        // 0 placeholders → full score (book has no illustrations to worry about).
        let total_illus = ast.illustrations.len();
        score += if total_illus == 0 { 0.15 } else { 0.0 };

        // Weight 4: book config was valid (always reached here = 0.15)
        score += 0.15;

        let level = if score < 0.80 {
            ManuscriptCompleteness::Blocking
        } else if score < 0.95 {
            ManuscriptCompleteness::Warning
        } else {
            ManuscriptCompleteness::Normal
        };

        (score, level)
    }
}

// ---------------------------------------------------------------------------
// Unit tests (TASK-4 ST001)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{IllustrationRef, ParsedChapter, ParsedManuscript};

    fn make_chapter(word_count: usize) -> ParsedChapter {
        ParsedChapter {
            title: "Test Chapter".to_string(),
            order: 0,
            file_path: "/tmp/test.md".to_string(),
            word_count,
            heading_level: 1,
            content: String::new(),
            footnotes: vec![],
            matter_type: None,
            index_entries: vec![],
        }
    }

    fn make_ast(
        chapters: Vec<ParsedChapter>,
        with_front_matter: bool,
        illustrations: Vec<IllustrationRef>,
    ) -> ParsedManuscript {
        ParsedManuscript {
            project_id: "test".to_string(),
            front_matter: if with_front_matter {
                vec![make_chapter(50)]
            } else {
                vec![]
            },
            chapters,
            back_matter: vec![],
            illustrations,
            toc_present: false,
            index_present: false,
            total_words: 0,
            errors: vec![],
        }
    }

    #[test]
    fn test_full_manuscript_gets_high_score() {
        let chapters = vec![make_chapter(300), make_chapter(400), make_chapter(250)];
        let ast = make_ast(chapters, true, vec![]);
        let (score, level) = CompletenessService::calculate(&ast);
        assert!(score >= 0.95, "Expected >= 0.95, got {}", score);
        assert_eq!(level, ManuscriptCompleteness::Normal);
    }

    #[test]
    fn test_chapters_below_threshold_gives_blocking() {
        // 1 of 4 chapters has content → chapters score = 0.25 * 0.50 = 0.125
        // No front-matter → +0
        // No illustrations → +0.15
        // Config valid → +0.15
        // Total = 0.425 → Blocking
        let chapters = vec![
            make_chapter(200), // ok
            make_chapter(50),  // too short
            make_chapter(30),
            make_chapter(10),
        ];
        let ast = make_ast(chapters, false, vec![]);
        let (score, level) = CompletenessService::calculate(&ast);
        assert!(score < 0.80, "Expected < 0.80, got {}", score);
        assert_eq!(level, ManuscriptCompleteness::Blocking);
    }

    #[test]
    fn test_partial_manuscript_gives_warning() {
        // 4 of 5 chapters with content = 0.80 * 0.50 = 0.40
        // With front-matter = +0.20
        // No illustrations = +0.15
        // Config = +0.15
        // Total = 0.90 → Warning
        let chapters = vec![
            make_chapter(200),
            make_chapter(200),
            make_chapter(200),
            make_chapter(200),
            make_chapter(50), // empty
        ];
        let ast = make_ast(chapters, true, vec![]);
        let (score, level) = CompletenessService::calculate(&ast);
        assert!(
            score >= 0.80 && score < 0.95,
            "Expected 0.80..0.95, got {}",
            score
        );
        assert_eq!(level, ManuscriptCompleteness::Warning);
    }

    #[test]
    fn test_illustrations_present_reduce_score() {
        // All chapters ok (1.0 * 0.50), front-matter (0.20), config (0.15)
        // But illustrations present = +0.0 instead of +0.15
        // Total = 0.85 → Warning
        let chapters = vec![make_chapter(300), make_chapter(300)];
        let illus = vec![IllustrationRef {
            name: "test".to_string(),
            description: "desc".to_string(),
            context: String::new(),
            line_number: 1,
            file_path: String::new(),
        }];
        let ast = make_ast(chapters, true, illus);
        let (score, level) = CompletenessService::calculate(&ast);
        assert!(score < 0.95);
        assert_eq!(level, ManuscriptCompleteness::Warning);
    }
}
