// BES Book Formatter — CoverService (module-7-cover-design)
//
// Geração de capa via Typst, cálculo de lombada, exportação PNG/JPEG.

use std::path::Path;

use crate::models::{CoverConfig, CoverTemplate, SpineWidthResult};
use crate::services::SidecarManager;

// ---------------------------------------------------------------------------
// Cálculo de lombada
// ---------------------------------------------------------------------------

/// Calcula largura da lombada em mm.
///
/// Fórmulas por plataforma/papel:
///   KDP (white/cream): 0.0025 pol/página
///   IngramSpark: 0.002252 pol/página
///   Genérico: 0.0025 pol/página (KDP padrão)
pub fn calculate_spine_width(
    page_count: u32,
    platform: &str,
    paper_type: &str,
) -> SpineWidthResult {
    if page_count == 0 {
        return SpineWidthResult {
            spine_width_mm: 0.0,
            spine_width_inches: 0.0,
            page_count: 0,
            platform: platform.to_string(),
            paper_type: paper_type.to_string(),
        };
    }

    let thickness_per_page_inches: f64 = match (platform, paper_type) {
        ("ingram", "cream") => 0.002347,
        ("ingram", _) => 0.002252,
        (_, _) => 0.0025, // amazon-kdp and generic (white & cream same)
    };

    let spine_inches = page_count as f64 * thickness_per_page_inches;
    let spine_mm = spine_inches * 25.4;

    SpineWidthResult {
        spine_width_mm: (spine_mm * 100.0).round() / 100.0,
        spine_width_inches: (spine_inches * 1000.0).round() / 1000.0,
        page_count,
        platform: platform.to_string(),
        paper_type: paper_type.to_string(),
    }
}

/// Largura da frente da capa em mm por plataforma (6"×9" padrão).
pub fn front_cover_width_mm(_platform: &str) -> f64 {
    152.4 // 6 inches × 25.4
}

/// Bleed em mm por plataforma.
pub fn bleed_mm(_platform: &str) -> f64 {
    3.175 // 0.125" × 25.4 — KDP e IngramSpark
}

// ---------------------------------------------------------------------------
// Geração de conteúdo Typst
// ---------------------------------------------------------------------------

/// Gera conteúdo Typst para preview de capa.
pub fn build_cover_typst(
    config: &CoverConfig,
    title: &str,
    author: &str,
    front_width_mm: f64,
    height_mm: f64,
) -> String {
    let spine_mm = config.spine_width_mm.max(5.0); // mínimo 5mm para renderizar
    let total_width = front_width_mm * 2.0 + spine_mm;

    let cover_image_block = match &config.cover_image_path {
        Some(path) => format!(
            r#"#image("{}", width: 100%, height: 65%, fit: "cover")"#,
            path.replace('\\', "/")
        ),
        None => String::new(),
    };

    let back_cover = if config.back_cover_text.is_empty() {
        String::new()
    } else {
        config.back_cover_text.clone()
    };

    format!(
        r#"#set page(
  width: {total_width}mm,
  height: {height_mm}mm,
  margin: 0mm,
)

// Verso (esquerdo)
#place(top + left,
  block(
    width: {front_width_mm}mm,
    height: {height_mm}mm,
    fill: rgb("{secondary_color}"),
  )[
    #align(left + bottom, pad(20mm)[
      #text(fill: rgb("{primary_color}"), size: 11pt)[{back_cover}]
    ])
  ]
)

// Lombada (centro)
#place(top + left, dx: {front_width_mm}mm,
  block(
    width: {spine_mm}mm,
    height: {height_mm}mm,
    fill: rgb("{primary_color}"),
  )[
    #align(center + horizon)[
      #rotate(90deg)[
        #text(fill: white, size: 10pt, weight: "bold")[{title}]
        #h(8mm)
        #text(fill: white, size: 9pt)[{author}]
      ]
    ]
  ]
)

