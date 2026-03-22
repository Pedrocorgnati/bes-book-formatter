pub mod book_config_service;
pub mod common;
pub mod docx_service;
pub mod epub_renderer;
pub mod epub_service;
pub mod html_service;
pub mod completeness_service;
pub mod filesystem_service;
pub mod font_service;
pub mod illustration_service;
pub mod illustration_sync;
pub mod migration_service;
pub mod parser_service;
pub mod pdf_ebook_service;
pub mod pdf_print_service;
pub mod platform_presets;
pub mod preflight_service;
pub mod sidecar_manager;
pub mod preview_service;
pub mod typography_rules;
pub mod typography_service;

pub use book_config_service::BookConfigService;
pub use preview_service::PreviewService;
pub use docx_service::{DocxService, SimpleExportService};
pub use epub_renderer::{
    generate_colophon, generate_epub_toc_nav, generate_epub_toc_ncx, highlight_code_block,
    html_escape, render_footnote_section_html, render_ornament_html, replace_footnote_markers,
    TocEntry,
};
pub use epub_service::EpubService;
pub use html_service::HtmlService;
pub use completeness_service::CompletenessService;
pub use filesystem_service::FilesystemService;
pub use font_service::{FontInfo, FontService};
pub use illustration_service::IllustrationService;
pub use illustration_sync::IllustrationSync;
pub use migration_service::MigrationService;
pub use parser_service::ParserService;
pub use pdf_ebook_service::PdfEbookService;
pub use pdf_print_service::PdfPrintService;
pub use platform_presets::{EbookPreset, PrintPreset};
pub use preflight_service::PreflightService;
pub use sidecar_manager::SidecarManager;
pub use typography_rules::apply_typography_rules;
pub use typography_service::TypographyService;
