use serde::{Deserialize, Serialize};

/// Project entity matching the `projects` SQLite table (ERD from LLD).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub bes_root_path: String,
    pub book_config_path: Option<String>,
    pub genre: Option<String>,
    pub language: String,
    pub config_version: Option<String>,
    pub last_opened: Option<String>,
    pub format_file_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completeness_score: Option<f64>,
    pub completeness_level: Option<String>,
    pub chapter_count: Option<i32>,
    pub illustration_count: Option<i32>,
    pub manuscript_root: Option<String>,
    pub output_dir: Option<String>,
}

/// DTO for creating a new project (import_project command).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewProject {
    pub name: String,
    pub bes_root_path: String,
    pub book_config_path: Option<String>,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub config_version: Option<String>,
    pub manuscript_root: Option<String>,
    pub output_dir: Option<String>,
}

/// DTO for updating a project.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProject {
    pub name: Option<String>,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub completeness_score: Option<f64>,
    pub completeness_level: Option<String>,
    pub chapter_count: Option<i32>,
    pub illustration_count: Option<i32>,
}