// Frente (direita)
#place(top + left, dx: {front_back_dx}mm,
  block(
    width: {front_width_mm}mm,
    height: {height_mm}mm,
    fill: rgb("{secondary_color}"),
  )[
    {cover_image_block}
    #align(left + bottom, pad(20mm)[
      #text(fill: rgb("{primary_color}"), size: 32pt, weight: "bold")[{title}]
      #linebreak()
      #text(fill: rgb("{primary_color}"), size: 14pt)[{author}]
    ])
  ]
)
"#,
        total_width = total_width,
        height_mm = height_mm,
        front_width_mm = front_width_mm,
        spine_mm = spine_mm,
        front_back_dx = front_width_mm + spine_mm,
        primary_color = config.primary_color.trim_start_matches('#'),
        secondary_color = config.secondary_color.trim_start_matches('#'),
        title = title,
        author = author,
        back_cover = back_cover,
        cover_image_block = cover_image_block,
    )
}

// ---------------------------------------------------------------------------
// Compilação Typst
// ---------------------------------------------------------------------------

/// Compila Typst para PNG base64 (preview, baixa resolução).
/// Usa SidecarManager::spawn_typst (timeout 5s).
pub async fn compile_typst_to_png_base64(
    typst_content: &str,
    ppi: u32,
) -> Result<String, String> {
    let tmp_dir = std::env::temp_dir().join(format!("bes-cover-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&tmp_dir)
        .map_err(|e| format!("SYS_001: Cannot create temp dir: {}", e))?;

    let input_path = tmp_dir.join("cover-preview.typ");
    let output_path = tmp_dir.join("cover-preview.png");

    std::fs::write(&input_path, typst_content)
        .map_err(|e| format!("SYS_001: Cannot write temp .typ: {}", e))?;

    let args = vec![
        "compile".to_string(),
        "--format".to_string(),
        "png".to_string(),
        "--ppi".to_string(),
        ppi.to_string(),
        input_path.to_string_lossy().to_string(),
        output_path.to_string_lossy().to_string(),
    ];

    SidecarManager::spawn_typst(&args, 5_000)
        .await
        .map_err(|e| format!("SYS_003: Typst preview timeout or error: {}", e.message))?;

    let png_bytes = std::fs::read(&output_path)
        .map_err(|e| format!("SYS_001: Cannot read PNG output: {}", e))?;

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp_dir);

    Ok(base64_encode(&png_bytes))
}

/// Gera PDF de capa via Typst.
pub async fn compile_typst_to_pdf(
    typst_content: &str,
    output_path: &Path,
) -> Result<(), String> {
    let tmp_dir = std::env::temp_dir().join(format!("bes-cover-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&tmp_dir)
        .map_err(|e| format!("SYS_001: Cannot create temp dir: {}", e))?;

    let input_path = tmp_dir.join("cover.typ");
    std::fs::write(&input_path, typst_content)
        .map_err(|e| format!("SYS_001: Cannot write .typ: {}", e))?;

    let args = vec![
        "compile".to_string(),
        "--format".to_string(),
        "pdf".to_string(),
        input_path.to_string_lossy().to_string(),
        output_path.to_string_lossy().to_string(),
    ];

    SidecarManager::spawn_typst(&args, 30_000)
        .await
        .map_err(|e| format!("SYS_001: Typst PDF compile error: {}", e.message))?;

    // Cleanup temp (mas não o output)
    let _ = std::fs::remove_dir_all(&tmp_dir);

    Ok(())
}

/// Exporta PNG de alta resolução via Typst (para marketing).
pub async fn compile_typst_to_png_file(
    typst_content: &str,
    output_path: &Path,
    ppi: u32,
) -> Result<(), String> {
    let tmp_dir = std::env::temp_dir().join(format!("bes-cover-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&tmp_dir)
        .map_err(|e| format!("SYS_001: Cannot create temp dir: {}", e))?;

    let input_path = tmp_dir.join("cover-export.typ");
    std::fs::write(&input_path, typst_content)
        .map_err(|e| format!("SYS_001: Cannot write .typ: {}", e))?;

    let args = vec![
        "compile".to_string(),
        "--format".to_string(),
        "png".to_string(),
        "--ppi".to_string(),
        ppi.to_string(),
        input_path.to_string_lossy().to_string(),
        output_path.to_string_lossy().to_string(),
    ];

    SidecarManager::spawn_typst(&args, 30_000)
        .await
        .map_err(|e| format!("SYS_001: Typst PNG export error: {}", e.message))?;

    let _ = std::fs::remove_dir_all(&tmp_dir);

    Ok(())
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

/// Retorna os templates built-in (12 mínimos: 2 por gênero × 6 gêneros).
pub fn get_builtin_templates() -> Vec<CoverTemplate> {
    vec![
        // FICÇÃO
        make_template("fiction-minimal", "fiction", "Ficção Minimalista",
            "Layout limpo com título em destaque e espaço para imagem central",
            "#991B1B", "#F8F6F0", &["clean", "modern"]),
        make_template("fiction-dramatic", "fiction", "Ficção Dramática",
            "Fundo escuro com título em branco; ideal para suspense e thriller",
            "#1E1E1E", "#EF4444", &["dark", "bold", "thriller"]),
        // NÃO-FICÇÃO
        make_template("non-fiction-clean", "non-fiction", "Não-ficção Clean",
            "Design corporativo limpo com tipografia forte",
            "#1D4ED8", "#F8FAFC", &["professional", "clean"]),
        make_template("non-fiction-bold", "non-fiction", "Não-ficção Bold",
            "Título grande em fundo contrastante, estilo self-help",
            "#D97706", "#FFFBEB", &["bold", "self-help"]),
        // TÉCNICO
        make_template("technical-code", "technical", "Técnico Código",
            "Estilo terminal/código para livros de programação",
            "#065F46", "#ECFDF5", &["tech", "code", "programming"]),
        make_template("technical-handbook", "technical", "Manual Técnico",
            "Layout de manual/referência com cores sóbrias",
            "#1E3A5F", "#F0F4F8", &["manual", "reference"]),
        // INFANTIL
        make_template("children-colorful", "children", "Infantil Colorido",
            "Cores vibrantes e tipografia arredondada para crianças",
            "#7C3AED", "#FEF3C7", &["colorful", "fun", "kids"]),
        make_template("children-playful", "children", "Infantil Lúdico",
            "Design lúdico com fundo gradiente e título grande",
            "#EC4899", "#FDF2F8", &["playful", "gradient"]),
        // POESIA
        make_template("poetry-elegant", "poetry", "Poesia Elegante",
            "Design minimalista e elegante, tipografia refinada",
            "#374151", "#FAF9F7", &["elegant", "minimal", "literary"]),
        make_template("poetry-abstract", "poetry", "Poesia Abstrata",
            "Composição abstrata com cores poéticas e layout assimétrico",
            "#6D28D9", "#EDE9FE", &["abstract", "artistic"]),
        // ACADÊMICO
        make_template("academic-formal", "academic", "Acadêmico Formal",
            "Design institucional com linha horizontal e tipografia formal",
            "#1E3A5F", "#FFFFFF", &["formal", "academic", "institutional"]),
        make_template("academic-modern", "academic", "Acadêmico Moderno",
            "Design moderno com acento colorido para publicações acadêmicas",
            "#0F766E", "#F0FDFA", &["modern", "academic"]),
    ]
}

fn make_template(id: &str, genre: &str, name: &str, description: &str,
    primary: &str, secondary: &str, tags: &[&str]) -> CoverTemplate {
    CoverTemplate {
        id: id.into(),
        genre: genre.into(),
        name: name.into(),
        description: description.into(),
        primary_color: primary.into(),
        secondary_color: secondary.into(),
        tags: tags.iter().map(|t| t.to_string()).collect(),
        typst_template: typst_template_for_genre(genre),
    }
}

/// Retorna template Typst diferenciado por gênero.
fn typst_template_for_genre(genre: &str) -> String {
    match genre {
        "fiction" => r#"// BES Cover Template — fiction
// Título grande centralizado, autor embaixo, espaço para imagem
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #align(center + horizon, pad(20mm)[
    #text(fill: rgb("PRIMARY"), size: 36pt, weight: "bold")[TITLE]
    #v(8mm)
    #line(length: 40%, stroke: 0.5pt + rgb("PRIMARY"))
    #v(4mm)
    #text(fill: rgb("PRIMARY"), size: 14pt, style: "italic")[AUTHOR]
  ])
]
"#.to_string(),

        "non-fiction" => r#"// BES Cover Template — non-fiction
// Layout profissional com barra lateral colorida
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #place(left, block(width: 8mm, height: 100%, fill: rgb("PRIMARY")))
  #align(left + bottom, pad(left: 20mm, bottom: 30mm)[
    #text(fill: rgb("PRIMARY"), size: 28pt, weight: "bold")[TITLE]
    #v(6mm)
    #text(fill: rgb("PRIMARY"), size: 13pt)[AUTHOR]
  ])
]
"#.to_string(),

        "technical" => r#"// BES Cover Template — technical
// Estilo terminal/código com fundo escuro e monospace
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("PRIMARY"))[
  #align(left + top, pad(20mm)[
    #text(fill: rgb("SECONDARY"), size: 11pt, font: "Courier New")[$ _]
    #v(40mm)
    #text(fill: rgb("SECONDARY"), size: 24pt, weight: "bold")[TITLE]
    #v(6mm)
    #line(length: 60%, stroke: 1pt + rgb("SECONDARY"))
    #v(4mm)
    #text(fill: rgb("SECONDARY"), size: 12pt)[AUTHOR]
  ])
]
"#.to_string(),

        "children" => r#"// BES Cover Template — children
