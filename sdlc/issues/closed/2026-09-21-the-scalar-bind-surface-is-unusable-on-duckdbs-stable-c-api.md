# The scalar-bind surface is unusable on DuckDB's stable C API

Status: Closed (non-issue). Superseded for the shipped product by the accepted C++ route under ADR 0081, 0201 and 0231. Fresh criterion review accepted `5ee51bf8`. This does not claim the upstream stable C API was repaired or authorize an upstream report. Native Intel and release-runner qualification remain in 0231 and 0128.

The earlier investigation below is historical. The [closure reconciliation](../../records/2026-09-28-platform-closure-reconciliation.md) maps original criteria to accepted proof.

Historical status before the C++ migration: Ticket 0110 landed on 2026-09-25 (`sdlc/records/0110-port-the-duckdb-surface.md`) and works around this page without fixing it. The extension registers each scalar through the raw C API with an init callback and no bind callback. `SET thinkthen_max_requests` caps one chunk's call, and `databases/duckdb/README.md` names this page as the lever for a per-query cap. The bind path stays broken, and the decision below stays Ian's.

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

## Sharpened by external comparison and official guidance, 2026-09-21

Two inputs landed today. A third-party extension over the same backend (colliber/duckdb-jev, read-only comparison preserved on the surfaces branch records) solves the key problem with DuckDB's Secret Manager - a jev secret type, redacted from duckdb_secrets(), failing at plan time when absent - and derives return types from the criteria constant at bind, both through the C++ API our stable-C path cannot reach. And the official guidance recommends the C++ template over the stable C API for exactly a paid-network extension's two hard requirements: secret integration and parallel or non-blocking control - the same two places this issue records the stable C API as broken or thin.

The fork, stated for the build team: stay on the stable C API with the first-row check and a settings-or-environment key, or move to the C++ API and gain bind-time typing, the secret manager, and the parallel controls, paying a rebuild per DuckDB release and one binary per version. Also adopted from the comparison regardless of the fork: token counters parsed from the reply's usage block, failure injection at the wire seam in the test stub, sqllogictests with DESCRIBE type assertions, and the README line the guidance requires - persistent secrets are stored unencrypted.

Two independent implementations now state one text per request as the API's floor; see 2026-09-21-one-state-per-request-caps-table-scale-classification.md.

## Staged Linux resolution, 2026-09-27

Ticket0201 implements the C++ bind and statement-lifetime path on Linux x86-64. Independent High review accepted final source `493a1461` and the rebuilt, unpacked stock-host proofs in `sdlc/records/0201-duckdb-cpp-api-prerequisite.md`. Ian approved staging that target while retaining the other packages. This issue remains open for Linux aarch64 and both macOS targets, which still use the C API and lack their pinned C++ archive and host proofs. This is no longer a pending choice from Ian.
