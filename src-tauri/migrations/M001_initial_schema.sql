-- ============================================
-- BES Book Formatter — SQLite Schema v1
-- M001_initial_schema
-- ============================================

-- Controle de versão do schema
CREATE TABLE IF NOT EXISTS schema_version (
    version       INTEGER PRIMARY KEY AUTOINCREMENT,
    migration_name TEXT NOT NULL,
    applied_at    DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Projetos BES
CREATE TABLE IF NOT EXISTS projects (
    id                  TEXT PRIMARY KEY NOT NULL,
    name                TEXT NOT NULL,
    bes_root_path       TEXT NOT NULL UNIQUE,
    book_config_path    TEXT,
    genre               TEXT,
    language            TEXT NOT NULL DEFAULT 'pt-BR',
    config_version      TEXT,
    last_opened         DATETIME,
    format_file_path    TEXT,
    created_at          DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at          DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    completeness_score  REAL CHECK (completeness_score IS NULL OR (completeness_score >= 0.0 AND completeness_score <= 1.0)),
    completeness_level  TEXT CHECK (completeness_level IS NULL OR completeness_level IN ('blocking', 'warning', 'normal')),
    chapter_count       INTEGER,
    illustration_count  INTEGER,
    manuscript_root     TEXT,
    output_dir          TEXT
);

CREATE INDEX IF NOT EXISTS idx_projects_last_opened ON projects(last_opened DESC);
CREATE INDEX IF NOT EXISTS idx_projects_genre ON projects(genre);

-- Catálogo de ilustrações
CREATE TABLE IF NOT EXISTS illustrations (
    id                TEXT PRIMARY KEY NOT NULL,
    project_id        TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    placeholder_name  TEXT NOT NULL,
    description       TEXT,
    state             TEXT NOT NULL DEFAULT 'pending'
                      CHECK (state IN ('pending', 'imported', 'linked')),
    image_path        TEXT,
    validated_dpi     INTEGER,
    alt_text          TEXT,
    width_px          INTEGER,
    height_px         INTEGER,
    color_space       TEXT CHECK (color_space IS NULL OR color_space IN ('srgb', 'cmyk')),
    created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_illustrations_project_id ON illustrations(project_id);
CREATE INDEX IF NOT EXISTS idx_illustrations_project_state ON illustrations(project_id, state);
CREATE UNIQUE INDEX IF NOT EXISTS uq_illustrations_project_placeholder ON illustrations(project_id, placeholder_name);

-- Preferências do usuário
CREATE TABLE IF NOT EXISTS user_preferences (
    key         TEXT PRIMARY KEY NOT NULL,
    value       TEXT NOT NULL,
    updated_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Registros padrão
INSERT OR IGNORE INTO user_preferences (key, value) VALUES ('theme', 'light');
INSERT OR IGNORE INTO user_preferences (key, value) VALUES ('ui_language', 'pt-BR');

-- Registrar migration
INSERT INTO schema_version (migration_name) VALUES ('M001_initial_schema')
