Das schema

Notebooks
---
id,
archived_at,
name,
description,
created_at,

NotebookVersions
---
id, Notebooks(id), created_at, updated_at

Paragraphs
---
id
NotebookVersions(id)
code
result
status
created_at
updated_at
...others

Secrets
---
id
data:BLOB
Notebooks(id)

