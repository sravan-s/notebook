pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS notebook_versions (
        id INTEGER PRIMARY KEY NOT NULL,
        notebook_id INTEGER NOT NULL,
        created_at INTEGER,
        updated_at INTEGER,
        dependencies TEXT,
        FOREIGN KEY (notebook_id) REFERENCES notebooks(id) ON DELETE CASCADE);
";
