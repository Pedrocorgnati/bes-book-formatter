use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{
    ApiResponse, Annotation, LayoutIssue, PreviewPageResponse, PreviewResult, TypoIssuePreview,
};
use crate::repositories::{AnnotationRepository, PreferenceRepository};
use crate::services::preview_service::PreviewService;
use crate::services::typography_service::TypographyService;
use crate::repositories::ProjectRepository;

// ---------------------------------------------------------------------------
// Legacy stubs (kept for compatibility)
// ---------------------------------------------------------------------------

/// Legacy stub — use render_preview_page instead.
///
/// Frontend: `const result = await invoke<ApiResponse<PreviewResult>>('render_preview', { projectId, page: 1 });`
#[tauri::command]
pub async fn render_preview(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page: u32,
) -> Result<ApiResponse<PreviewResult>, String> {
    let _ = (pool, project_id, page);
    Ok(ApiResponse::err(
        "SYS_050: Deprecated — use render_preview_page",
    ))
}

/// Detect orphans and widows (heuristic, module-5 TASK-3).
///
/// Frontend: `const issues = await invoke<ApiResponse<Vec<TypoIssuePreview>>>('detect_orphans_widows', { projectId });`
#[tauri::command]
pub async fn detect_orphans_widows(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Vec<TypoIssuePreview>>, String> {
    let pool_ref = pool.inner().clone();
    let project_repo = ProjectRepository::new(pool_ref.clone());

    let project = match project_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!("Project not found: {}", project_id)));
        }
        Err(e) => return Ok(ApiResponse::err(e.to_string())),
    };

    let manuscript_root = match project.manuscript_root.as_deref() {
        Some(r) => r.to_string(),
        None => return Ok(ApiResponse::ok(vec![])),
    };

    // Parse manuscript for heuristic analysis
    let manuscript =
        match crate::services::parser_service::ParserService::parse_manuscript(&project_id, &manuscript_root).await {
            Ok(m) => m,
            Err(e) => return Ok(ApiResponse::err(e.to_string())),
        };

    let typo_svc = TypographyService::new(pool_ref);
    let config = match typo_svc.get_typography_config(&project_id).await {
        Ok(c) => c,
        Err(e) => return Ok(ApiResponse::err(e.to_string())),
    };

    // Heuristic detection:
    // - Widow: last paragraph ends with < 3 words on its own "line"
    // - Orphan: paragraph's first "line" would be isolated at page bottom
    // Since we lack actual page layout, we estimate based on word counts and paragraph length.
    let chars_per_line = ((config.page_width - config.margin_inner - config.margin_outer) * 72.0
        / (config.font_size_body * 0.6)) as usize;
    let lines_per_page = ((config.page_height - config.margin_top - config.margin_bottom) * 72.0
        / (config.font_size_body * config.leading)) as usize;

    let mut issues: Vec<TypoIssuePreview> = Vec::new();
    let mut estimated_page = 1u32;
    let mut lines_on_page = 0usize;

    for chapter in manuscript.chapters.iter().chain(manuscript.back_matter.iter()) {
        for para in chapter.content.split("\n\n") {
            let para = para.trim();
            if para.is_empty() || para.starts_with('#') {
                continue;
            }

            let word_count = para.split_whitespace().count();
            let char_count = para.chars().count();
            let para_lines = (char_count / chars_per_line.max(1)).max(1);

            // Check for widow: last line of paragraph has < 3 words
            let last_line_words = (word_count % ((chars_per_line / 6).max(1))).min(word_count);
            if last_line_words > 0 && last_line_words < 3 && para_lines > 1 {
                let y_percent = (lines_on_page as f64 / lines_per_page.max(1) as f64 * 100.0)
                    .min(95.0);
                issues.push(TypoIssuePreview {
                    issue_type: "widow".to_string(),
                    page_number: estimated_page,
                    line_text: para.chars().take(50).collect::<String>() + "…",
                    line_y_percent: y_percent,
                    severity: "warning".to_string(),
                });
            }

            // Check for orphan: first line would be alone at page break
            if lines_on_page + 1 >= lines_per_page && para_lines > 1 {
                issues.push(TypoIssuePreview {
                    issue_type: "orphan".to_string(),
                    page_number: estimated_page,
                    line_text: para.chars().take(50).collect::<String>() + "…",
                    line_y_percent: 90.0,
                    severity: "warning".to_string(),
                });
            }

            lines_on_page += para_lines + 1; // +1 for paragraph spacing
            if lines_on_page >= lines_per_page {
                estimated_page += 1;
                lines_on_page = 0;
            }
        }
    }

    Ok(ApiResponse::ok(issues))
}

