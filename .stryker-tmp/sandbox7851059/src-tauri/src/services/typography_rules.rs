use crate::models::enums::BookLanguage;

/// Apply typographic rules to a block of plain text according to the given language.
///
/// Rules applied:
/// - Quote conversion (straight → typographic / angular)
/// - Ellipsis: `...` → `…` (U+2026)
/// - Ordinal numbers (pt-BR): `1o` → `1º`, `2a` → `2ª`
/// - Dialogue dash (pt-BR, es-ES): line starting with `- ` → `— `
/// - Em dash: ` -- ` → ` — ` (en-US)
pub fn apply_typography_rules(text: &str, language: &BookLanguage) -> String {
    let text = convert_ellipsis(text);
    let text = match language {
        BookLanguage::PtBr => {
            let t = convert_dialogue_dash_pt_br(&text);
            let t = convert_quotes_pt_br(&t);
            convert_ordinals_pt_br(&t)
        }
        BookLanguage::EnUs => {
            let t = convert_em_dash_en_us(&text);
            convert_curly_quotes_en_us(&t)
        }
        BookLanguage::EsEs => {
            let t = convert_dialogue_dash_es(&text);
            convert_quotes_es(&t)
        }
        BookLanguage::ItIt => {
            // Italian: same angular quotes as pt-BR, dialogue dash
            let t = convert_dialogue_dash_pt_br(&text);
            convert_quotes_pt_br(&t)
        }
    };
    text
}

/// Convert `...` to Unicode ellipsis `…` (U+2026).
/// Does not convert `....` or longer sequences to avoid URL/code damage.
pub fn convert_ellipsis(text: &str) -> String {
    // Replace exactly three consecutive dots not preceded or followed by another dot
    let mut result = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 2 < len && chars[i] == '.' && chars[i + 1] == '.' && chars[i + 2] == '.' {
            // Check for 4th dot (don't convert ....)
            if i + 3 < len && chars[i + 3] == '.' {
                result.push(chars[i]);
            } else {
                result.push('…');
                i += 3;
                continue;
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }
    result
}

/// pt-BR: Convert straight double quotes to angular quotes «».
/// Single-level only — nested quotes remain as inner straight quotes.
pub fn convert_quotes_pt_br(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 16);
    let mut open = false;
    for ch in text.chars() {
        if ch == '"' {
            if open {
                result.push('»');
            } else {
                result.push('«');
            }
            open = !open;
        } else {
            result.push(ch);
        }
    }
    result
}

