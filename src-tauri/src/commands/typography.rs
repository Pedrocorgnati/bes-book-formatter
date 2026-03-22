use sqlx::SqlitePool;
use tauri::{Manager, State};

use crate::error::AppError;
use crate::models::typography::{DpiValidation, TypographyConfig, UpdateTypographyConfig};
use crate::models::ApiResponse;
use crate::services::{FontInfo, FontService, TypographyService};

/// Get the typography configuration for a project.
/// Creates defaults based on the project's genre if none exists.
///
/// Frontend: `const config = await invoke<ApiResponse<TypographyConfig>>('get_typography_config', { projectId });`
#[tauri::command]
pub async fn get_typography_config(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<TypographyConfig>, String> {
    let svc = TypographyService::new(pool.inner().clone());
    match svc.get_typography_config(&project_id).await {
        Ok(config) => Ok(ApiResponse::ok(config)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Save the full typography configuration for a project.
/// Validates all fields (VAL_001-003) before persisting.
/// Emits `preview:config-changed` so the live preview can re-render (TASK-2).
///
/// Frontend: `await invoke('set_typography_config', { projectId, config });`
#[tauri::command]
pub async fn set_typography_config(
    pool: State<'_, SqlitePool>,
    app_handle: tauri::AppHandle,
    project_id: String,
    config: UpdateTypographyConfig,
) -> Result<ApiResponse<TypographyConfig>, String> {
    let svc = TypographyService::new(pool.inner().clone());
    match svc.update_typography_config(&project_id, config).await {
        Ok(updated) => {
            // Invalidate preview cache so live preview re-renders with new typography
            crate::commands::preview::invalidate_preview_cache(pool.inner(), &project_id).await;
            // Emit live preview event — frontend debounces 300ms before re-rendering
            let _ = app_handle.emit(
                "preview:config-changed",
                serde_json::json!({ "projectId": project_id, "changed": "typography" }),
            );
            Ok(ApiResponse::ok(updated))
        }
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Validate the DPI of an image file without processing it.
/// Returns DpiValidation with dpi, adequate flag, and optional warning message.
/// Performance: reads only header bytes, < 500ms.
///
/// Frontend: `const result = await invoke<ApiResponse<DpiValidation>>('validate_illustration_dpi', { filePath });`
#[tauri::command]
pub async fn validate_illustration_dpi(
    pool: State<'_, SqlitePool>,
    file_path: String,
) -> Result<ApiResponse<DpiValidation>, String> {
    let svc = crate::services::IllustrationService::new(pool.inner().clone());
    match svc.validate_illustration_dpi(&file_path).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Process an illustration: validate DPI, update state to IMPORTED.
/// Full processing pipeline (resize, convert, compress) implemented in TASK-5.
///
/// Frontend: `const illus = await invoke<ApiResponse<Illustration>>('process_illustration', { illustrationId, filePath, projectId });`
#[tauri::command]
pub async fn process_illustration(
    pool: State<'_, SqlitePool>,
    illustration_id: String,
    file_path: String,
    project_id: String,
) -> Result<ApiResponse<crate::models::illustration::Illustration>, String> {
    let svc = crate::services::IllustrationService::new(pool.inner().clone());
    match svc
        .process_illustration(&illustration_id, &file_path, &project_id)
        .await
    {
        Ok(illus) => Ok(ApiResponse::ok(illus)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Open a native file dialog so the user can select an OTF/TTF font file.
/// Returns the selected path or None if cancelled.
///
/// Frontend: `const path = await invoke<string | null>('select_font_file');`
#[tauri::command]
pub async fn select_font_file(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let file = app
        .dialog()
        .file()
        .set_title("Selecionar fonte OTF/TTF")
        .add_filter("Fontes OTF/TTF", &["otf", "ttf"])
        .blocking_pick_file();
    Ok(file.map(|f| f.to_string()))
}

/// List all available fonts for a project (bundled + custom uploaded).
///
/// Frontend: `const fonts = await invoke<ApiResponse<FontInfo[]>>('list_fonts', { projectId });`
#[tauri::command]
pub async fn list_fonts(
    app: tauri::AppHandle,
    project_id: String,
) -> Result<ApiResponse<Vec<FontInfo>>, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("SYS_001: Could not resolve app data dir: {}", e))?;
    let resource = app.path().resource_dir()
        .map_err(|e| format!("SYS_001: Could not resolve resource dir: {}", e))?;
    match FontService::list_fonts(&project_id, &app_data, &resource) {
        Ok(fonts) => Ok(ApiResponse::ok(fonts)),
        Err(e) => Ok(ApiResponse::err(e)),
    }
}

/// Upload a custom font (OTF/TTF) for a project.
///
/// Frontend: `await invoke('upload_font', { projectId, filePath });`
#[tauri::command]
pub async fn upload_font(
    app: tauri::AppHandle,
    project_id: String,
    file_path: String,
) -> Result<ApiResponse<FontInfo>, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("SYS_001: Could not resolve app data dir: {}", e))?;
    match FontService::upload_font(&project_id, &file_path, &app_data) {
        Ok(info) => Ok(ApiResponse::ok(info)),
        Err(e) => Ok(ApiResponse::err(e)),
    }
}

/// Delete a custom font for a project by name.
///
/// Frontend: `await invoke('delete_custom_font', { projectId, fontName });`
#[tauri::command]
pub async fn delete_custom_font(
    app: tauri::AppHandle,
    project_id: String,
    font_name: String,
) -> Result<ApiResponse<()>, String> {
    let app_data = app.path().app_data_dir()
        .map_err(|e| format!("SYS_001: Could not resolve app data dir: {}", e))?;
    match FontService::delete_custom_font(&project_id, &font_name, &app_data) {
        Ok(()) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e)),
    }
}

/// List all illustrations for a project.
///
/// Frontend: `const list = await invoke<ApiResponse<Illustration[]>>('list_illustrations', { projectId });`
#[tauri::command]
pub async fn list_illustrations(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Vec<crate::models::illustration::Illustration>>, String> {
    let svc = crate::services::IllustrationService::new(pool.inner().clone());
    match svc.list_illustrations(&project_id).await {
        Ok(list) => Ok(ApiResponse::ok(list)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Update the alt-text for an illustration and transition it to LINKED state.
///
/// Frontend: `await invoke('update_illustration_alt_text', { illustrationId, altText });`
#[tauri::command]
pub async fn update_illustration_alt_text(
    pool: State<'_, SqlitePool>,
    illustration_id: String,
    alt_text: String,
) -> Result<ApiResponse<crate::models::illustration::Illustration>, String> {
    let svc = crate::services::IllustrationService::new(pool.inner().clone());
    match svc.update_alt_text(&illustration_id, &alt_text).await {
        Ok(illus) => Ok(ApiResponse::ok(illus)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}
