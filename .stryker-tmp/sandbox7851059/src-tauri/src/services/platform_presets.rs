// BES Book Formatter — Platform Presets (module-4 TASK-2 ST004)
//
// Configurações de impressão por plataforma (KDP, IngramSpark, offset, generic).

use serde::{Deserialize, Serialize};

/// Configurações de impressão para uma plataforma específica.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintPreset {
    /// Nome da plataforma (ex: "amazon-kdp")
    pub platform: String,
    /// Bleed em polegadas (0.125" = 3.175mm)
    pub bleed_inches: f64,
    /// Área de segurança mínima das bordas em polegadas
    pub safety_margin_inches: f64,
    /// DPI mínimo para imagens internas
    pub interior_dpi: u32,
    /// DPI mínimo para capa
    pub cover_dpi: u32,
    /// Perfil PDF/X alvo: "pdf_x1a" | "pdf_x4"
    pub pdfx_profile: String,
    /// Espaço de cor: "cmyk" | "rgb"
    pub color_space: String,
    /// TAC máximo (Total Area Coverage) para CMYK — previne borrão na impressão
    pub max_tac_percent: u32,
    /// Avisar se páginas não são divisíveis por 4
    pub warn_page_multiple_of_4: bool,
    /// Formato de página padrão (width×height em polegadas)
    pub default_page_size: (f64, f64),
}

impl PrintPreset {
    /// Retorna preset para Amazon KDP Print (padrão: 6×9, PDF/X-1a, CMYK).
    pub fn kdp_print() -> Self {
        Self {
            platform: "amazon-kdp".to_string(),
            bleed_inches: 0.125,
            safety_margin_inches: 0.375,
            interior_dpi: 300,
            cover_dpi: 300,
            pdfx_profile: "pdf_x1a".to_string(),
            color_space: "cmyk".to_string(),
            max_tac_percent: 300,
            warn_page_multiple_of_4: true,
            default_page_size: (6.0, 9.0),
        }
    }

    /// Retorna preset para IngramSpark (PDF/X-4, bleed externo).
    pub fn ingram_spark() -> Self {
        Self {
            platform: "ingram_spark".to_string(),
            bleed_inches: 0.125,
            safety_margin_inches: 0.375,
            interior_dpi: 300,
            cover_dpi: 300,
            pdfx_profile: "pdf_x4".to_string(),
            color_space: "cmyk".to_string(),
            max_tac_percent: 300,
            warn_page_multiple_of_4: true,
            default_page_size: (6.0, 9.0),
        }
    }

    /// Retorna preset genérico (gráfica offset customizável).
    pub fn generic() -> Self {
        Self {
            platform: "generic".to_string(),
            bleed_inches: 0.118, // 3mm
            safety_margin_inches: 0.315, // 8mm
            interior_dpi: 300,
            cover_dpi: 300,
            pdfx_profile: "pdf_x1a".to_string(),
            color_space: "cmyk".to_string(),
            max_tac_percent: 300,
            warn_page_multiple_of_4: false,
            default_page_size: (5.83, 8.27), // A5
        }
    }

    /// Resolve preset pelo nome da plataforma (case-insensitive).
    pub fn for_platform(platform: &str) -> Self {
        match platform.to_lowercase().as_str() {
            "amazon-kdp" | "kdp" | "kdp_print" => Self::kdp_print(),
            "ingram_spark" | "ingramspark" => Self::ingram_spark(),
            _ => Self::generic(),
        }
    }

    /// Calcula dimensões da página com bleed (para marcas de corte).
    pub fn page_with_bleed(&self) -> (f64, f64) {
        let (w, h) = self.default_page_size;
        (w + self.bleed_inches * 2.0, h + self.bleed_inches * 2.0)
    }
}

/// Preset de e-book (sem bleed, RGB, dimensões por plataforma).
#[derive(Debug, Clone)]
pub struct EbookPreset {
    pub platform: String,
    /// Dimensões em polegadas (width, height)
    pub page_size: (f64, f64),
    pub color_space: String,
}

