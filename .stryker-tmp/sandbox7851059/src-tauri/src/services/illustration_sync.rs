use std::collections::HashSet;

use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::{illustration::NewIllustration, IllustrationRef};
use crate::repositories::IllustrationRepository;

/// Synchronises `@ILLUSTRATION_PLACEHOLDER` entries from the parsed AST to SQLite.
///
/// Rules:
///  - New placeholder in AST, absent from DB → INSERT with `state = PENDING`
///  - Placeholder exists in DB, still in AST → keep (never overwrite state)
///  - Placeholder in DB but gone from AST   → DELETE only if still `PENDING`
///    (keep IMPORTED / LINKED — user work is preserved)
pub struct IllustrationSync;

impl IllustrationSync {
    pub async fn sync(
        pool: &SqlitePool,
        project_id: &str,
        illustrations: &[IllustrationRef],
    ) -> Result<(), AppError> {
        let repo = IllustrationRepository::new(pool.clone());

        // Fetch current DB state
        let existing = repo.find_by_project(project_id).await?;

        let existing_names: HashSet<String> =
            existing.iter().map(|i| i.placeholder_name.clone()).collect();

        let ast_names: HashSet<String> = illustrations.iter().map(|i| i.name.clone()).collect();

        // Insert new placeholders not yet in DB
        for illus in illustrations {
            if !existing_names.contains(&illus.name) {
                let new_illus = NewIllustration {
                    project_id: project_id.to_string(),
                    placeholder_name: illus.name.clone(),
                    description: if illus.description.is_empty() {
                        None
                    } else {
                        Some(illus.description.clone())
                    },
                };
                repo.create(&new_illus).await?;
            }
        }

        // Remove PENDING entries no longer present in AST
        for illus in &existing {
            if !ast_names.contains(&illus.placeholder_name) && illus.state == "pending" {
                sqlx::query("DELETE FROM illustrations WHERE id = ?")
                    .bind(&illus.id)
                    .execute(pool)
                    .await
                    .map_err(AppError::from)?;
            }
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Unit tests (TASK-4 ST004)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewProject;
    use crate::repositories::ProjectRepository;

    async fn setup_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let sql = include_str!("../../migrations/M001_initial_schema.sql");
        for stmt in sql.split(';') {
            let trimmed = stmt.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }
            sqlx::query(trimmed).execute(&pool).await.unwrap();
        }
        pool
    }

    async fn make_project(pool: &SqlitePool) -> String {
        use uuid::Uuid;
        ProjectRepository::new(pool.clone())
            .create(&NewProject {
                name: "Test".to_string(),
                bes_root_path: format!("/tmp/test-{}", Uuid::new_v4()),
                book_config_path: None,
                genre: None,
                language: None,
                config_version: None,
                manuscript_root: None,
                output_dir: None,
            })
            .await
            .unwrap()
            .id
    }

    fn make_ref(name: &str, desc: &str) -> IllustrationRef {
        IllustrationRef {
            name: name.to_string(),
            description: desc.to_string(),
            context: String::new(),
            line_number: 1,
            file_path: "/tmp/ch.md".to_string(),
        }
    }

    #[tokio::test]
    async fn test_illustration_sync_creates_pending() {
        let pool = setup_pool().await;
        let project_id = make_project(&pool).await;

        IllustrationSync::sync(
            &pool,
            &project_id,
            &[make_ref("hero-battle", "Final battle")],
        )
        .await
        .unwrap();

        let repo = IllustrationRepository::new(pool);
        let items = repo.find_by_project(&project_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].placeholder_name, "hero-battle");
        assert_eq!(items[0].state, "pending");
    }

    #[tokio::test]
    async fn test_illustration_sync_preserves_imported_state() {
        let pool = setup_pool().await;
        let project_id = make_project(&pool).await;
        let repo = IllustrationRepository::new(pool.clone());

        // Insert and mark as imported
        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "cover".to_string(),
                description: Some("Book cover".to_string()),
            })
            .await
            .unwrap();
        repo.update_state(&illus.id, "imported").await.unwrap();

        // Sync again — 'cover' still in AST → should NOT be deleted or overwritten
        IllustrationSync::sync(&pool, &project_id, &[make_ref("cover", "Book cover")])
            .await
            .unwrap();

        let items = repo.find_by_project(&project_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].state, "imported"); // preserved
    }

    #[tokio::test]
    async fn test_illustration_sync_removes_deleted_pending() {
        let pool = setup_pool().await;
        let project_id = make_project(&pool).await;
        let repo = IllustrationRepository::new(pool.clone());

        // Insert 'old-fig' as PENDING
        repo.create(&NewIllustration {
            project_id: project_id.clone(),
            placeholder_name: "old-fig".to_string(),
            description: None,
        })
        .await
        .unwrap();

        // Sync with empty AST → 'old-fig' was PENDING → deleted
        IllustrationSync::sync(&pool, &project_id, &[]).await.unwrap();

        let items = repo.find_by_project(&project_id).await.unwrap();
        assert!(items.is_empty());
    }

    #[tokio::test]
    async fn test_illustration_sync_keeps_imported_when_removed_from_ast() {
        let pool = setup_pool().await;
        let project_id = make_project(&pool).await;
        let repo = IllustrationRepository::new(pool.clone());

        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "linked-fig".to_string(),
                description: None,
            })
            .await
            .unwrap();
        repo.update_state(&illus.id, "linked").await.unwrap();

        // Sync with empty AST — 'linked-fig' is LINKED → kept
        IllustrationSync::sync(&pool, &project_id, &[]).await.unwrap();

        let items = repo.find_by_project(&project_id).await.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].state, "linked");
    }
}
