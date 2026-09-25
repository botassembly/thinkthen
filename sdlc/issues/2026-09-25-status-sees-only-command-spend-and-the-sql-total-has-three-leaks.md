# status sees only command spend, and the SQL request total has three leaks

Status: Open

Found 2026-09-25 while drafting the talk's cost slide. Ian asked for these gaps to be filed. Each fact below names its source.

## What happens

1. **`status` never sees library, SQL, or data-frame spend.** The command's usage store is the only writer of `thinkthen-usage` ([recording.md](../../specification/recording.md), "Usage lives at"). The public engine builds its counters with `Counters::new(None)` (`crates/thinkthen/src/public/settings.rs:246`), so they live in memory and die with the process. Every SQL extension and the Rust Polars surface go through that engine. A user who spends through DuckDB or a data frame sees nothing in `thinkthen status`.
2. **DuckDB `thinkthen_warm` sits outside `thinkthen_max_requests_total`.** The warm aggregate cannot read session settings, so the total does not bind it (ticket 0110 item 17, `databases/duckdb/NOTES.md` on `ticket/0110-port-duckdb-surface`). Warm is the call that sends the most requests at once.
3. **PostgreSQL counts the total per backend process.** Each connection is its own backend (`sdlc/records/0111-build-postgresql-surface.md` on `ticket/0111-port-postgresql-surface`, "The request total"). A pool of 20 connections spends up to 20 times the total.
4. **Concurrent calls pass the total.** Each call reads what remains once, so one call per thread in flight, plus retries, can overshoot (ticket 0110 item 17).
5. **With the cache off, a repeated row bills again.** A SQL row and a data-frame row are each an engine call. A volatile function re-runs in WHERE, SELECT, and joins. The cache is the only thing that makes a repeat free, and `--no-cache` or its setting turns that off.

The command's own missing cap is already recorded in `closed/2026-09-21-where-a-user-could-lose-trust-a-first-list.md`, item 1. Ian said then he may not care about it for the command. This page does not reopen it.

## Why it matters

The talk says `status` tracks what you spend. That holds for the command only. A database user has the highest request counts and the weakest view of them. A reader who sets the total expects it to bound the process, and three paths walk past it.

## Options

1. **Persist library counts to the same usage store.** The engine opens the store the command uses, with the same best-effort rule. `status` then covers every surface. This widens what the library writes, which AGENTS.md limits, so it needs an ADR.
2. **Close the leaks in place.** Warm reads the total from the environment engine's setting. PostgreSQL documents the pool multiple beside the setting. The overshoot bound goes into each surface's README in one sentence.
3. **Document only.** The talk and each surface's README say `status` counts the command, and name the per-process totals and their limits.

The recommendation is 3 now, so the talk and the pages stay true, and 1 as a ticket after the surface batch lands. Ian can overturn this.
