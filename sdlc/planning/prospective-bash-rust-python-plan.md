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

Nineteen how-tos are green. `annotate`, `tag`, CSV input, and TSV input are built. Tables emit JSONL. The correction pass, duplicate-cache coalescing, `find`, and the flagship triage page are complete. The remaining order is:

1. **Finish `find`.** Done in ticket 0040. The preregistered 100- to 250-unit comparison passed, and how-to 15 is green from two reviewed recordings.
2. **Finish transforms.** Tickets 0042 and 0043 compare scalar runs and each named answer in `annotate` runs. Ticket 0044 polished page 16 and repaired string inputs in `cost.jq`. Ticket 0045 sweeps decisions, choices, and scores. Ticket 0046 averages repeated trials once per case. Ticket 0047 monitors policy actions and reviewer changes. Grouped sweep comes next, then human-label checks.
3. **Rewrite and review ADR 0017.** The experiment team applies Ian's interface, one-crate, engine, performance, fork, and release rulings, after the blocking-engine run in `experiments/211-thinkthen-blocking-engine/` answers its four questions. The answers open that folder's `FINDINGS.md`, and a dated line in `sdlc/issues/2026-09-20-feedback-to-the-experiment-team-after-both-harvests.md` says the report has landed. The build team checks that line, then reviews the rewrite before the merge chooses public modules.
4. **Build the engine in four steps.** Fold to one crate and move machinery without behavior changes; expose small Rust functions and shared replay cases; add process-wide width, cancellation, fork repair, and fast failure; then make cache locks and counters engine settings. The command remains the first caller. Every gate stays green after each step.
5. **Prepare the command release.** Complete help, the manual, installation, the agent skill, and how-to 18.

Stay with the blocking client and at most 32 request threads unless the step-three benchmark disproves it. A cancel starts no new request and lets sent requests finish. One record remains one logical judgment; equal digests send once when they share a cache. ADR review must land before the one-crate merge.

## Libraries and extensions

Build six libraries in this order: Rust, Python, JavaScript/TypeScript for Node, Ruby, R, then C. Each ships a prebuilt artifact, runs the recorded conformance cases, releases the host lock while Rust waits, and tests its thread, interrupt, and fork boundary where present. Bulk calls cross into Rust once and use full engine width. C owns its exported symbols and header; the engine exports none.

Then build DuckDB, SQLite, and PostgreSQL extensions in order. They reuse the engine, cache, recordings, cancellation, and conformance data. SQL functions receive explicit text values and never send an unnamed row. PostgreSQL remains `PARALLEL RESTRICTED` unless evidence supports another marking.

The first public release is 0.1.0 on every surface. Earlier builds use 0.0.N. All shipped surfaces share one current version; a surface added later joins that version. One check holds crate, packages, C header, and extension metadata to it. Registry-name claims remain Ian's outward action.
