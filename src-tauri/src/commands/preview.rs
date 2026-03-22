use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{ApiResponse, LayoutIssue, PreviewResult};

/// Render a single page preview using Typst.
///
/// Frontend: `const result = await invoke<ApiResponse<PreviewResult>>('render_preview', { projectId, page: 1 });`
#[tauri::command]
pub async fn render_preview(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page: u32,
) -> Result<ApiResponse<PreviewResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project from DB
    // 2. Generate Typst source for the requested page
    // 3. SidecarManager::spawn_typst with single-page render args
    // 4. Parse SVG output from Typst
    // 5. Return PreviewResult with SVG content

    let _ = (pool, project_id, page);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — render_preview. Run /auto-flow execute",
    ))
}

/// Detect orphans and widows in the layout.
///
/// Frontend: `const issues = await invoke<ApiResponse<Vec<LayoutIssue>>>('detect_orphans_widows', { projectId });`
#[tauri::command]
pub async fn detect_orphans_widows(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Vec<LayoutIssue>>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project from DB
    // 2. Full Typst render to get all pages
    // 3. Analyze page breaks for orphans/widows/short pages
    // 4. Return list of LayoutIssue

    let _ = (pool, project_id);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — detect_orphans_widows. Run /auto-flow execute",
    ))
}
