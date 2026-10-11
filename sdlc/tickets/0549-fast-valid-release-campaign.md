# 0549: Make the release campaign valid and fast

Status: OPEN.

Milestone: 0.2

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 772ae3e6b6c9692b9dabaa8421c480696324eaeb, reject

Reviews: revision 8c09ffdab0e202e680e3b9ca06342c5effc6e04d, accept

## Outcome

One `test-full-cases --run` on the Beelink runs every consumer, and no consumer fails for setup reasons. Each consumer finishes in under 900 seconds. Each consumer's duration appears in the summary.

## Evidence

- Starts from: the [2026-10-11 adversarial review](../records/2026-10-11-0-2-adversarial-review.md), test section.
  - In the rc.11 run, the time limit at `conformance/parity.py:444` killed Ada at 30:00 after 254 cases, so every Ada cell failed.
  - Eight consumer groups never ran. They are python, typescript, ruby, r, dart, sqlite, duckdb and postgresql.
    - The rc.11 command left `THINKTHEN_TOOLCHAINS`, `UV_CACHE_DIR`, `PUB_CACHE` and `R_LIBS_USER` unset.
    - `consumer_environment` (`parity.py:246-272`) passes those variables only when they are set. Otherwise each consumer looks in its empty scratch home.
    - The logs name what was missing: the Ruby 3.4.11 prefix, jsonlite 2.0.0, the SQLite amalgamation, the DuckDB v1.5.5 toolchain and the PostgreSQL server package. The python, typescript and dart logs are empty.
  - The other consumers took 87 minutes in series.
- Keeps: every required cell, the fake backend, the heavy lock and the per-consumer logs.
- Changes:
  - Default those variables to this machine's real toolchain and cache locations, so a bare `test-full-cases --run` finds them.
  - Run consumers in parallel, four at a time. Rust and rust-polars share one lane. The campaign takes the heavy lock once, and children inherit `THINKTHEN_HEAVY_LOCK_HELD`, so consumers do not queue on the lock.
  - Build the Ada package and one Ada consumer program once per campaign, and run every case from that program. Today each case compiles and statically links a new program (`libraries/ada/checks/full_case.py:53-59`), which costs about 5 of every 7 seconds.
  - Run each JVM language's cases in one JVM, and run the three JVM languages in parallel.
  - Raise `CARGO_BUILD_JOBS` for parity.
  - Drop the routine suite rerun and the duplicate Rust public-case pass from the `test-full-cases` preamble. The installed `rust` consumer keeps those cases.
  - Print each consumer's duration in the summary.
  - Remove the harness name from the lock path in `libraries/ada/check.sh:10`.
- Proof: one full run on the Beelink passes every consumer. Its summary shows the durations.
- Defers: 0555 slims the required case list.

## Progress

- 2026-10-11 started
- 2026-10-11 next: The completed campaign at 60ba71846 failed after 4785 seconds with retained logs; source optimization and setup corrections need focused proof and fresh code review before the next full run.
- 2026-10-11 next: Fresh review found that surfaces sanitation drops campaign compiler settings, so the developer is repairing that boundary and proving propagation before a full rerun.
