// BES Book Formatter — Shared service utilities (module-4 TASK-7 ST001)

/// Shared utility: convert a book title into a URL-safe slug.
pub fn sanitize_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_slug_basic() {
        assert_eq!(sanitize_slug("Meu Livro"), "meu-livro");
    }

    #[test]
    fn sanitize_slug_multiple_spaces() {
        assert_eq!(sanitize_slug("O   Grande   Gatsby"), "o-grande-gatsby");
    }

    #[test]
    fn sanitize_slug_unicode() {
        // char::is_alphanumeric() includes accented Unicode letters, so é/è are preserved
        assert_eq!(sanitize_slug("Café & Crème"), "café-crème");
    }

    #[test]
    fn sanitize_slug_special_chars_only() {
        // punctuation between words becomes a single dash
        assert_eq!(sanitize_slug("Book! (2024)"), "book-2024");
    }

    #[test]
    fn sanitize_slug_already_slug() {
        assert_eq!(sanitize_slug("my-book"), "my-book");
    }

    #[test]
    fn sanitize_slug_numbers() {
        assert_eq!(sanitize_slug("Book 2024"), "book-2024");
    }

    #[test]
    fn sanitize_slug_leading_trailing_separators() {
        assert_eq!(sanitize_slug("  My Book  "), "my-book");
    }
}
