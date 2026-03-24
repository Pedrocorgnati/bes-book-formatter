// BES Book Formatter — BesDocumentCacheRepository (module-6 TASK-1)
//
// CRUD sobre a tabela `bes_document_cache` com TTL e validação por hash.

use sqlx::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
pub struct BesDocumentCacheRow {
    pub id: String,
    pub project_id: String,
    pub document_type: String,
    pub content: String,
    pub parsed_json: Option<String>,
    pub file_path: String,
    pub file_hash: String,
    pub cached_at: String,
}

pub struct BesDocumentCacheRepository {
    pool: SqlitePool,
}

impl BesDocumentCacheRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Busca uma entrada de cache por project_id + document_type.
    pub async fn get(
        &self,
        project_id: &str,
        document_type: &str,
    ) -> Result<Option<BesDocumentCacheRow>, sqlx::Error> {
        sqlx::query_as::<_, BesDocumentCacheRow>(
            "SELECT id, project_id, document_type, content, parsed_json, \
             file_path, file_hash, cached_at \
             FROM bes_document_cache \
             WHERE project_id = ? AND document_type = ?",
        )
        .bind(project_id)
        .bind(document_type)
        .fetch_optional(&self.pool)
        .await
    }

    /// Insere ou atualiza uma entrada de cache (UPSERT).
    pub async fn upsert(
        &self,
        project_id: &str,
        document_type: &str,
        content: &str,
        file_path: &str,
        file_hash: &str,
        parsed_json: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO bes_document_cache \
             (project_id, document_type, content, file_path, file_hash, parsed_json, cached_at) \
             VALUES (?, ?, ?, ?, ?, ?, datetime('now')) \
             ON CONFLICT(project_id, document_type) DO UPDATE SET \
               content = excluded.content, \
               file_path = excluded.file_path, \
               file_hash = excluded.file_hash, \
               parsed_json = excluded.parsed_json, \
               cached_at = excluded.cached_at",
        )
        .bind(project_id)
        .bind(document_type)
        .bind(content)
        .bind(file_path)
        .bind(file_hash)
        .bind(parsed_json)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Remove todas as entradas de cache de um projeto (invalidação total).
    pub async fn invalidate_project(&self, project_id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM bes_document_cache WHERE project_id = ?",
        )
        .bind(project_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Remove entrada específica de cache.
    pub async fn invalidate_entry(
        &self,
        project_id: &str,
        document_type: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM bes_document_cache WHERE project_id = ? AND document_type = ?",
        )
        .bind(project_id)
        .bind(document_type)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Retorna true se a entrada existe e está dentro do TTL informado (em minutos).
    pub fn is_fresh(row: &BesDocumentCacheRow, ttl_minutes: u64) -> bool {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};

        let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&row.cached_at, "%Y-%m-%d %H:%M:%S")
        else {
            return false;
        };
        let cache_ts = dt.and_utc().timestamp() as u64;
        let now_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        now_ts.saturating_sub(cache_ts) < ttl_minutes * 60
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fresh_recent() {
        let row = BesDocumentCacheRow {
            id: "x".to_string(),
            project_id: "p".to_string(),
            document_type: "bdd".to_string(),
            content: "c".to_string(),
            parsed_json: None,
            file_path: "/f".to_string(),
            file_hash: "h".to_string(),
            cached_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        };
        assert!(BesDocumentCacheRepository::is_fresh(&row, 5));
    }

    #[test]
    fn test_is_fresh_expired() {
        let row = BesDocumentCacheRow {
            id: "x".to_string(),
            project_id: "p".to_string(),
            document_type: "bdd".to_string(),
            content: "c".to_string(),
            parsed_json: None,
            file_path: "/f".to_string(),
            file_hash: "h".to_string(),
            cached_at: (chrono::Utc::now() - chrono::Duration::minutes(10))
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        };
        assert!(!BesDocumentCacheRepository::is_fresh(&row, 5));
    }

    #[test]
    fn test_is_fresh_invalid_date() {
        let row = BesDocumentCacheRow {
            id: "x".to_string(),
            project_id: "p".to_string(),
            document_type: "bdd".to_string(),
            content: "c".to_string(),
            parsed_json: None,
            file_path: "/f".to_string(),
            file_hash: "h".to_string(),
            cached_at: "not-a-date".to_string(),
        };
        assert!(!BesDocumentCacheRepository::is_fresh(&row, 5));
    }
}
