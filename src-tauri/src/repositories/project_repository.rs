use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{NewProject, Project, UpdateProject};

pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Option<Project>, AppError> {
        let row = sqlx::query(
            "SELECT id, name, bes_root_path, book_config_path, genre, language,
                    config_version, last_opened, format_file_path,
                    created_at, updated_at, completeness_score,
                    completeness_level, chapter_count, illustration_count,
                    manuscript_root, output_dir
             FROM projects WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Self::row_to_project(&r)))
    }

    pub async fn find_all_recent(&self, limit: u32) -> Result<Vec<Project>, AppError> {
        let rows = sqlx::query(
            "SELECT id, name, bes_root_path, book_config_path, genre, language,
                    config_version, last_opened, format_file_path,
                    created_at, updated_at, completeness_score,
                    completeness_level, chapter_count, illustration_count,
                    manuscript_root, output_dir
             FROM projects ORDER BY last_opened DESC NULLS LAST, created_at DESC
             LIMIT ?",
        )
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(|r| Self::row_to_project(r)).collect())
    }

    pub async fn create(&self, data: &NewProject) -> Result<Project, AppError> {
        let id = Uuid::new_v4().to_string();
        let language = data.language.as_deref().unwrap_or("pt-BR");
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO projects (id, name, bes_root_path, book_config_path, genre,
                                   language, config_version, created_at, updated_at,
                                   manuscript_root, output_dir)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(&data.name)
        .bind(&data.bes_root_path)
        .bind(&data.book_config_path)
        .bind(&data.genre)
        .bind(language)
        .bind(&data.config_version)
        .bind(&now)
        .bind(&now)
        .bind(&data.manuscript_root)
        .bind(&data.output_dir)
        .execute(&self.pool)
        .await?;

        self.find_by_id(&id)
            .await?
            .ok_or_else(|| AppError::sys_internal("Failed to read project after insert"))
    }

    pub async fn update(&self, id: &str, data: &UpdateProject) -> Result<Project, AppError> {
        let now = chrono::Utc::now().to_rfc3339();

        // Build dynamic SET clauses
        let mut sets: Vec<String> = vec!["updated_at = ?1".to_string()];
        let mut values: Vec<Box<dyn sqlx::Encode<'_, sqlx::Sqlite> + Send + Sync>> = vec![];

        // For simplicity, use a fixed query with COALESCE to handle optionals
        sqlx::query(
            "UPDATE projects SET
                updated_at = ?,
                name = COALESCE(?, name),
                genre = COALESCE(?, genre),
                language = COALESCE(?, language),
                completeness_score = COALESCE(?, completeness_score),
                completeness_level = COALESCE(?, completeness_level),
                chapter_count = COALESCE(?, chapter_count),
                illustration_count = COALESCE(?, illustration_count)
             WHERE id = ?",
        )
        .bind(&now)
        .bind(&data.name)
        .bind(&data.genre)
        .bind(&data.language)
        .bind(data.completeness_score)
        .bind(&data.completeness_level)
        .bind(data.chapter_count)
        .bind(data.illustration_count)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::project_not_found(id))
    }

    pub async fn update_last_opened(&self, id: &str) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE projects SET last_opened = ?, updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(&now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM projects WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn find_by_bes_root(&self, bes_root_path: &str) -> Result<Option<Project>, AppError> {
        let row = sqlx::query(
            "SELECT id, name, bes_root_path, book_config_path, genre, language,
                    config_version, last_opened, format_file_path,
                    created_at, updated_at, completeness_score,
                    completeness_level, chapter_count, illustration_count,
                    manuscript_root, output_dir
             FROM projects WHERE bes_root_path = ?",
        )
        .bind(bes_root_path)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Self::row_to_project(&r)))
    }

    fn row_to_project(row: &sqlx::sqlite::SqliteRow) -> Project {
        Project {
            id: row.get("id"),
            name: row.get("name"),
            bes_root_path: row.get("bes_root_path"),
            book_config_path: row.get("book_config_path"),
            genre: row.get("genre"),
            language: row.get("language"),
            config_version: row.get("config_version"),
            last_opened: row.get("last_opened"),
            format_file_path: row.get("format_file_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            completeness_score: row.get("completeness_score"),
            completeness_level: row.get("completeness_level"),
            chapter_count: row.get("chapter_count"),
            illustration_count: row.get("illustration_count"),
            manuscript_root: row.get("manuscript_root"),
            output_dir: row.get("output_dir"),
        }
    }
}
