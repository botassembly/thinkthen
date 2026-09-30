# ADR 0113: Every surface adds to the usage totals

- Status: **Proposed**, 2026-09-30. A fresh reviewer checks it before ticket 0322 builds. Ian can overturn each item.
- Date: 2026-09-30

This ADR answers option 1 of `sdlc/issues/2026-09-25-status-sees-only-command-spend-and-the-sql-total-has-three-leaks.md`. It follows ruling 8 of `sdlc/planning/cleanup-2026-09-30.md` and builds on ADR 0111's one send stage. It amends ADR 0034, which kept persistence to the command.

## Context

The engine already owns one set of process counters, `engine/usage.rs::Counters`. They count every HTTP attempt before it goes, each retry, the tokens of each live reply, and each cache answer. Given a folder, they add those counts to the monthly `thinkthen.usage/1` files behind the requests, under one cross-process lock (ADRs 0034, 0049, 0097). `thinkthen status` reads those files.

The command builds its counters with `config::usage_path()`. The public builder builds them with `Counters::new(None)` in `public/settings.rs`. Every port, the C door and all three SQL extensions build their engine from `EngineBuilder::from_env()`. So their counts live in memory and die with the process. The one missing piece is a folder.

Tickets 0308 and 0311 showed the thin path: one read in `from_env` reaches every surface with no port code.

## Decision

### 1. What every surface records

Every surface records the command's four counts plus retries: requests sent, retries, live input tokens, live output tokens and cache answers. The count points stay where ADR 0111 section 7 puts them. The send stage counts each attempt and each live reply's tokens once. The lookup stage counts each question answered from the store. Replay adds nothing, as today. No surface gains a count point of its own.

### 2. Where the totals live

`EngineBuilder::from_env()` seeds the counters with `config::usage_path()`, the same folder the command uses: `$XDG_CACHE_HOME/thinkthen-usage`, then `$HOME/.cache/thinkthen-usage` on Linux, and `$HOME/Library/Caches/thinkthen-usage` on macOS. The files, lock, modes and count-only shape stay unchanged, so older readers still read them. `THINKTHEN_CACHE`, a SQL cache setting and `--no-cache` never move or disable them, as for the command.

`EngineBuilder::new()` reads no environment and so writes no usage. A Rust caller who builds an engine by hand keeps in-memory counts. The Rust Polars door uses the caller's engine and follows its choice.

The rule is "an engine that reads the environment adds to the environment's totals". That keeps the write inside CLAUDE.md's boundary: user-named files, the platform cache, and adjacent count-only usage totals.

### 3. Failures, forks and lifetimes

- Best effort, as for the command. The first failure stops later writes in that process. A library or SQL call has no warning line, so it stays silent. It never changes a result, an error or an exit.
- A missing or relative `HOME` and `XDG_CACHE_HOME` means no folder and no write, as today.
- The writer adds deltas, never totals. Two engines in one process each add only their own attempts, so nothing counts twice.
- A forked child builds fresh counters and leaks the inherited ones untouched (`engine/process.rs`). The parent's pending deltas are never written twice.
- Dropping an engine flushes its writer with ADR 0097's one-second lock deadline. A process that exits without dropping its engine, such as a PostgreSQL backend, can lose the counts since the last write. ADR 0034 already accepts that undercount for a crash.

### 4. How `status` reads them

`status` reads the same files with the same reader. Its sentence "It counts only what the command sends" goes. The totals do not split by surface, because the file shape is count-only and old readers must still read it.

A PostgreSQL backend runs as the server's operating-system user. Its counts land in that user's usage folder. A database administrator reads them with `thinkthen status` run as that user. That sum covers every backend of the server.

### 5. The three SQL leaks

- **DuckDB warm outside the total.** Closed at current source. `scalar_settings.cpp` passes the session total to the bridge warm path, which attaches the shared send budget.
- **Concurrent calls passing the total.** Closed at current source. `SendBudget` reserves each actual attempt atomically, retries included.
- **PostgreSQL counts per backend.** Kept as the documented scope. Each backend is its own process, and the request total and the token cap bind per process. Section 4 makes the pool's combined spend visible. A cluster-wide cap would need PostgreSQL shared memory and `shared_preload_libraries`. It waits until someone needs it.

The cache-off resend of a repeated row stays deliberate.

`specification/recording.md` still lists all three as open. Ticket 0322 corrects that paragraph.

### 6. How it fits ADR 0111

ADR 0111 slice 3 moves the public API, Polars, the C door and the SQL hosts onto `ask_all`. After it, one send stage and one lookup stage count for every surface through one `Counters`. This ADR then adds only the folder in `from_env`, a handful of lines. Building it before slice 3 would pin proofs on batch paths that slice deletes, and slice 2 already changes what a cache answer counts.

## Build order

One ticket, 0322, after ADR 0111 slice 3. Each slice lands green with the full suite, workspace clippy, `policy.py` and a fresh code review.

1. **Rust, C and the file-based SQL extensions.** `from_env` seeds the usage path. The test entry points (`sdlc/scripts/test`, `surfaces` and each surface check) point `XDG_CACHE_HOME` at scratch on Linux. On macOS they point `HOME` at scratch for test processes after pinning `CARGO_HOME` and `RUSTUP_HOME`. A guard fails the run if the real usage folder changed. Proof, all offline against the counted loopback backend, under a scratch usage folder:
   - The command sends one request. A Rust `from_env` engine sends two, one of them a retry after a 503. The C door sends one. The SQLite extension answers one row. Then `thinkthen status --json` reports requests sent equal to the listener's arrival count (5), retries 1, and input and output tokens equal to the sum the loopback replies reported.
   - The same Rust calls again with the cache on reach the listener zero times, and `status` adds cache answers equal to the questions answered from the store.
   - The same calls through `EngineBuilder::new()` leave the usage folder absent.
   - A usage folder with mode `0755` refuses the write. Each call still returns the same result, and the listener sees the same arrivals.
   - Two child processes each send three requests at once. `status` reports six. A forked child that sends one request after its parent sent one leaves a total of two.
2. **PostgreSQL and the pages.** PostgreSQL `check.sh` opens two connections that each send one request. The server user's `status` then reports two. `recording.md`, each SQL extension's README and the changelog say every surface adds to the totals and name PostgreSQL's per-backend cap.

## What this amends

| ADR | Change |
| --- | --- |
| 0034 | Every engine built from the environment persists, not only the command |
| 0111 section 7 | Unchanged counts; they now reach the usage files from every surface |

## What Ian can overturn

1. Libraries and SQL extensions write the usage totals by default, with no opt-out setting. A later `THINKTHEN_USAGE=off` would be one more `from_env` read.
2. `EngineBuilder::new()` writes nothing. Only `from_env` engines persist.
3. PostgreSQL writes to the server user's usage folder. The alternative is no persistence from PostgreSQL.
4. PostgreSQL keeps a per-backend cap. A cluster-wide cap through shared memory waits.
5. Library write failures stay silent.
6. `status` shows one combined total, with no split by surface.
7. The ticket waits for ADR 0111 slice 3.
