use std::collections::HashMap;

use sqlx::SqlitePool;
use tauri::State;

use crate::error::AppError;
use crate::models::{ApiResponse, InitResult, SidecarStatus};
use crate::repositories::PreferenceRepository;
use crate::services::{MigrationService, SidecarManager};

/// Initialize the database: apply pending migrations, load preferences, check sidecars.
///
/// Frontend: `const result = await invoke<ApiResponse<InitResult>>('init_database');`
#[tauri::command]
pub async fn init_database(
    pool: State<'_, SqlitePool>,
) -> Result<ApiResponse<InitResult>, String> {
    let pool = pool.inner().clone();

    // Apply pending migrations
    let migration_svc = MigrationService::new(pool.clone());
    let migrations_applied = migration_svc.apply_pending().await.map_err(AppError::into)?;

    // Verify integrity (first run of the day)
    let integrity_ok = migration_svc.verify_integrity().await.map_err(AppError::into)?;
    if !integrity_ok {
        return Ok(ApiResponse::err("DB_CORRUPTED: Database integrity check failed"));
    }

    // Load preferences
    let pref_repo = PreferenceRepository::new(pool);
    let preferences = pref_repo.get_all().await.map_err(AppError::into)?;

    // Check sidecars in parallel
    let (typst, gs, epubcheck) = tokio::join!(
        SidecarManager::check_sidecar("typst"),
        SidecarManager::check_sidecar("ghostscript"),
        SidecarManager::check_sidecar("epubcheck"),
    );

    let sidecars = vec![typst, gs, epubcheck];

    let mut warnings = Vec::new();
    for sc in &sidecars {
        if !sc.available {
            warnings.push(format!(
                "Sidecar '{}' não encontrado: {}",
                sc.name,
                sc.error.as_deref().unwrap_or("unknown")
            ));
        }
    }

    let result = InitResult {
        preferences,
        sidecars,
        migrations_applied,
    };

    if warnings.is_empty() {
        Ok(ApiResponse::ok(result))
    } else {
        Ok(ApiResponse::ok_with_warnings(result, warnings))
    }
}

/// Get all user preferences as a key-value map.
///
/// Frontend: `const prefs = await invoke<ApiResponse<HashMap>>('get_preferences');`
#[tauri::command]
pub async fn get_preferences(
    pool: State<'_, SqlitePool>,
) -> Result<ApiResponse<HashMap<String, String>>, String> {
    let repo = PreferenceRepository::new(pool.inner().clone());
    match repo.get_all().await {
        Ok(prefs) => Ok(ApiResponse::ok(prefs)),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Set a single user preference.
///
/// Frontend: `await invoke('set_preference', { key: 'theme', value: 'dark' });`
#[tauri::command]
pub async fn set_preference(
    pool: State<'_, SqlitePool>,
    key: String,
    value: String,
) -> Result<ApiResponse<bool>, String> {
    let repo = PreferenceRepository::new(pool.inner().clone());
    match repo.set(&key, &value).await {
        Ok(()) => Ok(ApiResponse::ok(true)),
        Err(e) => Ok(ApiResponse::err(e.message)),
    }
}

/// Check if a sidecar binary is available.
///
/// Frontend: `const status = await invoke<ApiResponse<SidecarStatus>>('check_sidecar', { name: 'typst' });`
#[tauri::command]
pub async fn check_sidecar(name: String) -> Result<ApiResponse<SidecarStatus>, String> {
    let status = SidecarManager::check_sidecar(&name).await;
    Ok(ApiResponse::ok(status))
}
