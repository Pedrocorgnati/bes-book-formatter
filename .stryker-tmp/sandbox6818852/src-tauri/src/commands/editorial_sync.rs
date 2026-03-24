// BES Book Formatter — Editorial Sync IPC Command (module-6 TASK-2)
//
// IPC command: sync_editorial_progress
// Lê EDITORIAL-PROGRESS.md do workspace BES, atualiza APENAS F10 com
// resultados de geração recentes, preserva F1-F9 e F11-F12.

use sqlx::SqlitePool;
use tauri::State;

use crate::services::editorial_progress_service::{EditorialProgress, EditorialProgressService};

/// Sincroniza o progresso editorial lendo EDITORIAL-PROGRESS.md.
/// Retorna o estado atual de todas as 12 fases (F1-F12).
/// Não modifica o arquivo — apenas lê e parseia.
#[tauri::command]
pub async fn sync_editorial_progress(
    project_id: String,
    workspace_path: String,
    project_name: String,
    pool: State<'_, SqlitePool>,
) -> Result<EditorialProgress, String> {
    let _ = pool; // pool disponível para futura integração com GenerationRepository
    let svc = EditorialProgressService::new();
    svc.read(&workspace_path, &project_name)
}

/// Atualiza a coluna F10 com os resultados de geração mais recentes.
/// Modifica o arquivo EDITORIAL-PROGRESS.md no workspace — APENAS linha F10.
/// F1-F9 e F11-F12 são preservados integralmente.
#[tauri::command]
pub async fn update_editorial_f10(
    project_id: String,
    workspace_path: String,
    project_name: String,
    formats_generated: Vec<String>,
    output_path: String,
    pool: State<'_, SqlitePool>,
) -> Result<EditorialProgress, String> {
    let _ = pool;
    let svc = EditorialProgressService::new();
    svc.update_f10(&workspace_path, &project_name, &formats_generated, &output_path)
}