// Cores vibrantes, tipografia grande e arredondada
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #align(center + top, pad(top: 30mm)[
    #text(fill: rgb("PRIMARY"), size: 42pt, weight: "bold")[TITLE]
    #v(60mm)
    #circle(radius: 20mm, fill: rgb("PRIMARY").lighten(80%))
    #v(20mm)
    #text(fill: rgb("PRIMARY"), size: 16pt)[AUTHOR]
  ])
]
"#.to_string(),

        "poetry" => r#"// BES Cover Template — poetry
// Minimalista e elegante com espaço negativo
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #align(right + horizon, pad(right: 25mm)[
    #text(fill: rgb("PRIMARY"), size: 22pt, style: "italic", weight: "regular")[TITLE]
    #v(12mm)
    #text(fill: rgb("PRIMARY"), size: 11pt, tracking: 2pt)[AUTHOR]
  ])
]
"#.to_string(),

        "academic" => r#"// BES Cover Template — academic
// Institucional com linha horizontal e tipografia formal
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #align(center + horizon, pad(20mm)[
    #line(length: 80%, stroke: 2pt + rgb("PRIMARY"))
    #v(8mm)
    #text(fill: rgb("PRIMARY"), size: 24pt, weight: "bold")[TITLE]
    #v(8mm)
    #line(length: 80%, stroke: 2pt + rgb("PRIMARY"))
    #v(12mm)
    #text(fill: rgb("PRIMARY"), size: 13pt, smallcaps: true)[AUTHOR]
  ])
]
"#.to_string(),

        _ => r#"// BES Cover Template — default
