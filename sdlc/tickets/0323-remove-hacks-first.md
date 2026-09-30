# 0323: Remove the hacks clear of running work

Status: built, awaiting code review. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 4 and ruling 8.

## Outcome

`core` names nothing from `public`, and `policy.py` refuses a core reference to `public` as it refuses `engine` and `cli`. `engine` reaches `public` only for the send budget, the process budget and the facade's error mapping, which ADR 0111 slice 3 rewrites. No function outside the ADR 0111 rewrite packs arguments into a tuple only to pass Clippy's argument limit. The ticket names each remaining hack with the slice that removes it. Ticket 0303 already made the folder rule one predicate.

## Evidence

- Starts from: the step 4 inventory below, taken with `rg` on `origin/main` at `7c4fdf977`. ADR 0111 section 10 names the deleted files, including the one-millisecond loops in the annotate former. Ticket 0303 made the folder rule "another owner or other-write". Ticket 0318 reused it for shared hosts. `policy.py` `core_policy_failures` refuses `crate::engine` and `crate::cli` in core and plants each failure in its self-check.
- Keeps: every exported name and path. `thinkthen::AttemptObservation`, `AttemptOutcome`, `SendBudgetDenial` and `EstimatedInputDenial` stay public through re-exports. The keeps also cover `specification/result.schema.json` byte for byte, all command output, and every 50 ms cancellation or host-interrupt poll. SQLite's interrupt, PostgreSQL's interrupt flags and a caller's token cannot wake a waiter, so a poll is the only way to see them. This matches pyo3's 50 ms signal check.
- Changes: four type moves, two argument fixes and one policy rule.
  - Move `AttemptObservation` and `AttemptOutcome` from `public/results/attempt.rs` into `core/result/`. The transport builds the value with a struct literal. The three-slot `response` tuple and the `new` constructor go.
  - Move `SendBudgetDenial` and `EstimatedInputDenial` from `public/options/budget.rs` into a new `core/budget.rs`. `public` re-exports all four.
  - `core::recognize::settle` takes `text` and `pieces` as two arguments, six in all.
  - `cli/recognize/dry_run.rs` passes the spec, the from-file flag and the limit as a named struct in place of the `question` triple.
  - `policy.py` adds `public` to the refused core roots. Its self-check plants `public` in the same forms the `engine` and `cli` plants use: a direct path, a raw identifier, `use … as`, braces, `super::super::` and globs. Each plant must fail for its named cause. A `public_value` import, a comment and a string literal stay allowed.
- Proof: `policy.py` passes. Its self-check refuses each planted `public` form with `reverse reference to public`, `reverse import of public` or the glob failure. The schema drift test passes with no schema change. The public member inventory test passes with no change. Conformance cases 41 to 50 and the recognize `--plan` tests pass unchanged. `sdlc/scripts/test`, workspace clippy with `-D warnings` on all targets, `sdlc/scripts/tickets` and the ratchet pass. The ratchet does not rise.
- Defers: every row marked b or c below. ADR 0111 slice 3 removes the process statics and the throttle. Once they go, `policy.py` can refuse `crate::public` in `engine` too. Slice 3 owns that check. Test sleep loops wait for step 5.

## Inventory

Class a: fix here. Class b: inside the ADR 0111 rewrite; the named slice removes it. Class c: in `libraries/python`, `r` or `ruby`, where 0314 slice 3 is in review. Keep: an honest cost with a stated reason. Paths are under `crates/thinkthen/src` unless they start with `databases/` or `libraries/`. Line numbers refer to `7c4fdf977`.

