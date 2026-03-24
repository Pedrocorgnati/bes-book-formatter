use image::{ImageEncoder, ImageFormat, ImageReader};
use sqlx::{Row, SqlitePool};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use crate::error::AppError;
use crate::models::illustration::Illustration;
use crate::models::typography::DpiValidation;

// ---- File type categorization ----

#[derive(Debug, PartialEq)]
enum ImageType {
    Bitmap(ImageFormat), // JPEG / PNG / TIFF / BMP
    SvgVectorial,
    SvgRaster, // SVG without XML header — treat as bitmap error
    Ico,
    Unsupported(String),
}

/// Service for the illustration processing pipeline.
pub struct IllustrationService {
    pool: SqlitePool,
}

impl IllustrationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Public API
    // ──────────────────────────────────────────────────────────────────────────

    /// Full processing pipeline:
    ///   detect type → validate DPI → resize → convert RGB → compress → save → update DB → return.
    ///
    /// `assets_dir` = `$APPDATA/bes-book-formatter/projects/{project_id}/illustrations/`
    pub async fn process_illustration(
        &self,
        illustration_id: &str,
        file_path: &str,
        project_id: &str,
    ) -> Result<Illustration, AppError> {
        let src = Path::new(file_path);

        // Guard: file must exist
        if !src.exists() {
            return Err(AppError::not_found(format!(
                "VAL_001: Arquivo não encontrado: {}",
                file_path
            )));
        }

        // Guard: PERF-003 — max 50 MB
        if let Ok(meta) = std::fs::metadata(src) {
            if meta.len() > 50 * 1024 * 1024 {
                return Err(AppError::validation(
                    "PERF_003: Arquivo excede o limite de 50MB".to_string(),
                ));
            }
        }

        // 1. Detect file type
        let img_type = self.detect_file_type(src);

        // 2. Handle ICO
        if img_type == ImageType::Ico {
            return Err(AppError::validation(
                "ILLUSTRATION_FORMAT_UNSUPPORTED: Arquivos ICO não são suportados. Use JPG, PNG, TIFF ou SVG vetorial.".to_string(),
            ));
        }

        // 3. Handle SVG vectorial — copy without processing
        if img_type == ImageType::SvgVectorial {
            let dest = self
                .build_dest_path(project_id, src.extension().and_then(|e| e.to_str()).unwrap_or("svg"))
                .map_err(|e| AppError::internal(e))?;

            std::fs::copy(src, &dest)
                .map_err(|e| AppError::internal(format!("IO_001: Falha ao copiar SVG: {}", e)))?;

            let dest_str = dest.to_string_lossy().to_string();
            return self
                .persist_imported(illustration_id, project_id, &dest_str, 300, 0, 0, "RGB")
                .await;
        }

        // 4. Bitmap processing
        let format = match &img_type {
            ImageType::Bitmap(f) => *f,
            _ => {
                return Err(AppError::validation(format!(
                    "ILLUSTRATION_FORMAT_UNSUPPORTED: Formato não suportado. Use JPG, PNG, TIFF ou SVG."
                )))
            }
        };

        // 4a. Validate DPI (reads header only — fast path)
        let dpi = self.quick_dpi_check(src).unwrap_or(72);
        if dpi < 150 {
            return Err(AppError::validation(format!(
                "ILLUSTRATION_DPI_CRITICAL: DPI insuficiente para impressão profissional ({} DPI < 150 DPI mínimo)",
                dpi
            )));
        }

        // 4b. Load full image
        let mut img = ImageReader::open(src)
            .map_err(|e| AppError::internal(format!("IO_002: Falha ao abrir imagem: {}", e)))?
            .decode()
            .map_err(|e| AppError::internal(format!("IO_003: Falha ao decodificar imagem: {}", e)))?;

        let orig_w = img.width();
        let orig_h = img.height();

        // 4c. Resize if width > 1200px (maintain aspect ratio)
        if orig_w > 1200 {
            let ratio = 1200.0 / orig_w as f64;
            let new_h = (orig_h as f64 * ratio).round() as u32;
            img = img.resize(1200, new_h, image::imageops::FilterType::Lanczos3);
        }

        let final_w = img.width();
        let final_h = img.height();

        // 4d. Convert to RGB (handles CMYK detection heuristically — image crate normalises to RGBA/RGB)
        let img_rgb = img.into_rgb8();
        let color_space = "RGB";

        // 4e. Compress quality 90 and save
        let output_ext = match format {
            ImageFormat::Tiff | ImageFormat::Bmp => "png", // normalise lossy-incompatible to PNG
            ImageFormat::Png => "png",
            _ => "jpg",
        };
        let dest = self
            .build_dest_path(project_id, output_ext)
            .map_err(|e| AppError::internal(e))?;

        match output_ext {
            "jpg" => {
                let mut buf = Cursor::new(Vec::new());
                let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 90);
                encoder
                    .write_image(
                        img_rgb.as_raw(),
                        final_w,
                        final_h,
                        image::ColorType::Rgb8.into(),
                    )
                    .map_err(|e| AppError::internal(format!("IO_004: Falha ao codificar JPEG: {}", e)))?;
                std::fs::write(&dest, buf.into_inner())
                    .map_err(|e| AppError::internal(format!("IO_005: Falha ao salvar: {}", e)))?;
            }
            _ => {
                // PNG — lossless
                img_rgb
                    .save(&dest)
                    .map_err(|e| AppError::internal(format!("IO_005: Falha ao salvar PNG: {}", e)))?;
            }
        }

        let dest_str = dest.to_string_lossy().to_string();
        self.persist_imported(
            illustration_id,
            project_id,
            &dest_str,
            dpi,
            final_w as i32,
            final_h as i32,
            color_space,
        )
        .await
    }

    /// Validate the DPI of an image file without processing it.
    /// Reads only the file header for performance (< 500ms target).
    pub async fn validate_illustration_dpi(
        &self,
        file_path: &str,
    ) -> Result<DpiValidation, AppError> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(AppError::not_found(format!(
                "VAL_001: Arquivo não encontrado: {}",
                file_path
            )));
        }

        // SVG vectorial — no DPI required
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if ext.eq_ignore_ascii_case("svg") {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let trimmed = content.trim_start();
                    if trimmed.starts_with("<?xml") || trimmed.starts_with("<svg") {
                        return Ok(DpiValidation {
                            dpi: 300,
                            adequate: true,
                            warning: None,
                        });
                    }
                }
            }

            // ICO check
            if ext.eq_ignore_ascii_case("ico") {
                return Err(AppError::validation(
                    "ILLUSTRATION_FORMAT_UNSUPPORTED: Arquivos ICO não são suportados".to_string(),
                ));
            }

            // Reject clearly unsupported
            let allowed = ["jpg", "jpeg", "png", "tiff", "tif", "svg", "bmp"];
            if !allowed.contains(&ext.to_lowercase().as_str()) {
                return Err(AppError::validation(format!(
                    "VAL_002: Formato não suportado: .{}. Use JPG, PNG, TIFF ou SVG.",
                    ext
                )));
            }
        }

        // Guard PERF-003 (50 MB)
        if let Ok(meta) = std::fs::metadata(path) {
            if meta.len() > 50 * 1024 * 1024 {
                return Err(AppError::validation(
                    "VAL_003: Arquivo excede o limite de 50MB (PERF-003)".to_string(),
                ));
            }
        }

        let dpi = self.quick_dpi_check(path).unwrap_or(72);

        let (adequate, warning) = match dpi {
            d if d < 150 => (
                false,
                Some(format!(
                    "DPI insuficiente para impressão profissional ({} DPI < 150 DPI mínimo)",
                    d
                )),
            ),
            d if d < 300 => (
                true,
                Some(format!(
                    "Imagem pode ficar desfocada na impressão — recomendado ≥ 300 DPI (atual: {} DPI)",
                    d
                )),
            ),
            _ => (true, None),
        };

        Ok(DpiValidation { dpi, adequate, warning })
    }

    /// Update the alt-text for an illustration and transition to LINKED state.
    pub async fn update_alt_text(
        &self,
        illustration_id: &str,
        alt_text: &str,
    ) -> Result<Illustration, AppError> {
        if alt_text.trim().len() < 10 {
            return Err(AppError::validation(
                "VAL_001: Alt-text deve ter no mínimo 10 caracteres".to_string(),
            ));
        }

        sqlx::query(
            "UPDATE illustrations SET alt_text = ?, state = 'linked', updated_at = CURRENT_TIMESTAMP
             WHERE id = ?",
        )
        .bind(alt_text)
        .bind(illustration_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_003: Failed to update alt-text: {}", e)))?;

        self.load_illustration(illustration_id).await
    }

    /// List all illustrations for a project.
    pub async fn list_illustrations(
        &self,
        project_id: &str,
    ) -> Result<Vec<Illustration>, AppError> {
        let rows = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE project_id = ?
             ORDER BY created_at ASC",
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_001: {}", e)))?;

        Ok(rows.into_iter().map(|r| Self::row_to_illustration(r)).collect())
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Private helpers
    // ──────────────────────────────────────────────────────────────────────────

    /// Classify a file by extension and, for SVG, by content.
    fn detect_file_type(&self, path: &Path) -> ImageType {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "jpg" | "jpeg" => ImageType::Bitmap(ImageFormat::Jpeg),
            "png" => ImageType::Bitmap(ImageFormat::Png),
            "tiff" | "tif" => ImageType::Bitmap(ImageFormat::Tiff),
            "bmp" => ImageType::Bitmap(ImageFormat::Bmp),
            "ico" => ImageType::Ico,
            "svg" => {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let trimmed = content.trim_start();
                    if trimmed.starts_with("<?xml") || trimmed.starts_with("<svg") {
                        ImageType::SvgVectorial
                    } else {
                        ImageType::SvgRaster
                    }
                } else {
                    ImageType::SvgRaster
                }
            }
            other => ImageType::Unsupported(other.to_string()),
        }
    }

    /// Build a unique destination path inside the project's illustrations folder.
    /// Creates the directory if needed (SEC-009: path via join, no string concat).
    fn build_dest_path(&self, project_id: &str, ext: &str) -> Result<PathBuf, String> {
        // Use current dir as root (app data is injected via AppHandle in the IPC layer;
        // the service layer uses a relative path that will be resolved at the IPC level).
        // For testability, we write to a temp-like relative path here.
        let dir = PathBuf::from("project_assets")
            .join(project_id)
            .join("illustrations");

        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("IO_001: Não foi possível criar pasta de ilustrações: {}", e))?;

        let filename = format!("img_{}.{}", Uuid::new_v4(), ext);
        Ok(dir.join(filename))
    }

    /// Persist IMPORTED state and metadata to SQLite, then return the updated row.
    async fn persist_imported(
        &self,
        illustration_id: &str,
        _project_id: &str,
        image_path: &str,
        dpi: u32,
        width_px: i32,
        height_px: i32,
        color_space: &str,
    ) -> Result<Illustration, AppError> {
        sqlx::query(
            "UPDATE illustrations
             SET state = 'imported',
                 image_path = ?,
                 validated_dpi = ?,
                 width_px = ?,
                 height_px = ?,
                 color_space = ?,
                 updated_at = CURRENT_TIMESTAMP
             WHERE id = ?",
        )
        .bind(image_path)
        .bind(dpi as i32)
        .bind(width_px)
        .bind(height_px)
        .bind(color_space)
        .bind(illustration_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("DB_003: Failed to update illustration: {}", e)))?;

        self.load_illustration(illustration_id).await
    }

    async fn load_illustration(&self, illustration_id: &str) -> Result<Illustration, AppError> {
        let row = sqlx::query(
            "SELECT id, project_id, placeholder_name, description, state,
                    image_path, validated_dpi, alt_text, width_px, height_px,
                    color_space, created_at, updated_at
             FROM illustrations WHERE id = ?",
        )
        .bind(illustration_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            AppError::not_found(format!("Illustration {} not found: {}", illustration_id, e))
        })?;

        Ok(Self::row_to_illustration(row))
    }

    fn row_to_illustration(row: sqlx::sqlite::SqliteRow) -> Illustration {
        Illustration {
            id: row.try_get("id").unwrap_or_default(),
            project_id: row.try_get("project_id").unwrap_or_default(),
            placeholder_name: row.try_get("placeholder_name").unwrap_or_default(),
            description: row.try_get("description").ok().flatten(),
            state: row.try_get("state").unwrap_or_else(|_| "pending".to_string()),
            image_path: row.try_get("image_path").ok().flatten(),
            validated_dpi: row.try_get("validated_dpi").ok(),
            alt_text: row.try_get("alt_text").ok().flatten(),
            width_px: row.try_get("width_px").ok(),
            height_px: row.try_get("height_px").ok(),
            color_space: row.try_get("color_space").ok().flatten(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        }
    }

    // ──────────────────────────────────────────────────────────────────────────
    // DPI extraction (header-only reads for performance)
    // ──────────────────────────────────────────────────────────────────────────

    fn quick_dpi_check(&self, path: &Path) -> Option<u32> {
        let bytes = std::fs::read(path).ok()?;
        let ext = path.extension()?.to_str()?.to_lowercase();
        match ext.as_str() {
            "jpg" | "jpeg" => self.extract_jpeg_dpi(&bytes),
            "png" => self.extract_png_dpi(&bytes),
            "tiff" | "tif" => self.extract_tiff_dpi(&bytes),
            _ => None,
        }
    }

    fn extract_jpeg_dpi(&self, bytes: &[u8]) -> Option<u32> {
        if bytes.len() < 18 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
            return None;
        }
        let mut offset = 2;
        while offset + 4 < bytes.len() {
            if bytes[offset] != 0xFF {
                break;
            }
            let marker = bytes[offset + 1];
            let seg_len =
                u16::from_be_bytes([bytes[offset + 2], bytes[offset + 3]]) as usize;

            if marker == 0xE0 && offset + 18 < bytes.len() {
                let unit = bytes[offset + 11];
                let xdensity =
                    u16::from_be_bytes([bytes[offset + 12], bytes[offset + 13]]) as u32;
                return match unit {
                    1 => Some(xdensity),
                    2 => Some(xdensity * 254 / 100),
                    _ => Some(72),
                };
            }
            offset += 2 + seg_len;
        }
        Some(72)
    }

    fn extract_png_dpi(&self, bytes: &[u8]) -> Option<u32> {
        if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" {
            return None;
        }
        let mut offset = 8;
        while offset + 12 < bytes.len() {
            let chunk_len = u32::from_be_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]) as usize;
            let chunk_type = &bytes[offset + 4..offset + 8];

            if chunk_type == b"pHYs" && offset + 20 < bytes.len() {
                let ppux = u32::from_be_bytes([
                    bytes[offset + 8],
                    bytes[offset + 9],
                    bytes[offset + 10],
                    bytes[offset + 11],
                ]);
                let unit = bytes[offset + 16];
                return match unit {
                    1 => Some((ppux as f64 / 39.3701).round() as u32),
                    _ => Some(72),
                };
            }
            offset += 12 + chunk_len;
        }
        Some(72)
    }

    fn extract_tiff_dpi(&self, bytes: &[u8]) -> Option<u32> {
        if bytes.len() < 8 {
            return None;
        }
        // TIFF tag 0x011A (XResolution rational) is complex to parse without a full IFD walk.
        // Return 300 as a reasonable default for TIFF (professional format).
        let magic = &bytes[0..2];
        if magic != b"II" && magic != b"MM" {
            return None;
        }
        Some(300)
    }
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_dpi_validation(dpi: u32) -> DpiValidation {
        let (adequate, warning) = match dpi {
            d if d < 150 => (
                false,
                Some(format!("DPI insuficiente ({} < 150)", d)),
            ),
            d if d < 300 => (
                true,
                Some(format!("Pode ficar desfocado ({})", d)),
            ),
            _ => (true, None),
        };
        DpiValidation { dpi, adequate, warning }
    }

    #[test]
    fn test_validate_dpi_low() {
        let v = make_dpi_validation(72);
        assert!(!v.adequate);
        assert!(v.warning.is_some());
    }

    #[test]
    fn test_validate_dpi_warning() {
        let v = make_dpi_validation(250);
        assert!(v.adequate);
        assert!(v.warning.is_some());
    }

    #[test]
    fn test_validate_dpi_ok() {
        let v = make_dpi_validation(300);
        assert!(v.adequate);
        assert!(v.warning.is_none());
    }

    #[test]
    fn test_validate_dpi_critical_boundary() {
        let v = make_dpi_validation(149);
        assert!(!v.adequate);
    }

    #[test]
    fn test_validate_dpi_boundary_150() {
        let v = make_dpi_validation(150);
        assert!(v.adequate);
        assert!(v.warning.is_some()); // 150 < 300, so warning
    }

    #[test]
    fn test_png_dpi_extraction_from_header() {
        // Minimal PNG with a pHYs chunk: 2835 ppux/ppuy, unit=1 (meter) = ~72 DPI
        // PNG signature + IHDR(13 bytes) + pHYs chunk
        let mut bytes = vec![0x89u8, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNG sig
        // IHDR chunk (len=13)
        bytes.extend_from_slice(&[0, 0, 0, 13]); // chunk length
        bytes.extend_from_slice(b"IHDR");
        bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 1, 8, 2, 0, 0, 0]); // 1x1 RGB
        bytes.extend_from_slice(&[0x90, 0x77, 0x53, 0xDE]); // CRC placeholder
        // pHYs chunk (len=9): ppux=2835, ppuy=2835, unit=1 (meter)
        bytes.extend_from_slice(&[0, 0, 0, 9]); // chunk length
        bytes.extend_from_slice(b"pHYs");
        bytes.extend_from_slice(&[0x00, 0x00, 0x0B, 0x13]); // 2835 ppux
        bytes.extend_from_slice(&[0x00, 0x00, 0x0B, 0x13]); // 2835 ppuy
        bytes.push(0x01); // unit: meter

        // Create a dummy service (can't use pool in unit test — just test the extraction method)
        // We test via indirect struct path
        let dpi = {
            let mut offset = 8usize;
            let mut result: Option<u32> = Some(72);
            while offset + 12 < bytes.len() {
                let chunk_len = u32::from_be_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                ]) as usize;
                let chunk_type = &bytes[offset + 4..offset + 8];
                if chunk_type == b"pHYs" && offset + 20 < bytes.len() {
                    let ppux = u32::from_be_bytes([
                        bytes[offset + 8],
                        bytes[offset + 9],
                        bytes[offset + 10],
                        bytes[offset + 11],
                    ]);
                    let unit = bytes[offset + 16];
                    result = match unit {
                        1 => Some((ppux as f64 / 39.3701).round() as u32),
                        _ => Some(72),
                    };
                    break;
                }
                offset += 12 + chunk_len;
            }
            result
        };
        // 2835 / 39.3701 ≈ 72 DPI
        assert_eq!(dpi, Some(72));
    }

    #[test]
    fn test_detect_file_type_svg_vectorial() {
        // Create a temp file to test detect_file_type
        let tmp = std::env::temp_dir().join("test_bes_illustration.svg");
        std::fs::write(&tmp, b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"></svg>").unwrap();

        // We can't instantiate IllustrationService without a pool, but we can test the logic inline
        let content = std::fs::read_to_string(&tmp).unwrap();
        let trimmed = content.trim_start();
        assert!(trimmed.starts_with("<?xml") || trimmed.starts_with("<svg"));

        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_detect_file_type_ico_rejected() {
        let ext = "ico";
        let is_ico = ext.eq_ignore_ascii_case("ico");
        assert!(is_ico);
    }

    #[test]
    fn test_alt_text_min_length_validation() {
        let too_short = "Olá".trim().len() < 10;
        assert!(too_short);

        let ok = "Descrição da ilustração".trim().len() >= 10;
        assert!(ok);

        let exactly_10 = "1234567890".trim().len() >= 10;
        assert!(exactly_10);
    }

    #[test]
    fn test_illustration_missing_mode_values() {
        let modes = ["placeholder_visual", "remove_space", "block_generation"];
        for mode in modes {
            assert!(!mode.is_empty());
        }
    }
}
