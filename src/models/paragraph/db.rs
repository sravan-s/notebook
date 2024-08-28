pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS paragraphs (
        id INTEGER PRIMARY KEY NOT NULL,
        notebook_id INTEGER NOT NULL,
        created_at INTEGER,
        updated_at INTEGER,
        status TINYINT,
        code TEXT,
        result TEXT,
        meta TEXT,
        FOREIGN KEY (notebook_id) REFERENCES notebook(id) ON DELETE CASCADE);
";

pub const GET_PARAGRAPHS_BY_NOTEBOOK_ID: &str = "
    SELECT id, created_at, updated_at, status, code, result, meta, notebook_id
    FROM paragraphs
    WHERE notebook_id = ?
";

pub const GET_PARAGRAPH_BY_ID: &str = "
    SELECT id, created_at, updated_at, status, code, result, meta, notebook_id
    FROM paragraphs
    WHERE id = ?
";
