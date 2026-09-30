# 0322: Every surface adds to the usage totals

Status: ready for ticket review (slice 1); slice 2 waits for ADR 0111 slice 3. Design: ADR 0113 (proposed). Plan: `sdlc/planning/cleanup-2026-09-30.md`, order item 8. Issue: `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`, option 1.

## Outcome

`thinkthen status` shows one combined total of what every surface spent. An engine built by `EngineBuilder::from_env()` adds its requests, retries, live tokens and cache answers to the count-only monthly files the command writes. That covers every port, the C door and the three SQL extensions. The SQL hosts flush their counts at process exit. `EngineBuilder::new()` writes nothing. No test writes the real usage folder. `specification/recording.md` stops listing the SQL leaks that current source already closed.

## Slices

1. Builds now: `from_env` seeds the usage path; `Engine::finish_usage()`; PostgreSQL `on_proc_exit` and SQLite and DuckDB `atexit` hooks; test isolation and its guard; the pages; every proof except cache answers.
2. Waits for ADR 0111 slice 3: the cache-answer proof. Slice 2 of ADR 0111 changes what a cache answer counts, and slice 3 moves the SQL hosts onto `ask_all`.

## Evidence

- Starts from: ADR 0113; tickets 0308 and 0311, where one `from_env` read reached every surface with no port code; `engine/usage.rs::Counters` and its `finish()`, which already persist given a folder; the fork rule in `engine/process.rs`; the static engines in `databases/postgresql/src/call.rs`, `databases/sqlite/src/settings.rs` and `databases/duckdb/src/engines.rs`; record `2026-09-28-accounting-after-c-facts.md`.
- Keeps: the usage files, lock, `0700` folder and `0600` file modes, `thinkthen.usage/1` shape and retry sidecar; best-effort persistence with ADR 0097's one-second finish deadline; the count points; the `status` reader and schema; every port constructor; `shared_host()`'s folder rules, which leave the totals on; the per-process request total and token cap, per backend in PostgreSQL; the existing fork and lock proofs in `engine/facade/fork_tests.rs` and `tests/backend/facts/usage_lock.rs`.
- Changes: `from_env` seeds the counters with `config::usage_path()`. `Engine::finish_usage()` calls `Counters::finish()` for the calling process's own counters only. PostgreSQL registers it with `on_proc_exit` and SQLite and DuckDB with `atexit` when each builds its first engine. Library and SQL write failures stay silent. The shared test helpers point `XDG_CACHE_HOME` (macOS: `HOME`, after pinning `CARGO_HOME` and `RUSTUP_HOME`) at scratch. The test entry points set a decoy outer folder and fail if its usage folder appears. PostgreSQL `check.sh` starts its server with a scratch `XDG_CACHE_HOME`. `recording.md`, the SQL READMEs and the changelog say every surface adds to the totals and name PostgreSQL's per-backend cap.
- Proof: offline, against the counted loopback backend, under a scratch usage folder. The command, a Rust `from_env` engine with one retried 503, the C door and a SQLite process that exits with no sleep send five attempts; `status --json` then reports 5 requests, 1 retry and the tokens the replies reported. `EngineBuilder::new()` leaves the folder absent. A `0755` usage folder changes no result and no arrival count. PostgreSQL: two connections that send one each and disconnect give the server user's `status` two. One deliberately unisolated run fails the guard; the record says so. Slice 2: a cached rerun reaches the listener zero times and adds cache answers.
- Defers: a cross-connection PostgreSQL cap through shared memory; a per-surface split in `status`; a new opt-out switch; reporting library write failures to the caller; the other ports over C beyond the C door case, which start from the same `from_env`; counts lost to a crash or to a port engine its host never frees.
