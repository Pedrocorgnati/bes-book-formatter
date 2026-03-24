use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// Maximum font file size: 10 MB
const MAX_FONT_SIZE_BYTES: u64 = 10 * 1024 * 1024;

/// Metadata about a font available in the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FontInfo {
    /// Display name (e.g. "EB Garamond").
    pub name: String,
    /// Absolute path to the font file.
    pub path: String,
    /// True for fonts bundled with the app; false for user-uploaded fonts.
    pub is_bundled: bool,
}

/// Bundled fonts always available in all projects.
static BUNDLED_FONTS: &[(&str, &str)] = &[
    ("EB Garamond",    "assets/fonts/EBGaramond-Regular.otf"),
    ("Source Serif 4", "assets/fonts/SourceSerif4-Regular.otf"),
    ("JetBrains Mono", "assets/fonts/JetBrainsMono-Regular.ttf"),
];

/// Service for managing custom font uploads and the font catalog.
pub struct FontService;

impl FontService {
    /// Upload a font file for a project.
    ///
    /// Validates extension and file size, then copies to the app data fonts directory.
    /// Returns the [`FontInfo`] for the uploaded font on success.
    pub fn upload_font(
        project_id: &str,
        source_path: &str,
        app_data_dir: &Path,
    ) -> Result<FontInfo, String> {
        let source = Path::new(source_path);

        // Validate extension before touching the filesystem
        let ext = source
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        match ext.as_deref() {
            Some("otf") | Some("ttf") => {}
            _ => {
                return Err(
                    "VAL_002: Somente arquivos OTF e TTF são suportados".to_string()
                )
            }
        }

        // SEC-010: Canonicalize to resolve symlinks and `..` segments before any fs operation.
        // This prevents path traversal attacks where `filePath` contains `../../../etc/passwd`.
        let source = source.canonicalize()
            .map_err(|e| format!("FS_001: Não foi possível acessar o arquivo: {}", e))?;

        // Validate size on the real (canonicalized) path
        let metadata = fs::metadata(&source)
            .map_err(|e| format!("FS_001: Não foi possível acessar o arquivo: {}", e))?;
        if metadata.len() > MAX_FONT_SIZE_BYTES {
            return Err(
                "VAL_003: Tamanho máximo excedido: fontes devem ter menos de 10MB".to_string()
            );
        }

        // SEC-010: Reject paths that resolve outside of user home-like directories.
        // Block system directories that should never be font sources.
        let source_str = source.to_string_lossy();
        #[cfg(unix)]
        if source_str.starts_with("/etc")
            || source_str.starts_with("/proc")
            || source_str.starts_with("/sys")
            || source_str.starts_with("/dev")
        {
            return Err("FS_001: Caminho de origem não permitido".to_string());
        }
        #[cfg(windows)]
        if source_str.to_lowercase().contains("\\windows\\system32")
            || source_str.to_lowercase().contains("\\windows\\syswow64")
        {
            return Err("FS_001: Caminho de origem não permitido".to_string());
        }

        // Ensure destination directory exists
        let dest_dir = app_data_dir
            .join("fonts")
            .join(project_id);
        fs::create_dir_all(&dest_dir)
            .map_err(|e| format!("SYS_001: Não foi possível criar diretório de fontes: {}", e))?;

        // Copy file using only the filename component (never the full source path)
        let file_name = source
            .file_name()
            .ok_or_else(|| "FS_001: Nome de arquivo inválido".to_string())?;
        let dest_path = dest_dir.join(file_name);
        fs::copy(&source, &dest_path)
            .map_err(|e| format!("SYS_001: Falha ao copiar fonte: {}", e))?;

        // Derive display name from filename
        let name = source
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.replace(['-', '_'], " "))
            .unwrap_or_else(|| "Fonte Personalizada".to_string());

