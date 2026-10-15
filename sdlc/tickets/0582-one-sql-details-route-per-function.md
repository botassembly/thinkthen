# 0582: Give each SQL function one details route

Status: OPEN.

Milestone: 0.2

Reviews: revision f583a17, accept
Reviews: revision 25423e6, accept

Reviews: revision 4c93d32, accept

Reviews: revision a05f4d1, accept

## Outcome

SQLite, DuckDB and PostgreSQL each return the full record for all ten functions through `thinkthen_<function>_details`, and the plain calls return only the plain value.

## Evidence

- Starts from `ticket/0543-final` and `ticket/0576-sqlite-typed-family`, surveyed 2026-10-14:
  - All three have `thinkthen_details` and `thinkthen_try_details` for decide, choose, score and tag only, and a separate `thinkthen_details_images`.
  - The SQLite branch deletes `_complete`, makes `thinkthen_details` and `thinkthen_details_images` typed tables and keeps `try_details` scalar.
  - DuckDB registers `thinkthen_native_records_<function>` for all ten and record-list table macros under eight plain names, not rank or relate. PostgreSQL overloads all ten plain names. Rows return `ordinal, value, result, facts, failure, observations`, plus `selection` in DuckDB.
- Ian, 2026-10-14, agreed: plain calls return the value, and one details form covers all ten functions with one spelling.

## Decisions

- PM decision, 2026-10-14: `thinkthen_<function>_details` takes the plain call's arguments or a record list, and returns typed rows: a struct in DuckDB, a composite in PostgreSQL, a table-valued function in SQLite. A record list gives one row per record: `ordinal, value, probability, result, failure`, with `facts` and `observations` on each row. SQLite takes the record list as a JSON array argument.
- PM decision, 2026-10-14: `probability` follows 0581: decide, filter, choose and find carry one; score, tag and annotate give null; rank, recognize and relate carry one per item.
- PM decision, 2026-10-14: a recoverable row failure comes back in `failure` and later rows complete. Cancellation, deadline and defect raise.
- PM decision, 2026-10-14: delete `thinkthen_details`, `thinkthen_try_details`, `thinkthen_details_images`, the `_complete` functions, `thinkthen_native_records_<function>` and the record-list overloads of the plain names. No stubs: the database's unknown-function error is the refusal. Images reach `_details` inside the record. Delete `Engine::details_many_recoverable_with` and the image `details_input_with` once nothing calls them.
- Supersedes: ADR 0080, ADR 0094, ADR 0105's details and try_details rows, the 0538 retention ruling, 0576 ruling (3) and the 0576 amendment of 2026-10-14 on `thinkthen_details`. The 0576 SQLite and PostgreSQL lanes build `_details` instead.

## Scope

- Changes: `databases/sqlite/`, `databases/duckdb/`, `databases/postgresql/`, `sdlc/generators/results/templates/sqlite.py`, `sdlc/generators/results/templates/postgresql.py`, `crates/thinkthen-host/src/sql_request.rs`, `crates/thinkthen/src/public/native_batch.rs`, `crates/thinkthen/src/public/images/calls.rs`, `sdlc/planning/databases/`. One lane per database.
- Keeps: every plain function, `_many`, `rank`, `rank_set`, `filter`, `relate`, `plan`, `thinkthen_decide_images`, `thinkthen_choose_images` and `thinkthen_score_images`.

## Proof

Each database runs one `_details` query per function over its saved recordings and reads `value`, `probability` and `requests_sent`. The old details, try_details, details_images, `_complete` and record-list tests go. Each database's check script passes.

## Defers

The DuckDB listing (0577), registry pages (0578) and website (0575) follow after this lands.

PM amendment, 2026-10-14. Rank, find and relate judge one whole set. Their record-list `_details` treats the whole list as that one set and makes one call: rank returns one row per record with its rank and probability, find returns the picked record, relate returns one row per edge. A failure of that call returns one failure row. No nested sets: a caller with several sets calls once per set, such as with a lateral join over groups.

PM amendment, 2026-10-14. The SQLite part runs inside the 0576 SQLite lane, so this ticket has no prerequisite.

PM amendment, 2026-10-14. A relate `_details` row carries only its own edge. Its `result` is the canonical Related record with `value` holding that one edge, and it keeps the question, original input, meta and answer_id. No row repeats the whole edge set. Rows grow linearly with edges. The native `Call.results` keeps the full set unchanged.

PM amendment, 2026-10-14. In a set function's `_details` rows (rank, relate), the call-wide observations appear once, on the first row, and later rows carry empty arrays. The first row is the one whose existing ordinal or index field is zero, so a caller finds it after reordering, and the reference docs say so. Every observation appears exactly once. Native output stays unchanged. One shared projection fix serves all three databases, each with a regression.
