CREATE TABLE IF NOT EXISTS notes (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    contents TEXT NULL,
    created_at INTEGER NOT NULL,
    modified_at INTEGER NULL DEFAULT NULL
);

CREATE INDEX idx_notes_created_at ON notes (created_at);
CREATE INDEX idx_nodes_title ON notes (title);
