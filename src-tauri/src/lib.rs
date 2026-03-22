pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod repositories;
pub mod services;

use tauri::Manager;

use commands::generation::{generate_epub, generate_pdf, run_epubcheck, run_preflight};
use commands::parser::{
    calculate_completeness, parse_manuscript, read_book_config, run_content_checklist,
};
use commands::preview::{detect_orphans_widows, render_preview};
use commands::projects::{delete_project, get_project, get_projects, import_project};
use commands::system::{check_sidecar, get_preferences, init_database, set_preference};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Initialize SQLite pool and store as managed state
            tauri::async_runtime::block_on(async move {
                let app_data_dir = app_handle
                    .path()
                    .app_local_data_dir()
                    .expect("Failed to resolve app local data dir");

                match db::create_pool(app_data_dir).await {
                    Ok(pool) => {
                        // Apply pending migrations on startup
                        let migration_svc = services::MigrationService::new(pool.clone());
                        match migration_svc.apply_pending().await {
                            Ok(applied) => {
                                if !applied.is_empty() {
                                    log::info!("Applied migrations: {:?}", applied);
                                }
                            }
                            Err(e) => {
                                log::error!("Migration failed: {}", e);
                            }
                        }

                        app_handle.manage(pool);
                    }
                    Err(e) => {
                        log::error!("Failed to initialize database: {}", e);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // System (Skeleton)
            init_database,
            get_preferences,
            set_preference,
            check_sidecar,
            // Projects (Skeleton + Rock-1)
            get_projects,
            get_project,
            import_project,
            delete_project,
            // Parser (Rock-1)
            parse_manuscript,
            read_book_config,
            calculate_completeness,
            run_content_checklist,
            // Generation (Rock-3)
            generate_epub,
            generate_pdf,
            run_epubcheck,
            run_preflight,
            // Preview (Rock-4)
            render_preview,
            detect_orphans_widows,
        ])
        .run(tauri::generate_context!())
        .expect("error while running BES Book Formatter");
}
