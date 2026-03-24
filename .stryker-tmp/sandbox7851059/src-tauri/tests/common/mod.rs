/// Shared helpers for all integration tests.
///
/// Usage in each test file:
///   mod common;
///   use common::{setup_pool, make_project, make_new_project};

use bes_book_formatter_lib::{
    models::{NewIllustration, NewProject},
    repositories::ProjectRepository,
    services::MigrationService,
};
use sqlx::SqlitePool;
use uuid::Uuid;

// ──────────────────────────────────────────────────────────
// DB setup
// ──────────────────────────────────────────────────────────

/// Creates an in-memory SQLite pool with **all migrations applied**.
///
/// Each call returns a fresh, isolated database — no state shared between tests.
pub async fn setup_pool() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory SQLite pool");

    let svc = MigrationService::new(pool.clone());
    svc.apply_pending()
        .await
        .expect("Failed to apply migrations");

    pool
}

// ──────────────────────────────────────────────────────────
// Factories
// ──────────────────────────────────────────────────────────

/// Builds a `NewProject` with unique bes_root_path to avoid UNIQUE conflicts.
pub fn make_new_project(name: &str) -> NewProject {
    NewProject {
        name: name.to_string(),
        bes_root_path: format!("/tmp/bes-integration-{}", Uuid::new_v4()),
        book_config_path: None,
        genre: Some("fiction".to_string()),
        language: Some("pt-BR".to_string()),
        config_version: Some("v1".to_string()),
        manuscript_root: None,
        output_dir: None,
    }
}

/// Creates a project in the DB and returns its ID.
pub async fn make_project(pool: &SqlitePool) -> String {
    make_project_named(pool, "Integration Test Book").await
}

/// Creates a project with a given name and returns its ID.
pub async fn make_project_named(pool: &SqlitePool, name: &str) -> String {
    let repo = ProjectRepository::new(pool.clone());
    repo.create(&make_new_project(name))
        .await
        .unwrap_or_else(|e| panic!("Failed to create test project '{}': {}", name, e))
        .id
}

/// Builds a `NewIllustration` for a given project.
pub fn make_new_illustration(project_id: &str, name: &str) -> NewIllustration {
    NewIllustration {
        project_id: project_id.to_string(),
        placeholder_name: name.to_string(),
        description: Some(format!("Test illustration: {}", name)),
    }
}
