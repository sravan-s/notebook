pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS paragraphs (
        id INTEGER PRIMARY KEY NOT NULL,
        notebook_version INTEGER NOT NULL,
        created_at INTEGER,
        updated_at INTEGER,
        status TINYINT,
        code TEXT,
        result TEXT,
        meta TEXT,
        FOREIGN KEY (notebook_version) REFERENCES notebook_versions(id) ON DELETE CASCADE);
";
