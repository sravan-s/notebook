pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS notebooks (
            id INTEGER PRIMARY KEY NOT NULL,
            archived_at INTEGER,
            name TEXT,
            description TEXT,
            created_at INTEGER
    );
";

pub const GET_NOTEBOOKS_NON_ARCHIVED: &str = "
    SELECT id, name, description, created_at FROM notebooks WHERE archived_at = 0
";
