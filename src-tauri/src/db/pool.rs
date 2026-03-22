use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::str::FromStr;

use crate::error::AppError;

/// Create a SQLite connection pool for the app's local data directory.
///
/// The database file is stored at `{app_local_data_dir}/bes-book-formatter.db`.
/// WAL mode is enabled for concurrent read during write operations.
pub async fn create_pool(app_data_dir: PathBuf) -> Result<SqlitePool, AppError> {
    // Ensure the directory exists
    tokio::fs::create_dir_all(&app_data_dir)
        .await
        .map_err(|e| AppError::db_permission_denied(&format!("{}: {}", app_data_dir.display(), e)))?;

    let db_path = app_data_dir.join("bes-book-formatter.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)
        .map_err(|e| AppError::db_init_failed(e.to_string()))?
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| AppError::db_init_failed(e.to_string()))?;

    Ok(pool)
}
