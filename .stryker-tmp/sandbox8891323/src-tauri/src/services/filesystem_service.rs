use std::path::{Path, PathBuf};

use crate::error::AppError;
use crate::models::{BookConfig, StructureReport};

/// Handles filesystem operations: reading BES repositories, book configs,
/// writing .bes-format files, and verifying BES directory structure.
pub struct FilesystemService;

impl FilesystemService {
    /// Verify that a path is a valid BES repository structure.
    pub async fn verify_bes_structure(bes_root: &str) -> Result<StructureReport, AppError> {
        let root = Path::new(bes_root);

        if !root.exists() {
            return Err(AppError::fs_path_not_found(bes_root));
        }

        if !root.is_dir() {
            return Err(AppError::val_invalid_format("besRootPath", "directory path"));
        }

        // Canonicalize to resolve symlinks (THREAT-001 mitigation)
        let canonical = root.canonicalize().map_err(|e| {
            AppError::fs_permission_denied(&format!("{}: {}", bes_root, e))
        })?;

        // Reject paths containing ".." after canonicalization
        if canonical.to_string_lossy().contains("..") {
            return Err(AppError::val_invalid_format(
                "besRootPath",
                "absolute path without path traversal",
            ));
        }

        let mut warnings = Vec::new();
        let mut book_config_path = None;
        let mut manuscript_root = None;

        // Search for book-config.json
        let config_path = canonical.join("book-config.json");
        let book_config_found = config_path.exists();
        if book_config_found {
            book_config_path = Some(config_path.to_string_lossy().to_string());
        } else {
            warnings.push("book-config.json not found in repository root".to_string());
        }

        // Search for manuscript directory
        for candidate in &["manuscript", "manuscripts", "src", "content"] {
            let path = canonical.join(candidate);
            if path.exists() && path.is_dir() {
                manuscript_root = Some(path.to_string_lossy().to_string());
                break;
            }
        }

        if manuscript_root.is_none() {
            warnings.push("No manuscript directory found (tried: manuscript, manuscripts, src, content)".to_string());
        }

        Ok(StructureReport {
            valid: book_config_found,
            book_config_found,
            book_config_path,
            manuscript_root,
            warnings,
        })
    }

    /// Read and parse a book-config.json file.
    pub async fn read_book_config(path: &str) -> Result<BookConfig, AppError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::fs_path_not_found(path)
            } else {
                AppError::fs_permission_denied(&format!("{}: {}", path, e))
            }
        })?;

        let config: BookConfig = serde_json::from_str(&content).map_err(|e| {
            AppError::config_parse_error(format!("Failed to parse book-config.json: {}", e))
        })?;

        Ok(config)
    }

    /// List all Markdown files in a manuscript directory (recursively).
    pub async fn list_manuscript_files(root: &str) -> Result<Vec<PathBuf>, AppError> {
        let root_path = Path::new(root);
        if !root_path.exists() {
            return Err(AppError::fs_path_not_found(root));
        }

        let mut files = Vec::new();
        Self::collect_md_files(root_path, &mut files).await?;
        files.sort();
        Ok(files)
    }

    /// Write the .bes-format JSON file to the BES repository root.
    pub async fn write_bes_format(
        bes_root: &str,
        data: &serde_json::Value,
    ) -> Result<(), AppError> {
        let path = Path::new(bes_root).join(".bes-format");
        let json = serde_json::to_string_pretty(data)?;
        tokio::fs::write(&path, json).await.map_err(|e| {
            AppError::fs_permission_denied(&format!("{}: {}", path.display(), e))
        })?;
        Ok(())
    }

    /// Recursively collect .md files.
    async fn collect_md_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), AppError> {
        let mut entries = tokio::fs::read_dir(dir).await.map_err(|e| {
            AppError::fs_permission_denied(&format!("{}: {}", dir.display(), e))
        })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| {
            AppError::sys_internal(format!("Failed to read directory entry: {}", e))
        })? {
            let path = entry.path();
            if path.is_dir() {
                Box::pin(Self::collect_md_files(&path, files)).await?;
            } else if path.extension().is_some_and(|ext| ext == "md") {
                files.push(path);
            }
        }

        Ok(())
    }
}
