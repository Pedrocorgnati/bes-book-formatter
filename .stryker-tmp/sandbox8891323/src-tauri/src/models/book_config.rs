use serde::{Deserialize, Serialize};

use crate::models::enums::Genre;

/// Genre-specific page dimensions in inches.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageDimensions {
    pub width_inches: f64,
    pub height_inches: f64,
    pub margin_top: f64,
    pub margin_bottom: f64,
    pub margin_inner: f64,
    pub margin_outer: f64,
}

/// Genre-specific typography defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypographyDefaults {
    pub body_font: String,
    pub heading_font: String,
    pub code_font: Option<String>,
    pub body_size_pt: f64,
    pub line_height: f64,
}

/// Returns the recommended page dimensions and typography for a given genre.
///
/// - Fiction / Romance → 5.5×8.5, EB Garamond
/// - Technical        → 7×10, Source Serif 4 + JetBrains Mono
/// - Children / YA    → 8.5×8.5, Source Serif 4 (14pt)
/// - Academic         → 6×9, Source Serif 4
/// - All others       → 6×9, Source Serif 4
pub fn genre_defaults(genre: &Genre) -> (PageDimensions, TypographyDefaults) {
    match genre {
        Genre::Fiction | Genre::Romance => (
            PageDimensions {
                width_inches: 5.5,
                height_inches: 8.5,
                margin_top: 0.875,
                margin_bottom: 0.875,
                margin_inner: 0.875,
                margin_outer: 0.625,
            },
            TypographyDefaults {
                body_font: "EB Garamond".to_string(),
                heading_font: "EB Garamond".to_string(),
                code_font: None,
                body_size_pt: 11.0,
                line_height: 1.5,
            },
        ),
        Genre::Technical => (
            PageDimensions {
                width_inches: 7.0,
                height_inches: 10.0,
                margin_top: 1.0,
                margin_bottom: 1.0,
                margin_inner: 1.0,
                margin_outer: 0.75,
            },
            TypographyDefaults {
                body_font: "Source Serif 4".to_string(),
                heading_font: "Source Serif 4".to_string(),
                code_font: Some("JetBrains Mono".to_string()),
                body_size_pt: 10.5,
                line_height: 1.5,
            },
        ),
        Genre::Children | Genre::Ya => (
            PageDimensions {
                width_inches: 8.5,
                height_inches: 8.5,
                margin_top: 1.0,
                margin_bottom: 1.0,
                margin_inner: 0.875,
                margin_outer: 0.875,
            },
            TypographyDefaults {
                body_font: "Source Serif 4".to_string(),
                heading_font: "Source Serif 4".to_string(),
                code_font: None,
                body_size_pt: 14.0,
                line_height: 1.6,
            },
        ),
        Genre::Academic => (
            PageDimensions {
                width_inches: 6.0,
                height_inches: 9.0,
                margin_top: 1.0,
                margin_bottom: 1.0,
                margin_inner: 0.875,
                margin_outer: 0.75,
            },
            TypographyDefaults {
                body_font: "Source Serif 4".to_string(),
                heading_font: "Source Serif 4".to_string(),
                code_font: Some("JetBrains Mono".to_string()),
                body_size_pt: 11.0,
                line_height: 1.5,
            },
        ),
        Genre::Poetry => (
            PageDimensions {
                width_inches: 5.5,
                height_inches: 8.5,
                margin_top: 0.875,
                margin_bottom: 0.875,
                margin_inner: 0.875,
                margin_outer: 0.625,
            },
            TypographyDefaults {
                body_font: "EB Garamond".to_string(),
                heading_font: "EB Garamond".to_string(),
                code_font: None,
                body_size_pt: 12.0,
                line_height: 1.6,
            },
        ),
        // Nonfiction, SelfHelp, Business, Management → 6×9
        _ => (
            PageDimensions {
                width_inches: 6.0,
                height_inches: 9.0,
                margin_top: 0.875,
                margin_bottom: 0.875,
                margin_inner: 0.875,
                margin_outer: 0.625,
            },
            TypographyDefaults {
                body_font: "Source Serif 4".to_string(),
                heading_font: "Source Serif 4".to_string(),
                code_font: None,
                body_size_pt: 11.0,
                line_height: 1.5,
            },
        ),
    }
}
