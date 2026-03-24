use regex::Regex;
use sqlx::SqlitePool;
use std::path::Path;
use tauri::State;

use crate::error::AppError;
use crate::models::{
    enums::{Genre, ManuscriptCompleteness},
    genre_defaults,
    responses::{PageDimensionsResponse, TypographyDefaultsResponse},
    ApiResponse, BookConfig, ChecklistItem, ChecklistResult, CompletenessResult, Illustration,
    ManuscriptAst, ChapterNode, SectionNode, IllustrationPlaceholder, ManuscriptMetadata,
};
use crate::repositories::{IllustrationRepository, ProjectRepository};
use crate::services::{BookConfigService, CompletenessService, IllustrationSync, ParserService};

// ---------------------------------------------------------------------------
// IPC: parse_manuscript
// ---------------------------------------------------------------------------

/// Parse the BES manuscript for a project and return its AST.
///
/// Also calculates completeness, syncs illustration placeholders to SQLite,
/// and updates the project record.
///
/// Frontend:
/// ```ts
/// const ast = await invoke<ApiResponse<ManuscriptAst>>('parse_manuscript', { projectId });
/// ```
#[tauri::command]
pub async fn parse_manuscript(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<ManuscriptAst>, String> {
    if project_id.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: project_id cannot be empty"));
    }

    let db = pool.inner().clone();
    let proj_repo = ProjectRepository::new(db.clone());

    let project = match proj_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_081: Project not found: {}",
                project_id
            )))
        }
        Err(e) => return Ok(ApiResponse::err(e.message)),
    };

    let manuscript_root = match &project.manuscript_root {
        Some(r) if !r.is_empty() => r.clone(),
        _ => {
            return Ok(ApiResponse::err(
                "MANUSCRIPT_001: manuscript_root not set — re-import the project",
            ))
        }
    };

    // Parse
    let parsed = match ParserService::parse_manuscript(&project_id, &manuscript_root).await {
        Ok(p) => p,
        Err(e) => return Ok(ApiResponse::err(e.message)),
    };

    // Calculate completeness
    let (score, level) = CompletenessService::calculate(&parsed);

    // Sync illustration placeholders → SQLite
    if let Err(e) = IllustrationSync::sync(&db, &project_id, &parsed.illustrations).await {
        log::warn!("Illustration sync failed for project {}: {}", project_id, e.message);
    }

    // Persist completeness + counts on the project row
    let level_str = match &level {
        ManuscriptCompleteness::Blocking => "blocking",
        ManuscriptCompleteness::Warning => "warning",
        ManuscriptCompleteness::Normal => "normal",
    };

    let _ = sqlx::query(
        "UPDATE projects
         SET completeness_score = ?, completeness_level = ?,
             chapter_count = ?, illustration_count = ?, updated_at = ?
         WHERE id = ?",
    )
    .bind(score)
    .bind(level_str)
    .bind(parsed.chapters.len() as i32)
    .bind(parsed.illustrations.len() as i32)
    .bind(chrono::Utc::now().to_rfc3339())
    .bind(&project_id)
    .execute(&db)
    .await;

    // Build IPC response from parsed data
    let warnings: Vec<String> = parsed
        .errors
        .iter()
        .map(|e| format!("{}: {}", e.code, e.message))
        .collect();

    let ast = ManuscriptAst {
        chapters: parsed
            .chapters
            .iter()
            .map(|c| ChapterNode {
                slug: slug_from_title(&c.title),
                title: c.title.clone(),
                word_count: c.word_count as i32,
            })
            .collect(),
        front_matter: parsed
            .front_matter
            .iter()
            .map(|c| SectionNode {
                slug: slug_from_title(&c.title),
                title: c.title.clone(),
            })
            .collect(),
        back_matter: parsed
            .back_matter
            .iter()
            .map(|c| SectionNode {
                slug: slug_from_title(&c.title),
                title: c.title.clone(),
            })
            .collect(),
        illustrations: parsed
            .illustrations
            .iter()
            .map(|i| {
                let chapter_slug = parsed
                    .chapters
                    .iter()
                    .find(|c| c.file_path == i.file_path)
                    .map(|c| slug_from_title(&c.title))
                    .unwrap_or_default();
                IllustrationPlaceholder {
                    name: i.name.clone(),
                    description: i.description.clone(),
                    chapter_slug,
                    position: i.line_number as i32,
                }
            })
            .collect(),
        metadata: ManuscriptMetadata {
            total_words: parsed.total_words as i32,
            total_chapters: parsed.chapters.len() as i32,
            total_illustrations: parsed.illustrations.len() as i32,
        },
    };

    if warnings.is_empty() {
        Ok(ApiResponse::ok(ast))
    } else {
        Ok(ApiResponse::ok_with_warnings(ast, warnings))
    }
}

