use sqlx::{Row, SqlitePool};

use crate::error::AppError;

/// Manages SQLite schema migrations for the BES Book Formatter.
/// Migrations are embedded at compile time from the migrations/ directory.
pub struct MigrationService {
    pool: SqlitePool,
}

/// A single migration definition.
struct Migration {
    version: i32,
    name: &'static str,
    sql: &'static str,
}

/// All migrations in order. Add new migrations here.
const MIGRATIONS: &[Migration] = &[Migration {
    version: 1,
    name: "M001_initial_schema",
    sql: include_str!("../../migrations/M001_initial_schema.sql"),
}];

impl MigrationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Returns the current schema version (0 if no migrations applied).
    pub async fn get_current_version(&self) -> Result<i32, AppError> {
        // Ensure schema_version table exists
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS schema_version (
                version INTEGER PRIMARY KEY AUTOINCREMENT,
                migration_name TEXT NOT NULL,
                applied_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;

        let row = sqlx::query("SELECT COALESCE(MAX(version), 0) as ver FROM schema_version")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get::<i32, _>("ver"))
    }

    /// Applies all pending migrations and returns their names.
    pub async fn apply_pending(&self) -> Result<Vec<String>, AppError> {
        let current = self.get_current_version().await?;
        let mut applied = Vec::new();

        for migration in MIGRATIONS {
            if migration.version > current {
                log::info!(
                    "Applying migration: {} (v{})",
                    migration.name,
                    migration.version
                );

                // Execute migration SQL (may contain multiple statements)
                for statement in migration.sql.split(';') {
                    let trimmed = statement.trim();
                    if !trimmed.is_empty() {
                        sqlx::query(trimmed)
                            .execute(&self.pool)
                            .await
                            .map_err(|e| {
                                AppError::db_init_failed(format!(
                                    "Migration {} failed at statement: {} — error: {}",
                                    migration.name, trimmed, e
                                ))
                            })?;
                    }
                }

                applied.push(migration.name.to_string());
            }
        }

        Ok(applied)
    }

    /// Runs PRAGMA integrity_check on the database.
    pub async fn verify_integrity(&self) -> Result<bool, AppError> {
        let row = sqlx::query("PRAGMA integrity_check")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::db_init_failed(e.to_string()))?;

        let result: String = row.get(0);
        Ok(result == "ok")
    }
}
