// BES Book Formatter — Generation IPC Commands (module-4 TASK-0 → TASK-5)
//
// 8 comandos IPC para geração de saídas:
//   generate_epub, generate_pdf, generate_pdf_print, generate_pdf_ebook,
//   generate_docx, get_generation_results, run_epubcheck, run_preflight,
//   cancel_generation

use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::responses::{ApiResponse, GenerationResult, PreflightResult, ValidationResult};
use crate::services::{DocxService, EpubService, HtmlService, PdfEbookService, PdfPrintService, PreflightService};

// ── generate_epub ─────────────────────────────────────────────────────────

/// Gera EPUB 3.3 completo para um projeto.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_epub', { projectId, platform: 'kdp' });`
#[tauri::command]
pub async fn generate_epub(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
) -> Result<ApiResponse<GenerationResult>, String> {
    match EpubService::generate(&pool, &project_id, &platform).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── generate_pdf (legado — delega para generate_pdf_print ou generate_pdf_ebook) ──

/// Gera PDF para um projeto. Delega para o serviço correto baseado no `format`.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_pdf', { projectId, format: 'pdf_ebook', platform: 'kdp' });`
#[tauri::command]
pub async fn generate_pdf(
    pool: State<'_, SqlitePool>,
    project_id: String,
    format: String,
    platform: String,
) -> Result<ApiResponse<GenerationResult>, String> {
    let result = match format.as_str() {
        "pdf_print" | "pdf_x1a" => {
            PdfPrintService::generate(&pool, &project_id, &platform, "pdf_x1a").await
        }
        _ => PdfEbookService::generate(&pool, &project_id, &platform).await,
    };

    match result {
        Ok(r) => Ok(ApiResponse::ok(r)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── generate_pdf_print ────────────────────────────────────────────────────

/// Gera PDF para impressão (PDF/X-1a ou PDF/X-4) via Typst + Ghostscript.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_pdf_print', { projectId, platform: 'kdp_print', pdfxProfile: 'pdf_x1a' });`
#[tauri::command]
pub async fn generate_pdf_print(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
    pdfx_profile: Option<String>,
) -> Result<ApiResponse<GenerationResult>, String> {
    let profile = pdfx_profile.as_deref().unwrap_or("pdf_x1a");
    match PdfPrintService::generate(&pool, &project_id, &platform, profile).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── generate_pdf_ebook ────────────────────────────────────────────────────

/// Gera PDF/A-3 para e-book via Typst (RGB, hyperlinks, tagged PDF).
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_pdf_ebook', { projectId, platform: 'kdp' });`
#[tauri::command]
pub async fn generate_pdf_ebook(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
) -> Result<ApiResponse<GenerationResult>, String> {
    match PdfEbookService::generate(&pool, &project_id, &platform).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── generate_docx ─────────────────────────────────────────────────────────

/// Gera DOCX com estilos Word nomeados.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_docx', { projectId, platform: 'generic' });`
#[tauri::command]
pub async fn generate_docx(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: Option<String>,
) -> Result<ApiResponse<GenerationResult>, String> {
    let plat = platform.as_deref().unwrap_or("generic");
    match DocxService::generate(&pool, &project_id, plat).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── get_generation_results ────────────────────────────────────────────────

/// Retorna histórico de gerações para um projeto.
///
/// Frontend: `const results = await invoke<ApiResponse<StoredGenerationResult[]>>('get_generation_results', { projectId });`
#[tauri::command]
pub async fn get_generation_results(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Vec<crate::models::generation::StoredGenerationResult>>, String> {
    let results = sqlx::query_as::<_, DbGenResult>(
        "SELECT id, project_id, format, platform, output_path, file_size_bytes, duration_ms,
                status, errors, warnings, created_at
         FROM generation_results
         WHERE project_id = ?
         ORDER BY created_at DESC
         LIMIT 50",
    )
    .bind(&project_id)
    .fetch_all(&*pool)
    .await;

    match results {
        Ok(rows) => {
            let mapped: Vec<crate::models::generation::StoredGenerationResult> = rows
                .into_iter()
                .map(|r| crate::models::generation::StoredGenerationResult {
                    id: r.id,
                    project_id: r.project_id,
                    format: r.format,
                    platform: r.platform,
                    output_path: r.output_path,
                    file_size_bytes: r.file_size_bytes,
                    duration_ms: r.duration_ms,
                    status: r.status,
                    errors: r.errors,
                    warnings: r.warnings,
                    created_at: r.created_at,
                })
                .collect();
            Ok(ApiResponse::ok(mapped))
        }
        Err(e) => Ok(ApiResponse::err(format!("GEN_080: DB error: {}", e))),
    }
}

// ── run_epubcheck ─────────────────────────────────────────────────────────

/// Valida um arquivo EPUB via EPUBCheck.
///
/// Frontend: `const result = await invoke<ApiResponse<ValidationResult>>('run_epubcheck', { epubPath });`
#[tauri::command]
pub async fn run_epubcheck(epub_path: String) -> Result<ApiResponse<ValidationResult>, String> {
    match EpubService::run_epubcheck(&epub_path).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── run_preflight ─────────────────────────────────────────────────────────

/// Executa checklist pré-geração para um projeto.
///
/// Frontend: `const result = await invoke<ApiResponse<PreflightResult>>('run_preflight', { projectId, format: 'epub3' });`
#[tauri::command]
pub async fn run_preflight(
    pool: State<'_, SqlitePool>,
    project_id: String,
    format: Option<String>,
) -> Result<ApiResponse<PreflightResult>, String> {
    let fmt = format.as_deref().unwrap_or("epub3");
    match PreflightService::pre_generation_check(&pool, &project_id, fmt).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── generate_html ─────────────────────────────────────────────────────────

/// Gera HTML5 responsivo para um projeto.
///
/// Frontend: `const result = await invoke<ApiResponse<GenerationResult>>('generate_html', { projectId, platform: 'generic' });`
#[tauri::command]
pub async fn generate_html(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: Option<String>,
) -> Result<ApiResponse<GenerationResult>, String> {
    let plat = platform.as_deref().unwrap_or("generic");
    match HtmlService::generate(&pool, &project_id, plat).await {
        Ok(result) => Ok(ApiResponse::ok(result)),
        Err(e) => Ok(ApiResponse::err(format!("{}: {}", e.code, e.message))),
    }
}

// ── cancel_generation ─────────────────────────────────────────────────────

/// Cancela uma geração em andamento (best-effort: mata processos filhos).
///
/// Frontend: `await invoke('cancel_generation', { projectId });`
#[tauri::command]
pub async fn cancel_generation(project_id: String) -> Result<ApiResponse<bool>, String> {
    // TODO(module-4): Implementar cancelamento real de processos filhos (Typst/GS/EPUBCheck).
    // Requer refatoração dos services para aceitar CancellationToken e manter referência
    // ao child process PID. Por enquanto, apenas sinaliza intenção de cancelamento.
    log::info!("cancel_generation called for {}", project_id);
    Ok(ApiResponse::ok(true))
}

// ── DB row helper ─────────────────────────────────────────────────────────

#[derive(sqlx::FromRow)]
struct DbGenResult {
    id: String,
    project_id: String,
    format: String,
    platform: String,
    output_path: Option<String>,
    file_size_bytes: Option<i64>,
    duration_ms: Option<i64>,
    status: String,
    errors: String,
    warnings: String,
    created_at: String,
}