#set page(width: 152.4mm, height: 228.6mm, margin: 0mm)
#block(width: 100%, height: 100%, fill: rgb("SECONDARY"))[
  #align(left + bottom, pad(20mm)[
    #text(fill: rgb("PRIMARY"), size: 32pt, weight: "bold")[TITLE]
    #linebreak()
    #text(fill: rgb("PRIMARY"), size: 14pt)[AUTHOR]
  ])
]
"#.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Merge cover + body via Ghostscript
// ---------------------------------------------------------------------------

/// Concatena cover PDF + body PDF em um único output via Ghostscript.
///
/// Se Ghostscript falhar, retorna Err com mensagem SIDECAR_001.
pub async fn merge_cover_with_body(
    cover_pdf: &Path,
    body_pdf: &Path,
    output_pdf: &Path,
) -> Result<(), String> {
    let args = vec![
        "-dBATCH".to_string(),
        "-dNOPAUSE".to_string(),
        "-sDEVICE=pdfwrite".to_string(),
        format!("-sOutputFile={}", output_pdf.to_string_lossy()),
        cover_pdf.to_string_lossy().to_string(),
        body_pdf.to_string_lossy().to_string(),
    ];

    SidecarManager::spawn_ghostscript(&args, 120_000)
        .await
        .map_err(|e| format!("SIDECAR_001: Ghostscript merge failed: {}", e.message))?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Validações
// ---------------------------------------------------------------------------

/// Valida formato de cor hex (#RRGGBB).
pub fn validate_hex_color(hex: &str) -> Result<(), String> {
    if !hex.starts_with('#') || hex.len() != 7 {
        return Err(format!(
            "VAL_002: Invalid hex color '{}'. Expected format: #RRGGBB",
            hex
        ));
    }
    if !hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(format!("VAL_002: Invalid hex characters in '{}'", hex));
    }
    Ok(())
}

