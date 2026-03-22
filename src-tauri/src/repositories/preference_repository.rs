use sqlx::{Row, SqlitePool};
use std::collections::HashMap;

use crate::error::AppError;

const VALID_KEYS: &[&str] = &["theme", "ui_language", "analytics_opt_in"];

pub struct PreferenceRepository {
    pool: SqlitePool,
}

impl PreferenceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let row = sqlx::query("SELECT value FROM user_preferences WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|r| r.get::<String, _>("value")))
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        if !VALID_KEYS.contains(&key) {
            return Err(AppError::new(
                "PREF_001",
                &format!("Invalid preference key: {}. Valid keys: {:?}", key, VALID_KEYS),
            ));
        }

        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT OR REPLACE INTO user_preferences (key, value, updated_at) VALUES (?, ?, ?)",
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Set a preference value without key validation (for dynamic keys like preview state).
    pub async fn set_raw(&self, key: &str, value: &str) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            "INSERT OR REPLACE INTO user_preferences (key, value, updated_at) VALUES (?, ?, ?)",
        )
        .bind(key)
        .bind(value)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_all(&self) -> Result<HashMap<String, String>, AppError> {
        let rows = sqlx::query("SELECT key, value FROM user_preferences")
            .fetch_all(&self.pool)
            .await?;

        let mut map = HashMap::new();
        for row in &rows {
            map.insert(
                row.get::<String, _>("key"),
                row.get::<String, _>("value"),
            );
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[tokio::test]
    async fn test_set_and_get() {
        let pool = setup_test_pool().await;
        let repo = PreferenceRepository::new(pool);

        // Set a valid key
        repo.set("theme", "dark").await.unwrap();
        let value = repo.get("theme").await.unwrap();
        assert_eq!(value, Some("dark".to_string()));
    }

    #[tokio::test]
    async fn test_set_invalid_key_returns_pref_001() {
        let pool = setup_test_pool().await;
        let repo = PreferenceRepository::new(pool);

        let result = repo.set("invalid_key", "value").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.code, "PREF_001");
    }

    #[tokio::test]
    async fn test_get_all_with_defaults() {
        let pool = setup_test_pool().await;
        let repo = PreferenceRepository::new(pool);

        let all = repo.get_all().await.unwrap();
        // Migration seeds: theme=light, ui_language=pt-BR, analytics_opt_in=false
        assert_eq!(all.get("theme"), Some(&"light".to_string()));
        assert_eq!(all.get("ui_language"), Some(&"pt-BR".to_string()));
        assert_eq!(all.get("analytics_opt_in"), Some(&"false".to_string()));
    }
}
