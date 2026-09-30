# ADR 0113: Every surface adds to the usage totals

- Status: **Proposed**, 2026-09-30. A fresh reviewer checks it before ticket 0322 builds. Ian can overturn each item.
- Date: 2026-09-30

This ADR answers option 1 of `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`. It follows ruling 8 of `sdlc/planning/cleanup-2026-09-30.md`. It amends ADR 0034, which kept persistence to the command.

## Context

The engine already owns one set of process counters, `engine/usage.rs::Counters`. They count every HTTP attempt before it goes, each retry, the tokens of each live reply, and each cache answer. Given a folder, a write-behind thread adds those counts to the monthly `thinkthen.usage/1` files under one cross-process lock (ADRs 0034, 0049, 0097). `Counters::finish()` waits for that thread under ADR 0097's one-second lock deadline. `thinkthen status` reads the files.

The command builds its counters with `config::usage_path()`. The public builder builds them with `Counters::new(None)` in `public/settings.rs`. Every port, the C door and all three SQL extensions build their engine from `EngineBuilder::from_env()`. So their counts live in memory and die with the process. The missing piece is a folder.

Tickets 0308 and 0311 showed the thin path: one read in `from_env` reaches every surface with no port code.

## Decision

### 1. What every surface records

Every surface records the command's counts: requests sent, retries, live input tokens, live output tokens and cache answers. The count points stay in the shared counters and the send and lookup stages. Replay adds nothing, as today. No surface gains a count point of its own.

### 2. Where the totals live

`EngineBuilder::from_env()` seeds the counters with `config::usage_path()`, the folder the command uses: `$XDG_CACHE_HOME/thinkthen-usage`, then `$HOME/.cache/thinkthen-usage` on Linux, and `$HOME/Library/Caches/thinkthen-usage` on macOS. Each surface uses the command's existing switch if it has one. None gains a new switch. `THINKTHEN_CACHE`, a SQL cache setting and `--no-cache` never move or disable the totals.

The files, lock, modes and count-only shape stay unchanged, so older readers still read them. The writer already demands a `0700` folder and `0600` files and refuses anything else.

`EngineBuilder::shared_host()` leaves the totals on. Its rules cover answer, record and replay folders, which hold judged text. The usage folder holds counts only.

`EngineBuilder::new()` reads no environment and writes no usage. A Rust caller who builds an engine by hand keeps in-memory counts. The Rust Polars door follows the caller's engine.

The rule is "an engine that reads the environment adds to the environment's totals". That stays inside CLAUDE.md's boundary: user-named files, the platform cache, and adjacent count-only usage totals.

### 3. Failures, forks and exit

- Best effort, as for the command. The first failure stops later writes in that process. A library or SQL call has no warning line, so it stays silent. It never changes a result, an error or an exit.
- A missing or relative `HOME` and `XDG_CACHE_HOME` means no folder and no write, as today.
- The writer adds deltas, never totals. Two engines in one process each add only their own attempts.
- A forked child builds fresh counters and leaks the inherited ones untouched (`engine/process.rs`). The parent's pending deltas are never written twice.
- Accepted: a fork during a write leaves the child holding a copy of the lock's open file. A long-lived child stalls other writers until it exits. Each stalled writer gives up at its deadline and loses only its advisory counts.
- Dropping an engine flushes its writer, as today.
- **Static hosts flush at exit.** The SQL hosts never drop their engines: PostgreSQL keeps `ENGINES` in a static, SQLite a `OnceLock`, DuckDB a static registry. So a new `Engine::finish_usage()` calls `Counters::finish()`. PostgreSQL registers it with `on_proc_exit` when a backend builds its first engine. SQLite and DuckDB register it with `atexit` when they build their first engine. The hook reaches only the calling process's own counters. A forked child that never built its own does nothing. The hook lives in the SQL extensions. The library and the C door keep no process-wide hook.

Why flush at exit: `finish()` already exists, the command already uses it, and its one-second deadline bounds the wait. Each host adds one registration. The other choice, accepting the loss, leaves every SQL host undercounting its last calls. It also leaves no race-free proof for PostgreSQL, whose backends never drop an engine. A crash still loses the tail, as ADR 0034 accepts.

### 4. How `status` reads them

`status` reads the same files with the same reader. It shows one combined total. Its sentence "It counts only what the command sends" goes. The file shape is count-only, so nothing splits by surface.

