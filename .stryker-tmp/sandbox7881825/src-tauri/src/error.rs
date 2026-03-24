use serde::{Deserialize, Serialize};

/// Application error codes mapped from ERROR-CATALOG.md.
/// Format: {MODULE}_{NNN} — module prefix + 3-digit sequential number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppError {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl AppError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(code: &str, message: impl Into<String>, details: serde_json::Value) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
            details: Some(details),
        }
    }

    // --- Validation errors (VAL_*) ---

    pub fn val_required_field(field: &str) -> Self {
        Self::new("VAL_001", format!("Required field missing: {}", field))
    }

    pub fn val_invalid_format(field: &str, expected: &str) -> Self {
        Self::new("VAL_002", format!("Invalid format for field '{}': expected {}", field, expected))
    }

    pub fn val_out_of_range(field: &str, value: &str, min: &str, max: &str) -> Self {
        Self::new("VAL_003", format!("Value out of range for '{}': got {}, expected [{}, {}]", field, value, min, max))
    }

    // --- System errors (SYS_*) ---

    pub fn sys_internal(message: impl Into<String>) -> Self {
        Self::new("SYS_001", message)
    }

    pub fn sys_not_implemented(operation: &str) -> Self {
        Self::new("SYS_050", format!("Not implemented: {} — run /auto-flow execute", operation))
    }

    // --- Database errors (DB_*) ---

    pub fn db_init_failed(message: impl Into<String>) -> Self {
        Self::new("DB_001", message)
    }

    pub fn db_corrupted() -> Self {
        Self::new("DB_002", "Database integrity check failed — consider recreating the database")
    }

    pub fn db_permission_denied(path: &str) -> Self {
        Self::new("DB_003", format!("Permission denied for database path: {}", path))
    }

    pub fn db_query_failed(message: impl Into<String>) -> Self {
        Self::new("DB_050", message)
    }

    // --- Filesystem errors (FS_*) ---

    pub fn fs_path_not_found(path: &str) -> Self {
        Self::new("FS_001", format!("Path not found: {}", path))
    }

    pub fn fs_permission_denied(path: &str) -> Self {
        Self::new("FS_002", format!("Permission denied: {}", path))
    }

    // --- Config errors (CONFIG_*) ---

    pub fn config_parse_error(message: impl Into<String>) -> Self {
        Self::new("CONFIG_001", message)
    }

    pub fn config_version_unknown(version: &str) -> Self {
        Self::new("CONFIG_002", format!("Unknown book config version: {}. Falling back to V1.", version))
    }

    // --- Sidecar errors (SIDECAR_*) ---

    pub fn sidecar_not_found(name: &str) -> Self {
        Self::new("SIDECAR_001", format!("Sidecar not found: {}", name))
    }

    pub fn sidecar_timeout(name: &str, timeout_ms: u64) -> Self {
        Self::new("SIDECAR_002", format!("Sidecar '{}' exceeded timeout of {}ms", name, timeout_ms))
    }

    pub fn sidecar_crash(name: &str, exit_code: i32, stderr: &str) -> Self {
        Self::with_details(
            "SIDECAR_003",
            format!("Sidecar '{}' crashed with exit code {}", name, exit_code),
            serde_json::json!({ "stderr": stderr }),
        )
    }

    // --- Project errors (PROJECT_*) ---

    pub fn project_duplicate(path: &str) -> Self {
        Self::new("PROJECT_080", format!("Duplicate bes_root_path — a project already exists for path: {}", path))
    }

    pub fn project_not_found(id: &str) -> Self {
        Self::new("PROJECT_081", format!("Project not found: {}", id))
    }

    // --- Manuscript errors (MANUSCRIPT_*) ---

    pub fn manuscript_empty(root: &str) -> Self {
        Self::new("MANUSCRIPT_001", format!("No .md files found in manuscript root: {}", root))
    }

    // --- Illustration errors (ILLUSTRATION_*) ---

    pub fn illustration_invalid_transition(from: &str, to: &str) -> Self {
        Self::new("ILLUSTRATION_050", format!("Invalid state transition: {} → {}", from, to))
    }

    // --- Generic convenience helpers ---

    /// Generic internal/system error (SYS_001).
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new("SYS_001", message)
    }

    /// Generic not-found error (FS_001).
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new("FS_001", message)
    }

    /// Generic validation error — detects VAL code from message prefix if present.
    pub fn validation(message: impl Into<String>) -> Self {
        let msg: String = message.into();
        let code = if msg.starts_with("VAL_001") {
            "VAL_001"
        } else if msg.starts_with("VAL_002") {
            "VAL_002"
        } else if msg.starts_with("VAL_003") {
            "VAL_003"
        } else {
            "VAL_001"
        };
        Self::new(code, msg)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AppError {}

// Allow AppError to be returned from Tauri commands
impl From<AppError> for String {
    fn from(err: AppError) -> String {
        serde_json::to_string(&err).unwrap_or_else(|_| format!("[{}] {}", err.code, err.message))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::db_query_failed(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::new("FS_050", err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::config_parse_error(err.to_string())
    }
}
