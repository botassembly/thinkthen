# The scalar-bind surface is unusable on DuckDB's stable C API

Status: Open

Found by experiment 207 (`experiments/207-thinkthen-db/duckdb/NOTES.md`, entry on the round-one build, isolated step by step). Nobody opens an issue upstream. Ian decides.

## What breaks

On DuckDB v1.5.5, built through `extension-template-rs` with `USE_UNSTABLE_C_API=1`, the scalar-function bind path exists but does not work:

- `duckdb_scalar_function_set_bind` fires the callback at plan time. This part works; the callback ran before any request.
- `duckdb_bind_get_parameter_count` returns `-7` as `idx_t`, garbage.
- `duckdb_bind_get_parameter` segfaults.
- `duckdb_bind_set_error` segfaults.
- Returning from the bind callback with no bind data aborts the process with a double free at cleanup.

The same surface has a second trap: setting an error or result text needs the scalar-specific setter. The generic one is wired for a different info layout, and the scalar setter is the one that copies the text into the info struct correctly.

## Smallest reproduction

An extension that registers a scalar function with a bind callback and calls any of the parameter accessors inside it. The 207 module kept the code: the registration path sets no bind, and the finding with the step-by-step isolation sits in the notes.

## What it costs

No bind-time question check on the stable C API. The extension checks the question at the first row instead, inside the chunk, before any crossing; a bad threshold costs zero requests, proven on the stub. Rule 4 of `sdlc/planning/databases/README.md` keeps the bind-time check as an aspiration with this page as the citation, and `duckdb.md` says so.

## The decision that is Ian's

Whether to report this to DuckDB, work around it indefinitely, or push the wiring upstream through `duckdb-rs`. Nobody opens an issue upstream without his word.
