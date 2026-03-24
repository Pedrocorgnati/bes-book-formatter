use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::models::responses::BookConfig;

// ---------------------------------------------------------------------------
// Raw JSON structs for V1 / V2 deserialization
// (camelCase fields as typically found in book-config.json)
// ---------------------------------------------------------------------------

/// Intermediate struct for deserializing V1/V2 JSON book-config files.
/// Fields match the most common key names found in BES projects.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawBookConfigJson {
    pub version: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    /// V2 field: points to the manuscript directory relative to the BES root.
    pub manuscript_root: Option<String>,
    pub outline_root: Option<String>,
    pub output_dir: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub isbn: Option<String>,
}

/// Intermediate struct for deserializing V3 YAML (bes-format.yaml).
/// Keys may be snake_case in YAML.
#[derive(Debug, Deserialize)]
struct RawBesFormatYaml {
    pub version: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub manuscript_root: Option<String>,
    pub outline_root: Option<String>,
    pub output_dir: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub isbn: Option<String>,
}

// ---------------------------------------------------------------------------
// Book Config Service
// ---------------------------------------------------------------------------

pub struct BookConfigService;

impl BookConfigService {
    /// Read and normalise a book config from a BES root directory.
    ///
    /// Detection order:
    ///  1. `bes-format.yaml`  → V3
    ///  2. `book-config.json` → V1 (no `manuscript_root`) or V2 (has it)
    pub async fn read_book_config(bes_root: &str) -> Result<BookConfig, AppError> {
        let root = Path::new(bes_root);

        // V3: bes-format.yaml
        let yaml_path = root.join("bes-format.yaml");
        if yaml_path.exists() {
            return Self::read_yaml_config(&yaml_path.to_string_lossy()).await;
        }

        // V1 / V2: book-config.json
        let json_path = root.join("book-config.json");
        if json_path.exists() {
            return Self::read_json_config(&json_path.to_string_lossy()).await;
        }

        Err(AppError::fs_path_not_found(&format!(
            "{}/bes-format.yaml or book-config.json",
            bes_root
        )))
    }

    /// Write a normalised `BookConfig` as `bes-format.yaml` to the BES root.
    ///
    /// Uses SEC-009: path built with `Path::join()`, not string concatenation.
    pub async fn write_bes_format(bes_root: &str, config: &BookConfig) -> Result<(), AppError> {
        let path = Path::new(bes_root).join("bes-format.yaml");

        // Convert to a YAML-friendly intermediate (snake_case keys)
        #[derive(Serialize)]
        struct YamlOut<'a> {
            version: &'a str,
            title: &'a str,
            author: &'a str,
            language: Option<&'a str>,
            genre: Option<&'a str>,
            manuscript_root: Option<&'a str>,
            outline_root: Option<&'a str>,
            output_dir: Option<&'a str>,
            platforms: Option<&'a Vec<String>>,
            isbn: Option<&'a str>,
        }

        let out = YamlOut {
            version: "v3",
            title: &config.title,
            author: &config.author,
            language: config.language.as_deref(),
            genre: config.genre.as_deref(),
            manuscript_root: config.manuscript_root.as_deref(),
            outline_root: config.outline_root.as_deref(),
            output_dir: config.output_dir.as_deref(),
            platforms: config.platforms.as_ref(),
            isbn: config.isbn.as_deref(),
        };

        let yaml = serde_yaml::to_string(&out).map_err(|e| {
            AppError::config_parse_error(format!("Failed to serialize bes-format.yaml: {}", e))
        })?;

        tokio::fs::write(&path, yaml).await.map_err(|e| {
            AppError::fs_permission_denied(&format!("{}: {}", path.display(), e))
        })?;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    async fn read_json_config(path: &str) -> Result<BookConfig, AppError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::fs_path_not_found(path)
            } else {
                AppError::fs_permission_denied(&format!("{}: {}", path, e))
            }
        })?;

        let raw: RawBookConfigJson = serde_json::from_str(&content).map_err(|e| {
            AppError::config_parse_error(format!("Failed to parse book-config.json: {}", e))
        })?;

        // Determine version: V1 has no manuscript_root; V2 has it.
        let version = raw
            .version
            .or_else(|| {
                if raw.manuscript_root.is_some() {
                    Some("v2".to_string())
                } else {
                    Some("v1".to_string())
                }
            });

        Ok(BookConfig {
            version,
            title: raw.title.unwrap_or_default(),
            author: raw.author.unwrap_or_default(),
            language: raw.language,
            genre: raw.genre,
            manuscript_root: raw.manuscript_root,
            outline_root: raw.outline_root,
            output_dir: raw.output_dir,
            platforms: raw.platforms,
            isbn: raw.isbn,
            page_dimensions: None,
            typography: None,
        })
    }

    async fn read_yaml_config(path: &str) -> Result<BookConfig, AppError> {
        let content = tokio::fs::read_to_string(path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AppError::fs_path_not_found(path)
            } else {
                AppError::fs_permission_denied(&format!("{}: {}", path, e))
            }
        })?;

        let raw: RawBesFormatYaml = serde_yaml::from_str(&content).map_err(|e| {
            AppError::config_parse_error(format!("Failed to parse bes-format.yaml: {}", e))
        })?;

        Ok(BookConfig {
            version: raw.version.or_else(|| Some("v3".to_string())),
            title: raw.title.unwrap_or_default(),
            author: raw.author.unwrap_or_default(),
            language: raw.language,
            genre: raw.genre,
            manuscript_root: raw.manuscript_root,
            outline_root: raw.outline_root,
            output_dir: raw.output_dir,
            platforms: raw.platforms,
            isbn: raw.isbn,
            page_dimensions: None,
            typography: None,
        })
    }
}

