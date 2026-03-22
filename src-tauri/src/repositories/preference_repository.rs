use sqlx::{Row, SqlitePool};
use std::collections::HashMap;

use crate::error::AppError;

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