/// Valida plataforma suportada.
pub fn validate_platform(platform: &str) -> Result<(), String> {
    match platform {
        "amazon-kdp" | "ingram" | "generic" => Ok(()),
        _ => Err(format!(
            "VAL_002: Unsupported platform '{}'. Use: amazon-kdp | ingram | generic",
            platform
        )),
    }
}

/// Valida formato de exportação de imagem.
pub fn validate_export_format(format: &str) -> Result<(), String> {
    match format {
        "png" | "jpeg" => Ok(()),
        _ => Err(format!(
            "VAL_002: Invalid export format '{}'. Use: png | jpeg",
            format
        )),
    }
}

/// Valida resolução DPI para exportação (150–600).
pub fn validate_resolution(resolution: u32) -> Result<(), String> {
    if resolution < 150 || resolution > 600 {
        return Err(format!(
            "VAL_003: Resolution {} out of range [150, 600] DPI",
            resolution
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Utilidades
// ---------------------------------------------------------------------------

/// Codificação base64 (sem crate adicional).
fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    let mut i = 0;
    let len = data.len();

    while i < len {
        let b0 = data[i] as u32;
        let b1 = if i + 1 < len { data[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < len { data[i + 2] as u32 } else { 0 };

        result.push(CHARS[((b0 >> 2) & 0x3F) as usize] as char);
        result.push(CHARS[(((b0 << 4) | (b1 >> 4)) & 0x3F) as usize] as char);
        result.push(if i + 1 < len { CHARS[(((b1 << 2) | (b2 >> 6)) & 0x3F) as usize] as char } else { '=' });
        result.push(if i + 2 < len { CHARS[(b2 & 0x3F) as usize] as char } else { '=' });

        i += 3;
    }
    result
}

// ---------------------------------------------------------------------------
// Testes unitários
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_spine_width_kdp_200_pages() {
        let result = calculate_spine_width(200, "amazon-kdp", "white");
        assert!((result.spine_width_mm - 12.7).abs() < 0.01, "KDP 200p = 12.7mm, got {}", result.spine_width_mm);
        assert!((result.spine_width_inches - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_calculate_spine_width_ingram_200_pages() {
        let result = calculate_spine_width(200, "ingram", "white");
        // 200 × 0.002252 × 25.4 = 11.45mm
        assert!((result.spine_width_mm - 11.44).abs() < 0.1, "Ingram 200p ~= 11.44mm, got {}", result.spine_width_mm);
    }

    #[test]
    fn test_calculate_spine_width_zero_pages() {
        let result = calculate_spine_width(0, "amazon-kdp", "white");
        assert_eq!(result.spine_width_mm, 0.0);
        assert_eq!(result.spine_width_inches, 0.0);
    }

    #[test]
    fn test_calculate_spine_width_generic() {
        let result = calculate_spine_width(300, "generic", "cream");
        // Usa KDP formula: 300 × 0.0025 × 25.4 = 19.05mm
        assert!((result.spine_width_mm - 19.05).abs() < 0.01);
    }

    #[test]
    fn test_validate_hex_color_valid() {
        assert!(validate_hex_color("#991B1B").is_ok());
        assert!(validate_hex_color("#FFFFFF").is_ok());
        assert!(validate_hex_color("#000000").is_ok());
        assert!(validate_hex_color("#aabbcc").is_ok());
    }

    #[test]
    fn test_validate_hex_color_invalid() {
        assert!(validate_hex_color("#ZZZ").is_err());
        assert!(validate_hex_color("991B1B").is_err());
        assert!(validate_hex_color("#GGGGGG").is_err());
        assert!(validate_hex_color("#12345").is_err());
        assert!(validate_hex_color("").is_err());
    }

    #[test]
    fn test_validate_platform() {
        assert!(validate_platform("amazon-kdp").is_ok());
        assert!(validate_platform("ingram").is_ok());
        assert!(validate_platform("generic").is_ok());
        assert!(validate_platform("invalid-platform").is_err());
        assert!(validate_platform("").is_err());
    }

    #[test]
    fn test_validate_export_format() {
        assert!(validate_export_format("png").is_ok());
        assert!(validate_export_format("jpeg").is_ok());
        assert!(validate_export_format("bmp").is_err());
        assert!(validate_export_format("gif").is_err());
    }

    #[test]
    fn test_validate_resolution() {
        assert!(validate_resolution(150).is_ok());
        assert!(validate_resolution(300).is_ok());
        assert!(validate_resolution(600).is_ok());
        assert!(validate_resolution(149).is_err());
        assert!(validate_resolution(601).is_err());
    }

    #[test]
    fn test_get_builtin_templates_count() {
        let templates = get_builtin_templates();
        assert_eq!(templates.len(), 12, "Should have exactly 12 built-in templates");
    }

    #[test]
    fn test_get_builtin_templates_genres_coverage() {
        let templates = get_builtin_templates();
        for genre in &["fiction", "non-fiction", "technical", "children", "poetry", "academic"] {
            let count = templates.iter().filter(|t| t.genre == *genre).count();
            assert!(count >= 2, "Genre '{}' should have >= 2 templates, got {}", genre, count);
        }
    }

    #[test]
    fn test_build_cover_typst_contains_content() {
        let config = CoverConfig {
            id: "test".into(),
            project_id: "proj".into(),
            template_id: "fiction-minimal".into(),
            genre: "fiction".into(),
            platform: "amazon-kdp".into(),
            title_override: None,
            subtitle: None,
            author_override: None,
            back_cover_text: "Sinopse do livro".into(),
            primary_color: "#991B1B".into(),
            secondary_color: "#F8F6F0".into(),
            font_title: "Playfair Display".into(),
            font_author: "Lato".into(),
            cover_image_path: None,
            cover_image_original: None,
            cover_image_dpi: None,
            page_count: 200,
            spine_width_mm: 12.7,
            paper_type: "white".into(),
            created_at: "2026-01-01 00:00:00".into(),
            updated_at: "2026-01-01 00:00:00".into(),
        };

        let typst = build_cover_typst(&config, "Meu Livro", "Autor Teste", 152.4, 228.6);
        assert!(typst.contains("Meu Livro"), "Should contain book title");
        assert!(typst.contains("Autor Teste"), "Should contain author name");
        assert!(typst.contains("991B1B"), "Should contain primary color");
        assert!(typst.contains("F8F6F0"), "Should contain secondary color");
        assert!(typst.contains("Sinopse do livro"), "Should contain back cover text");
    }
}