// ---------------------------------------------------------------------------
// Unit tests (TASK-2 ST005)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tokio::fs;

    fn tempdir() -> PathBuf {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        let dir = std::env::temp_dir().join(format!("bes_config_test_{}", id));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn test_read_v1_minimal_config() {
        let dir = tempdir();
        fs::write(
            dir.join("book-config.json"),
            r#"{"title":"My Book","author":"Jane Doe","genre":"fiction"}"#,
        )
        .await
        .unwrap();

        let config = BookConfigService::read_book_config(&dir.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(config.title, "My Book");
        assert_eq!(config.author, "Jane Doe");
        assert_eq!(config.genre, Some("fiction".to_string()));
        assert_eq!(config.version, Some("v1".to_string()));
    }

    #[tokio::test]
    async fn test_read_v2_with_manuscript_root() {
        let dir = tempdir();
        fs::write(
            dir.join("book-config.json"),
            r#"{"title":"My Book","author":"Jane","manuscriptRoot":"./manuscript","genre":"nonfiction"}"#,
        )
        .await
        .unwrap();

        let config = BookConfigService::read_book_config(&dir.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(config.manuscript_root, Some("./manuscript".to_string()));
        assert_eq!(config.version, Some("v2".to_string()));
    }

    #[tokio::test]
    async fn test_read_v3_yaml_full() {
        let dir = tempdir();
        fs::write(
            dir.join("bes-format.yaml"),
            "version: v3\ntitle: My YAML Book\nauthor: John\ngenre: technical\nlanguage: pt-BR\n",
        )
        .await
        .unwrap();

        let config = BookConfigService::read_book_config(&dir.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(config.title, "My YAML Book");
        assert_eq!(config.genre, Some("technical".to_string()));
        assert_eq!(config.version, Some("v3".to_string()));
    }

    #[tokio::test]
    async fn test_write_bes_format_roundtrip() {
        let dir = tempdir();
        let config = BookConfig {
            version: Some("v3".to_string()),
            title: "Roundtrip Book".to_string(),
            author: "Author".to_string(),
            language: Some("pt-BR".to_string()),
            genre: Some("nonfiction".to_string()),
            manuscript_root: Some("./manuscript".to_string()),
            outline_root: None,
            output_dir: Some("./output".to_string()),
            platforms: Some(vec!["kdp".to_string()]),
            isbn: None,
            page_dimensions: None,
            typography: None,
        };

        BookConfigService::write_bes_format(&dir.to_string_lossy(), &config)
            .await
            .unwrap();

        // Read back
        let read_back = BookConfigService::read_book_config(&dir.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(read_back.title, "Roundtrip Book");
        assert_eq!(read_back.genre, Some("nonfiction".to_string()));
    }

    #[tokio::test]
    async fn test_missing_config_returns_error() {
        let dir = tempdir();

        let result = BookConfigService::read_book_config(&dir.to_string_lossy()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_yaml_takes_priority_over_json() {
        let dir = tempdir();
        // Both files exist — YAML should win (V3 priority)
        fs::write(
            dir.join("book-config.json"),
            r#"{"title":"JSON Book","author":"A"}"#,
        )
        .await
        .unwrap();
        fs::write(
            dir.join("bes-format.yaml"),
            "title: YAML Book\nauthor: B\n",
        )
        .await
        .unwrap();

        let config = BookConfigService::read_book_config(&dir.to_string_lossy())
            .await
            .unwrap();

        assert_eq!(config.title, "YAML Book");
        assert_eq!(config.version, Some("v3".to_string()));
    }

    #[tokio::test]
    async fn test_invalid_json_returns_error() {
        let dir = tempdir();
        fs::write(dir.join("book-config.json"), "{ invalid json }")
            .await
            .unwrap();

        let result = BookConfigService::read_book_config(&dir.to_string_lossy()).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, "CONFIG_001");
    }
}
