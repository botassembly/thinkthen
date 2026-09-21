# Prospective plan for the command, engine, and libraries

Date: 2026-09-20. Status: temporary, prospective. Budget: 4,000 characters.

Write tickets as work begins. The [main plan](plan.md) records landed work. Ian can revise it.

## Destination

One `thinkthen` crate has three layers:

1. **Pure core rules.** Questions, thresholds, wire shapes, digests, and judgments touch no file, socket, process, clock, or environment variable.
2. **One Rust engine.** Sending, retries, scheduling, cancellation, cache, record, replay, and counters are written once.
3. **Thin interfaces.** The command owns arguments, terminal I/O, exit codes, and messages. Language and database shims convert native types and call the engine. Host languages copy no rule, retry, or scheduler.

The public surface stays eight verbs, question setup, and required types. Bash uses the command. Rust, Python, JavaScript/TypeScript, Ruby, R, and C use libraries named `thinkthen`. DuckDB, SQLite, and PostgreSQL extensions follow without holding launch.

## Current state and accepted order

Nineteen how-tos are green. The commands, JSONL output, CSV and TSV input, corrections, duplicate-cache coalescing, flagship page, and transforms are complete. Ticket 0050 declines a `report` verb and defers a read-only transform catalog until after the crate move. Ticket 0051 records the ADR 0017 build review. Job 2's DuckDB interrupt proof inside Python is complete. The order at this boundary is the amendment, Job 3's conformance cases, then the one-crate merge.

1. **Build the engine in four steps.** Step 1 folds to one crate through a private bounded event and error bridge. The command keeps arguments, credentials, framing, its detached reader, output, diagnostics, and exit codes. The engine owns scoped workers, scheduling, transport, recording, and cache locks. The move preserves retries and every command contract. Its gates plant the two core-purity failures, make every CLI dependency optional including `csv-core`, inspect a standalone no-default-features package and graph, keep core doctests, prove library unwind and command-only abort, and prove that no engine worker survives a call. Later steps expose Rust functions, add width, cancellation, fork repair, and fast failure, then move cache and counters into settings.
2. **Prepare the command release.** Complete help, the manual, installation, the agent skill, how-to 18, and the read-only transform catalog.

Stay blocking and at no more than 32 request threads unless the step-three benchmark disproves it. A cancel starts no new request and lets sent requests finish. One record stays one judgment; equal digests send once when they share a cache.

## Libraries and extensions

Build six libraries in this order: Rust, Python, JavaScript/TypeScript for Node, Ruby, R, then C. Each ships a prebuilt artifact, runs the recorded conformance cases, releases the host lock while Rust waits, and tests its thread, interrupt, and fork boundary where present. Bulk calls cross into Rust once and use full engine width. C owns its exported symbols and header; the engine exports none.

Then build DuckDB, SQLite, and PostgreSQL extensions in order. They reuse the engine, cache, recordings, cancellation, and conformance data. SQL functions receive explicit text values and never send an unnamed row. PostgreSQL remains `PARALLEL RESTRICTED` unless evidence supports another marking.

The first public release is 0.1.0 on every surface. Earlier builds use 0.0.N. All shipped surfaces share one current version; a surface added later joins that version. One check holds crate, packages, C header, and extension metadata to it. Registry-name claims remain Ian's outward action.