// ---------------------------------------------------------------------------
// Module-5 TASK-1: Page-Spread Viewer IPC Commands
// ---------------------------------------------------------------------------

/// Render a page (or spread) to PNG base64. Core preview command.
///
/// Frontend: `const res = await invoke<ApiResponse<PreviewPageResponse>>('render_preview_page', { projectId, page, zoom, spread });`
#[tauri::command]
pub async fn render_preview_page(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page: u32,
    zoom: f32,
    spread: bool,
) -> Result<ApiResponse<PreviewPageResponse>, String> {
    let svc = PreviewService::new(pool.inner().clone());
    match svc.render_page(&project_id, page, zoom, spread).await {
        Ok(r) => Ok(ApiResponse::ok(r)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Get total page count for a project (uses cached render if available).
///
/// Frontend: `const count = await invoke<ApiResponse<number>>('get_page_count', { projectId });`
#[tauri::command]
pub async fn get_page_count(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<u32>, String> {
    let svc = PreviewService::new(pool.inner().clone());
    match svc.get_page_count(&project_id).await {
        Ok(count) => Ok(ApiResponse::ok(count)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Persist last visited page in user_preferences.
///
/// Frontend: `await invoke('navigate_to_page', { projectId, page });`
#[tauri::command]
pub async fn navigate_to_page(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page: u32,
) -> Result<ApiResponse<()>, String> {
    if page < 1 {
        return Ok(ApiResponse::err("Page must be >= 1"));
    }
    let pref_repo = PreferenceRepository::new(pool.inner().clone());
    let key = format!("preview_page_{}", project_id);
    match pref_repo.set_raw(&key, &page.to_string()).await {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Persist zoom level in user_preferences.
///
/// Frontend: `await invoke('set_zoom_level', { projectId, zoom });`
#[tauri::command]
pub async fn set_zoom_level(
    pool: State<'_, SqlitePool>,
    project_id: String,
    zoom: f32,
) -> Result<ApiResponse<()>, String> {
    let valid_zooms = [0.5f32, 0.75, 1.0, 1.25, 1.5, 0.0];
    if !valid_zooms.contains(&zoom) {
        return Ok(ApiResponse::err(format!(
            "Invalid zoom: {}. Valid: 0.5, 0.75, 1.0, 1.25, 1.5, 0.0",
            zoom
        )));
    }
    let pref_repo = PreferenceRepository::new(pool.inner().clone());
    let key = format!("preview_zoom_{}", project_id);
    match pref_repo.set_raw(&key, &zoom.to_string()).await {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Persist spread view preference.
///
/// Frontend: `await invoke('toggle_spread_view', { projectId, enabled });`
#[tauri::command]
pub async fn toggle_spread_view(
    pool: State<'_, SqlitePool>,
    project_id: String,
    enabled: bool,
) -> Result<ApiResponse<()>, String> {
    let pref_repo = PreferenceRepository::new(pool.inner().clone());
    let key = format!("preview_spread_{}", project_id);
    match pref_repo.set_raw(&key, if enabled { "true" } else { "false" }).await {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Module-5 TASK-3: Distraction-Free, Annotations, Orphan/Widow
// ---------------------------------------------------------------------------

/// Toggle distraction-free mode and emit Tauri event.
///
/// Frontend: `await invoke('toggle_distraction_free', { enabled });`
#[tauri::command]
pub async fn toggle_distraction_free(
    pool: State<'_, SqlitePool>,
    app_handle: tauri::AppHandle,
    enabled: bool,
) -> Result<ApiResponse<()>, String> {
    let pref_repo = PreferenceRepository::new(pool.inner().clone());
    if let Err(e) = pref_repo
        .set_raw("distraction_free", if enabled { "true" } else { "false" })
        .await
    {
        return Ok(ApiResponse::err(e.to_string()));
    }
    // Emit event so frontend can hide/show sidebar and header
    let _ = app_handle.emit(
        "preview:distraction-free",
        serde_json::json!({ "enabled": enabled }),
    );
    Ok(ApiResponse::ok(()))
}

/// Add an annotation to a page.
///
/// Frontend: `const ann = await invoke<ApiResponse<Annotation>>('add_annotation', { ... });`
#[tauri::command]
pub async fn add_annotation(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page_number: u32,
    x_percent: f64,
    y_percent: f64,
    annotation_type: String,
    color: String,
    content: String,
) -> Result<ApiResponse<Annotation>, String> {
    // Validations
    if page_number < 1 {
        return Ok(ApiResponse::err("page_number must be >= 1"));
    }
    if !(0.0..=100.0).contains(&x_percent) || !(0.0..=100.0).contains(&y_percent) {
        return Ok(ApiResponse::err(
            "x_percent and y_percent must be between 0 and 100",
        ));
    }
    if !["comment", "highlight", "flag"].contains(&annotation_type.as_str()) {
        return Ok(ApiResponse::err(
            "annotation_type must be one of: comment, highlight, flag",
        ));
    }
    if content.len() > 1000 {
        return Ok(ApiResponse::err("content exceeds 1000 characters"));
    }
    if !color.starts_with('#') || color.len() != 7 {
        return Ok(ApiResponse::err("color must be a hex color like #FF5733"));
    }

    let repo = AnnotationRepository::new(pool.inner().clone());
    match repo
        .add(
            &project_id,
            page_number,
            x_percent,
            y_percent,
            &annotation_type,
            &color,
            &content,
        )
        .await
    {
        Ok(ann) => Ok(ApiResponse::ok(ann)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// List annotations for a project (optionally filtered by page).
///
/// Frontend: `const anns = await invoke<ApiResponse<Annotation[]>>('get_annotations', { projectId, pageNumber });`
#[tauri::command]
pub async fn get_annotations(
    pool: State<'_, SqlitePool>,
    project_id: String,
    page_number: Option<u32>,
) -> Result<ApiResponse<Vec<Annotation>>, String> {
    let repo = AnnotationRepository::new(pool.inner().clone());
    let result = match page_number {
        Some(page) => repo.list_by_page(&project_id, page).await,
        None => repo.list_by_project(&project_id).await,
    };
    match result {
        Ok(anns) => Ok(ApiResponse::ok(anns)),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

/// Delete an annotation by ID.
///
/// Frontend: `await invoke('delete_annotation', { annotationId });`
#[tauri::command]
pub async fn delete_annotation(
    pool: State<'_, SqlitePool>,
    annotation_id: String,
) -> Result<ApiResponse<()>, String> {
    let repo = AnnotationRepository::new(pool.inner().clone());
    match repo.delete(&annotation_id).await {
        Ok(_) => Ok(ApiResponse::ok(())),
        Err(e) => Ok(ApiResponse::err(e.to_string())),
    }
}

// ---------------------------------------------------------------------------
// Cache invalidation (called by typography and parser commands)
// ---------------------------------------------------------------------------

/// Invalidate preview cache after typography change (TASK-2 live preview).
pub async fn invalidate_preview_cache(pool: &SqlitePool, project_id: &str) {
    let svc = PreviewService::new(pool.clone());
    svc.invalidate_cache(project_id).await;
}
