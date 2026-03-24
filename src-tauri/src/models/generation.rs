// BES Book Formatter — Generation Models (module-4 TASK-0)
//
// GenOptions: parâmetros de entrada para IPC commands de geração.
// StoredGenerationResult: registro persistido no SQLite.

use serde::{Deserialize, Serialize};

/// Parâmetros de entrada para todos os IPC commands de geração.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenOptions {
    /// Formato de saída: "epub3" | "pdf_print" | "pdf_ebook" | "docx" | "html5" | ...
    pub format: String,
    /// Plataforma alvo: "kdp" | "kdp_print" | "ingram_spark" | "apple_books" | "kobo" | "generic"
    pub platform: String,
    /// Cor do papel (impressão): "white_70lb" | "cream_60lb"
    pub paper_color: Option<String>,
    /// DPI alvo para imagens (300 | 600 para print)
    pub dpi: Option<u32>,
    /// Incluir marcas de corte / bleed (para PDF print)
    pub include_bleed: Option<bool>,
    /// Perfil PDF/X: "pdf_x1a" | "pdf_x4" (apenas para pdf_print)
    pub pdfx_profile: Option<String>,
}

/// Registro de geração persistido no SQLite (tabela `generation_results`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoredGenerationResult {
    pub id: String,
    pub project_id: String,
    pub format: String,
    pub platform: String,
    pub output_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub duration_ms: Option<i64>,
    pub status: String,
    /// JSON array serializado como TEXT no SQLite
    pub errors: String,
    /// JSON array serializado como TEXT no SQLite
    pub warnings: String,
    pub created_at: String,
}
