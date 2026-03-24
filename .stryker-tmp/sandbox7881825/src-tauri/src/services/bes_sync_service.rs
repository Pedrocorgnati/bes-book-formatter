// BES Book Formatter — BES Sync Service (module-6 TASK-0 scaffold / TASK-1 impl)
//
// Lê documentos BES do workspace do usuário (BDD.md, BOOK-ARCHITECTURE.md,
// METADATA.yaml, EDITORIAL-PROGRESS.md) com cache SQLite TTL 5 minutos.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

// ─── Structs de domínio ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BesMetadata {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub isbn: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub publication_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BesDocuments {
    pub bdd: String,
    pub book_architecture: String,
    pub metadata: BesMetadata,
    pub editorial_progress: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BesWorkspaceInfo {
    pub project_id: String,
    pub workspace_path: String,
    pub is_valid: bool,
    pub missing_files: Vec<String>,
    pub detected_files: Vec<String>,
}

// ─── Row SQLite ────────────────────────────────────────────────────────────

#[derive(Debug, sqlx::FromRow)]
struct CacheRow {
    content: String,
    file_hash: String,
    cached_at: String,
}

// ─── Serviço ───────────────────────────────────────────────────────────────

pub struct BesSyncService {
    db: Arc<SqlitePool>,
    /// TTL em minutos (padrão 5).
    cache_ttl_minutes: u64,
}

