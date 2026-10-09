# 0519: Make the SQL surfaces consistent and self-described

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

## Outcome

The review amendment below requires a reviewed SQL NULL and image contract before changing current assertions.

DuckDB, SQLite and PostgreSQL treat NULL one way, take images as binary values, return native JSON types where results are JSON, and describe every function inside the database.

## Evidence

- Starts from: [the 2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md). SQLite raises an error for a NULL question in rank sets but returns NULL in the complete calls. Complete calls take images as JSON arrays of byte numbers. DuckDB returns VARCHAR for annotate and details. No surface registers function descriptions.
- Keeps: Function names and current accepted inputs.
- Changes: One NULL rule in the guide applied to all three. BLOB and bytea images. DuckDB JSON type. Function comments in each extension. Claim `databases/**` per slice.
- Proof: Shared SQL cases cover NULL inputs, a BLOB image and the description query on each database.
- Defers: Extension signing and registry packaging to 0517.

## Review amendment

The PM accepted this NULL contract in ask 5 of the 2026-10-09 thin-first-class message: required NULL scalar operands return SQL NULL; required NULL table or set operands produce zero rows; neither inspects other operands, reads files nor sends. Optional settings NULL means omitted settings; backend failures remain failures. This ruling supersedes the SQLite rank tests that expect errors. Update those assertions and record the rule in the SQL specification during implementation.

Preserve valid existing image forms while defining native binary overloads and each database's actual description query in the reviewed design. SQLite function comments alone are not an interface. The initial SQLite NULL slice claims `databases/sqlite/src/many.rs`, `databases/sqlite/tests/test_rank.py`, `databases/sqlite/tests/test_values.py` and `databases/sqlite/README.md`. Claim DuckDB JSON return changes and binary input separately before coding. Count zero sends and unnecessary reads. Signing and registry submission remain outside dependencies; 0517 owns local native package inventory, not their submission. This amendment supersedes that deferral above.

## Surface assessment amendment

Name a complete-result binary-image call for each database and prove it through the installed extension. Existing binary image helpers and bare judgment overloads do not establish that the complete-result API accepts native bytes without caller-built JSON byte arrays. Reuse those helpers and shared Rust admission; preserve existing valid forms.

Assert DuckDB's actual return type for each changed JSON-valued function and consume the value through its native JSON operations. Use each database's supported JSON representation; do not invent a distinct SQLite storage type. Check description discoverability against the actual registered public function inventory with the database's declared query. Include required-NULL calls beside malformed partner operands and count zero extension reads/sends; optional NULL settings remain omission and backend failures remain failures. These checks cover the promised outcomes using existing SQL runners.
