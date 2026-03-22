use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{ApiResponse, GenerationResult, PreflightResult, ValidationResult};

/// Generate EPUB 3.3 for a project.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_epub', { projectId, platform: 'kdp' });`
#[tauri::command]
pub async fn generate_epub(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
) -> Result<ApiResponse<GenerationResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project + illustrations from DB
    // 2. Load manuscript AST (cached or re-parse)
    // 3. Apply Handlebars templates for EPUB
    // 4. Process LINKED illustrations via Sharp
    // 5. Generate .epub file in output/books/{slug}/
    // 6. Run EPUBCheck validation
    // 7. Return GenerationResult

    let _ = (pool, project_id, platform);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — generate_epub. Run /auto-flow execute",
    ))
}

/// Generate PDF (ebook or print) for a project.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_pdf', { projectId, format: 'pdf_ebook', platform: 'kdp' });`
#[tauri::command]
pub async fn generate_pdf(
    pool: State<'_, SqlitePool>,
    project_id: String,
    format: String,
    platform: String,
) -> Result<ApiResponse<GenerationResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project from DB
    // 2. Generate Typst source (.typ) from manuscript AST
    // 3. spawn_typst: pass 1 (pagination) + pass 2 (TOC + refs)
    // 4. If pdf_print: spawn_ghostscript for PDF/X conversion with ICC profile
    // 5. Write .pdf to output/books/{slug}/
    // 6. Return GenerationResult

    let _ = (pool, project_id, format, platform);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — generate_pdf. Run /auto-flow execute",
    ))
}

/// Validate an EPUB file using EPUBCheck.
///
/// Frontend: `const result = await invoke<ApiResponse<ValidationResult>>('run_epubcheck', { epubPath });`
#[tauri::command]
pub async fn run_epubcheck(epub_path: String) -> Result<ApiResponse<ValidationResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Verify epub_path exists
    // 2. SidecarManager::spawn_epubcheck(epub_path, 30_000)
    // 3. Parse JSON output from EPUBCheck
    // 4. Return ValidationResult

    let _ = epub_path;
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — run_epubcheck. Run /auto-flow execute",
    ))
}

/// Run preflight checks before generation.
///
/// Frontend: `const result = await invoke<ApiResponse<PreflightResult>>('run_preflight', { projectId });`
#[tauri::command]
pub async fn run_preflight(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<PreflightResult>, String> {
    // TODO: Implementar via /auto-flow execute
    // 1. Load project completeness_level from DB
    // 2. Check illustration states (all should be LINKED for print)
    // 3. Check sidecar availability for target format
    // 4. Build blocking/warning items
    // 5. Return PreflightResult

    let _ = (pool, project_id);
    Ok(ApiResponse::err(
        "SYS_050: Not implemented — run_preflight. Run /auto-flow execute",
    ))
}
