// BES Book Formatter — Cover IPC Commands (module-7-cover-design)
//
// 6 comandos IPC: get_cover_config, calculate_spine_width, save_cover_config,
// generate_cover_pdf, get_cover_templates, export_cover_image

use std::path::Path;

use sqlx::SqlitePool;
use tauri::State;

use crate::models::{ApiResponse, CoverConfig, CoverConfigInput, CoverTemplate, SpineWidthResult};
use crate::repositories::{CoverConfigRepository, ProjectRepository};
use crate::services::cover_service;
use crate::services::SidecarManager;

// ---------------------------------------------------------------------------
// get_cover_config
// ---------------------------------------------------------------------------

/// Retorna a configuração de capa salva para um projeto, ou null se não existir.
///
/// Frontend: `const config = await invoke<ApiResponse<CoverConfig | null>>('get_cover_config', { projectId });`
#[tauri::command]
pub async fn get_cover_config(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<Option<CoverConfig>>, String> {
    let pool_ref = pool.inner().clone();
    let cover_repo = CoverConfigRepository::new(pool_ref);

    match cover_repo.get_by_project(&project_id).await {
        Ok(config) => Ok(ApiResponse::ok(config)),
        Err(e) => Ok(ApiResponse::err(format!("DB_001: {}", e))),
    }
}

// ---------------------------------------------------------------------------
// calculate_spine_width
// ---------------------------------------------------------------------------

/// Calcula largura da lombada para o projeto dado.
///
/// Busca page_count do projeto; se não disponível, retorna largura 0mm com aviso.
///
/// Frontend: `const result = await invoke<ApiResponse<SpineWidthResult>>('calculate_spine_width', { projectId, platform, paperType });`
#[tauri::command]
pub async fn calculate_spine_width(
    pool: State<'_, SqlitePool>,
    project_id: String,
    platform: String,
    paper_type: String,
) -> Result<ApiResponse<SpineWidthResult>, String> {
    // Validar plataforma
    if let Err(e) = cover_service::validate_platform(&platform) {
        return Ok(ApiResponse::err(e));
    }

    let pool_ref = pool.inner().clone();
    let project_repo = ProjectRepository::new(pool_ref.clone());

    // Buscar page_count do projeto
    // Verificar que projeto existe
    let _project = match project_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_080: Project not found: {}",
                project_id
            )));
        }
        Err(e) => {
            return Ok(ApiResponse::err(format!("DB_001: {}", e)));
        }
    };

    // Obter page_count da cover_config (se houver)
    let cover_repo = CoverConfigRepository::new(pool_ref.clone());
    let page_count = if let Ok(Some(cover)) = cover_repo.get_by_project(&project_id).await {
        cover.page_count as u32
    } else {
        0u32
    };

    let result = cover_service::calculate_spine_width(page_count, &platform, &paper_type);

    if page_count == 0 {
        let warnings = vec![
            "VAL_003: page_count=0 — generate PDF first for accurate spine calculation".to_string(),
        ];
        return Ok(ApiResponse {
            data: Some(result),
            error: None,
            warnings,
        });
    }

    Ok(ApiResponse::ok(result))
}

// ---------------------------------------------------------------------------
// save_cover_config
// ---------------------------------------------------------------------------

/// Salva/atualiza configuração de capa no SQLite.
///
/// Valida cores hex, plataforma e DPI da imagem (se fornecida).
///
/// Frontend: `const config = await invoke<ApiResponse<CoverConfig>>('save_cover_config', { config: { ... } });`
#[tauri::command]
pub async fn save_cover_config(
    pool: State<'_, SqlitePool>,
    config: CoverConfigInput,
) -> Result<ApiResponse<CoverConfig>, String> {
    // Validar plataforma
    if let Some(ref platform) = config.platform {
        if let Err(e) = cover_service::validate_platform(platform) {
            return Ok(ApiResponse::err(e));
        }
    }

    // Validar cores hex se fornecidas
    if let Some(ref color) = config.primary_color {
        if let Err(e) = cover_service::validate_hex_color(color) {
            return Ok(ApiResponse::err(e));
        }
    }
    if let Some(ref color) = config.secondary_color {
        if let Err(e) = cover_service::validate_hex_color(color) {
            return Ok(ApiResponse::err(e));
        }
    }

    let pool_ref = pool.inner().clone();

    // Calcular spine_width automaticamente
    let cover_repo = CoverConfigRepository::new(pool_ref.clone());
    let existing = cover_repo.get_by_project(&config.project_id).await.unwrap_or(None);

    let platform = config.platform.as_deref()
        .or(existing.as_ref().map(|c| c.platform.as_str()))
        .unwrap_or("amazon-kdp");
    let paper_type = config.paper_type.as_deref()
        .or(existing.as_ref().map(|c| c.paper_type.as_str()))
        .unwrap_or("white");
    let page_count = config.page_count
        .or(existing.as_ref().map(|c| c.page_count))
        .unwrap_or(0) as u32;

    let spine_result = cover_service::calculate_spine_width(page_count, platform, paper_type);

    match cover_repo.upsert(&config, spine_result.spine_width_mm).await {
        Ok(saved) => {
            // DPI warning se imagem com DPI < 300
            let mut warnings = Vec::new();
            if let Some(dpi) = saved.cover_image_dpi {
                if dpi < 300 {
                    warnings.push(format!(
                        "VAL_003: Cover image DPI ({}) is below 300. Print quality may be poor.",
                        dpi
                    ));
                }
            }
            if warnings.is_empty() {
                Ok(ApiResponse::ok(saved))
            } else {
                Ok(ApiResponse { data: Some(saved), error: None, warnings })
            }
        }
        Err(e) => Ok(ApiResponse::err(format!("DB_001: Failed to save cover config: {}", e))),
    }
}