        Ok(FontInfo {
            name,
            path: dest_path.to_string_lossy().into_owned(),
            is_bundled: false,
        })
    }

    /// List all fonts available for a project (bundled + uploaded custom fonts).
    pub fn list_fonts(
        project_id: &str,
        app_data_dir: &Path,
        tauri_resource_dir: &Path,
    ) -> Result<Vec<FontInfo>, String> {
        let mut fonts: Vec<FontInfo> = Vec::new();

        // Bundled fonts
        for (name, rel_path) in BUNDLED_FONTS {
            let path = tauri_resource_dir.join(rel_path);
            fonts.push(FontInfo {
                name: name.to_string(),
                path: path.to_string_lossy().into_owned(),
                is_bundled: true,
            });
        }

        // Custom fonts for this project
        let custom_dir = app_data_dir.join("fonts").join(project_id);
        if custom_dir.exists() {
            let entries = fs::read_dir(&custom_dir)
                .map_err(|e| format!("FS_001: Falha ao listar fontes: {}", e))?;
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if ext_lower == "otf" || ext_lower == "ttf" {
                        let name = path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .map(|s| s.replace(['-', '_'], " "))
                            .unwrap_or_else(|| "Fonte".to_string());
                        fonts.push(FontInfo {
                            name,
                            path: path.to_string_lossy().into_owned(),
                            is_bundled: false,
                        });
                    }
                }
            }
        }

        Ok(fonts)
    }

    /// Delete a custom font for a project.
    ///
    /// Finds the file by display name (case-insensitive) in the project fonts directory.
    pub fn delete_custom_font(
        project_id: &str,
        font_name: &str,
        app_data_dir: &Path,
    ) -> Result<(), String> {
        let custom_dir = app_data_dir.join("fonts").join(project_id);
        if !custom_dir.exists() {
            return Err("FS_001: Diretório de fontes não encontrado".to_string());
        }

        let entries = fs::read_dir(&custom_dir)
            .map_err(|e| format!("FS_001: Falha ao listar fontes: {}", e))?;

        let normalized_name = font_name.replace(['-', '_', ' '], "").to_lowercase();

        let mut found: Option<PathBuf> = None;
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let stem_norm = stem.replace(['-', '_', ' '], "").to_lowercase();
                if stem_norm == normalized_name {
                    found = Some(path);
                    break;
                }
            }
        }

        match found {
            Some(path) => {
                fs::remove_file(&path)
                    .map_err(|e| format!("SYS_001: Falha ao deletar fonte: {}", e))?;
                Ok(())
            }
            None => Err(format!("FS_001: Fonte '{}' não encontrada", font_name)),
        }
    }
}

// ---- Unit tests ----

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn make_temp_otf(dir: &Path, filename: &str, size_bytes: usize) -> PathBuf {
        let path = dir.join(filename);
        let mut f = std::fs::File::create(&path).unwrap();
        // Minimal OTF-like header (just enough bytes for size validation)
        f.write_all(&vec![0u8; size_bytes]).unwrap();
        path
    }

    #[test]
    fn test_upload_otf_file() {
        let src_dir = TempDir::new().unwrap();
        let app_dir = TempDir::new().unwrap();

        let font_path = make_temp_otf(src_dir.path(), "MyFont.otf", 1024);
        let result = FontService::upload_font(
            "proj-1",
            font_path.to_str().unwrap(),
            app_dir.path(),
        );
        assert!(result.is_ok(), "{:?}", result);
        let info = result.unwrap();
        assert_eq!(info.is_bundled, false);
        assert!(info.name.contains("MyFont"));
    }

    #[test]
    fn test_upload_invalid_extension_rejected() {
        let src_dir = TempDir::new().unwrap();
        let app_dir = TempDir::new().unwrap();

        let font_path = make_temp_otf(src_dir.path(), "document.pdf", 512);
        let result = FontService::upload_font(
            "proj-1",
            font_path.to_str().unwrap(),
            app_dir.path(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("VAL_002"));
    }

    #[test]
    fn test_upload_oversized_font_rejected() {
        let src_dir = TempDir::new().unwrap();
        let app_dir = TempDir::new().unwrap();

        // 11MB file
        let font_path = make_temp_otf(src_dir.path(), "HugeFont.otf", 11 * 1024 * 1024);
        let result = FontService::upload_font(
            "proj-1",
            font_path.to_str().unwrap(),
            app_dir.path(),
        );
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("VAL_003"));
    }

    #[test]
    fn test_list_fonts_includes_custom() {
        let src_dir = TempDir::new().unwrap();
        let app_dir = TempDir::new().unwrap();
        let res_dir = TempDir::new().unwrap();

        let font_path = make_temp_otf(src_dir.path(), "CustomFont.otf", 512);
        FontService::upload_font("proj-2", font_path.to_str().unwrap(), app_dir.path()).unwrap();

        let fonts = FontService::list_fonts("proj-2", app_dir.path(), res_dir.path()).unwrap();
        // Should include the custom font (bundled won't exist in temp resource dir but won't error)
        let custom: Vec<_> = fonts.iter().filter(|f| !f.is_bundled).collect();
        assert_eq!(custom.len(), 1);
        assert!(custom[0].name.contains("CustomFont"));
    }
}
