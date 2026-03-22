use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{ApiResponse, NewProject, Project};
use crate::repositories::{IllustrationRepository, ProjectRepository};
use crate::services::FilesystemService;

/// List recent projects.
///
/// Frontend: `const projects = await invoke<ApiResponse<Project[]>>('get_projects', { limit: 20 });`
#[tauri::command]
pub async fn get_projects(
    pool: State<'_, SqlitePool>,
    limit: Option<u32>,
) -> Result<ApiResponse<Vec<Project>>, String> {
    let repo = ProjectRepository::new(pool.inner().clone());
    match repo.find_all_recent(limit.unwrap_or(20)).await {
        Ok(projects) => Ok(ApiResponse::ok(projects)),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Get a single project by ID.
///
/// Frontend: `const project = await invoke<ApiResponse<Project>>('get_project', { id });`
#[tauri::command]
pub async fn get_project(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ApiResponse<Project>, String> {
    let repo = ProjectRepository::new(pool.inner().clone());
    match repo.find_by_id(&id).await {
        Ok(Some(project)) => {
            // Update last_opened timestamp
            let _ = repo.update_last_opened(&id).await;
            Ok(ApiResponse::ok(project))
        }
        Ok(None) => Ok(ApiResponse::err(format!("Project not found: {}", id))),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Import a BES project: verify structure, read config, create in SQLite.
///
/// Frontend: `const project = await invoke<ApiResponse<Project>>('import_project', { besRoot: '/path/to/bes' });`
#[tauri::command]
pub async fn import_project(
    pool: State<'_, SqlitePool>,
    bes_root: String,
) -> Result<ApiResponse<Project>, String> {
    let db = pool.inner().clone();
    let proj_repo = ProjectRepository::new(db.clone());

    // Check if project already exists for this path
    if let Ok(Some(existing)) = proj_repo.find_by_bes_root(&bes_root).await {
        return Ok(ApiResponse::err(format!(
            "PROJECT_081: A project already exists for this path (id: {})",
            existing.id
        )));
    }

    // Verify BES structure (path traversal protection from THREAT-001)
    let structure = FilesystemService::verify_bes_structure(&bes_root)
        .await
        .map_err(AppError::into)?;

    let mut warnings = structure.warnings.clone();

    // Read book config if found
    let (name, genre, language, config_version, manuscript_root, output_dir) =
        if let Some(ref config_path) = structure.book_config_path {
            match FilesystemService::read_book_config(config_path).await {
                Ok(config) => (
                    config.title.clone(),
                    config.genre.clone(),
                    config.language.clone(),
                    config.version.clone(),
                    config.manuscript_root.clone(),
                    config.output_dir.clone(),
                ),
                Err(e) => {
                    warnings.push(format!("Failed to parse book-config.json: {}", e.message));
                    (
                        bes_root
                            .split('/')
                            .last()
                            .unwrap_or("Untitled")
                            .to_string(),
                        None,
                        None,
                        None,
                        None,
                        None,
                    )
                }
            }
        } else {
            (
                bes_root
                    .split('/')
                    .last()
                    .unwrap_or("Untitled")
                    .to_string(),
                None,
                None,
                None,
                structure.manuscript_root.clone(),
                None,
            )
        };

    // Create project in SQLite
    let new_project = NewProject {
        name,
        bes_root_path: bes_root.clone(),
        book_config_path: structure.book_config_path,
        genre,
        language,
        config_version,
        manuscript_root: manuscript_root.or(structure.manuscript_root),
        output_dir,
    };

    match proj_repo.create(&new_project).await {
        Ok(project) => {
            if warnings.is_empty() {
                Ok(ApiResponse::ok(project))
            } else {
                Ok(ApiResponse::ok_with_warnings(project, warnings))
            }
        }
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Delete a project and its illustrations (CASCADE).
///
/// Frontend: `await invoke('delete_project', { id });`
#[tauri::command]
pub async fn delete_project(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<ApiResponse<bool>, String> {
    let repo = ProjectRepository::new(pool.inner().clone());
    match repo.delete(&id).await {
        Ok(true) => Ok(ApiResponse::ok(true)),
        Ok(false) => Ok(ApiResponse::err(format!("Project not found: {}", id))),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}
