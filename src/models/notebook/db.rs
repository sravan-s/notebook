pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS notebooks (
            id INTEGER PRIMARY KEY NOT NULL,
            name TEXT,
            description TEXT,
            created_at INTEGER,
            updated_at INTEGER,
            dependencies TEXT,
            archived_at INTEGER);
";

pub const GET_NOTEBOOKS_NON_ARCHIVED: &str = "
    SELECT id, name, description, updated_at FROM notebooks ORDER BY updated_at DESC
";

pub const GET_NOTEBOOK_BY_ID: &str =
    "SELECT id, name, description, created_at, updated_at, archived_at FROM notebooks WHERE id = ?";

pub const INSERT_NOTEBOOK: &str = "
    INSERT INTO notebooks (name, description, created_at, updated_at, dependencies, archived_at)
    VALUES($1, $2, $3, $4, $5, 0);";

pub const DELETE_NOTEBOOK_BY_ID: &str = "UPDATE notebooks SET archived_at = $1 WHERE id = $2";
