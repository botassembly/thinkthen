# 0519: Make the SQL surfaces consistent and self-described

Status: OPEN.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

## Outcome

DuckDB, SQLite and PostgreSQL treat NULL one way, accept images as native binary values in the complete-result calls, return the database's native JSON type where a result is JSON, and describe every public function inside the database.

## Evidence

- Starts from: the [2026-10-09 binding decision](../decisions/2026-10-09-thin-first-class-bindings.md) and the [0521 assessment](../records/0521-surface-contract-assessment.md). SQLite raises an error for a NULL question in rank sets but returns NULL in the complete calls. Complete calls take images as JSON arrays of byte numbers. DuckDB returns VARCHAR for annotate and details. No surface registers function descriptions.
- Keeps: Function names and every current valid input form, including the existing binary image helpers. Backend failures remain failures.
- Changes: The NULL rule, accepted by the PM in ask 5 of the 2026-10-09 thin-first-class message:
  - a required NULL scalar operand returns SQL NULL;
  - a required NULL table or set operand produces zero rows;
  - neither inspects other operands, reads files nor sends;
  - a NULL optional setting means the setting is omitted.
  This supersedes the SQLite rank tests that expect errors. Record the rule in the SQL specification and the guide's SQL section. Slices, in order:
  - SQLite NULL: claim `databases/sqlite/src/many.rs`, `databases/sqlite/tests/test_rank.py`, `databases/sqlite/tests/test_values.py` and `databases/sqlite/README.md`.
  - Binary images: name one complete-result binary-image call per database, taking BLOB in DuckDB and SQLite and bytea in PostgreSQL, that reuses the existing helpers and shared Rust admission. Claim each file before coding.
  - DuckDB JSON: return DuckDB's JSON type from each JSON-valued function. Use each database's supported JSON representation and invent no separate SQLite storage type. Claim the files before coding.
  - Descriptions: register a description for every public function and name each database's discovery query. SQLite function comments alone are not an interface.
- Proof: The existing SQL runners check each outcome through the installed extension.
  - Required-NULL calls sit beside malformed partner operands and count zero extension reads and sends. Optional NULL settings behave as omitted.
  - Each database's complete-result binary call accepts native bytes with no caller-built JSON byte array.
  - DuckDB's actual return type is asserted for each changed function, and the value is consumed through DuckDB's JSON operations.
  - Each discovery query returns a description for the full registered public function inventory.
- Defers: Extension signing and registry submission stay outside this release's dependencies. Local native package inventory belongs to 0501. 0495 applies these rules during the DuckDB migration.
