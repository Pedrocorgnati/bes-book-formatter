use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{Illustration, NewIllustration};

pub struct IllustrationRepository {
    pool: SqlitePool,
}

impl IllustrationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_project(&self, project_id: &str) -> Result<Vec<Illustration>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE project_id = ? ORDER BY created_at ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(Self::row_to_illustration).collect())
    }

    pub async fn find_by_state(
        &self,
        project_id: &str,
        state: &str,
    ) -> Result<Vec<Illustration>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE project_id = ? AND state = ?",
        )
        .bind(project_id)
        .bind(state)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(Self::row_to_illustration).collect())
    }

    pub async fn create(&self, data: &NewIllustration) -> Result<Illustration, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO illustrations (id, project_id, placeholder_name, description,
                                        state, created_at, updated_at)
             VALUES (?, ?, ?, ?, 'pending', ?, ?)",
        )
        .bind(&id)
        .bind(&data.project_id)
        .bind(&data.placeholder_name)
        .bind(&data.description)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        self.find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::sys_internal("Failed to read illustration after insert"))
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Illustration>, AppError> {
        let row = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.as_ref().map(Self::row_to_illustration))
    }

    pub async fn update_state(&self, id: &str, state: &str) -> Result<Illustration, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE illustrations SET state = ?, updated_at = ? WHERE id = ?")
            .bind(state)
            .bind(&now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::sys_internal("Illustration not found after update"))
    }

    pub async fn update_image(
        &self,
        id: &str,
        path: &str,
        dpi: i32,
        width: Option<i32>,
        height: Option<i32>,
        color_space: Option<&str>,
    ) -> Result<Illustration, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE illustrations
             SET image_path = ?, validated_dpi = ?, width_px = ?, height_px = ?,
                 color_space = ?, state = 'imported', updated_at = ?
             WHERE id = ?",
        )
        .bind(path)
        .bind(dpi)
        .bind(width)
        .bind(height)
        .bind(color_space)
        .bind(&now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::sys_internal("Illustration not found after update"))
    }

    pub async fn count_by_project(&self, project_id: &str) -> Result<i32, AppError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM illustrations WHERE project_id = ?")
            .bind(project_id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i32, _>("count"))
    }

    pub async fn find_pending(&self, project_id: &str) -> Result<Vec<Illustration>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE project_id = ? AND state = 'pending' ORDER BY created_at ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(Self::row_to_illustration).collect())
    }

    pub async fn update_alt_text(&self, id: &str, alt_text: &str) -> Result<Illustration, AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE illustrations SET alt_text = ?, updated_at = ? WHERE id = ?")
            .bind(alt_text)
            .bind(&now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::sys_internal("Illustration not found after update"))
    }

    fn row_to_illustration(row: &sqlx::sqlite::SqliteRow) -> Illustration {
        Illustration {
            id: row.get("id"),
            project_id: row.get("project_id"),
            placeholder_name: row.get("placeholder_name"),
            description: row.get("description"),
            state: row.get("state"),
            image_path: row.get("image_path"),
            validated_dpi: row.get("validated_dpi"),
            alt_text: row.get("alt_text"),
            width_px: row.get("width_px"),
            height_px: row.get("height_px"),
            color_space: row.get("color_space"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::NewProject;
    use crate::repositories::ProjectRepository;
    use sqlx::SqlitePool;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        let migration_sql = include_str!("../../migrations/M001_initial_schema.sql");
        for statement in migration_sql.split(';') {
            let trimmed = statement.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }
            sqlx::query(trimmed).execute(&pool).await.unwrap();
        }
        pool
    }

    /// Creates a project and returns its ID (illustrations have FK to projects).
    async fn create_test_project(pool: &SqlitePool) -> String {
        let repo = ProjectRepository::new(pool.clone());
        let project = repo
            .create(&NewProject {
                name: "Test Book".to_string(),
                bes_root_path: format!("/tmp/test-{}", Uuid::new_v4()),
                book_config_path: None,
                genre: None,
                language: None,
                config_version: None,
                manuscript_root: None,
                output_dir: None,
            })
            .await
            .unwrap();
        project.id
    }

    #[tokio::test]
    async fn test_create_and_find_by_project() {
        let pool = setup_test_pool().await;
        let project_id = create_test_project(&pool).await;
        let repo = IllustrationRepository::new(pool);

        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "cover".to_string(),
                description: Some("Book cover".to_string()),
            })
            .await
            .unwrap();

        assert_eq!(illus.placeholder_name, "cover");
        assert_eq!(illus.state, "pending");

        let list = repo.find_by_project(&project_id).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, illus.id);
    }

    #[tokio::test]
    async fn test_update_state() {
        let pool = setup_test_pool().await;
        let project_id = create_test_project(&pool).await;
        let repo = IllustrationRepository::new(pool);

        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "ch1-fig1".to_string(),
                description: None,
            })
            .await
            .unwrap();

        assert_eq!(illus.state, "pending");

        let updated = repo.update_state(&illus.id, "imported").await.unwrap();
        assert_eq!(updated.state, "imported");
    }

    #[tokio::test]
    async fn test_find_pending() {
        let pool = setup_test_pool().await;
        let project_id = create_test_project(&pool).await;
        let repo = IllustrationRepository::new(pool);

        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "fig-a".to_string(),
                description: None,
            })
            .await
            .unwrap();

        // Initially pending
        let pending = repo.find_pending(&project_id).await.unwrap();
        assert_eq!(pending.len(), 1);

        // After state change, no longer pending
        repo.update_state(&illus.id, "imported").await.unwrap();
        let pending = repo.find_pending(&project_id).await.unwrap();
        assert!(pending.is_empty());
    }

    #[tokio::test]
    async fn test_update_alt_text() {
        let pool = setup_test_pool().await;
        let project_id = create_test_project(&pool).await;
        let repo = IllustrationRepository::new(pool);

        let illus = repo
            .create(&NewIllustration {
                project_id: project_id.clone(),
                placeholder_name: "fig-alt".to_string(),
                description: None,
            })
            .await
            .unwrap();

        assert!(illus.alt_text.is_none());

        let updated = repo.update_alt_text(&illus.id, "A sunset over the mountains").await.unwrap();
        assert_eq!(updated.alt_text, Some("A sunset over the mountains".to_string()));
    }
}
