# ADR 0038: DuckDB relate runs on the caller's database, not the caller's connection

- Status: Decided by the library team on 2026-09-22 on the lane's command-backed evidence. It is input to the build team's C++ fork decision. Ian can overturn.
- Date: 2026-09-22
- Record: this ADR is the DuckDB relate option A ruling that `databases/duckdb/NOTES.md` (Finding 3) cites.

## Decision

`thinkthen_relate` runs its query on a connection belonging to the caller's database, resolved at bind time through `duckdb_table_function_get_client_context` and `duckdb_client_context_get_catalog` against a per-database registry built at init. The one process-global connection is gone.

The caller's own connection is not reachable through the stable C API: `duckdb_query` requires a `duckdb_connection`; a client context exposes catalogs, config, the file system, and a connection id only; and temporary tables live in per-connection `ClientData`. Temporary-table and open-transaction visibility is therefore a boundary of the stable C API, not a defect this surface can fix. The boundary is pinned by a test, and a query naming a temporary table answers with the boundary in words instead of the raw catalog error. Same-name databases in one process cannot be told apart through the client context and are refused as a defect rather than guessed.

## Why this option

Three options were weighed on the lane's evidence. Option B — changing relate's SQL shape so records cross as values the caller's own SQL builds (a LIST of structs) — would make temporary tables and transactions work, but it changes the settled string form used by the drawn call and every relate conformance case, a contract-level change beyond the lane. Option C — document the defects and leave the wrong-database bug — was refused. Option A removes the wrong-database bug and the stale global without changing the settled shape, so it was ruled.

## Evidence

From `databases/duckdb/NOTES.md` on branch `surfaces`:

- The context accessors expose no connection: catalog, config, file system, connection id.
- Temporary tables live in per-connection `ClientData` (`temporary_objects`); a live temporary-table call answers `Catalog Error: Table with name tt does not exist`.
- Before the fix, two loaded databases shared the last-loaded connection: the two-database probe failed with "no recorded answer for the rule caused_by on these records; the recording covers founded, works_for" — a.db's query ran against b.db's table.
- After the fix: `a.db: 4 edges (want 4), b.db: 11 edges (want 11)`.
- The boundary message: "the relate query names the temporary table tt, and the stable C API cannot run a query on the calling connection, so relate cannot see temporary tables; materialize it (CREATE TABLE ... AS SELECT) or run the query directly".

## For the build team

This boundary is exactly the one the C++ door would lift. Temporary-table and open-transaction visibility for relate is impossible through the stable C API; the C++ fork under the build team's consideration (the `duckdb-rs` bind-callback question) is what removes it. The lane recorded this as MERGE-NOTE material.

The second review found further defects in this area — two in-memory databases both registering as "memory"; `ATTACH … USE other` counting the main database's table; a saved connection keeping a closed database file locked; relate holding any SQL and committing on its own; a nested relate hanging. Those are filed in `sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md` and remain open.

## Proof

Landed at `0345a42` ("Route DuckDB's relate to the caller's database, respect its file switch, and contain callback panics"). The fix's suites run from `check.sh`: `tools/two_databases.py` proves each database's relate runs on its own connection, `tools/security_suite.sh` proves the boundary message and the raw-error pass-through, and the conformance slice exits 0 with no stub.
