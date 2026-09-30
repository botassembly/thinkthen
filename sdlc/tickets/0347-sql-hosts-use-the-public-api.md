# 0347: The SQL hosts call the public API instead of copying engine code

Status: ready. Plan: `sdlc/planning/issue-priorities-2026-09-30.md`, batch B4. Pays items 1, 2, 3, 9 and 10 of Debt 018, `sdlc/issues/2026-09-25-public-library-api-gaps.md`. Starts after tickets 0344 and 0335 slice 2 land, because 0344 edits `databases/postgresql/src/relate.rs` and 0335 slice 2 edits the SQLite and DuckDB check scripts.

## Outcome

SQLite, DuckDB and PostgreSQL hold no copy of engine code that the public API could supply. PostgreSQL parses inline relate rules with a public parser and raises its refusals as `thinkthen::Error`. A loaded question is asked directly, so no SQL host matches both arms of `LoadedQuestion`. SQLite and PostgreSQL write `thinkthen_plan` with `serde_json::to_string` of the crate's `PlanEstimate`. The `Engine::usage` doc says what it counts, and every SQL host that sums several engines does it through one crate helper. Every SQL result, refusal sentence and plan byte stays the same.

## Evidence

- Starts from: the debt issue's items, rechecked on main `d8018dd96`. Item 1: `Engine::usage` says "This process's totals" (`public/engine.rs`) while each `EngineBuilder::build` makes its own `Counters` (`public/settings.rs`), and PostgreSQL keeps an `ENGINES` list and sums it (`databases/postgresql/src/call.rs`). Item 2: `inline_rule` in `core/relate_file.rs` and its copy in `databases/postgresql/src/relate.rs`. Item 3: `Error::of`, `usage`, `local`, `defect` and `cancelled` are `pub(crate)` (`public/error.rs`), so PostgreSQL keeps `Refusal` in `call.rs`. Item 9: 59 `LoadedQuestion::` matches across the three SQL hosts; only `Question` and `BandedQuestion` implement `DecisionQuestion`. Item 10: `json!` plan objects in `databases/sqlite/src/scalars/plan.rs` and `databases/postgresql/src/keyed.rs`, whose member names already equal `PlanEstimate`'s `Serialize` output. `json!` writes keys in alphabetical order, because the crate never enables `preserve_order`, while serializing the struct writes its declared field order.
- Keeps: every SQL value, NULL, failed marker, refusal sentence and PostgreSQL message shape (`thinkthen <kind>: <message> (retryable: yes|no)`); the plan bytes of both hosts; DuckDB's native plan `STRUCT`; the process request totals owned by `engine/limits.rs`; each host's usage totals under ADR 0113.
- Changes: one change per item, and the issue update.
  - Item 1: `Engine::usage` stays per engine, and its doc and `specification/` say so. PostgreSQL sums its engines through one crate helper, the existing `Tally` if it fits. A process-wide in-memory total would make a second engine's reading depend on its neighbours, and ADR 0113's durable totals already give the process view.
  - Item 2: a public `RelationRule` parser for `NAME` and `NAME=SOURCE:TARGET`, which the command and PostgreSQL both call.
  - Item 3: public constructors for the error kinds PostgreSQL raises, or the smallest public surface that lets it drop `Refusal`. Each new public item goes in the ticket's added public declarations block at landing.
  - Item 9: `LoadedQuestion` implements `DecisionQuestion`, and the fifteen host files that match both arms (four SQLite, three PostgreSQL, eight DuckDB; 59 matches) call the engine once.
  - Item 10: both hosts build the plan with `serde_json::to_value(&estimate)` and write that value, so SQLite keeps its alphabetical key order byte for byte. PostgreSQL's `JsonB` orders keys itself.
  - The issue drops these items and stays open for items 6 and 7, features after 0.1.
- Proof: the three SQL checks (`databases/{sqlite,duckdb,postgresql}/check.sh`) and their conformance runners pass with no expected byte changed. A byte-for-byte test pins each host's `thinkthen_plan` output before and after. A public API test runs the inline rule parser's edge table: bare name, `NAME=SOURCE:TARGET`, a missing colon, extra `=` or `:`, and an empty side, each with the command's refusal. A Rust test builds two engines in one process and pins what each `usage()` returns, matching the new doc. `policy.py`, `sdlc/scripts/test`, workspace clippy with `-D warnings`, the C door tests, the Polars tests, `tickets`, and `lint` in a clean checkout.
- Defers: items 6 and 7 of the issue.

## What the build taught us
