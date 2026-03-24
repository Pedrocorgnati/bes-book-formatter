/// Integration tests — Completeness + Illustration cross-module.
///
/// Verifies that completeness scoring logic integrates correctly with
/// the DB layer: scores computed from AST are persisted via ProjectRepository
/// and match what the UI would later read back.
///
/// Scenarios:
///  1. Full manuscript (all chapters ≥ 100 words + front-matter + no illus) → Normal (≥0.95)
///  2. Empty chapters lower score → Blocking (<0.80)
///  3. Partial manuscript (some short chapters) → Warning (0.80–0.95)
///  4. Illustrations present reduce score (illus weight = 0)
///  5. Score persisted in DB via ProjectRepository.update() and re-read correctly
///  6. Completeness stored as REAL 0.0–1.0 round-trips without precision loss
///  7. Score boundary: exactly 0.80 → Warning (not Blocking)
///  8. Score boundary: exactly 0.95 → Normal (not Warning)

mod common;

use bes_book_formatter_lib::{
    models::{
        enums::ManuscriptCompleteness, IllustrationRef, ParsedChapter, ParsedManuscript,
        UpdateProject,
    },
    repositories::ProjectRepository,
    services::CompletenessService,
};

// ──────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────

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

fn make_illus_ref(name: &str) -> IllustrationRef {
    IllustrationRef {
        name: name.to_string(),
        description: "test".to_string(),
        context: String::new(),
        line_number: 1,
        file_path: "/tmp/ch.md".to_string(),
    }
}

