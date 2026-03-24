-- M005: annotations table for preview review annotations (module-5 TASK-3)
CREATE TABLE IF NOT EXISTS annotations (
    id              TEXT    PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    project_id      TEXT    NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    page_number     INTEGER NOT NULL CHECK(page_number >= 1),
    x_percent       REAL    NOT NULL CHECK(x_percent BETWEEN 0 AND 100),
    y_percent       REAL    NOT NULL CHECK(y_percent BETWEEN 0 AND 100),
    annotation_type TEXT    NOT NULL DEFAULT 'comment'
                            CHECK(annotation_type IN ('comment', 'highlight', 'flag')),
    color           TEXT    NOT NULL DEFAULT '#FFC107',
    content         TEXT    NOT NULL DEFAULT '',
    created_at      TEXT    NOT NULL DEFAULT (datetime('now')),
    updated_at      TEXT    NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_annotations_project_page
    ON annotations(project_id, page_number);

CREATE TRIGGER IF NOT EXISTS annotations_updated_at
AFTER UPDATE ON annotations
BEGIN
    UPDATE annotations SET updated_at = datetime('now') WHERE id = NEW.id;
END;

INSERT INTO schema_version (migration_name) VALUES ('M005_annotations');
