use serde::{Deserialize, Serialize};

/// Illustration entity matching the `illustrations` SQLite table.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Illustration {
    pub id: String,
    pub project_id: String,
    pub placeholder_name: String,
    pub description: Option<String>,
    pub state: String,
    pub image_path: Option<String>,
    pub validated_dpi: Option<i32>,
    pub alt_text: Option<String>,
    pub width_px: Option<i32>,
    pub height_px: Option<i32>,
    pub color_space: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// DTO for creating a new illustration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewIllustration {
    pub project_id: String,
    pub placeholder_name: String,
    pub description: Option<String>,
}
