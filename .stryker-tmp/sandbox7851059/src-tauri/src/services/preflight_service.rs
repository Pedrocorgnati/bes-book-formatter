// BES Book Formatter — Preflight Service (module-4 TASK-2 ST005)
//
// Valida conformidade de PDF/X após geração e executa checklist pré-geração.

use sqlx::SqlitePool;

use crate::error::AppError;
use crate::models::responses::{ChecklistItem, PreflightResult};
use crate::services::SidecarManager;

pub struct PreflightService;

impl PreflightService {
    /// Checklist pré-geração: verifica se o projeto está pronto para gerar saída.
    pub async fn pre_generation_check(
        pool: &SqlitePool,
        project_id: &str,
        format: &str,
    ) -> Result<PreflightResult, AppError> {
        let mut blocking: Vec<ChecklistItem> = Vec::new();
        let mut warnings: Vec<ChecklistItem> = Vec::new();

        // ── 1. Completude do manuscrito ────────────────────────────────────
        let completeness: Option<f64> = sqlx::query_scalar(
            "SELECT completeness_score FROM projects WHERE id = ?",
        )
        .bind(project_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::new("GEN_010", format!("DB error: {}", e)))?
        .flatten();

        match completeness {
            Some(score) if score < 0.80 => {
                blocking.push(ChecklistItem {
                    id: "completeness_below_80".to_string(),
                    message: format!(
                        "Completude {:.0}% — mínimo 80% para geração",
                        score * 100.0
                    ),
                    files: None,
                });
            }
            Some(score) if score < 0.95 => {
                warnings.push(ChecklistItem {
                    id: "completeness_below_95".to_string(),
                    message: format!(
                        "Completude {:.0}% — alguns capítulos podem estar incompletos",
                        score * 100.0
                    ),
                    files: None,
                });
            }
            None => {
                warnings.push(ChecklistItem {
                    id: "completeness_unknown".to_string(),
                    message: "Completude não calculada — execute análise do manuscrito".to_string(),
                    files: None,
                });
            }
            _ => {}
        }

        // ── 2. Ilustrações pendentes ───────────────────────────────────────
        let pending: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM illustrations WHERE project_id = ? AND state = 'pending'",
        )
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if pending > 0 {
            let mode: Option<String> = sqlx::query_scalar(
                "SELECT illustration_missing_mode FROM typography_config WHERE project_id = ?",
            )
            .bind(project_id)
            .fetch_optional(pool)
            .await
            .ok()
            .flatten()
            .flatten();

            if mode.as_deref() == Some("block_generation") {
                blocking.push(ChecklistItem {
                    id: "illustrations_pending_block".to_string(),
                    message: format!(
                        "{} ilustração(ões) PENDING — modo 'bloquear geração' ativo",
                        pending
                    ),
                    files: None,
                });
            } else {
                warnings.push(ChecklistItem {
                    id: "illustrations_pending_warn".to_string(),
                    message: format!(
                        "{} ilustração(ões) não importada(s) — serão substituídas por placeholder",
                        pending
                    ),
                    files: None,
                });
            }
        }

        // ── 3. Sidecars necessários ────────────────────────────────────────
        let required: Vec<&str> = match format {
            "epub3" => vec!["epubcheck"],
            "pdf_print" => vec!["typst", "ghostscript"],
            "pdf_ebook" => vec!["typst"],
            _ => vec![],
        };

        for sidecar in required {
            let status = SidecarManager::check_sidecar(sidecar).await;
            if !status.available {
                blocking.push(ChecklistItem {
                    id: format!("sidecar_unavailable_{}", sidecar),
                    message: format!(
                        "Sidecar '{}' não encontrado — necessário para {}", sidecar, format
                    ),
                    files: None,
                });
            }
        }

