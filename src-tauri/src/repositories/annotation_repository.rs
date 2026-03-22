use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::Annotation;

pub struct AnnotationRepository {
    pool: SqlitePool,
}

impl AnnotationRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Add a new annotation to a page.
    pub async fn add(
        &self,
        project_id: &str,
        page_number: u32,
        x_percent: f64,
        y_percent: f64,
        annotation_type: &str,
        color: &str,
        content: &str,
    ) -> Result<Annotation, AppError> {
        let id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "INSERT INTO annotations
                (id, project_id, page_number, x_percent, y_percent,
                 annotation_type, color, content, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&id)
        .bind(project_id)
        .bind(page_number as i64)
        .bind(x_percent)
        .bind(y_percent)
        .bind(annotation_type)
        .bind(color)
        .bind(content)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;

        Ok(Annotation {
            id,
            project_id: project_id.to_string(),
            page_number,
            x_percent,
            y_percent,
            annotation_type: annotation_type.to_string(),
            color: color.to_string(),
            content: content.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    /// List annotations for a project filtered by page.
    pub async fn list_by_page(
        &self,
        project_id: &str,
        page_number: u32,
    ) -> Result<Vec<Annotation>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, page_number, x_percent, y_percent,
                    annotation_type, color, content, created_at, updated_at
             FROM annotations
             WHERE project_id = ? AND page_number = ?
             ORDER BY created_at ASC",
        )
        .bind(project_id)
        .bind(page_number as i64)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(Self::row_to_annotation).collect())
    }

    /// List all annotations for a project (all pages).
    pub async fn list_by_project(&self, project_id: &str) -> Result<Vec<Annotation>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, page_number, x_percent, y_percent,
                    annotation_type, color, content, created_at, updated_at
             FROM annotations
             WHERE project_id = ?
             ORDER BY page_number ASC, created_at ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.iter().map(Self::row_to_annotation).collect())
    }

    /// Delete an annotation by ID.
    pub async fn delete(&self, annotation_id: &str) -> Result<(), AppError> {
        sqlx::query("DELETE FROM annotations WHERE id = ?")
            .bind(annotation_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    fn row_to_annotation(row: &sqlx::sqlite::SqliteRow) -> Annotation {
        Annotation {
            id: row.get("id"),
            project_id: row.get("project_id"),
            page_number: row.get::<i64, _>("page_number") as u32,
            x_percent: row.get("x_percent"),
            y_percent: row.get("y_percent"),
            annotation_type: row.get("annotation_type"),
            color: row.get("color"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        // Create minimal schema
        sqlx::query(
            "CREATE TABLE projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                bes_root_path TEXT NOT NULL,
                book_config_path TEXT,
                genre TEXT,
                language TEXT NOT NULL DEFAULT 'pt-BR',
                config_version TEXT,
                last_opened TEXT,
                format_file_path TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                completeness_score REAL,
                completeness_level TEXT,
                chapter_count INTEGER,
                illustration_count INTEGER,
                manuscript_root TEXT,
                output_dir TEXT
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT INTO projects (id, name, bes_root_path, language, created_at, updated_at)
             VALUES ('proj-test', 'Test Project', '/tmp/bes', 'pt-BR', ?, ?)",
        )
        .bind(&now)
        .bind(&now)
        .execute(&pool)
        .await
        .unwrap();

        let annotation_sql = include_str!("../../migrations/M005_annotations.sql");
        for stmt in annotation_sql.split(';') {
            let trimmed = stmt.trim();
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }
            let _ = sqlx::query(trimmed).execute(&pool).await;
        }
        pool
    }

    #[tokio::test]
    async fn test_add_and_list_annotation() {
        let pool = setup_test_pool().await;
        let repo = AnnotationRepository::new(pool);

        let ann = repo
            .add("proj-test", 1, 50.0, 30.0, "comment", "#FFC107", "Good point")
            .await
            .unwrap();

        assert_eq!(ann.project_id, "proj-test");
        assert_eq!(ann.page_number, 1);
        assert_eq!(ann.annotation_type, "comment");
        assert_eq!(ann.content, "Good point");

        let listed = repo.list_by_page("proj-test", 1).await.unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, ann.id);
    }

    #[tokio::test]
    async fn test_list_by_project_all_pages() {
        let pool = setup_test_pool().await;
        let repo = AnnotationRepository::new(pool);

        repo.add("proj-test", 1, 10.0, 10.0, "comment", "#FFC107", "Page 1 note")
            .await
            .unwrap();
        repo.add("proj-test", 3, 20.0, 20.0, "flag", "#EF4444", "Issue here")
            .await
            .unwrap();

        let all = repo.list_by_project("proj-test").await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_annotation() {
        let pool = setup_test_pool().await;
        let repo = AnnotationRepository::new(pool);

        let ann = repo
            .add("proj-test", 2, 50.0, 50.0, "highlight", "#FFC107", "")
            .await
            .unwrap();

        repo.delete(&ann.id).await.unwrap();

        let listed = repo.list_by_page("proj-test", 2).await.unwrap();
        assert!(listed.is_empty());
    }
}