impl EbookPreset {
    pub fn for_platform(platform: &str) -> Self {
        match platform.to_lowercase().as_str() {
            "kobo" => Self {
                platform: "kobo".to_string(),
                page_size: (5.5, 8.5),
                color_space: "rgb".to_string(),
            },
            "apple_books" | "apple-books" => Self {
                platform: "apple_books".to_string(),
                page_size: (6.0, 9.0),
                color_space: "rgb".to_string(),
            },
            _ => Self {
                platform: "kdp".to_string(),
                page_size: (6.0, 9.0),
                color_space: "rgb".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── PrintPreset ──────────────────────────────────────────────────────────

    #[test]
    fn kdp_print_preset_values() {
        let p = PrintPreset::kdp_print();
        assert_eq!(p.platform, "amazon-kdp");
        assert!((p.bleed_inches - 0.125).abs() < f64::EPSILON);
        assert_eq!(p.interior_dpi, 300);
        assert_eq!(p.cover_dpi, 300);
        assert_eq!(p.pdfx_profile, "pdf_x1a");
        assert_eq!(p.color_space, "cmyk");
        assert_eq!(p.default_page_size, (6.0, 9.0));
        assert!(p.warn_page_multiple_of_4);
    }

    #[test]
    fn ingram_spark_preset_uses_pdf_x4() {
        let p = PrintPreset::ingram_spark();
        assert_eq!(p.platform, "ingram_spark");
        assert_eq!(p.pdfx_profile, "pdf_x4");
        assert_eq!(p.color_space, "cmyk");
        assert!((p.bleed_inches - 0.125).abs() < f64::EPSILON);
    }

    #[test]
    fn generic_preset_defaults() {
        let p = PrintPreset::generic();
        assert_eq!(p.platform, "generic");
        assert_eq!(p.pdfx_profile, "pdf_x1a");
        assert!(!p.warn_page_multiple_of_4);
        // A5 dimensions
        assert_eq!(p.default_page_size, (5.83, 8.27));
    }

    #[test]
    fn for_platform_aliases_resolve_kdp() {
        for alias in &["amazon-kdp", "kdp", "kdp_print", "KDP"] {
            let p = PrintPreset::for_platform(alias);
            assert_eq!(p.platform, "amazon-kdp", "alias '{}' should resolve to amazon-kdp", alias);
        }
    }

    #[test]
    fn for_platform_aliases_resolve_ingram() {
        for alias in &["ingram_spark", "ingramspark", "IngramSpark"] {
            let p = PrintPreset::for_platform(alias);
            assert_eq!(p.platform, "ingram_spark", "alias '{}' should resolve to ingram_spark", alias);
        }
    }

    #[test]
    fn for_platform_unknown_falls_back_to_generic() {
        let p = PrintPreset::for_platform("unknown-press");
        assert_eq!(p.platform, "generic");
    }

    #[test]
    fn page_with_bleed_adds_two_bleeds() {
        let p = PrintPreset::kdp_print();
        let (w, h) = p.page_with_bleed();
        let expected_w = 6.0 + 0.125 * 2.0;
        let expected_h = 9.0 + 0.125 * 2.0;
        assert!((w - expected_w).abs() < f64::EPSILON);
        assert!((h - expected_h).abs() < f64::EPSILON);
    }

    // ── EbookPreset ──────────────────────────────────────────────────────────

    #[test]
    fn ebook_preset_kdp_default_dimensions() {
        let p = EbookPreset::for_platform("kdp");
        assert_eq!(p.platform, "kdp");
        assert_eq!(p.page_size, (6.0, 9.0));
        assert_eq!(p.color_space, "rgb");
    }

    #[test]
    fn ebook_preset_kobo_dimensions() {
        let p = EbookPreset::for_platform("kobo");
        assert_eq!(p.platform, "kobo");
        assert_eq!(p.page_size, (5.5, 8.5));
        assert_eq!(p.color_space, "rgb");
    }

    #[test]
    fn ebook_preset_apple_books() {
        let p = EbookPreset::for_platform("apple-books");
        assert_eq!(p.platform, "apple_books");
        assert_eq!(p.page_size, (6.0, 9.0));
    }

    #[test]
    fn ebook_preset_unknown_falls_back_to_kdp() {
        let p = EbookPreset::for_platform("generic-reader");
        assert_eq!(p.platform, "kdp");
    }
}
