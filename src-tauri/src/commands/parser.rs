use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{
    ApiResponse, BookConfig, ChecklistResult, CompletenessResult, ManuscriptAst,
};
use crate::services::FilesystemService;

/// Parse a BES manuscript and return its AST.
///
/// Frontend: `const ast = await invoke<ApiResponse<ManuscriptAst>>('parse_manuscript', { projectId });`
#[tauri::command]
pub async fn parse_manuscript(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<ManuscriptAst>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project from DB to get manuscript_root
    // 2. Read all .md files from manuscript_root
    // 3. Parse with unified.js/remark equivalent (or call Node sidecar)
    // 4. Build AST with chapters, front/back matter, illustrations
    // 5. Return ManuscriptAst

    let _ = (pool, project_id);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — parse_manuscript. Run /auto-flow execute",
    ))
}

/// Read and validate a book-config.json file.
///
/// Frontend: `const config = await invoke<ApiResponse<BookConfig>>('read_book_config', { path });`
#[tauri::command]
pub async fn read_book_config(path: String) -> Result<ApiResponse<BookConfig>, String> {
    match FilesystemService::read_book_config(&path).await {
        Ok(config) => {
            let mut warnings = Vec::new();

            // Warn about unknown config version
            if let Some(ref version) = config.version {
                if !["v1", "v2", "v3"].contains(&version.as_str()) {
                    warnings.push(format!(
                        "CONFIG_002: Unknown book config version '{}'. Falling back to V1.",
                        version
                    ));
                }
            }

            if warnings.is_empty() {
                Ok(ApiResponse::ok(config))
            } else {
                Ok(ApiResponse::ok_with_warnings(config, warnings))
            }
        }
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Calculate manuscript completeness score.
///
/// Frontend: `const result = await invoke<ApiResponse<CompletenessResult>>('calculate_completeness', { projectId });`
#[tauri::command]
pub async fn calculate_completeness(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<CompletenessResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project from DB
    // 2. Read manuscript files
    // 3. Calculate: chapters with content / total chapters
    // 4. Classify: < 0.80 = blocking, 0.80-0.95 = warning, >= 0.95 = normal
    // 5. Update project record with score/level
    // 6. Return CompletenessResult

    let _ = (pool, project_id);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — calculate_completeness. Run /auto-flow execute",
    ))
}

/// Run content checklist before generation.
///
/// Frontend: `const result = await invoke<ApiResponse<ChecklistResult>>('run_content_checklist', { projectId });`
#[tauri::command]
pub async fn run_content_checklist(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<ChecklistResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project and illustrations from DB
    // 2. Check: completeness level, empty chapters, unlinked illustrations
    // 3. Build blocking/warning/info items
    // 4. Return ChecklistResult

    let _ = (pool, project_id);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — run_content_checklist. Run /auto-flow execute",
    ))
}
