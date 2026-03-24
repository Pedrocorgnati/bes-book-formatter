-- M004 — generation_results
-- Rastreia cada geração de output (EPUB, PDF, DOCX) com metadados e resultado da validação.

-- UP
CREATE TABLE IF NOT EXISTS generation_results (
    id           TEXT    PRIMARY KEY NOT NULL,
    project_id   TEXT    NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    format       TEXT    NOT NULL,   -- "epub3" | "pdf_print" | "pdf_ebook" | "docx" | "html5" | etc.
    platform     TEXT    NOT NULL,   -- "kdp" | "kdp_print" | "ingram_spark" | "apple_books" | "generic"
    output_path  TEXT,
    file_size_bytes INTEGER,
    duration_ms  INTEGER,
    status       TEXT    NOT NULL DEFAULT 'pending', -- "success" | "error" | "pending"
    errors       TEXT    NOT NULL DEFAULT '[]',      -- JSON array of strings
    warnings     TEXT    NOT NULL DEFAULT '[]',      -- JSON array of strings
    created_at   DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_generation_results_project_id
    ON generation_results (project_id);

CREATE INDEX IF NOT EXISTS idx_generation_results_created_at
    ON generation_results (project_id, created_at DESC);

-- DOWN (para reversão: DROP TABLE generation_results;)

INSERT INTO schema_version (migration_name) VALUES ('M004_generation_results');
