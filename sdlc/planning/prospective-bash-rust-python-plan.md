# Completion overview for the command, engine, and libraries

Updated 2026-09-23. Budget: 4,000 characters. The [main plan](plan.md) holds delivery history; the [build queue](build-queue-2026-09-21.md) controls execution order. The [current assessment](mainline-readiness-2026-09-23.md) records checked evidence and the 0074 checklist.

Write bounded tickets as work begins. Ian can revise the order.

## Destination

One `thinkthen` crate has three layers:

1. **Pure core rules.** Questions, thresholds, wire shapes, digests, and judgments touch no file, socket, process, clock, or environment variable.
2. **One Rust engine.** Sending, retries, scheduling, cancellation, cache, record, replay, and counters are written once.
3. **Thin interfaces.** The command owns arguments, terminal I/O, exit codes, and messages. Language and database shims convert native types and call the engine. Host languages copy no rule, retry, or scheduler.

The agreed product has ten judgment functions. Eight are on main. `recognize` and `relate` remain to build in the production engine and command. The first release targets 0.1 across the command, Rust, Python, TypeScript, Ruby, R, C, DuckDB, SQLite, and PostgreSQL. The library branch's stand-in checks do not prove real-engine integration.

## Current state and accepted order

The one-package move, request identity, partial-question results, backend profiles, returned records, durable recording, bounded default cache, status/counts, structured descriptions/evidence, and nineteen green how-tos are landed. Ticket 0072 added fast refused connections. Ticket 0073 added private cancellation. The source package remains `0.0.1`, non-publishable, with no public judgment API.

1. **Complete 0074.** Preserve its worktree. Finish the accepted test split and cancellation acknowledgment, prove output and Unix signal status, fill failure-path tests, obtain independent review, run integrated local gates, then land and push. Its draft and ticket are uncommitted.
2. **Finish private controls.** Process-wide width, whole-call deadlines, fork recovery, and host signal ownership precede the public API.
3. **Build the last two functions.** Reconcile recognition/relation policy and request contracts, then add pure rules, engine/command paths, and shared cases. Preserve recorded baselines. Packing remains optional and requires accepted evidence.
4. **Expose and integrate the library.** Open Rust over stable controls and all ten functions, add C, then merge separately reviewed adapters from `surfaces`. Replace the stand-in and duplicated behavior. Prove bulk calls, request counts, details without another call, host interruption, and owned results.
5. **Prove installation.** Build archives/checksums, the agreed Homebrew/download-script paths, and native packages. Test local artifacts in clean installations. Complete help, manual, agent skill, transform catalog, how-to 18, metadata, and release documentation.

Keep bounded blocking workers. Cancellation starts no attempt whose start check observes it and lets started attempts finish. Equal digests send once when they share a cache.

## Libraries and extensions

The library team owns `surfaces`. Read its latest independent verification before reopening historical findings. Optional Polars ships the Series path in 0.1; its plugin expression waits. Spreadsheets, serve mode, Windows work, and an eleventh function remain outside this tranche.

GitHub Actions remains paused. Run `sdlc/scripts/{install,lint,test,spec}` sequentially under the local-gate ruling. Install may fetch dependencies/advisories; tests use local fixtures and make no paid call.

All shipped surfaces share one version. Publication and registry-name claims retain their existing authority.