// ---------------------------------------------------------------------------
// generate_cover_pdf
// ---------------------------------------------------------------------------

/// Gera PDF de capa (frente + lombada + verso) via Typst.
///
/// Retorna base64 PNG do preview (baixa resolução: 72 PPI).
///
/// Frontend: `const preview = await invoke<ApiResponse<string>>('generate_cover_pdf', { projectId });`
#[tauri::command]
pub async fn generate_cover_pdf(
    pool: State<'_, SqlitePool>,
    project_id: String,
) -> Result<ApiResponse<String>, String> {
    // Verificar disponibilidade do Typst antes de tentar compilar
    let typst_status = SidecarManager::check_sidecar("typst").await;
    if !typst_status.available {
        return Ok(ApiResponse::err(
            "SIDECAR_001: Typst not available. Install Typst or check sidecar configuration.",
        ));
    }

    let pool_ref = pool.inner().clone();

    let cover_repo = CoverConfigRepository::new(pool_ref.clone());
    let cover_config = match cover_repo.get_by_project(&project_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::err(
                "COVER_001: Cover config not found. Save cover config first.",
            ));
        }
        Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
    };

    let project_repo = ProjectRepository::new(pool_ref);
    let project = match project_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_080: Project not found: {}",
                project_id
            )));
        }
        Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
    };

    let title = cover_config
        .title_override
        .as_deref()
        .unwrap_or(&project.name);
    let author = cover_config
        .author_override
        .as_deref()
        .unwrap_or_else(|| "");

    let front_width = cover_service::front_cover_width_mm(&cover_config.platform);
    let height_mm = 228.6; // 9 inches × 25.4

    let typst_content =
        cover_service::build_cover_typst(&cover_config, title, author, front_width, height_mm);

    // Gerar preview PNG base64 (72 PPI para preview rápido)
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        cover_service::compile_typst_to_png_base64(&typst_content, 72),
    )
    .await
    {
        Ok(Ok(base64)) => Ok(ApiResponse::ok(base64)),
        Ok(Err(e)) => Ok(ApiResponse::err(e)),
        Err(_) => Ok(ApiResponse::err(
            "SYS_003: Cover preview timed out (>5s). Try again.",
        )),
    }
}

// ---------------------------------------------------------------------------
// get_cover_templates
// ---------------------------------------------------------------------------

/// Lista templates de capa por gênero.
///
/// `genre = None` retorna todos os templates (12 built-in).
///
/// Frontend: `const templates = await invoke<ApiResponse<CoverTemplate[]>>('get_cover_templates', { genre: 'fiction' });`
#[tauri::command]
pub async fn get_cover_templates(
    genre: Option<String>,
) -> Result<ApiResponse<Vec<CoverTemplate>>, String> {
    let all_templates = cover_service::get_builtin_templates();

    let filtered: Vec<CoverTemplate> = match genre {
        Some(ref g) if g != "all" => all_templates
            .into_iter()
            .filter(|t| t.genre == g.as_str())
            .collect(),
        _ => all_templates,
    };

    Ok(ApiResponse::ok(filtered))
}

// ---------------------------------------------------------------------------
// export_cover_image
// ---------------------------------------------------------------------------

