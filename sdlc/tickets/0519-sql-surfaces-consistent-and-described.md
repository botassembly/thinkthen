# 0519: Make the SQL surfaces consistent and self-described

Status: COMPLETE.

Milestone: 0.2

Depends on: 0511

Reviews: revision 4cd756859, reject

Reviews: revision c79e3f65e05eb4f12fa46d8f7ab003d477258c9c, accept

Reviews: revision 4d59e7c6a, accept

Reviews: revision a087f6dc3, accept

Reviews: revision 9c8f5660b5e23ff0cbd92ccac0a814d4f92b9e30, accept

Reviews: revision cd33a4bf1e25eab03e082654ba36d637c7aa35f8, reject

Reviews: revision f4e6590ad291b2525deb3633fc28862c74b9d0d0, accept

Reviews: revision 8350dce534b0ecf209bb6e6570186799d95da2be, accept

Reviews: revision 5669417bbab9d88ed53cadd51152bed26817c18b, accept

Reviews: revision 7b49a4bafc92d103d8e83a9f0538db4eab491e6d, accept

Reviews: revision 8cce4c8ba0121ee23d1b32bf75bdf82403011d8b, accept

Reviews: revision 0b608d7a4bb41ec3a618f7778b863d0ab9db94d8, reject

Reviews: revision 099f0c54aacf30f330615899861186e61ecb8f65, accept

Reviews: revision 7f77442976806000e6cc303caf09b0ed933287b2, accept

Landed: 37ae83a

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

## Progress

- 2026-10-10 started
- 2026-10-10 landed ece970c80; next: PostgreSQL native bytea complete-image input is implemented and reviewed; focused installed ownership, replay, NULL and secrecy cases pass. Finish DuckDB native JSON and in-database descriptions. Full installed and platform qualification remain held.
- 2026-10-10 landed 99dd412fe; next: DuckDB now returns native JSON for serialized results and JSON fields; focused installed checks and fresh review pass. Finish in-database descriptions across SQLite PostgreSQL and DuckDB. Full installed and platform qualification remain held.
- 2026-10-10 landed b3d9bc4f5; next: SQLite exposes descriptions from its actual connection registrations, with reviewed truthful usage wording. PostgreSQL binary complete images and DuckDB JSON are landed. Finish PostgreSQL and DuckDB function descriptions; full installed and platform qualification remain held.
- 2026-10-10 landed d41b1279d; next: PostgreSQL now registers native function descriptions without changing signatures or attributes; focused installed catalog checks and fresh review pass. Finish reviewed DuckDB descriptions, then the planned final code review. Full installed and platform qualification remain held.
- 2026-10-10 landed b5342c8fc; next: Native image complete calls, DuckDB JSON and in-database descriptions are implemented across SQL surfaces with focused checks and review. Queue inspection confirms PostgreSQL legacy required-NULL paths still validate other operands; fix this remaining approved NULL rule before the final code review. Full installed and platform qualification remain held.
- 2026-10-10 landed fcb6ea928; next: PostgreSQL required NULLs now return NULL or zero rows before partner validation, files and sends; focused installed cases and fresh review pass. Finish the reviewed DuckDB NULL correction, then the planned code review. Full installed and platform qualification remain held.
- 2026-10-10 started