impl BesSyncService {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        let ttl = std::env::var("CACHE_TTL_MINUTES")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(5);
        Self { db, cache_ttl_minutes: ttl }
    }

    // ── Leitura de docs ────────────────────────────────────────────────

    pub async fn read_bes_docs(
        &self,
        project_id: &str,
        workspace_path: &str,
    ) -> Result<BesDocuments, String> {
        let ws = Path::new(workspace_path);

        // Verifica cache para cada doc
        let bdd = self.read_doc_cached(project_id, ws, "bdd", "BDD.md").await?;
        let architecture = self
            .read_doc_cached(project_id, ws, "book_architecture", "BOOK-ARCHITECTURE.md")
            .await
            .unwrap_or_default();
        let metadata_raw = self
            .read_doc_cached(project_id, ws, "metadata", "METADATA.yaml")
            .await
            .or_else(|_| self.read_doc_cached_sync(ws, "METADATA.yml"))
            .unwrap_or_default();
        let editorial_raw = self
            .read_doc_cached(project_id, ws, "editorial_progress", "EDITORIAL-PROGRESS.md")
            .await
            .unwrap_or_default();

        let metadata = self.parse_metadata(&metadata_raw)?;

        Ok(BesDocuments {
            bdd,
            book_architecture: architecture,
            metadata,
            editorial_progress: editorial_raw,
        })
    }

    pub async fn get_bes_metadata(
        &self,
        project_id: &str,
        workspace_path: &str,
    ) -> Result<Option<BesMetadata>, String> {
        let ws = Path::new(workspace_path);
        let raw = self
            .read_doc_cached(project_id, ws, "metadata", "METADATA.yaml")
            .await
            .or_else(|_| {
                let yml_path = ws.join("METADATA.yml");
                std::fs::read_to_string(&yml_path).map_err(|e| e.to_string())
            });

        match raw {
            Ok(content) => Ok(Some(self.parse_metadata(&content)?)),
            Err(_) => Ok(None),
        }
    }

    pub async fn validate_bes_workspace(
        &self,
        path: &str,
    ) -> Result<BesWorkspaceInfo, String> {
        let ws = Path::new(path);
        let required = ["BDD.md"];
        let optional = [
            "BOOK-ARCHITECTURE.md",
            "METADATA.yaml",
            "METADATA.yml",
            "EDITORIAL-PROGRESS.md",
        ];

        let mut missing = Vec::new();
        let mut detected = Vec::new();

        for f in &required {
            let p = ws.join(f);
            if p.exists() {
                detected.push(f.to_string());
            } else {
                missing.push(f.to_string());
            }
        }

        for f in &optional {
            if ws.join(f).exists() {
                detected.push(f.to_string());
            }
        }

        // Verifica pasta de capítulos (qualquer subpasta com arquivos .md)
        let has_chapters = ws
            .read_dir()
            .map(|mut rd| {
                rd.any(|e| {
                    e.map(|entry| entry.path().is_dir()).unwrap_or(false)
                })
            })
            .unwrap_or(false);

        if !has_chapters {
            missing.push("pasta de capítulos".to_string());
        }

        let is_valid = missing.is_empty();

        Ok(BesWorkspaceInfo {
            project_id: String::new(),
            workspace_path: path.to_string(),
            is_valid,
            missing_files: missing,
            detected_files: detected,
        })
    }

    pub async fn invalidate_cache(&self, project_id: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM bes_document_cache WHERE project_id = ?")
            .bind(project_id)
            .execute(self.db.as_ref())
            .await
            .map_err(|e| format!("Cache invalidation error: {e}"))?;
        Ok(())
    }

    // ── Helpers privados ───────────────────────────────────────────────

    async fn read_doc_cached(
        &self,
        project_id: &str,
        workspace: &Path,
        doc_type: &str,
        filename: &str,
    ) -> Result<String, String> {
        let file_path = workspace.join(filename);
        let content_disk = std::fs::read_to_string(&file_path)
            .map_err(|e| format!("{filename} não encontrado em {}: {e}", workspace.display()))?;

        let hash = Self::sha256_hex(&content_disk);

        // Tentar cache hit
        let row: Option<CacheRow> = sqlx::query_as(
            "SELECT content, file_hash, cached_at FROM bes_document_cache \
             WHERE project_id = ? AND document_type = ?",
        )
        .bind(project_id)
        .bind(doc_type)
        .fetch_optional(self.db.as_ref())
        .await
        .map_err(|e| format!("Cache query error: {e}"))?;

        if let Some(row) = row {
            if row.file_hash == hash && self.is_row_fresh(&row.cached_at) {
                return Ok(row.content);
            }
        }

        // Cache miss ou stale — armazenar novo
        sqlx::query(
            "INSERT INTO bes_document_cache \
             (project_id, document_type, content, file_path, file_hash, cached_at) \
             VALUES (?, ?, ?, ?, ?, datetime('now')) \
             ON CONFLICT(project_id, document_type) DO UPDATE SET \
               content = excluded.content, \
               file_path = excluded.file_path, \
               file_hash = excluded.file_hash, \
               cached_at = excluded.cached_at",
        )
        .bind(project_id)
        .bind(doc_type)
        .bind(&content_disk)
        .bind(file_path.to_string_lossy().as_ref())
        .bind(&hash)
        .execute(self.db.as_ref())
        .await
        .map_err(|e| format!("Cache write error: {e}"))?;

        Ok(content_disk)
    }

    fn read_doc_cached_sync(&self, workspace: &Path, filename: &str) -> Result<String, String> {
        std::fs::read_to_string(workspace.join(filename)).map_err(|e| e.to_string())
    }

    fn is_row_fresh(&self, cached_at: &str) -> bool {
        // cached_at é "YYYY-MM-DD HH:MM:SS" (SQLite datetime)
        use std::time::{SystemTime, UNIX_EPOCH};

        let Ok(cache_dt) = chrono::NaiveDateTime::parse_from_str(cached_at, "%Y-%m-%d %H:%M:%S")
        else {
            return false;
        };
        let cache_ts = cache_dt.and_utc().timestamp() as u64;
        let now_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        now_ts.saturating_sub(cache_ts) < self.cache_ttl_minutes * 60
    }

    fn sha256_hex(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn parse_metadata(&self, raw: &str) -> Result<BesMetadata, String> {
        // Suporta frontmatter YAML (---\n...\n---) ou YAML puro
        let yaml_str = if raw.starts_with("---") {
            raw.splitn(3, "---").nth(1).unwrap_or(raw)
        } else {
            raw
        };

        let value: serde_yaml::Value = serde_yaml::from_str(yaml_str)
            .map_err(|e| format!("METADATA.yaml inválido: {e}"))?;

        let get_str = |key: &str| -> String {
            value[key]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or_default()
        };
        let get_opt_str = |key: &str| -> Option<String> {
            value[key].as_str().map(|s| s.to_string())
        };
        let get_keywords = || -> Option<Vec<String>> {
            value["keywords"].as_sequence().map(|seq| {
                seq.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
        };

        Ok(BesMetadata {
            title: get_str("title"),
            author: get_str("author"),
            genre: get_str("genre"),
            isbn: get_opt_str("isbn"),
            description: get_opt_str("description"),
            keywords: get_keywords(),
            language: get_opt_str("language"),
            publisher: get_opt_str("publisher"),
            publication_date: get_opt_str("publication_date"),
        })
    }
}

// ─── Testes ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn make_workspace(files: &[(&str, &str)]) -> TempDir {
        let dir = tempfile::tempdir().unwrap();
        for (name, content) in files {
            let path = dir.path().join(name);
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut f = std::fs::File::create(&path).unwrap();
            f.write_all(content.as_bytes()).unwrap();
        }
        dir
    }

    fn svc() -> BesSyncService {
        // Para testes unitários de parse/hash/validate (sem acesso real ao DB),
        // criamos um pool lazy com in-memory SQLite. Os testes que usam svc()
        // não invocam queries SQL.
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();
        BesSyncService::new(Arc::new(pool))
    }

    #[test]
    fn test_sha256_hex_deterministic() {
        let h1 = BesSyncService::sha256_hex("hello");
        let h2 = BesSyncService::sha256_hex("hello");
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn test_sha256_hex_different_inputs() {
        let h1 = BesSyncService::sha256_hex("hello");
        let h2 = BesSyncService::sha256_hex("world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_parse_metadata_yaml_plain() {
        let svc = svc();
        let yaml = "title: Meu Livro\nauthor: João Silva\ngenre: Fiction\nisbn: \"978-0-1234\"\n";
        let meta = svc.parse_metadata(yaml).unwrap();
        assert_eq!(meta.title, "Meu Livro");
        assert_eq!(meta.author, "João Silva");
        assert_eq!(meta.genre, "Fiction");
        assert_eq!(meta.isbn, Some("978-0-1234".to_string()));
    }

    #[test]
    fn test_parse_metadata_with_frontmatter() {
        let svc = svc();
        let yaml = "---\ntitle: Com Frontmatter\nauthor: Maria\ngenre: Romance\n---\n";
        let meta = svc.parse_metadata(yaml).unwrap();
        assert_eq!(meta.title, "Com Frontmatter");
        assert_eq!(meta.author, "Maria");
    }

    #[test]
    fn test_parse_metadata_invalid() {
        let svc = svc();
        let bad = "{{{{not valid yaml}}}}";
        // serde_yaml pode retornar Ok com value String — apenas garantir sem panic
        let _ = svc.parse_metadata(bad); // pode Ok ou Err, não deve panic
    }

    #[tokio::test]
    async fn test_validate_bes_workspace_valid() {
        let svc = svc();
        let dir = make_workspace(&[
            ("BDD.md", "# BDD"),
            ("chapters/ch01.md", "# Cap 1"),
        ]);
        let info = svc.validate_bes_workspace(dir.path().to_str().unwrap()).await.unwrap();
        assert!(info.is_valid);
        assert!(info.missing_files.is_empty());
        assert!(info.detected_files.contains(&"BDD.md".to_string()));
    }

    #[tokio::test]
    async fn test_validate_bes_workspace_missing_bdd() {
        let svc = svc();
        let dir = make_workspace(&[("chapters/ch01.md", "# Cap 1")]);
        let info = svc.validate_bes_workspace(dir.path().to_str().unwrap()).await.unwrap();
        assert!(!info.is_valid);
        assert!(info.missing_files.contains(&"BDD.md".to_string()));
    }

    #[test]
    fn test_is_row_fresh_recent() {
        let svc = svc();
        // Data recente (agora)
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        assert!(svc.is_row_fresh(&now));
    }

    #[test]
    fn test_is_row_fresh_expired() {
        let svc = svc();
        // Data de 10 min atrás — deve expirar com TTL=5min
        let old = (chrono::Utc::now() - chrono::Duration::minutes(10))
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        assert!(!svc.is_row_fresh(&old));
    }
}
