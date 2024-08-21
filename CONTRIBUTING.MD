## Basics
Webserver is in vite + TS + react
Backend is in rust

## Backend/Models
Every model should have:
```
model/api_handlers.rs
model/db.rs
model/model.rs
---
model/mod.rs // declare above 3 into this file
```
And declare in models/mod.rs
Then you can use them in src/db.rs or src/api_handlers.rs

### Style guide/Backend
prefer qualified imports, ie - "notebook::db::CREATE_TABLE"
Throw errors as values in api_handlers ie -> `pub fn db_handler() -> anyhow::Result<> {}`
Its okay to panic in errors at bootstrap process
