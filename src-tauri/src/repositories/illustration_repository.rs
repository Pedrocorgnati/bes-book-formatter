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