| Kind | Where | Class | Why |
| --- | --- | --- | --- |
| Upward import | `core/result/meta.rs:31,78` | a | `AttemptObservation` moves to core |
| Upward import | `engine/mod.rs:60,70,128`, `engine/http/observation.rs:3` | a | Same move |
| Upward import | `engine/error.rs:51`, `engine/send_budget.rs:86-134` (denials) | a | Denial enums move to core |
| Upward import | `engine/send_budget.rs:10,18-20,51,80`, `engine/mod.rs:46` | b, slice 3 | `SendBudget` and its reservations belong to the process budget |
| Upward import | `engine/facade.rs:143`, `engine/facade/annotate.rs:15` | b, slice 3 | Facade |
| Lint tuple | `core/recognize.rs:187` `source` | a | Six plain arguments fit |
| Lint tuple | `cli/recognize/dry_run.rs:83,106` `question` | a | Named struct |
| Lint tuple | `public/results/attempt.rs:43` `response` | a | Struct literal after the move |
| Lint tuple | `public/engine.rs:140-141` `totals`, `source` | b, slice 3 | Engine construction and the process budget |
| Lint tuple | `public/results/observation.rs:197` `receipt` | b, slice 3 | Only annotate batching calls it |
| Lint tuple | `engine/facade/recognize.rs:365-366` | b, slice 4 | Facade recognize moves to `ask_all` |
| Lint | `(usize, usize)` spans in `core/recognize*`; `(String, String)` map key in `core/batch.rs:257`; `cli/asked.rs:24` alias; about 40 `allow` or `expect` with reasons | keep | Honest; no contortion |
| 1 ms sleep | `cli/annotate/batching/former.rs:92,101` | b, slice 2 | ADR 0111 section 10 deletes them |
| 1 ms sleep | `engine/mod.rs:493,498,520` `REBUILD_POLL` | b, slice 3 | Serves `PROCESS_WIDTH` and `PROCESS_GATES` |
| 50 ms poll | `engine/schedule.rs:313`, `engine/annotate_schedule.rs:181`, `cli/asking/batched.rs:205`, `public/batch.rs:147`, `public/batch/annotation.rs:156`, `public/batch/planned.rs:243` | b, slices 2-3 | Old schedulers and batch readers |
| 50 ms poll | `engine/mod.rs:316,448`, `engine/workers.rs:84,222`, `engine/backoff.rs:171`, `engine/usage/lock.rs:40`, `cli/interrupt.rs:316`, `databases/sqlite/src/worker.rs:123`, `databases/postgresql/src/call.rs:267` | keep | Host and token checks cannot wake a waiter; the usage lock wakes on its deadline (0315) |
| 50 ms poll | `libraries/python/src/stream.rs:280,400`, `worker.rs:236,426`, `libraries/ruby/src/lib.rs:138`, `libraries/r/.../calls/worker.rs:168` | c | Python's is pyo3's signal check and stays |
| Test sleep | 33 in `crates/thinkthen/tests`, 8 in `src` test modules, 6 in `databases/*` test modules, 3 in `libraries/c/tests` | step 5 | Test cleanup; many sit in b files |
| Static | `engine/mod.rs:490` `PROCESS_WIDTH`, `:526` `WIDTH_CHILD`; `engine/backoff.rs:275` `PROCESS_GATES`; `public/options/budget.rs:189` `process_budget`; `public/mod.rs:72` `default_engine` | b, slice 3 | The throttle and process statics |
| Static | `engine/recorder.rs:28`, `engine/recorder/identity.rs:17` `WRITES` | b, slice 5 | Recording store deleted |
| Static | `databases/postgresql/src/call.rs:147` `ACTIVE_THROTTLE`, `:151,152`; `databases/duckdb/src/engines.rs:62,138`; `databases/sqlite/src/settings.rs:75,90`, `budget.rs:19`, `question.rs:182,184` | b, slice 3 | SQL hosts move to `ask_all` |
| Static | `cli/interrupt.rs:330`, `cli/file_size.rs:23`, `engine/workers.rs:12-19`, `cli/audit/write.rs:160`, SQLite and DuckDB signal and API tables, PostgreSQL settings, `libraries/c/src/failures.rs:163`, test-only thread-locals | keep | Signals, panic hooks, host tables and per-thread C errors are process-wide by nature; a counter names temporary files |
| Static | `libraries/python/src/worker.rs:33`, `lib.rs:67`, `arrow/gate.rs:26,27,71`; `libraries/r/.../lib.rs:163`, `calls/worker.rs:185` | c | 0314 slice 3 |
| Folder rule | `engine/usage.rs:386` exact 0700 | keep | Guards the folder thinkthen creates for its own totals |
