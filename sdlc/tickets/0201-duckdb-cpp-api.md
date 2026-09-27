---
flow: build
priority: 201
opens: databases/duckdb sdlc/scripts/policy.py sdlc/scripts/surfaces sdlc/scripts/package specification/settings.md sdlc/planning/databases/duckdb.md sdlc/planning/adr/0047-bindings-are-unpublished-crates-over-the-public-api.md CHANGELOG.md sdlc/issues sdlc/records sdlc/tickets
---

# 0201: Move the DuckDB extension to the C++ API

Status: accepted 2026-09-27 by the Codex queue owner after fresh read-only review and its correction. The reviewed design is `c7000a4a`; `sdlc/records/0200-0201-design-review.md` records the paired review. Owner: Codex. Runtime files remain unclaimed while ticket 0200 implements the shared SQL contract.

## Outcome and authority

A DuckDB query using a ThinkThen scalar gets a usable bind step and a real query lifetime. A constant invalid question fails before any backend request. A signal that arrives after the last engine call belongs to the ending query, so the next query answers normally. Ticket 0200's DuckDB query deadline uses the same lifetime across chunks and expressions. The extension is built through DuckDB's C++ API, while the one Rust engine still makes every judgment.

Ian directed Batch I3 and the C++ migration. `sdlc/planning/work-plan-2026-09-27.md` places this after the SQL error and budget design. This ticket implements both open issues, not merely a comparison or spike. The coordinator owns its runtime lane after the 0200 design settles. Ian can overturn the bridge, key, and packaging choices in ADR 0081. The C++ migration itself is his direction.

## Prior experiment evidence and current main