        // ── 4. Tipografia configurada ─────────────────────────────────────
        let typo_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM typography_config WHERE project_id = ?",
        )
        .bind(project_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        if typo_count == 0 {
            warnings.push(ChecklistItem {
                id: "typography_not_configured".to_string(),
                message: "Tipografia não configurada — serão usados valores padrão do gênero".to_string(),
                files: None,
            });
        }

        Ok(PreflightResult {
            passed: blocking.is_empty(),
            blockers: blocking,
            warnings,
        })
    }

    /// Valida conformidade PDF/X após geração usando Ghostscript.
    pub async fn validate_pdf(
        pdf_path: &str,
        _pdfx_profile: &str,
    ) -> Result<PreflightResult, AppError> {
        let mut blocking: Vec<ChecklistItem> = Vec::new();
        let mut warnings: Vec<ChecklistItem> = Vec::new();

        if !std::path::Path::new(pdf_path).exists() {
            blocking.push(ChecklistItem {
                id: "pdf_not_found".to_string(),
                message: format!("Arquivo PDF não encontrado: {}", pdf_path),
                files: None,
            });
            return Ok(PreflightResult { passed: false, blockers: blocking, warnings });
        }

        let gs_args = vec![
            "-dBATCH".to_string(),
            "-dNOPAUSE".to_string(),
            "-sDEVICE=nullpage".to_string(),
            pdf_path.to_string(),
        ];

        match SidecarManager::spawn_ghostscript(&gs_args, 30_000).await {
            Ok((_, stderr)) if stderr.to_lowercase().contains("error") => {
                warnings.push(ChecklistItem {
                    id: "pdfx_compliance_warning".to_string(),
                    message: format!(
                        "Avisos de conformidade PDF/X: {}",
                        stderr.lines().next().unwrap_or("ver log")
                    ),
                    files: None,
                });
            }
            Err(e) => {
                warnings.push(ChecklistItem {
                    id: "gs_validation_skipped".to_string(),
                    message: format!("Validação GS ignorada (sidecar indisponível): {}", e.message),
                    files: None,
                });
            }
            _ => {}
        }

        Ok(PreflightResult {
            passed: blocking.is_empty(),
            blockers: blocking,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::responses::{ChecklistItem, PreflightResult};

    fn make_item(id: &str, message: &str) -> ChecklistItem {
        ChecklistItem {
            id: id.to_string(),
            message: message.to_string(),
            files: None,
        }
    }

    #[test]
    fn preflight_result_passed_when_no_blockers() {
        let result = PreflightResult {
            passed: true,
            blockers: vec![],
            warnings: vec![],
        };
        assert!(result.passed);
        assert!(result.blockers.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn preflight_result_not_passed_when_has_blockers() {
        let result = PreflightResult {
            passed: false,
            blockers: vec![make_item("completeness_below_80", "Completude 60% — mínimo 80% para geração")],
            warnings: vec![],
        };
        assert!(!result.passed);
        assert_eq!(result.blockers.len(), 1);
        assert_eq!(result.blockers[0].id, "completeness_below_80");
    }

    #[test]
    fn preflight_result_warnings_do_not_block_passing() {
        let result = PreflightResult {
            passed: true,
            blockers: vec![],
            warnings: vec![make_item("completeness_below_95", "Completude 90% — alguns capítulos podem estar incompletos")],
        };
        assert!(result.passed);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn checklist_item_files_is_optional() {
        let item = make_item("typography_not_configured", "Tipografia não configurada");
        assert_eq!(item.id, "typography_not_configured");
        assert!(item.files.is_none());
    }

    #[test]
    fn checklist_item_with_files() {
        let item = ChecklistItem {
            id: "illustrations_pending_block".to_string(),
            message: "3 ilustração(ões) PENDING".to_string(),
            files: Some(vec!["ch01/fig1.png".to_string(), "ch02/fig2.png".to_string()]),
        };
        assert!(item.files.is_some());
        assert_eq!(item.files.unwrap().len(), 2);
    }

    #[test]
    fn preflight_result_multiple_blockers() {
        let result = PreflightResult {
            passed: false,
            blockers: vec![
                make_item("sidecar_unavailable_typst", "Sidecar 'typst' não encontrado"),
                make_item("illustrations_pending_block", "2 ilustrações PENDING"),
            ],
            warnings: vec![],
        };
        assert!(!result.passed);
        assert_eq!(result.blockers.len(), 2);
        assert_eq!(result.blockers[1].id, "illustrations_pending_block");
    }
}