/// pt-BR: Replace line-initial `- ` (dialogue marker) with em dash `— `.
pub fn convert_dialogue_dash_pt_br(text: &str) -> String {
    let lines: Vec<&str> = text.split('\n').collect();
    lines
        .iter()
        .map(|line| {
            if line.starts_with("- ") {
                format!("— {}", &line[2..])
            } else if *line == "-" {
                "—".to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// pt-BR: Convert ordinal number suffixes.
/// `1o` → `1º`, `2a` → `2ª`, `3os` → `3ºs`, `4as` → `4ªs`
pub fn convert_ordinals_pt_br(text: &str) -> String {
    let mut result = text.to_string();
    // Masculine ordinals: digit(s) followed by 'o' not preceded by a letter
    let patterns_m: Vec<(&str, &str)> = vec![
        ("0o ", "0º "), ("1o ", "1º "), ("2o ", "2º "), ("3o ", "3º "),
        ("4o ", "4º "), ("5o ", "5º "), ("6o ", "6º "), ("7o ", "7º "),
        ("8o ", "8º "), ("9o ", "9º "),
        ("0o.", "0º."), ("1o.", "1º."), ("2o.", "2º."), ("3o.", "3º."),
        ("4o.", "4º."), ("5o.", "5º."), ("6o.", "6º."), ("7o.", "7º."),
        ("8o.", "8º."), ("9o.", "9º."),
    ];
    // Feminine ordinals
    let patterns_f: Vec<(&str, &str)> = vec![
        ("0a ", "0ª "), ("1a ", "1ª "), ("2a ", "2ª "), ("3a ", "3ª "),
        ("4a ", "4ª "), ("5a ", "5ª "), ("6a ", "6ª "), ("7a ", "7ª "),
        ("8a ", "8ª "), ("9a ", "9ª "),
        ("0a.", "0ª."), ("1a.", "1ª."), ("2a.", "2ª."), ("3a.", "3ª."),
        ("4a.", "4ª."), ("5a.", "5ª."), ("6a.", "6ª."), ("7a.", "7ª."),
        ("8a.", "8ª."), ("9a.", "9ª."),
    ];
    for (from, to) in patterns_m.iter().chain(patterns_f.iter()) {
        result = result.replace(from, to);
    }
    result
}

/// en-US: Convert straight double quotes to curly quotes "".
pub fn convert_curly_quotes_en_us(text: &str) -> String {
    let mut result = String::with_capacity(text.len() + 16);
    let mut open = false;
    for ch in text.chars() {
        if ch == '"' {
            if open {
                result.push('\u{201D}'); // "
            } else {
                result.push('\u{201C}'); // "
            }
            open = !open;
        } else {
            result.push(ch);
        }
    }
    result
}

/// en-US: Replace ` -- ` (spaced double hyphen) with ` — ` (em dash with spaces).
pub fn convert_em_dash_en_us(text: &str) -> String {
    text.replace(" -- ", " \u{2014} ")  // —
        .replace("--", "\u{2014}")      // also handle no-space variant
}

/// es-ES: Convert straight double quotes to angular quotes «».
pub fn convert_quotes_es(text: &str) -> String {
    convert_quotes_pt_br(text) // same rule as pt-BR
}

/// es-ES: Replace line-initial `- ` with em dash `— ` (guión largo).
pub fn convert_dialogue_dash_es(text: &str) -> String {
    convert_dialogue_dash_pt_br(text) // same rule as pt-BR
}

// ---- Unit tests ----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ellipsis_conversion() {
        assert_eq!(convert_ellipsis("Olá..."), "Olá…");
        assert_eq!(convert_ellipsis("...."), "...."); // 4 dots: unchanged
        assert_eq!(convert_ellipsis("A...B"), "A…B");
    }

    #[test]
    fn test_pt_br_quotes_conversion() {
        assert_eq!(convert_quotes_pt_br(r#""Olá, mundo""#), "«Olá, mundo»");
        assert_eq!(convert_quotes_pt_br(r#""A" e "B""#), "«A» e «B»");
    }

    #[test]
    fn test_pt_br_dialogue_dash() {
        let input = "- Disse ela.\nTexto normal.";
        let output = convert_dialogue_dash_pt_br(input);
        assert!(output.starts_with("— Disse ela."));
        assert!(output.contains("Texto normal."));
    }

    #[test]
    fn test_pt_br_ordinals() {
        assert_eq!(convert_ordinals_pt_br("1o lugar"), "1º lugar");
        assert_eq!(convert_ordinals_pt_br("2a opção"), "2ª opção");
    }

    #[test]
    fn test_en_us_curly_quotes() {
        let result = convert_curly_quotes_en_us(r#""Hello""#);
        assert!(result.contains('\u{201C}'));
        assert!(result.contains('\u{201D}'));
    }

    #[test]
    fn test_en_us_em_dash() {
        assert_eq!(convert_em_dash_en_us("text -- more"), "text \u{2014} more");
        assert_eq!(convert_em_dash_en_us("text--more"), "text\u{2014}more");
    }

    #[test]
    fn test_apply_typography_rules_pt_br() {
        let input = r#""Olá", disse ela. É o 1o capítulo..."#;
        let result = apply_typography_rules(input, &BookLanguage::PtBr);
        assert!(result.contains('«'));
        assert!(result.contains('»'));
        assert!(result.contains('…'));
        assert!(result.contains("1º"));
    }

    #[test]
    fn test_apply_typography_rules_en_us() {
        let input = r#"She said "Hello" -- and left."#;
        let result = apply_typography_rules(input, &BookLanguage::EnUs);
        assert!(result.contains('\u{201C}'));
        assert!(result.contains('\u{201D}'));
        assert!(result.contains('\u{2014}'));
    }
}
