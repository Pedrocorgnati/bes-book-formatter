// BES Book Formatter — Editorial Progress Service (module-6 TASK-2)
//
// Lê EDITORIAL-PROGRESS.md, parseia tabela Markdown de 12 fases (F1-F12),
// e sincroniza a coluna F10 (geração) com os resultados mais recentes.
//
// REGRA CRÍTICA: F10 é a ÚNICA fase modificável por este serviço.
//                F1-F9 e F11-F12 NUNCA devem ser alterados.

use serde::{Deserialize, Serialize};
use std::path::Path;

// ─── Structs ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EditorialStatus {
    Done,
    InProgress,
    Pending,
    Blocked,
    Skipped,
}

impl EditorialStatus {
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "done" | "✅" | "concluído" | "concluida" => Self::Done,
            "in_progress" | "em andamento" | "🔄" => Self::InProgress,
            "blocked" | "bloqueado" | "🔴" => Self::Blocked,
            "skipped" | "pulado" | "⏭️" => Self::Skipped,
            _ => Self::Pending,
        }
    }

    fn to_markdown(&self) -> &'static str {
        match self {
            Self::Done => "✅",
            Self::InProgress => "🔄",
            Self::Pending => "⬜",
            Self::Blocked => "🔴",
            Self::Skipped => "⏭️",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseStatus {
    pub phase_id: String,   // "F1" ... "F12"
    pub phase_name: String,
    pub status: EditorialStatus,
    pub date: Option<String>,
    pub responsible: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorialProgress {
    pub project_name: String,
    pub phases: Vec<PhaseStatus>, // sempre 12 fases
    pub last_updated: String,     // ISO datetime
}

// ─── Template padrão ───────────────────────────────────────────────────────

const PHASE_NAMES: [(&str, &str); 12] = [
    ("F1", "Brief"),
    ("F2", "PRD"),
    ("F3", "Otimização SystemForge"),
    ("F4", "WBS"),
    ("F5", "WBS+"),
    ("F6", "Business"),
    ("F7", "Execução"),
    ("F8", "Complemento Manual"),
    ("F9", "QA"),
    ("F10", "Geração"),
    ("F11", "Deploy"),
    ("F12", "Marketing"),
];

pub fn default_template(project_name: &str) -> String {
    let header = format!(
        "# EDITORIAL-PROGRESS — {project_name}\n\n\
         | Fase | Nome | Status | Data | Responsável | Notas |\n\
         |------|------|--------|------|-------------|-------|\n"
    );
    let rows: String = PHASE_NAMES
        .iter()
        .map(|(id, name)| {
            format!("| {id} | {name} | ⬜ | — | — | — |\n")
        })
        .collect();
    format!("{}{}", header, rows)
}

// ─── Serviço ───────────────────────────────────────────────────────────────

pub struct EditorialProgressService;

impl EditorialProgressService {
    pub fn new() -> Self {
        Self
    }

    /// Lê EDITORIAL-PROGRESS.md, parseia as 12 fases e retorna o estado atual.
    /// Cria arquivo com template padrão se não existir.
    pub fn read(
        &self,
        workspace_path: &str,
        project_name: &str,
    ) -> Result<EditorialProgress, String> {
        let path = Path::new(workspace_path).join("EDITORIAL-PROGRESS.md");

        let raw = if path.exists() {
            std::fs::read_to_string(&path)
                .map_err(|e| format!("Erro ao ler EDITORIAL-PROGRESS.md: {e}"))?
        } else {
            let template = default_template(project_name);
            std::fs::write(&path, &template)
                .map_err(|e| format!("Erro ao criar EDITORIAL-PROGRESS.md: {e}"))?;
            template
        };

        self.parse_markdown(&raw, project_name)
    }

    /// Atualiza APENAS a linha F10 com os dados de geração mais recentes.
    /// Preserva integralmente F1-F9 e F11-F12.
    pub fn update_f10(
        &self,
        workspace_path: &str,
        project_name: &str,
        formats_generated: &[String],
        output_path: &str,
    ) -> Result<EditorialProgress, String> {
        let path = Path::new(workspace_path).join("EDITORIAL-PROGRESS.md");

        let raw = if path.exists() {
            std::fs::read_to_string(&path)
                .map_err(|e| format!("Erro ao ler EDITORIAL-PROGRESS.md: {e}"))?
        } else {
            let template = default_template(project_name);
            std::fs::write(&path, &template)
                .map_err(|e| format!("Erro ao criar EDITORIAL-PROGRESS.md: {e}"))?;
            template
        };

        let date_str = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let formats_str = formats_generated.join(", ");
        let notes = format!("{formats_str} → {output_path}");

        let updated = self.replace_f10_row(&raw, &date_str, &notes)?;
        std::fs::write(&path, &updated)
            .map_err(|e| format!("Erro ao escrever EDITORIAL-PROGRESS.md: {e}"))?;

        self.parse_markdown(&updated, project_name)
    }

    // ── Parsing Markdown Table ─────────────────────────────────────────

    fn parse_markdown(&self, raw: &str, project_name: &str) -> Result<EditorialProgress, String> {
        let mut phases: Vec<PhaseStatus> = Vec::with_capacity(12);

        for line in raw.lines() {
            let trimmed = line.trim();
            if !trimmed.starts_with('|') || trimmed.starts_with("| Fase") || trimmed.starts_with("|---") {
                continue;
            }

            let cells: Vec<&str> = trimmed
                .trim_matches('|')
                .split('|')
                .map(|c| c.trim())
                .collect();

            if cells.len() < 2 {
                continue;
            }

            let phase_id = cells[0].to_string();
            if !phase_id.starts_with('F') {
                continue;
            }

            let phase_name = cells.get(1).unwrap_or(&"").to_string();
            let status = EditorialStatus::from_str(cells.get(2).unwrap_or(&""));
            let date = cells.get(3).map(|s| s.to_string()).filter(|s| s != "—");
            let responsible = cells.get(4).map(|s| s.to_string()).filter(|s| s != "—");
            let notes = cells.get(5).map(|s| s.to_string()).filter(|s| s != "—");

            phases.push(PhaseStatus {
                phase_id,
                phase_name,
                status,
                date,
                responsible,
                notes,
            });
        }

        // Garantir 12 fases (preencher com defaults se ausentes)
        if phases.len() < 12 {
            for (id, name) in PHASE_NAMES.iter().skip(phases.len()) {
                phases.push(PhaseStatus {
                    phase_id: id.to_string(),
                    phase_name: name.to_string(),
                    status: EditorialStatus::Pending,
                    date: None,
                    responsible: None,
                    notes: None,
                });
            }
        }

        Ok(EditorialProgress {
            project_name: project_name.to_string(),
            phases,
            last_updated: chrono::Utc::now().to_rfc3339(),
        })
    }

    fn replace_f10_row(&self, raw: &str, date: &str, notes: &str) -> Result<String, String> {
        let mut updated_lines: Vec<String> = Vec::new();
        let mut f10_found = false;

        for line in raw.lines() {
            let trimmed = line.trim();
            // Detectar linha F10 na tabela Markdown
            if trimmed.starts_with("| F10") || trimmed.starts_with("|F10") {
                f10_found = true;
                // Parsear células existentes para preservar nome e responsável
                let cells: Vec<&str> = trimmed
                    .trim_matches('|')
                    .split('|')
                    .map(|c| c.trim())
                    .collect();
                let phase_id = cells.get(0).unwrap_or(&"F10");
                let phase_name = cells.get(1).unwrap_or(&"Geração");
                let responsible = cells.get(4).unwrap_or(&"—");

                // F10 → Done com data, notas de formatos gerados
                updated_lines.push(format!(
                    "| {phase_id} | {phase_name} | {} | {date} | {responsible} | {notes} |",
                    EditorialStatus::Done.to_markdown()
                ));
            } else {
                updated_lines.push(line.to_string());
            }
        }

        if !f10_found {
            return Err("Linha F10 não encontrada em EDITORIAL-PROGRESS.md".to_string());
        }

        Ok(updated_lines.join("\n"))
    }
}

impl Default for EditorialProgressService {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Testes ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn write_file(dir: &TempDir, name: &str, content: &str) {
        let mut f = std::fs::File::create(dir.path().join(name)).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    const SAMPLE_MD: &str = "# EDITORIAL-PROGRESS — Meu Projeto\n\n\
| Fase | Nome | Status | Data | Responsável | Notas |\n\
|------|------|--------|------|-------------|-------|\n\
| F1 | Brief | ✅ | 2026-01-01 | Pedro | — |\n\
| F2 | PRD | ✅ | 2026-01-02 | Pedro | — |\n\
| F3 | Otimização SystemForge | ⬜ | — | — | — |\n\
| F4 | WBS | ⬜ | — | — | — |\n\
| F5 | WBS+ | ⬜ | — | — | — |\n\
| F6 | Business | ⏭️ | — | — | Pulado |\n\
| F7 | Execução | ⬜ | — | — | — |\n\
| F8 | Complemento Manual | ⬜ | — | — | — |\n\
| F9 | QA | ⬜ | — | — | — |\n\
| F10 | Geração | ⬜ | — | — | — |\n\
| F11 | Deploy | ⬜ | — | — | — |\n\
| F12 | Marketing | ⬜ | — | — | — |\n";

    #[test]
    fn test_parse_markdown_12_phases() {
        let svc = EditorialProgressService::new();
        let result = svc.parse_markdown(SAMPLE_MD, "Meu Projeto").unwrap();
        assert_eq!(result.phases.len(), 12);
        assert_eq!(result.project_name, "Meu Projeto");
    }

    #[test]
    fn test_parse_f1_done() {
        let svc = EditorialProgressService::new();
        let result = svc.parse_markdown(SAMPLE_MD, "Meu Projeto").unwrap();
        let f1 = result.phases.iter().find(|p| p.phase_id == "F1").unwrap();
        assert_eq!(f1.status, EditorialStatus::Done);
    }

    #[test]
    fn test_parse_f6_skipped() {
        let svc = EditorialProgressService::new();
        let result = svc.parse_markdown(SAMPLE_MD, "Meu Projeto").unwrap();
        let f6 = result.phases.iter().find(|p| p.phase_id == "F6").unwrap();
        assert_eq!(f6.status, EditorialStatus::Skipped);
    }

    #[test]
    fn test_f10_update_preserves_other_phases() {
        let svc = EditorialProgressService::new();
        let updated = svc.replace_f10_row(SAMPLE_MD, "2026-03-22", "epub → /output/epub/").unwrap();

        // F1 deve estar intacto
        assert!(updated.contains("| F1 | Brief | ✅"));
        // F10 deve ser Done
        assert!(updated.contains("| F10 | Geração | ✅"));
        // F11 deve estar intacto
        assert!(updated.contains("| F11 | Deploy | ⬜"));
    }

    #[test]
    fn test_f10_update_contains_notes() {
        let svc = EditorialProgressService::new();
        let updated = svc
            .replace_f10_row(SAMPLE_MD, "2026-03-22", "epub, pdf-print → .bes-output/")
            .unwrap();
        assert!(updated.contains("epub, pdf-print → .bes-output/"));
    }

    #[test]
    fn test_default_template_creation() {
        let template = default_template("Teste");
        assert!(template.contains("F10"));
        assert!(template.contains("F1"));
        assert!(template.contains("F12"));
        let count = template.matches("| F").count();
        assert_eq!(count, 12);
    }

    #[test]
    fn test_read_creates_file_if_missing() {
        let dir = tempfile::tempdir().unwrap();
        let svc = EditorialProgressService::new();
        let result = svc.read(dir.path().to_str().unwrap(), "Novo Projeto").unwrap();
        assert_eq!(result.phases.len(), 12);
        assert!(dir.path().join("EDITORIAL-PROGRESS.md").exists());
    }

    #[test]
    fn test_update_f10_writes_file() {
        let dir = tempfile::tempdir().unwrap();
        write_file(&dir, "EDITORIAL-PROGRESS.md", SAMPLE_MD);
        let svc = EditorialProgressService::new();
        let result = svc
            .update_f10(
                dir.path().to_str().unwrap(),
                "Meu Projeto",
                &["epub".to_string(), "pdf-print".to_string()],
                ".bes-output/epub/",
            )
            .unwrap();
        let f10 = result.phases.iter().find(|p| p.phase_id == "F10").unwrap();
        assert_eq!(f10.status, EditorialStatus::Done);
    }
}
