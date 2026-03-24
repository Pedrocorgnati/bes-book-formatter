-- ============================================
-- BES Book Formatter — Migration M003
-- Typography Configuration Table
-- ============================================

CREATE TABLE IF NOT EXISTS typography_configs (
    id                    TEXT PRIMARY KEY NOT NULL,
    project_id            TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    font_body             TEXT NOT NULL DEFAULT 'EB Garamond',
    font_heading          TEXT NOT NULL DEFAULT 'EB Garamond',
    font_code             TEXT,
    font_size_body        REAL NOT NULL DEFAULT 11.0 CHECK(font_size_body BETWEEN 8.0 AND 48.0),
    font_size_h1          REAL NOT NULL DEFAULT 22.0,
    font_size_h2          REAL NOT NULL DEFAULT 18.0,
    font_size_h3          REAL NOT NULL DEFAULT 14.0,
    font_size_h4          REAL NOT NULL DEFAULT 12.0,
    leading               REAL NOT NULL DEFAULT 1.4,
    paragraph_indent      REAL NOT NULL DEFAULT 1.5,
    tracking              REAL NOT NULL DEFAULT 0.0,
    kerning               INTEGER NOT NULL DEFAULT 1,
    justification         INTEGER NOT NULL DEFAULT 1,
    hyphenation           INTEGER NOT NULL DEFAULT 1,
    hyphenation_language  TEXT NOT NULL DEFAULT 'pt-BR',
    orphan_control        INTEGER NOT NULL DEFAULT 2,
    widow_control         INTEGER NOT NULL DEFAULT 2,
    drop_cap_style        TEXT NOT NULL DEFAULT 'none',
    ornament_style        TEXT NOT NULL DEFAULT 'none',
    baseline_grid         REAL NOT NULL DEFAULT 12.0,
    genre_preset          TEXT NOT NULL DEFAULT 'nonfiction',
    custom_overrides      TEXT NOT NULL DEFAULT '{}',
    page_width            REAL NOT NULL DEFAULT 6.0,
    page_height           REAL NOT NULL DEFAULT 9.0,
    margin_top            REAL NOT NULL DEFAULT 0.75,
    margin_bottom         REAL NOT NULL DEFAULT 0.75,
    margin_inner          REAL NOT NULL DEFAULT 1.0,
    margin_outer          REAL NOT NULL DEFAULT 0.75,
    chapter_start         TEXT NOT NULL DEFAULT 'odd',
    illustration_missing_mode TEXT NOT NULL DEFAULT 'placeholder_visual',
    created_at            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at            DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id)
);

CREATE INDEX IF NOT EXISTS idx_typography_configs_project_id
    ON typography_configs(project_id);

-- Registrar migration
INSERT INTO schema_version (migration_name) VALUES ('M003_typography_config');