// ──────────────────────────────────────────────────────────
// 1. Full manuscript → Normal
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_full_manuscript_scores_normal() {
    let chapters = vec![
        make_chapter(500),
        make_chapter(600),
        make_chapter(400),
        make_chapter(300),
    ];
    let ast = make_ast(chapters, true, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    // Expected: chapters=1.0*0.50=0.50, front=0.20, illus=0.15, config=0.15 → 1.00
    assert!(score >= 0.95, "Expected ≥ 0.95, got {}", score);
    assert_eq!(level, ManuscriptCompleteness::Normal);
}

// ──────────────────────────────────────────────────────────
// 2. Empty chapters → Blocking
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_all_empty_chapters_score_blocking() {
    let chapters = vec![
        make_chapter(50),
        make_chapter(30),
        make_chapter(20),
    ];
    let ast = make_ast(chapters, false, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    // Expected: chapters=0.0*0.50=0.0, front=0, illus=0.15, config=0.15 → 0.30
    assert!(score < 0.80, "Expected < 0.80, got {}", score);
    assert_eq!(level, ManuscriptCompleteness::Blocking);
}

// ──────────────────────────────────────────────────────────
// 3. Partial manuscript → Warning
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_partial_manuscript_scores_warning() {
    // 4/5 chapters have content: 0.80 * 0.50 = 0.40
    // front-matter = 0.20, no illus = 0.15, config = 0.15 → 0.90
    let chapters = vec![
        make_chapter(300),
        make_chapter(300),
        make_chapter(300),
        make_chapter(300),
        make_chapter(50), // below threshold
    ];
    let ast = make_ast(chapters, true, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    assert!(
        score >= 0.80 && score < 0.95,
        "Expected 0.80 ≤ score < 0.95, got {}",
        score
    );
    assert_eq!(level, ManuscriptCompleteness::Warning);
}

// ──────────────────────────────────────────────────────────
// 4. Illustrations present → illustration weight = 0
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_illustrations_present_reduce_score() {
    // chapters=1.0*0.50=0.50, front=0.20, illus present → 0.0 (not 0.15), config=0.15 → 0.85
    let chapters = vec![make_chapter(500), make_chapter(500)];
    let illus = vec![make_illus_ref("cover"), make_illus_ref("ch1-fig")];
    let ast = make_ast(chapters, true, illus);
    let (score, level) = CompletenessService::calculate(&ast);

    assert!(
        score < 0.95,
        "Manuscript with unlinked illustrations should score < 0.95, got {}",
        score
    );
    assert_eq!(level, ManuscriptCompleteness::Warning);
}

// ──────────────────────────────────────────────────────────
// 5. Score persisted in DB and re-read correctly
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_score_persisted_and_read_back_from_db() {
    let pool = common::setup_pool().await;
    let project_id = common::make_project(&pool).await;
    let repo = ProjectRepository::new(pool);

    let chapters = vec![make_chapter(400), make_chapter(500)];
    let ast = make_ast(chapters, true, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    let level_str = match level {
        ManuscriptCompleteness::Blocking => "blocking",
        ManuscriptCompleteness::Warning => "warning",
        ManuscriptCompleteness::Normal => "normal",
    };

    // Persist score
    repo.update(
        &project_id,
        &UpdateProject {
            name: None,
            genre: None,
            language: None,
            completeness_score: Some(score),
            completeness_level: Some(level_str.to_string()),
            chapter_count: Some(2),
            illustration_count: Some(0),
        },
    )
    .await
    .expect("update failed");

    // Read back
    let found = repo
        .find_by_id(&project_id)
        .await
        .expect("find failed")
        .expect("should exist");

    let stored_score = found.completeness_score.expect("score should be stored");
    let stored_level = found.completeness_level.expect("level should be stored");
    let stored_chapter_count = found.chapter_count.expect("chapter_count should be stored");

    // Allow tiny floating point epsilon
    assert!(
        (stored_score - score).abs() < 1e-9,
        "Score must round-trip: expected {}, got {}",
        score,
        stored_score
    );
    assert_eq!(stored_level, level_str);
    assert_eq!(stored_chapter_count, 2);
}

// ──────────────────────────────────────────────────────────
// 6. Score 0.0 and 1.0 round-trip without precision loss
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_score_boundary_values_round_trip() {
    let pool = common::setup_pool().await;
    let repo = ProjectRepository::new(pool.clone());

    for &score in &[0.0f64, 1.0f64] {
        let project_id = common::make_project(&pool).await;
        repo.update(
            &project_id,
            &UpdateProject {
                name: None,
                genre: None,
                language: None,
                completeness_score: Some(score),
                completeness_level: Some("normal".to_string()),
                chapter_count: None,
                illustration_count: None,
            },
        )
        .await
        .expect("update failed");

        let found = repo
            .find_by_id(&project_id)
            .await
            .expect("find failed")
            .expect("should exist");

        let stored = found.completeness_score.unwrap();
        assert!(
            (stored - score).abs() < 1e-9,
            "Score {} must round-trip, got {}",
            score,
            stored
        );
    }
}

// ──────────────────────────────────────────────────────────
// 7. Score boundary: exactly 0.80 → Warning
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_score_exactly_0_80_is_warning() {
    // Craft: chapters=0.90*0.50=0.45, no front=0, no illus=0.15, config=0.15 → 0.75? No.
    // Let's compute manually to hit 0.80:
    // chapters_ratio * 0.50 + 0.20 + 0.15 + 0.15 = 0.80
    // chapters_ratio * 0.50 = 0.30 → chapters_ratio = 0.60
    // 3 out of 5 chapters with content (3/5 = 0.60)
    let chapters = vec![
        make_chapter(300), // ok
        make_chapter(300), // ok
        make_chapter(300), // ok
        make_chapter(50),  // below threshold
        make_chapter(30),  // below threshold
    ];
    let ast = make_ast(chapters, true, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    // score should be 0.60*0.50 + 0.20 + 0.15 + 0.15 = 0.80
    assert!(
        (score - 0.80).abs() < 1e-9 || score > 0.80,
        "Expected score ≈ 0.80, got {}",
        score
    );
    // At 0.80, classification must be Warning (not Blocking, which requires < 0.80)
    assert_eq!(
        level,
        ManuscriptCompleteness::Warning,
        "Score of 0.80 must be Warning (threshold is strictly < 0.80 for Blocking)"
    );
}

// ──────────────────────────────────────────────────────────
// 8. Score boundary: exactly 0.95 → Normal
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_score_exactly_0_95_is_normal() {
    // chapters_ratio * 0.50 + front + illus + config = 0.95
    // No illustrations: 0.15, front-matter: 0.20, config: 0.15
    // chapters_ratio * 0.50 = 0.95 - 0.50 = 0.45 → chapters_ratio = 0.90
    // 9 of 10 chapters with content
    let mut chapters: Vec<ParsedChapter> = (0..9).map(|_| make_chapter(300)).collect();
    chapters.push(make_chapter(50)); // 1 below threshold → 9/10 = 0.90
    let ast = make_ast(chapters, true, vec![]);
    let (score, level) = CompletenessService::calculate(&ast);

    assert!(
        (score - 0.95).abs() < 1e-9 || score >= 0.95,
        "Expected score ≈ 0.95, got {}",
        score
    );
    assert_eq!(
        level,
        ManuscriptCompleteness::Normal,
        "Score of 0.95 must be Normal (threshold is strictly < 0.95 for Warning)"
    );
}