A PostgreSQL backend runs as the server's operating-system user. Its counts land in that user's usage folder. A database administrator reads them with `thinkthen status` run as that user. That sum covers every backend of the server.

### 5. The three SQL leaks

- **DuckDB warm outside the total.** Closed at current source. `scalar_settings.cpp` passes the session total to the bridge warm path, which attaches the shared send budget.
- **Concurrent calls passing the total.** Closed at current source. `SendBudget` reserves each attempt atomically, retries included.
- **PostgreSQL counts per backend.** Kept. The request total and the token cap bind per backend. Section 4 makes the pool's combined spend visible. A cross-connection cap would need PostgreSQL shared memory and `shared_preload_libraries`. It is deferred.

The cache-off resend of a repeated row stays deliberate. `specification/recording.md` still lists all three as open. Ticket 0322 corrects that paragraph.

### 6. How it fits ADR 0111

The folder and the exit flush sit in the shared counters, below every batch path, so ADR 0111 slice 3 does not move them. Requests, retries and tokens count the same before and after it. Only the cache-answer count waits: ADR 0111 slice 2 changes what a cache answer counts, and slice 3 moves the SQL hosts onto `ask_all`. The cache-answer proof therefore waits for slice 3.

## Test isolation

- The shared test helpers set `XDG_CACHE_HOME` to a scratch folder under `CARGO_TARGET_TMPDIR` for every engine and child process they start. On macOS they set `HOME` instead, after pinning `CARGO_HOME` and `RUSTUP_HOME`. So plain `cargo test` never writes the real folder.
- `sdlc/scripts/test`, `surfaces` and each surface check set the outer `XDG_CACHE_HOME` (or `HOME` on macOS) to a decoy folder. A test that bypasses the helpers writes the decoy. The guard fails the run if the decoy's usage folder exists. Ian's own `thinkthen` runs write only the real folder, so they never trip the guard.
- PostgreSQL `check.sh` starts its server with a scratch `XDG_CACHE_HOME`. It runs `status` with the same value.
- The build proves the guard once: one deliberately unisolated run fails it, and the record says so.

## Build order

Ticket 0322. Each slice lands green with the full suite, workspace clippy, `policy.py` and a fresh code review. All proof runs offline against the counted loopback backend.

1. **Builds now: persistence, exit flush and isolation.** `from_env` seeds the usage path. `Engine::finish_usage()` and the three host hooks land. The test isolation above lands. Proof:
   - The command sends one request. A Rust `from_env` engine sends two, one a retry after a 503. The C door sends one. The SQLite extension answers one row and its process exits without a sleep. `thinkthen status --json` then reports requests equal to the listener's arrivals (5), retries 1, and the tokens the replies reported.
   - The same calls through `EngineBuilder::new()` leave the usage folder absent.
   - A `0755` usage folder refuses the write. Each call returns the same result, and the listener sees the same arrivals.
   - PostgreSQL `check.sh` opens two connections that each send one request and disconnect. The server user's `status` then reports two.
   - Forks and concurrent writers reuse the existing proofs unchanged: `engine/facade/fork_tests.rs` and `tests/backend/facts/usage_lock.rs`.
   - `recording.md`, each SQL extension's README and the changelog say every surface adds to the totals and name PostgreSQL's per-backend cap.
2. **Waits for ADR 0111 slice 3: the cache-answer proof.** The slice 1 calls again with the cache on reach the listener zero times. `status` adds cache answers equal to the questions answered from the store.

## What this amends

| ADR | Change |
| --- | --- |
| 0034 | Every engine built from the environment persists, not only the command |
| 0111 section 7 | Unchanged counts; they now reach the usage files from every surface |

## Decisions Ian can overturn

The coordinator settled items 1 to 4 as defaults on 2026-09-30.

1. Libraries and SQL extensions built with `from_env` write the count-only totals by default. A surface uses the command's existing switch if it has one. None gains a new switch.
2. PostgreSQL writes to the server user's usage folder, counts only. `shared_host()` leaves the totals on.
3. PostgreSQL keeps its per-backend cap. A cross-connection cap is deferred.
4. `status` shows one combined total, with no split by surface.
5. `EngineBuilder::new()` writes nothing.
6. Library write failures stay silent.
7. Static hosts flush at exit through `on_proc_exit` and `atexit`.
8. A forked child holding the usage lock stalls writers until it exits.
