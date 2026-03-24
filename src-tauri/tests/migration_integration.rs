/// Integration tests — MigrationService.
///
/// Verifies the full migration chain: from a blank SQLite DB to all 7 migrations
/// applied in sequence. Also tests idempotency and version tracking.
///
/// Scenarios:
///  1. Fresh DB: apply_pending() applies all 7 migrations
///  2. Second call to apply_pending() applies nothing (idempotent)
///  3. get_current_version() returns 7 after full apply
///  4. All expected tables exist after migration
///  5. verify_integrity() passes on fresh migrated DB
///  6. schema_version table records all migration names
///  7. Default preference seeds are present after M001

mod common;

use bes_book_formatter_lib::services::MigrationService;
use sqlx::Row; // needed for row.get() on SqliteRow
use sqlx::SqlitePool;

async fn blank_pool() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create blank in-memory pool")
}

// ──────────────────────────────────────────────────────────
// 1. apply_pending() on fresh DB applies all migrations
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_apply_pending_runs_all_migrations_on_fresh_db() {
    let pool = blank_pool().await;
    let svc = MigrationService::new(pool);

    let applied = svc.apply_pending().await.expect("apply_pending failed");

    assert_eq!(
        applied.len(),
        7,
        "Expected 7 migrations to be applied on fresh DB, got: {:?}",
        applied
    );
    assert!(applied.contains(&"M001_initial_schema".to_string()));
    assert!(applied.contains(&"M002_add_error_state".to_string()));
    assert!(applied.contains(&"M003_typography_config".to_string()));
    assert!(applied.contains(&"M004_generation_results".to_string()));
    assert!(applied.contains(&"M005_annotations".to_string()));
    assert!(applied.contains(&"M006_bes_document_cache".to_string()));
    assert!(applied.contains(&"M007_cover_configs".to_string()));
}

// ──────────────────────────────────────────────────────────
// 2. Second apply_pending() applies nothing (idempotent)
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_apply_pending_is_idempotent() {
    let pool = blank_pool().await;
    let svc = MigrationService::new(pool);

    svc.apply_pending().await.expect("first apply failed");
    let second_apply = svc.apply_pending().await.expect("second apply failed");

    assert!(
        second_apply.is_empty(),
        "Second apply_pending must return empty (all already applied), got: {:?}",
        second_apply
    );
}

// ──────────────────────────────────────────────────────────
// 3. get_current_version() → 7 after full apply
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_get_current_version_returns_7_after_full_apply() {
    let pool = blank_pool().await;
    let svc = MigrationService::new(pool);

    svc.apply_pending().await.expect("apply failed");
    let version = svc.get_current_version().await.expect("version check failed");

    assert_eq!(version, 7, "Expected version 7 after all migrations");
}

// ──────────────────────────────────────────────────────────
// 4. All expected tables exist after migration
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_all_tables_exist_after_migration() {
    let pool = common::setup_pool().await;

    let expected_tables = [
        "schema_version",
        "projects",
        "illustrations",
        "user_preferences",
        "typography_configs",
    ];

    for table in &expected_tables {
        let row = sqlx::query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name=?",
        )
        .bind(table)
        .fetch_optional(&pool)
        .await
        .unwrap_or_else(|e| panic!("Query failed for table '{}': {}", table, e));

        assert!(
            row.is_some(),
            "Expected table '{}' to exist after all migrations",
            table
        );
    }
}

// ──────────────────────────────────────────────────────────
// 5. verify_integrity() passes on fresh migrated DB
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_integrity_check_passes_on_migrated_db() {
    let pool = common::setup_pool().await;
    let svc = MigrationService::new(pool);

    let ok = svc.verify_integrity().await.expect("integrity_check failed");
    assert!(ok, "PRAGMA integrity_check must return 'ok' on a fresh migrated DB");
}

// ──────────────────────────────────────────────────────────
// 6. schema_version records all migration names
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_schema_version_records_all_migrations() {
    let pool = common::setup_pool().await;

    let rows = sqlx::query("SELECT migration_name FROM schema_version ORDER BY version ASC")
        .fetch_all(&pool)
        .await
        .expect("query failed");

    let names: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("migration_name"))
        .collect();

    assert_eq!(names.len(), 7, "Expected 7 entries in schema_version");
    assert_eq!(names[0], "M001_initial_schema");
    assert_eq!(names[6], "M007_cover_configs");
}

// ──────────────────────────────────────────────────────────
// 7. Default preference seeds from M001
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_m001_seeds_preference_defaults() {
    let pool = common::setup_pool().await;

    let rows = sqlx::query("SELECT key, value FROM user_preferences ORDER BY key ASC")
        .fetch_all(&pool)
        .await
        .expect("query failed");

    let map: std::collections::HashMap<String, String> = rows
        .iter()
        .map(|r| {
            (
                r.get::<String, _>("key"),
                r.get::<String, _>("value"),
            )
        })
        .collect();

    assert_eq!(map.get("theme"), Some(&"light".to_string()));
    assert_eq!(map.get("ui_language"), Some(&"pt-BR".to_string()));
    assert_eq!(map.get("analytics_opt_in"), Some(&"false".to_string()));
}

// ──────────────────────────────────────────────────────────
// 8. projects table has the expected columns (schema validation)
// ──────────────────────────────────────────────────────────

#[tokio::test]
async fn test_projects_table_has_expected_columns() {
    let pool = common::setup_pool().await;

    let rows = sqlx::query("PRAGMA table_info(projects)")
        .fetch_all(&pool)
        .await
        .expect("PRAGMA query failed");

    let columns: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("name"))
        .collect();

    let expected = [
        "id", "name", "bes_root_path", "book_config_path", "genre", "language",
        "config_version", "last_opened", "format_file_path", "created_at", "updated_at",
        "completeness_score", "completeness_level", "chapter_count", "illustration_count",
        "manuscript_root", "output_dir",
    ];

    for col in &expected {
        assert!(
            columns.contains(&col.to_string()),
            "Expected column '{}' in projects table. Found: {:?}",
            col,
            columns
        );
    }
}