/// Exporta capa como PNG/JPEG de alta resolução para uso em marketing.
///
/// Salva em `<project_workspace>/.bes-output/cover/`.
///
/// Frontend: `const path = await invoke<ApiResponse<string>>('export_cover_image', { projectId, format: 'png', resolution: 300 });`
#[tauri::command]
pub async fn export_cover_image(
    pool: State<'_, SqlitePool>,
    project_id: String,
    format: String,
    resolution: u32,
) -> Result<ApiResponse<String>, String> {
    // Verificar disponibilidade do Typst
    let typst_status = SidecarManager::check_sidecar("typst").await;
    if !typst_status.available {
        return Ok(ApiResponse::err(
            "SIDECAR_001: Typst not available. Install Typst or check sidecar configuration.",
        ));
    }

    // Validar formato e resolução
    if let Err(e) = cover_service::validate_export_format(&format) {
        return Ok(ApiResponse::err(e));
    }
    if let Err(e) = cover_service::validate_resolution(resolution) {
        return Ok(ApiResponse::err(e));
    }

    let pool_ref = pool.inner().clone();

    let cover_repo = CoverConfigRepository::new(pool_ref.clone());
    let cover_config = match cover_repo.get_by_project(&project_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::err(
                "COVER_001: Cover config not found. Save cover config first.",
            ));
        }
        Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
    };

    let project_repo = ProjectRepository::new(pool_ref);
    let project = match project_repo.find_by_id(&project_id).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Ok(ApiResponse::err(format!(
                "PROJECT_080: Project not found: {}",
                project_id
            )));
        }
        Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
    };

    let title = cover_config
        .title_override
        .as_deref()
        .unwrap_or(&project.name);
    let author = cover_config.author_override.as_deref().unwrap_or("");

    let front_width = cover_service::front_cover_width_mm(&cover_config.platform);
    let height_mm = 228.6;

    let typst_content =
        cover_service::build_cover_typst(&cover_config, title, author, front_width, height_mm);

    // Determinar caminho de saída
    let workspace = &project.bes_root_path;
    let output_dir = Path::new(workspace).join(".bes-output").join("cover");
    std::fs::create_dir_all(&output_dir)
        .map_err(|e| format!("SYS_001: Cannot create output dir: {}", e))?;

    let slug = slugify(&project.name);
    let output_filename = format!("{}-cover.{}", slug, if format == "jpeg" { "jpg" } else { &format });
    let output_path = output_dir.join(&output_filename);

    // Compilar via Typst
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        cover_service::compile_typst_to_png_file(&typst_content, &output_path, resolution),
    )
    .await;

    match result {
        Ok(Ok(())) => {
            let path_str = output_path.to_string_lossy().to_string();
            Ok(ApiResponse::ok(path_str))
        }
        Ok(Err(e)) => Ok(ApiResponse::err(e)),
        Err(_) => Ok(ApiResponse::err(
            "SYS_003: Export timed out (>30s). Try again.",
        )),
    }
}

// ---------------------------------------------------------------------------
// render_mockup_3d
// ---------------------------------------------------------------------------

/// Renders a 3D mockup of the book cover.
/// Returns the cover PDF preview as base64 PNG (same as generate_cover_pdf)
/// plus spine width data for CSS 3D transform rendering on the frontend.
///
/// The actual 3D rendering is done client-side via CSS transforms in CoverPreview.svelte.
/// This command provides the data needed: cover image + spine dimensions.
///
/// Frontend: `const mockup = await invoke<ApiResponse<Mockup3dResult>>('render_mockup_3d', { projectId, coverPath });`
#[tauri::command]
pub async fn render_mockup_3d(
    pool: State<'_, SqlitePool>,
    project_id: String,
    cover_path: Option<String>,
) -> Result<ApiResponse<serde_json::Value>, String> {
    let pool_ref = pool.inner().clone();

    // Get cover config
    let cover_repo = CoverConfigRepository::new(pool_ref.clone());
    let cover_config = match cover_repo.get_by_project(&project_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::err(
                "COVER_001: Cover config not found. Save cover config first.",
            ));
        }
        Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
    };

    // Calculate spine width
    let page_count = cover_config.page_count as u32;
    let spine = cover_service::calculate_spine_width(
        page_count,
        &cover_config.platform,
        &cover_config.paper_type,
    );

    // Generate cover preview base64 (72 PPI for 3D mockup)
    let typst_status = SidecarManager::check_sidecar("typst").await;
    let preview_base64 = if typst_status.available {
        let project_repo = ProjectRepository::new(pool_ref);
        let project = match project_repo.find_by_id(&project_id).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                return Ok(ApiResponse::err(format!(
                    "PROJECT_080: Project not found: {}",
                    project_id
                )));
            }
            Err(e) => return Ok(ApiResponse::err(format!("DB_001: {}", e))),
        };

        let title = cover_config
            .title_override
            .as_deref()
            .unwrap_or(&project.name);
        let author = cover_config.author_override.as_deref().unwrap_or("");
        let front_width = cover_service::front_cover_width_mm(&cover_config.platform);
        let height_mm = 228.6;

        let typst_content =
            cover_service::build_cover_typst(&cover_config, title, author, front_width, height_mm);

        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            cover_service::compile_typst_to_png_base64(&typst_content, 72),
        )
        .await
        {
            Ok(Ok(b64)) => Some(b64),
            _ => None,
        }
    } else {
        None
    };

    // Return structured data for CSS 3D rendering
    let result = serde_json::json!({
        "previewBase64": preview_base64,
        "spineWidthMm": spine.spine_width_mm,
        "spineWidthPx": (spine.spine_width_mm * 3.7795).round() as u32,
        "coverWidthMm": cover_service::front_cover_width_mm(&cover_config.platform),
        "coverHeightMm": 228.6,
        "primaryColor": cover_config.primary_color,
        "renderMode": "css3d"
    });

    Ok(ApiResponse::ok(result))
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
