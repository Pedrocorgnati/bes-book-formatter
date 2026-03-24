-- M007_cover_configs.sql
-- Cover configuration for module-7-cover-design (Rock-6)
-- Persists the full cover config: template, colors, texts, image, spine dimensions.

CREATE TABLE IF NOT EXISTS cover_configs (
    id              TEXT    PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    project_id      TEXT    NOT NULL UNIQUE REFERENCES projects(id) ON DELETE CASCADE,

    -- Template e gênero
    template_id     TEXT    NOT NULL DEFAULT 'minimal',
    genre           TEXT    NOT NULL DEFAULT 'fiction',
    platform        TEXT    NOT NULL DEFAULT 'amazon-kdp'
                            CHECK(platform IN ('amazon-kdp', 'ingram', 'generic')),

    -- Textos da capa
    title_override  TEXT,
    subtitle        TEXT,
    author_override TEXT,
    back_cover_text TEXT    NOT NULL DEFAULT '',

    -- Visual
    primary_color   TEXT    NOT NULL DEFAULT '#991B1B',
    secondary_color TEXT    NOT NULL DEFAULT '#F8F6F0',
    font_title      TEXT    NOT NULL DEFAULT 'Playfair Display',
    font_author     TEXT    NOT NULL DEFAULT 'Lato',

    -- Imagem de capa
    cover_image_path     TEXT,
    cover_image_original TEXT,
    cover_image_dpi      INTEGER,

    -- Dimensões calculadas
    page_count      INTEGER NOT NULL DEFAULT 0 CHECK(page_count >= 0),
    spine_width_mm  REAL    NOT NULL DEFAULT 0.0 CHECK(spine_width_mm >= 0.0),
    paper_type      TEXT    NOT NULL DEFAULT 'white'
                            CHECK(paper_type IN ('white', 'cream')),

    -- Metadata
    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_cover_configs_project ON cover_configs(project_id);

INSERT INTO schema_version (migration_name) VALUES ('M007_cover_configs');
