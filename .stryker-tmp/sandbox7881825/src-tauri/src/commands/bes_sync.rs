// BES Book Formatter — BES Sync IPC Commands (module-6 TASK-0 / TASK-1)
//
// 4 IPC commands expostos ao frontend via Tauri:
//   - validate_bes_workspace
//   - read_bes_docs
//   - get_bes_metadata
//   - invalidate_bes_cache

use sqlx::SqlitePool;
use std::sync::Arc;
use tauri::State;

use crate::services::bes_sync_service::{BesDocuments, BesMetadata, BesSyncService, BesWorkspaceInfo};

/// Valida se um diretório é um workspace BES válido.
/// Verifica presença de BDD.md + pasta de capítulos.
#[tauri::command]
pub async fn validate_bes_workspace(
    workspace_path: String,
    pool: State<'_, SqlitePool>,
) -> Result<BesWorkspaceInfo, String> {
    let svc = BesSyncService::new(Arc::new(pool.inner().clone()));
    svc.validate_bes_workspace(&workspace_path).await
}

/// Lê os 4 documentos BES do workspace com cache TTL 5 minutos.
/// Retorna BDD.md, BOOK-ARCHITECTURE.md, METADATA.yaml e EDITORIAL-PROGRESS.md.
#[tauri::command]
pub async fn read_bes_docs(
    project_id: String,
    workspace_path: String,
    pool: State<'_, SqlitePool>,
) -> Result<BesDocuments, String> {
    let svc = BesSyncService::new(Arc::new(pool.inner().clone()));
    svc.read_bes_docs(&project_id, &workspace_path).await
}

/// Extrai apenas os metadados do METADATA.yaml (título, autor, gênero, ISBN, etc.).
#[tauri::command]
pub async fn get_bes_metadata(
    project_id: String,
    workspace_path: String,
    pool: State<'_, SqlitePool>,
) -> Result<Option<BesMetadata>, String> {
    let svc = BesSyncService::new(Arc::new(pool.inner().clone()));
    svc.get_bes_metadata(&project_id, &workspace_path).await
}

/// Invalida todo o cache BES de um projeto (força re-leitura na próxima chamada).
#[tauri::command]
pub async fn invalidate_bes_cache(
    project_id: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let svc = BesSyncService::new(Arc::new(pool.inner().clone()));
    svc.invalidate_cache(&project_id).await
}
