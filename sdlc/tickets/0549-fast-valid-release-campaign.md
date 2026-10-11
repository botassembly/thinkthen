# 0549: Make the release campaign valid and fast

Status: OPEN.

Milestone: 0.2

## Outcome

One `test-full-cases --run` on the Beelink runs every consumer, and no consumer fails for setup reasons. The campaign finishes well inside the per-consumer time limit. Each consumer's duration appears in the summary.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), test section.
  - In the rc.11 run, the time limit at `conformance/parity.py:444` killed Ada at 30:00 after 254 cases, so every Ada cell failed.
  - Nine consumers never ran because `consumer_environment` (`parity.py:246`) drops the toolchain and cache paths. Those consumers are python, typescript, ruby, r, dart, sqlite, duckdb and postgresql.
  - The other consumers took 87 minutes in series.
- Keeps: every required cell, the fake backend, the heavy lock and the per-consumer logs.
- Changes:
  - Pass the toolchain and cache locations through to each consumer's scratch home.
  - Run consumers in parallel, four at a time. Rust and rust-polars share one lane.
  - Build the Ada consumer once and run all its cases from that build. Today each case compiles and statically links a new program (`libraries/ada/checks/full_case.py:53-59`), which costs about 5 of every 7 seconds. Build the Ada package once per campaign.
  - Run each JVM language's cases in one JVM, and run the three JVM languages in parallel.
  - Raise `CARGO_BUILD_JOBS` for parity.
  - Drop the routine suite rerun and the duplicate Rust public-case pass from the `test-full-cases` preamble. The installed `rust` consumer keeps those cases.
  - Print each consumer's duration in the summary.
  - Remove the harness name from the lock path in `libraries/ada/check.sh:10`.
- Proof: one full run on the Beelink passes every consumer. Its summary shows the durations.
- Defers: slimming the required case list waits on Ian's ruling, named in the review record.
