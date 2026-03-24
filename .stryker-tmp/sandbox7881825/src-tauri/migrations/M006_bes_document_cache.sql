-- M006_bes_document_cache.sql
-- BES Document Cache com TTL e validação por SHA-256 (module-6 TASK-1)

CREATE TABLE IF NOT EXISTS bes_document_cache (
    id            TEXT    PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    project_id    TEXT    NOT NULL,
    document_type TEXT    NOT NULL,
    content       TEXT    NOT NULL,
    parsed_json   TEXT,
    file_path     TEXT    NOT NULL,
    file_hash     TEXT    NOT NULL,
    cached_at     TEXT    NOT NULL DEFAULT (datetime('now')),

    CONSTRAINT fk_bdc_project
        FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    CONSTRAINT ck_bdc_document_type
        CHECK (document_type IN ('bdd', 'book_architecture', 'metadata', 'editorial_progress')),
    CONSTRAINT uq_bdc_entry
        UNIQUE (project_id, document_type)
);

CREATE INDEX IF NOT EXISTS idx_bdc_project_type
    ON bes_document_cache(project_id, document_type);

CREATE INDEX IF NOT EXISTS idx_bdc_project
    ON bes_document_cache(project_id);

INSERT INTO schema_version (migration_name) VALUES ('M006_bes_document_cache');
