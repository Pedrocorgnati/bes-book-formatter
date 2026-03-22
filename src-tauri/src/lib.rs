pub mod commands;
pub mod db;
pub mod error;
pub mod models;
pub mod repositories;
pub mod services;

use tauri::Manager;

use commands::generation::{
    cancel_generation, generate_docx, generate_epub, generate_html, generate_pdf, generate_pdf_ebook,
    generate_pdf_print, get_generation_results, run_epubcheck, run_preflight,
};
use commands::parser::{
    calculate_completeness, get_illustrations, parse_manuscript, read_book_config,
    run_content_checklist, select_directory, write_bes_format,
};
use commands::preview::{
    detect_orphans_widows, render_preview,
    render_preview_page, get_page_count, navigate_to_page, set_zoom_level, toggle_spread_view,
    toggle_distraction_free, add_annotation, get_annotations, delete_annotation,
};
use commands::projects::{delete_project, get_project, get_projects, import_project};
use commands::system::{check_sidecar, get_preference, get_preferences, init_database, set_preference};
use commands::typography::{
    delete_custom_font, get_typography_config, list_fonts, list_illustrations, select_font_file,
    set_typography_config, upload_font, validate_illustration_dpi, process_illustration,
    update_illustration_alt_text,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("[SYS_001] Rust panic: {}", info);
        // In production, this would write to a crash log file
    }));

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
            get_preference,
            set_preference,
            check_sidecar,
            // Projects (Skeleton + Rock-1)
            get_projects,
            get_project,
            import_project,
            delete_project,
            // Parser / Book Config (Rock-1 — module-2)
            parse_manuscript,
            read_book_config,
            select_directory,
            write_bes_format,
            get_illustrations,
            calculate_completeness,
            run_content_checklist,
            // Generation (Rock-3 — module-4)
            generate_epub,
            generate_html,
            generate_pdf,
            generate_pdf_print,
            generate_pdf_ebook,
            generate_docx,
            get_generation_results,
            run_epubcheck,
            run_preflight,
            cancel_generation,
            // Preview (Rock-4 / module-5)
            render_preview,
            detect_orphans_widows,
            render_preview_page,
            get_page_count,
            navigate_to_page,
            set_zoom_level,
            toggle_spread_view,
            toggle_distraction_free,
            add_annotation,
            get_annotations,
            delete_annotation,
            // Typography & Illustrations (Rock-2 — module-3)
            get_typography_config,
            set_typography_config,
            validate_illustration_dpi,
            process_illustration,
            list_illustrations,
            update_illustration_alt_text,
            // Font management (module-3 TASK-2)
            select_font_file,
            list_fonts,
            upload_font,
            delete_custom_font,
        ])
        .run(tauri::generate_context!())
        .expect("error while running BES Book Formatter");
}