- Local experiment 207, `experiments/207-thinkthen-db/duckdb/NOTES.md` entry 3, ran DuckDB v1.5.5 through `extension-template-rs` and isolated the raw C scalar bind failure: the callback fires, parameter count returns `-7`, parameter access and error setting crash, and returning without bind data double-frees. `sdlc/issues/2026-09-21-the-scalar-bind-surface-is-unusable-on-duckdbs-stable-c-api.md` records it. No upstream issue was opened.
- Ticket 0110 and `sdlc/records/0110-port-the-duckdb-surface.md` worked around bind by registering without it and validating in the first chunk. Ticket 0118 ported relate. Local experiment 253, `experiments/253-thinkthen-duckdb-sqlite-spike/REPORT.md`, proved the caller-context settings and file access path on the pinned stock CLI; its D1 to D5 logs are reused.
- `sdlc/issues/2026-09-23-duckdb-scalars-through-the-c-api-for-a-query-hook.md` says a SIGINT after a chunk's last call can affect the next query. Ticket 0110 supplied per-execution init state, but the late-signal spend remains open. `databases/duckdb/src/signal.rs` still uses timing and signal-count heuristics.
- Read on `origin/main` after claim `9b8bf076`: `databases/duckdb/Cargo.toml` pins `libduckdb-sys = 1.10505.0` with `loadable-extension`; `tools/version.env` pins the v1.5.5 CLI; `src/scalars/ffi.rs` registers all nine scalars through the raw C API with init but no bind; `src/scalars.rs` groups up to one chunk by question and deadline; `README.md` fixes the SQL names and values. The two issues still describe current code.
- Official tagged v1.5.5 [ScalarFunction header](https://raw.githubusercontent.com/duckdb/duckdb/v1.5.5/src/include/duckdb/function/scalar_function.hpp) has bind and local state callbacks. [ClientContextState](https://raw.githubusercontent.com/duckdb/duckdb/v1.5.5/src/include/duckdb/main/client_context_state.hpp) has `QueryBegin` and `QueryEnd`. [ClientContext dispatch](https://raw.githubusercontent.com/duckdb/duckdb/v1.5.5/src/main/client_context.cpp) calls them at statement boundaries. This is source evidence, not a completed bridge proof.

## Retained behavior

- Keep all nine existing scalar names and overloads, and add ADR 0080's tenth scalar `thinkthen_try_details`, with its typed JSON failed value. Keep `thinkthen_usage()`, `thinkthen_warm`, and `thinkthen_relate` with their present SQL argument order, result types, NULL behavior, `thinkthen <kind>: ` error prefix, and millisecond trailing deadline. Keep `WHERE thinkthen_decide(...)` and `ORDER BY thinkthen_probability(...)` as SQL filter and rank forms. `LIMIT` truncates rows and does not implement the grouped `find` contract; its separate issue remains open. Preserve `thinkthen_annotate` as JSON text and the nested types for tag, recognize, and relations.
- Keep chunk grouping, first-seen deduplication by question and text, one Rust engine, its process throttle, cache, recordings, and counters. A NULL row sends nothing. A backend failure remains an error unless the separate I2 try-value contract asks for a value.
- Keep `THINKTHEN_API_KEY` and the existing settings. `@file` and a SQL cache folder still use the calling database's file permissions. Relate still sees committed data through its separate connection, with its present row and plan guards. A C++ API opportunity to see temporary tables does not silently change that contract.
- Keep the stock-host v1.5.5 target and the rule that a release package loads without a Rust toolchain. No paid or live call enters a gate.

## Change and ownership

1. Pin DuckDB C++ v1.5.5 source and the C++ extension template. Build one C++ extension binary linked to a Rust static library. Export only a small C ABI from Rust. The C++ side owns DuckDB registration, bind, vectors, settings, file access, table and aggregate functions, and statement state. Rust owns question parsing and calls to the public engine. Neither side unwinds, retains a borrowed buffer, or frees the other's memory across the bridge. The first proof fixes typed row/result encoding and ownership, including nested lists and structs, before porting a verb.
2. Add bind callbacks for all scalar overloads. For the nine ordinary scalars, validate foldable constant questions, question sets, member lists, relation rules, and deadlines at bind. Store validated values in copyable `FunctionData`. Permit nonconstant arguments and validate every row of an ordinary scalar's chunk before that chunk sends a request. Bind sends nothing. A bad constant on an ordinary scalar raises the classified error with zero requests. `thinkthen_try_details` follows ADR 0080's row recovery instead: bind stores a bad foldable question as typed failure data without raising, and invoke emits a safe failed JSON value for its non-`NULL` rows. Nonconstant questions are handled independently per row. Its usage, local, and backend failures do not abort later good rows; host interrupt, cancellation, deadline, and defect stay fatal. Register all ten with their settled result types and volatile marking.
3. Install one per-connection `ClientContextState` as the statement owner. Capture the signal sequence and one monotonic I2 expiry at query begin. `SET thinkthen_query_budget_ms = n` follows ADR 0080: `-1` means none, `0` means spent, and positive representable milliseconds set the budget. Share it across all scalar expressions and chunks. A state installed during its first bind initializes that active query once, since it missed `QueryBegin`; the first-use fallback starts late and does not claim to bound earlier SQL work. At query end, clear the owner and spend a late signal. A newly begun query snapshots a fresh sequence. The Rust call receives the remaining query time and any smaller trailing deadline. An expired budget refuses before send. Query deadline and host cancellation remain statement errors.
4. Port all ten scalars, the usage table, warm aggregate, and relate to C++ DuckDB host calls. Remove the raw DuckDB C API dependency, registration, and metadata ABI mode in the same landing change. Keep the process-wide Rust engine registry, throttle, and counters, including I2's resident-engine and retired-counter rule. Update `check.sh`, setup, source checks, package metadata, the policy plants, ratchets, pages, and the two issue dispositions with the behavior that actually passes.

The accepted prerequisite proof at `981c686d` showed a caught Rust panic still printed its payload twice to stderr. Before porting a verb, amend the proof bridge and verify a fixed, payload-free defect status, no synthetic key or evidence marker in stdout, stderr, or the SQL error, and a successful next query. An unrelated thread must still reach the host's prior panic hook. Install any delegating hook once, never around each call; mark bridge calls by thread-local depth. Record the process-global hook ownership and coexistence limit. Every Rust export, including free, and every C++ callback must keep unwind or exceptions on its own side of the ABI. A fresh read-only review accepts this bounded correction before the verb port.

No secret type enters this ticket. The key channel stays as in the retained behavior. A secret type would change configuration and persistent-storage behavior and needs a separate decision. No upstream DuckDB report is sent under this ticket.

## Proof matrix

| Behavior | Proof against stock v1.5.5 host and loopback stub | Deliberate regression |
| --- | --- | --- |
| C++ and Rust package load | Build through pinned C++ source, load packaged file in stock CLI and Python with only runtime libraries; call decide and one nested result | Remove Rust link or metadata and load fails |
| Ordinary bind validates before send | `PREPARE`/`DESCRIBE` and direct ordinary SQL with a bad constant question, set, list, or deadline; count zero stub requests and exact error kind | Move check back to invoke and prepare succeeds |
| Try-value bind preserves row recovery | Prepare one statement with a bad constant try-details arm and a later good row. Execute it and see a typed safe usage failure followed by an answered value, with no request for the bad row. Repeat with a bad `@file` for local failure. | Treat try-details bind or whole-chunk validation as fatal and the statement aborts |
| Row-dependent input remains legal | Two question values in a column, one bad value in a later chunk; ordinary scalar chunks validate before their own first send, while try-details emits one failed value and keeps later good rows | Treat all questions as constants or let a try-details bad row abort the chunk |
| Query owner spans work | Two scalar expressions and more than 2,048 rows under one query deadline; expiry stops later chunk before send; next query succeeds | Reset expiry in `init_local_state` or per chunk |
| Signal belongs to ending query | Deliver SIGINT after final ThinkThen call while query still runs; finish/interrupt it, then run a new query and see its answer | Keep timing heuristic and next query cancels |
| Failure and prepared cleanup | Repeat a failed query and a prepared statement on one connection; each new execution gets a fresh owner | Omit QueryEnd or reuse stale owner |
| Existing SQL contract | Run current `tools/{verbs,settings,signal,relate,databases,conformance,site_examples}.py` and type assertions, including NULL, JSON text, lists and structs | Change a name, type, or file access path and a case fails |
| Distribution compatibility | Match the v1.5.5 CLI and Python module; a wrong-version host refuses the artifact and the package records version/platform | Build only an in-tree shell artifact |

The new tests protect the bind, query-lifetime, and package regressions that the current suites cannot see. Tests operate through SQL and the loopback stub; they add no test-only public API. The existing suite keeps its contract checks rather than copying an inventory into a second layer. A red proof for each new boundary runs before the fix, or the builder records why the old binary cannot express that boundary. The final `install`, `lint`, `test`, `spec`, and `surfaces` rungs pass under `THINKTHEN_HEAVY_LOCK=/run/user/1000/thinkthen-heavy.lock`; the DuckDB surface's own `check.sh` passes. Cold external Cargo or CMake runs use `flock -o` and never nest the lock. No gate uses a paid backend.

## Build order and stop rules

1. Land the accepted 0200/ADR 0080 contract before wiring its DuckDB query deadline. Rebase the runtime lane and confirm its precise settings and error form. The design and first bridge proof may proceed without touching 0200's runtime files.
2. Prove a tiny packaged C++ extension linked to Rust loads in the stock CLI, and pin the source and toolchain digests. Stop if the stock host cannot load it, if a bridge panic or exception crosses the ABI, or if nested value ownership is unclear. Do not write the full port around an unproved bridge.
3. Prove ordinary bind and try-details' prepared bad-constant recovery, then `ClientContextState` first-use, prepared, failure, multi-expression, and late-signal behavior. Stop if the state cannot identify a single statement across those paths. Revisit the ADR before using a timing heuristic as a substitute.
4. Port all functions, run the existing suites and full ladder, check a build without test hooks, update docs and issue disposition, and record exact package compatibility. Stop before dropping a SQL name or a retained behavior, weakening file access, adding a credential channel, adding another dependency, or widening the public Rust API. Bring any such change through a reviewed ADR amendment.

The C++ source pin and bridge are new build inputs. A fresh code reviewer checks their origin, license, dependency graph, cross-language ownership, exception and panic boundary, and version/platform packaging. The source and Python ratchets move to measured totals; the build record explains added lines and where the old C glue was deleted first. `sdlc/scripts/policy.py` plants a forbidden dependency and a missing C++ check so gate changes cannot quietly drop either proof.

## Routing, scope, and deferred gaps

Design writer: Codex in `worktrees/thinkthen-codex-3`. Fresh read-only Codex design reviewer, then coordinator acceptance. Runtime builder uses the same ticket branch after a separate runtime file claim; a fresh read-only Codex code reviewer checks the final change. Coordinator lands and pushes.

The design-stage claim opens only this ticket and ADR 0081. Runtime opens `databases/duckdb/`, `sdlc/scripts/policy.py`, `sdlc/scripts/surfaces`, `sdlc/scripts/package`, `specification/settings.md`, `sdlc/planning/databases/duckdb.md`, ADR 0047's DuckDB section, `CHANGELOG.md`, the two named issues, and a build record. Shared pages merge after the other lane lands. No `crates/thinkthen`, `conformance/`, other database extension, live probe, or marketing file changes without a reviewed ticket amendment.

The key remains in the environment, and DuckDB's secret manager stays a separate feature decision. An upstream report on the C API bind crash remains Ian's choice. New DuckDB releases each require a rebuilt and tested binary. Ticket 0201 closes the two issues only after the migration and proofs land.

## Evidence

- Starts from: Local experiment 207's v1.5.5 bind crash, local experiment 253's stock-host settings and file proofs, the two open DuckDB C API issues, the 0110 and 0118 records, current `origin/main` `9b8bf076`, and DuckDB's tagged v1.5.5 C++ headers linked above.
- Keeps: Every shipped SQL function and overload, its types, NULL and error behavior, millisecond deadlines, caller file permissions, committed-data relate boundary, Rust engine and counters, environment key, and stock-host package proof. ADR 0080's new try-details scalar keeps row recovery across the migration.
- Changes: A pinned C++ extension and Rust C bridge replace raw C DuckDB registration and host access; ordinary scalar bind validates constants while try-details bind retains typed failures as values; `ClientContextState` owns statement signals and the I2 DuckDB time budget.
- Proof: A stock-host bridge smoke, bind zero-request cases, multi-expression and multi-chunk statement lifetime, prepared and failure cleanup, late SIGINT followed by a successful query, existing loopback and conformance suites, negative version check, and the full five-rung ladder.
- Defers: A DuckDB secret type and any upstream report, each with its own authority; no part of the required C++ migration, bind fix, or query-lifetime fix is deferred.
