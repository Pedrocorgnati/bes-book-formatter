-- ============================================
-- BES Book Formatter — Migration M002
-- Add 'error' state to illustrations table
-- ============================================

-- SQLite does not support ALTER TABLE to modify CHECK constraints.
-- The illustrations table uses CHECK (state IN ('pending','imported','linked')).
-- We need to recreate the table with the updated constraint.
-- Since this is a desktop app pre-production, the standard SQLite
-- table recreation approach is used for safety.

CREATE TABLE IF NOT EXISTS illustrations_new (
    id                TEXT PRIMARY KEY NOT NULL,
    project_id        TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    placeholder_name  TEXT NOT NULL,
    description       TEXT,
    state             TEXT NOT NULL DEFAULT 'pending'
                      CHECK (state IN ('pending', 'imported', 'linked', 'error')),
    image_path        TEXT,
    validated_dpi     INTEGER,
    alt_text          TEXT,
    width_px          INTEGER,
    height_px         INTEGER,
    color_space       TEXT CHECK (color_space IS NULL OR color_space IN ('srgb', 'cmyk')),
    created_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at        DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO illustrations_new SELECT * FROM illustrations;
DROP TABLE illustrations;
ALTER TABLE illustrations_new RENAME TO illustrations;
CREATE INDEX IF NOT EXISTS idx_illustrations_project_id ON illustrations(project_id);
CREATE UNIQUE INDEX IF NOT EXISTS uq_illustrations_project_placeholder ON illustrations(project_id, placeholder_name);
CREATE INDEX IF NOT EXISTS idx_illustrations_project_state ON illustrations(project_id, state);

-- Registrar migration
INSERT INTO schema_version (migration_name) VALUES ('M002_add_error_state')
