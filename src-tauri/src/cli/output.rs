// BES Book Formatter — CLI Output Formatters (module-6 TASK-3)
//
// Funções de formatação de saída: human-readable (tabela) e JSON.

use serde::Serialize;

/// Imprime valor como JSON formatado (--json flag).
pub fn print_json<T: Serialize>(value: &T) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

/// Imprime tabela simples com colunas alinhadas.
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        println!("(nenhum resultado)");
        return;
    }

    // Calcular largura máxima por coluna
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }

    let separator: String = widths
        .iter()
        .map(|w| "-".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("+");

    // Header
    print_row(headers, &widths);
    println!("+{separator}+");
    for row in rows {
        let cells: Vec<&str> = row.iter().map(|s| s.as_str()).collect();
        print_row(&cells, &widths);
    }
}

fn print_row(cells: &[&str], widths: &[usize]) {
    let row: String = cells
        .iter()
        .enumerate()
        .map(|(i, cell)| {
            let w = widths.get(i).copied().unwrap_or(cell.len());
            format!(" {cell:<w$} ")
        })
        .collect::<Vec<_>>()
        .join("|");
    println!("|{row}|");
}

/// Imprime mensagem de sucesso verde (ANSI 256).
pub fn print_success(msg: &str) {
    println!("\x1b[32m✅ {msg}\x1b[0m");
}

/// Imprime mensagem de erro vermelho.
pub fn print_error(msg: &str) {
    eprintln!("\x1b[31m❌ {msg}\x1b[0m");
}

/// Imprime aviso amarelo.
pub fn print_warning(msg: &str) {
    println!("\x1b[33m⚠️  {msg}\x1b[0m");
}
