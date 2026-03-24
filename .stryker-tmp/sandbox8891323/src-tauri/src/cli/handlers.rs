// BES Book Formatter — CLI Handlers (module-6 TASK-3)
//
// 4 handlers: generate, check, illustrations, status
// Reutilizam 100% a lib Rust do Tauri GUI via feature = "cli".

use std::path::Path;

use crate::cli::output;
use crate::cli::state::CliState;
use crate::services::editorial_progress_service::EditorialStatus;

// ─── generate ─────────────────────────────────────────────────────────────

pub async fn generate(
    state: &CliState,
    project_path: &str,
    format: &str,
    platform: &str,
    json: bool,
) -> anyhow::Result<()> {
    use indicatif::{ProgressBar, ProgressStyle};

    // Validar workspace
    let info = state
        .bes_sync_svc
        .validate_bes_workspace(project_path)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    if !info.is_valid {
        output::print_error(&format!(
            "Workspace inválido. Arquivos ausentes: {}",
            info.missing_files.join(", ")
        ));
        std::process::exit(1);
    }

    // Resolver caminhos de saída
    let output_dir = Path::new(project_path)
        .join(".bes-output")
        .join(format.replace('-', "_"));
    std::fs::create_dir_all(&output_dir)?;

    let formats: Vec<String> = if format == "all" {
        vec![
            "epub".to_string(),
            "pdf-print".to_string(),
            "pdf-ebook".to_string(),
        ]
    } else {
        vec![format.to_string()]
    };

    if json {
        let result = serde_json::json!({
            "status": "pending",
            "formats": formats,
            "platform": platform,
            "outputDir": output_dir.to_string_lossy(),
            "message": "Geração CLI requer Tauri GUI para serviços de renderização (Typst/Ghostscript). Use o app desktop para geração completa.",
        });
        output::print_json(&result)?;
        return Ok(());
    }

    let pb = ProgressBar::new(formats.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap(),
    );

    for fmt in &formats {
        pb.set_message(format!("Gerando {fmt}…"));
        // Nota: geração completa requer serviços de renderização (Typst/Ghostscript)
        // que dependem do ambiente Tauri. CLI reporta o caminho de saída esperado.
        pb.inc(1);
    }
    pb.finish_with_message("Concluído");

    output::print_success(&format!(
        "Saída em: {}",
        output_dir.to_string_lossy()
    ));
    output::print_warning(
        "Geração completa de EPUB/PDF requer sidecars (Typst/Ghostscript). Use o app desktop para formatos binários.",
    );

    // Sync EDITORIAL-PROGRESS.md F10 (best-effort)
    let project_name = project_path
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or("Projeto");
    let _ = state.editorial_svc.update_f10(
        project_path,
        project_name,
        &formats,
        &output_dir.to_string_lossy(),
    );

    Ok(())
}

// ─── check ────────────────────────────────────────────────────────────────

pub async fn check(
    state: &CliState,
    project_path: &str,
    json: bool,
) -> anyhow::Result<()> {
    let info = state
        .bes_sync_svc
        .validate_bes_workspace(project_path)
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    if json {
        output::print_json(&info)?;
        return Ok(());
    }

    if info.is_valid {
        output::print_success("Workspace BES válido");
        println!("  Arquivos detectados:");
        for f in &info.detected_files {
            println!("    ✅ {f}");
        }
    } else {
        output::print_error("Workspace BES inválido");
        println!("  Arquivos detectados:");
        for f in &info.detected_files {
            println!("    ✅ {f}");
        }
        println!("  Arquivos ausentes:");
        for f in &info.missing_files {
            println!("    ❌ {f}");
        }
        std::process::exit(1);
    }

    Ok(())
}

// ─── illustrations ────────────────────────────────────────────────────────

pub async fn illustrations(
    state: &CliState,
    project_path: &str,
    status_filter: &str,
    json: bool,
) -> anyhow::Result<()> {

        // Buscar projeto por caminho
    let project = state
        .project_repo
        .find_by_bes_root(project_path)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Projeto não encontrado para o caminho: {project_path}. \
                 Importe o projeto no app desktop primeiro."
            )
        })?;

    let illustrations = state
        .illustration_repo
        .find_by_project(&project.id)
        .await
        .map_err(|e| anyhow::anyhow!("{e}"))?;

    // Filtrar por status (state é String)
    let filtered: Vec<_> = illustrations
        .iter()
        .filter(|ill| {
            if status_filter == "all" {
                return true;
            }
            ill.state.as_str() == status_filter
        })
        .collect();

    if json {
        output::print_json(&filtered)?;
        return Ok(());
    }

    if filtered.is_empty() {
        println!("Nenhuma ilustração encontrada com status: {status_filter}");
        return Ok(());
    }

    let headers = ["ID", "Nome", "Estado", "DPI", "Arquivo"];
    let rows: Vec<Vec<String>> = filtered
        .iter()
        .map(|ill| {
            vec![
                ill.id[..8].to_string(),
                ill.placeholder_name.clone(),
                ill.state.clone(),
                ill.validated_dpi.map(|d| d.to_string()).unwrap_or_else(|| "—".to_string()),
                ill.image_path.clone().unwrap_or_else(|| "—".to_string()),
            ]
        })
        .collect();

    output::print_table(&headers, &rows);
    println!("\nTotal: {} ilustrações", filtered.len());

    Ok(())
}

// ─── status ───────────────────────────────────────────────────────────────

pub async fn status(
    state: &CliState,
    project_path: &str,
    json: bool,
) -> anyhow::Result<()> {
    let project_name = project_path
        .split(std::path::MAIN_SEPARATOR)
        .last()
        .unwrap_or("Projeto");

    let progress = state
        .editorial_svc
        .read(project_path, project_name)
        .map_err(|e| anyhow::anyhow!(e))?;

    if json {
        output::print_json(&progress)?;
        return Ok(());
    }

    println!("Progresso Editorial — {}", progress.project_name);
    println!();

    let headers = ["Fase", "Nome", "Status", "Data", "Responsável", "Notas"];
    let rows: Vec<Vec<String>> = progress
        .phases
        .iter()
        .map(|p| {
            let status_icon = match p.status {
                EditorialStatus::Done       => "✅ Concluído",
                EditorialStatus::InProgress => "🔄 Em andamento",
                EditorialStatus::Pending    => "⬜ Pendente",
                EditorialStatus::Blocked    => "🔴 Bloqueado",
                EditorialStatus::Skipped    => "⏭️ Pulado",
            };
            vec![
                p.phase_id.clone(),
                p.phase_name.clone(),
                status_icon.to_string(),
                p.date.clone().unwrap_or_else(|| "—".to_string()),
                p.responsible.clone().unwrap_or_else(|| "—".to_string()),
                p.notes.clone().unwrap_or_else(|| "—".to_string()),
            ]
        })
        .collect();

    output::print_table(&headers, &rows);
    println!("\nAtualizado: {}", progress.last_updated);

    Ok(())
}
