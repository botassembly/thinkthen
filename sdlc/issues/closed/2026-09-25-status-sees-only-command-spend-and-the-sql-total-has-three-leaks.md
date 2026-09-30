# status sees only command spend, and the SQL request total has three leaks

Status: closed 2026-09-30. Fixed by ticket 0322 slice 1 (`c893b2e15`) under ADR 0113: every library and SQL extension adds to the usage totals that `status` reads. The cache-answer proof stays with 0322 slice 2. A PostgreSQL pool still has one request total per connection, the documented scope ADR 0113 keeps.

Found 2026-09-25 while drafting the talk's cost slide. Ian asked for these gaps to be filed. Each fact below names its source.

## What happened at filing, with current corrections

1. **`status` never sees library, SQL, or data-frame spend.** The command's usage store is the only writer of `thinkthen-usage` ([recording.md](../../../specification/recording.md), "Usage lives at"). The public engine builds its counters with `Counters::new(None)` (`crates/thinkthen/src/public/settings.rs:246`), so they live in memory and die with the process. Every SQL extension and the Rust Polars surface go through that engine. A user who spends through DuckDB or a data frame sees nothing in `thinkthen status`.
2. **DuckDB warm was outside `thinkthen_max_requests_total` at filing.** The old aggregate could not read session settings (ticket 0110 item 17). Current C++ settings pass the total into the bridge warm path, which attaches the shared send budget before actual attempts. This leak is fixed; the original observation stays as history.
3. **PostgreSQL counts the total per backend process.** Each connection is its own backend (`sdlc/records/0111-build-postgresql-surface.md`, "The request total"). A pool of 20 connections can spend up to 20 times a per-backend total. This remains the documented scope, not an account-wide cap.
4. **Concurrent calls could pass the total at filing.** The old read-once calculation could overshoot (ticket 0110 item 17). Current `SendBudget` reserves actual attempts atomically, including retries, across the relevant process/backend paths; this overshoot is fixed within that scope.
5. **With the cache off, a repeated row can send again.** A SQL row and a data-frame row are each an engine call. A volatile function can re-run in WHERE, SELECT, and joins. This is deliberate cache-off behavior, not an escape from the send budget.

The command's own missing cap is already recorded in `closed/2026-09-21-where-a-user-could-lose-trust-a-first-list.md`, item 1. Ian said then he may not care about it for the command. This page does not reopen it.

## Why it matters

The talk's unqualified `status` claim held for the command only; ticket 0126 qualified the pages. At current source, durable non-command spend remains the open visibility issue. SQL totals bind actual sends within their documented process/backend scope, while a PostgreSQL pool has several such scopes.

## Options

1. **Persist library counts to the same usage store.** The engine opens the store the command uses, with the same best-effort rule. `status` then covers every surface. This widens what the library writes, which AGENTS.md limits, so it needs an ADR.
2. **Close the filing-state leaks in place.** Warm now reads the session total through its bridge, and atomic send reservation prevents the concurrent overshoot within each process/backend. PostgreSQL's pool multiple remains a documented scope rather than an account-wide ceiling.
3. **Document only.** The talk and each surface's README say `status` counts the command, and name the per-process totals and their limits.

The recommendation is 3 now, so the talk and the pages stay true, and 1 as a ticket after the surface batch lands. Ian can overturn this.

## Factual preparation after typed C facts, 2026-09-28

At main `23371cc9`, `cli/edge.rs` builds `Counters::new(usage_path)` and `cli/status.rs` reads that persisted store. `public/settings.rs` builds native engines with `Counters::new(None)`, inherited by frame and SQL hosts, so their volatile counters do not enter `status`. `public/options/budget.rs` reserves each actual send atomically; SQLite `settings.rs`/worker, PostgreSQL `call.rs`, and DuckDB `src/engines.rs` attach it. DuckDB C++ `scalar_settings.cpp` passes `thinkthen_max_requests_total` to bridge `warm/ffi.rs::options_for`. This source correction does not establish a cross-process or account-wide cap. Option 1 needs a reviewed durability/write-boundary decision, including deduplication, best-effort failures, fork/process lifetimes and the old-reader count-only shape. The smallest later proof uses isolated XDG state and a CLI plus native/frame/SQL call before `status`; reuse retained SQL send proofs instead of repeating them. See `sdlc/records/2026-09-28-accounting-after-c-facts.md`.
