// BES Book Formatter — CLI State (module-6 TASK-3)
//
// CliState gerencia SQLite standalone e repositórios para o CLI.
// Sem Tauri app — a pool é criada diretamente para o processo CLI.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;

use crate::repositories::{IllustrationRepository, ProjectRepository};
use crate::services::{BesSyncService, EditorialProgressService};

pub struct CliState {
    pub pool: SqlitePool,
    pub project_repo: ProjectRepository,
    pub illustration_repo: IllustrationRepository,
    pub bes_sync_svc: BesSyncService,
    pub editorial_svc: EditorialProgressService,
}

impl CliState {
    /// Inicializa o estado CLI buscando o banco de dados do projeto BES.
    /// Tenta localizar `bes-book-formatter.db` no diretório de dados da app.
    pub async fn new(project_path: &str) -> Result<Self, String> {
        let db_path = Self::resolve_db_path(project_path)?;
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let options = SqliteConnectOptions::from_str(&db_url)
            .map_err(|e| format!("Configuração do banco inválida: {e}"))?
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .foreign_keys(true)
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(3)
            .connect_with(options)
            .await
            .map_err(|e| format!("Falha ao conectar ao banco: {e}"))?;

        let pool_arc = std::sync::Arc::new(pool.clone());

        Ok(Self {
            project_repo: ProjectRepository::new(pool.clone()),
            illustration_repo: IllustrationRepository::new(pool.clone()),
            bes_sync_svc: BesSyncService::new(pool_arc),
            editorial_svc: EditorialProgressService::new(),
            pool,
        })
    }

    fn resolve_db_path(project_path: &str) -> Result<std::path::PathBuf, String> {
        // 1. Verifica se há .bes-output/.bes-book-formatter.db no projeto
        let local_db = Path::new(project_path)
            .join(".bes-output")
            .join("bes-book-formatter.db");
        if local_db.exists() {
            return Ok(local_db);
        }

        // 2. Fallback: diretório de dados da app (~/.local/share/bes-book-formatter/)
        let data_dir = dirs_next::data_local_dir()
            .ok_or("Não foi possível determinar o diretório de dados da app")?
            .join("bes-book-formatter")
            .join("bes-book-formatter.db");

        Ok(data_dir)
    }
}
