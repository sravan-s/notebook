pub const CREATE_TABLE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS secrets (
        id INTEGER PRIMARY KEY NOT NULL,
        notebook_id INTEGER NOT NULL,
        data BLOB,
        FOREIGN KEY (notebook_id) REFERENCES notebooks(id) ON DELETE CASCADE);
";

/*
 * Todo: encrypt secrets
 * This must be encrypted, I will do after setting up firecracker
 * */

pub const INSERT_SECRET: &str = "
    INSERT INTO secrets (notebook_id, data)
    VALUES($1, $2);
";
