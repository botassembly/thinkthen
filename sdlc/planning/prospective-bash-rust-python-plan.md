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

Eighteen how-tos are green. `annotate`, `tag`, CSV input, and TSV input are built. Tables emit JSONL. The correction pass is complete. The remaining order is:

1. **Coalesce duplicate cache misses.** One bounded per-digest lock stops equal concurrent requests from paying twice or recording conflicting replies.
2. **Build `find`.** Repeat its comparison with `rank --top 1` on 100- to 250-line documents. Build the command and how-to 15 if the result holds.
3. **Finish page 16 and the transforms.** Land the flagship triage policy, then comparison, sweep, monitors, grouped sweep, and the check against human labels.
4. **Rewrite and review ADR 0017.** The experiment team applies Ian's interface, one-crate, engine, performance, fork, and release rulings. The build team reviews it before the merge chooses public modules.
5. **Build the engine in four steps.** Fold to one crate and move machinery without behavior changes; expose small Rust functions and shared replay cases; add process-wide width, cancellation, fork repair, and fast failure; then make cache locks and counters engine settings. The command remains the first caller. Every gate stays green after each step.
6. **Prepare the command release.** Complete help, the manual, installation, the agent skill, and how-to 18.

Stay with the blocking client and at most 32 request threads unless the step-three benchmark disproves it. A cancel starts no new request and lets sent requests finish. One record remains one request. Cache locking must land before `find`; ADR review must land before the one-crate merge.

## Libraries and extensions

Build six libraries in this order: Rust, Python, JavaScript/TypeScript for Node, Ruby, R, then C. Each ships a prebuilt artifact, runs the recorded conformance cases, releases the host lock while Rust waits, and tests its thread, interrupt, and fork boundary where present. Bulk calls cross into Rust once and use full engine width. C owns its exported symbols and header; the engine exports none.

Then build DuckDB, SQLite, and PostgreSQL extensions in order. They reuse the engine, cache, recordings, cancellation, and conformance data. SQL functions receive explicit text values and never send an unnamed row. PostgreSQL remains `PARALLEL RESTRICTED` unless evidence supports another marking.

The first public release is 0.1.0 on every surface. Earlier builds use 0.0.N. All shipped surfaces share one current version; a surface added later joins that version. One check holds crate, packages, C header, and extension metadata to it. Registry-name claims remain Ian's outward action.