// ---------------------------------------------------------------------------
// IPC: read_book_config
// ---------------------------------------------------------------------------

/// Read and validate a book config from a BES root directory path.
/// Supports V1 (JSON), V2 (JSON + manuscriptRoot), V3 (bes-format.yaml).
///
/// Frontend (wizard step 2):
/// ```ts
/// const cfg = await invoke<ApiResponse<BookConfig>>('read_book_config', { path: selectedPath });
/// ```
#[tauri::command]
pub async fn read_book_config(path: String) -> Result<ApiResponse<BookConfig>, String> {
    if path.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: path cannot be empty"));
    }

    match BookConfigService::read_book_config(&path).await {
        Ok(mut config) => {
            let mut warnings = Vec::new();

            if let Some(ref version) = config.version {
                if !["v1", "v2", "v3"].contains(&version.as_str()) {
                    warnings.push(format!(
                        "CONFIG_002: Unknown config version '{}'. Treating as V1.",
                        version
                    ));
                }
            }

            // Apply genre defaults when page_dimensions/typography are not set
            if config.page_dimensions.is_none() || config.typography.is_none() {
                if let Some(ref genre_str) = config.genre {
                    if let Ok(genre) = serde_json::from_value::<Genre>(
                        serde_json::Value::String(genre_str.clone()),
                    ) {
                        let (dims, typo) = genre_defaults(&genre);

                        if config.page_dimensions.is_none() {
                            config.page_dimensions = Some(PageDimensionsResponse {
                                width_inches: dims.width_inches,
                                height_inches: dims.height_inches,
                                margin_top: dims.margin_top,
                                margin_bottom: dims.margin_bottom,
                                margin_inner: dims.margin_inner,
                                margin_outer: dims.margin_outer,
                            });
                        }

                        if config.typography.is_none() {
                            config.typography = Some(TypographyDefaultsResponse {
                                body_font: typo.body_font,
                                heading_font: typo.heading_font,
                                code_font: typo.code_font,
                                body_size_pt: typo.body_size_pt,
                                line_height: typo.line_height,
                            });
                        }
                    }
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

// ---------------------------------------------------------------------------
// IPC: select_directory
// ---------------------------------------------------------------------------

/// Open a native folder-picker dialog and return the selected path.
///
/// Returns `None` if the user cancelled.
///
/// Frontend (wizard step 1):
/// ```ts
/// const path = await invoke<string | null>('select_directory');
/// ```
#[tauri::command]
pub async fn select_directory(app: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder = app
        .dialog()
        .file()
        .set_title("Select BES project folder")
        .blocking_pick_folder();

    Ok(folder.map(|f| f.to_string()))
}

// ---------------------------------------------------------------------------
// IPC: write_bes_format
// ---------------------------------------------------------------------------

/// Persist an updated `BookConfig` as `bes-format.yaml` and sync project metadata.
///
/// Frontend (wizard step 3 / settings):
/// ```ts
/// await invoke('write_bes_format', { projectId, config });
/// ```
#[tauri::command]
pub async fn write_bes_format(
    pool: State<'_, SqlitePool>,
    app_handle: tauri::AppHandle,
    project_id: String,
    config: BookConfig,
) -> Result<ApiResponse<bool>, String> {
    if project_id.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: project_id cannot be empty"));
    }
    if config.title.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: config.title cannot be empty"));
    }

    let db = pool.inner().clone();
    let proj_repo = ProjectRepository::new(db.clone());

    let project = match proj_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_081: Project not found: {}",
                project_id
            )))
        }
        Err(e) => return Ok(ApiResponse::err(e.message)),
    };

    match BookConfigService::write_bes_format(&project.bes_root_path, &config).await {
        Ok(()) => {
            // Sync project metadata
            let _ = sqlx::query(
                "UPDATE projects SET genre = ?, language = ?, config_version = 'v3', updated_at = ? WHERE id = ?",
            )
            .bind(&config.genre)
            .bind(&config.language)
            .bind(chrono::Utc::now().to_rfc3339())
            .bind(&project_id)
            .execute(&db)
            .await;

            // Invalidate preview cache + emit live preview event (TASK-2)
            crate::commands::preview::invalidate_preview_cache(&db, &project_id).await;
            let _ = app_handle.emit(
                "preview:ast-changed",
                serde_json::json!({ "projectId": project_id, "changed": "manuscript" }),
            );

            Ok(ApiResponse::ok(true))
        }
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

// ---------------------------------------------------------------------------
// IPC: get_illustrations
// ---------------------------------------------------------------------------

/// List all illustration records for a project.
///
/// Frontend:
/// ```ts
/// const illus = await invoke<ApiResponse<Illustration[]>>('get_illustrations', { projectId });
/// ```
#[tauri::command]
pub async fn get_illustrations(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Vec<Illustration>>, String> {
    if project_id.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: project_id cannot be empty"));
    }

    let repo = IllustrationRepository::new(pool.inner().clone());

    match repo.find_by_project(&project_id).await {
        Ok(items) => Ok(ApiResponse::ok(items)),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

// ---------------------------------------------------------------------------
// IPC: calculate_completeness
// ---------------------------------------------------------------------------

/// Re-parse the manuscript and return a fresh completeness report.
///
/// Frontend:
/// ```ts
/// const result = await invoke<ApiResponse<CompletenessResult>>('calculate_completeness', { projectId });
/// ```
#[tauri::command]
pub async fn calculate_completeness(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<CompletenessResult>, String> {
    if project_id.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: project_id cannot be empty"));
    }

    let db = pool.inner().clone();
    let proj_repo = ProjectRepository::new(db.clone());

    let project = match proj_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_081: Project not found: {}",
                project_id
            )))
        }
        Err(e) => return Ok(ApiResponse::err(e.message)),
    };

    let manuscript_root = match &project.manuscript_root {
        Some(r) if !r.is_empty() => r.clone(),
        _ => return Ok(ApiResponse::err("MANUSCRIPT_001: manuscript_root not set")),
    };

    match ParserService::parse_manuscript(&project_id, &manuscript_root).await {
        Ok(parsed) => {
            let (score, level) = CompletenessService::calculate(&parsed);

            let level_str = match level {
                ManuscriptCompleteness::Blocking => "blocking",
                ManuscriptCompleteness::Warning => "warning",
                ManuscriptCompleteness::Normal => "normal",
            };

            let empty_chapters: Vec<String> = parsed
                .chapters
                .iter()
                .filter(|c| c.word_count <= 100)
                .map(|c| c.file_path.clone())
                .collect();

            let warnings: Vec<String> = parsed
                .errors
                .iter()
                .filter(|e| e.level == "warning")
                .map(|e| e.message.clone())
                .collect();

            Ok(ApiResponse::ok(CompletenessResult {
                score,
                level: level_str.to_string(),
                total_chapters: parsed.chapters.len() as i32,
                chapters_with_content: parsed
                    .chapters
                    .iter()
                    .filter(|c| c.word_count > 100)
                    .count() as i32,
                empty_chapters,
                placeholder_count: parsed.illustrations.len() as i32,
                warnings,
            }))
        }
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

// ---------------------------------------------------------------------------
// IPC: run_content_checklist
// ---------------------------------------------------------------------------

/// Run the pre-generation content checklist for a project.
///
/// Frontend:
/// ```ts
/// const result = await invoke<ApiResponse<ChecklistResult>>('run_content_checklist', { projectId });
/// ```
#[tauri::command]
pub async fn run_content_checklist(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<ChecklistResult>, String> {
    if project_id.trim().is_empty() {
        return Ok(ApiResponse::err("VAL-001: project_id cannot be empty"));
    }

    let db = pool.inner().clone();
    let proj_repo = ProjectRepository::new(db.clone());
    let illus_repo = IllustrationRepository::new(db.clone());

    let project = match proj_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_081: Project not found: {}",
                project_id
            )))
        }
        Err(e) => return Ok(ApiResponse::err(e.message)),
    };

    let mut blocking: Vec<ChecklistItem> = Vec::new();
    let mut warning_items: Vec<ChecklistItem> = Vec::new();
    let mut info_items: Vec<ChecklistItem> = Vec::new();

    // Check completeness level stored in the project record
    if let Some(ref level) = project.completeness_level {
        match level.as_str() {
            "blocking" => {
                blocking.push(ChecklistItem {
                    id: "COMPLETENESS_BLOCKING".to_string(),
                    message: format!(
                        "Manuscript completeness is below 80% (score: {:.0}%). Generation blocked.",
                        project.completeness_score.unwrap_or(0.0) * 100.0
                    ),
                    files: None,
                });
            }
            "warning" => {
                warning_items.push(ChecklistItem {
                    id: "COMPLETENESS_WARNING".to_string(),
                    message: format!(
                        "Manuscript is {:.0}% complete. Output may be incomplete.",
                        project.completeness_score.unwrap_or(0.0) * 100.0
                    ),
                    files: None,
                });
            }
            _ => {}
        }
    }

    // Check for pending (unlinked) illustrations
    match illus_repo.find_by_state(&project_id, "pending").await {
        Ok(pending) if !pending.is_empty() => {
            warning_items.push(ChecklistItem {
                id: "ILLUSTRATIONS_PENDING".to_string(),
                message: format!(
                    "{} illustration(s) still PENDING — no image has been linked.",
                    pending.len()
                ),
                files: Some(pending.iter().map(|i| i.placeholder_name.clone()).collect()),
            });
        }
        _ => {}
    }

    // ---- CK_EMPTY_CHAPTER / CK_DRAFT_MARKERS / CK_TOC_COVERAGE ----
    // Re-parse manuscript to inspect chapter contents
    let manuscript_root = match &project.manuscript_root {
        Some(r) if !r.is_empty() => r.clone(),
        _ => String::new(),
    };

    if !manuscript_root.is_empty() {
        if let Ok(parsed) = ParserService::parse_manuscript(&project_id, &manuscript_root).await {
            // CK_EMPTY_CHAPTER: chapters with word_count <= 100
            let empty_chapters: Vec<String> = parsed
                .chapters
                .iter()
                .filter(|c| c.word_count <= 100)
                .map(|c| c.file_path.clone())
                .collect();

            if !empty_chapters.is_empty() {
                warning_items.push(ChecklistItem {
                    id: "EMPTY_CHAPTERS".to_string(),
                    message: format!(
                        "{} chapter(s) with 100 words or fewer — may be stubs.",
                        empty_chapters.len()
                    ),
                    files: Some(empty_chapters),
                });
            }

            // CK_DRAFT_MARKERS: TODO, FIXME, DRAFT, XXX in chapter contents
            let draft_re = Regex::new(r"(?i)\b(TODO|FIXME|DRAFT|XXX)\b").unwrap();
            let draft_files: Vec<String> = parsed
                .chapters
                .iter()
                .filter(|c| draft_re.is_match(&c.content))
                .map(|c| c.file_path.clone())
                .collect();

            if !draft_files.is_empty() {
                warning_items.push(ChecklistItem {
                    id: "DRAFT_MARKERS".to_string(),
                    message: format!(
                        "{} chapter(s) contain draft markers (TODO/FIXME/DRAFT/XXX).",
                        draft_files.len()
                    ),
                    files: Some(draft_files),
                });
            }

            // CK_TOC_COVERAGE: front_matter present but no TOC
            if !parsed.front_matter.is_empty() && !parsed.toc_present {
                warning_items.push(ChecklistItem {
                    id: "TOC_MISSING".to_string(),
                    message: "Front-matter present but no TOC_PLACEHOLDER found.".to_string(),
                    files: None,
                });
            }
        }
    }

    // ---- CK_SPEC_STRUCTURE: expected directory structure ----
    if !manuscript_root.is_empty() {
        let root = Path::new(&manuscript_root);

        if !root.join("chapters").exists() {
            blocking.push(ChecklistItem {
                id: "MISSING_CHAPTERS_DIR".to_string(),
                message: "Required 'chapters/' directory not found in manuscript root.".to_string(),
                files: None,
            });
        }

        if !root.join("front-matter").exists() {
            info_items.push(ChecklistItem {
                id: "NO_FRONT_MATTER".to_string(),
                message: "No 'front-matter/' directory — front matter will be skipped.".to_string(),
                files: None,
            });
        }

        if !root.join("back-matter").exists() {
            info_items.push(ChecklistItem {
                id: "NO_BACK_MATTER".to_string(),
                message: "No 'back-matter/' directory — back matter will be skipped.".to_string(),
                files: None,
            });
        }
    }

    let passed = blocking.is_empty();

    Ok(ApiResponse::ok(ChecklistResult {
        passed,
        blockers: blocking,
        warnings: warning_items,
        info: info_items,
    }))
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn slug_from_title(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
