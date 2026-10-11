# 0549: Make the release campaign valid and fast

Status: OPEN.

Milestone: 0.2

Reviews: revision f23c075965dc12a966847fee80c13bfab3b29385, accept

Reviews: revision 772ae3e6b6c9692b9dabaa8421c480696324eaeb, reject

Reviews: revision 8c09ffdab0e202e680e3b9ca06342c5effc6e04d, accept

Reviews: revision c68d38b815f5dd54cd5fd4045f44753f37ac463e, reject

Reviews: revision 95fb7fe8811154bfbd00b8a98a3655d99c85eb0d, accept

Reviews: revision e90429a, accept

Reviews: revision e90429a, accept

Reviews: revision cca27bf, reject

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
- Changes: the campaign runner, in these steps.
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
- 2026-10-11 next: The reviewed corrected campaign at 8c09ffdab is running in the background with retained ownership and logs; its owner will await exit before reading results.
- 2026-10-11 next: The corrected campaign at 8c09ffdab failed after 5569 seconds; retained logs show grouped consumer budgets, duplicate setup work and three specific fixture or privacy failures that require focused repairs before another run.
- 2026-10-11 next: Family and database focused repairs passed on c68d38b815, including actual installed setup and retained unique checks; a fresh code reviewer is checking the slice before another full campaign.
- 2026-10-11 next: Fresh review found unstable DuckDB full-build flags and timeout escalation that can leave descendants alive; both are being repaired with focused checks before another full campaign.
- 2026-10-11 next: The reviewed source passed its preamble but the retry stopped before consumers on a missing parity destination; retained logs precede a new owned canonical run with the actual directory created.
- 2026-10-11 next: The completed canonical run passed 27 applicable consumers below the limit but Flutter timed out before asserting cases; a fresh developer is diagnosing its prepared execution, with all logs retained and Objective-C awaiting macOS.
- 2026-10-11 next: The compiled Flutter application passed all 255 cases in 341.05 seconds including preparation and passed fresh review; the canonical full campaign runs in the background with its owner awaiting exit, followed by official routine test and lint on the unchanged commit.
- 2026-10-11 next: The canonical compiled-Flutter campaign completed with every applicable consumer below the limit and Flutter passing all 255 cases, but R failed its installed startup check after passing parity; retained logs feed a fresh scoped repair and no broader gates ran.
- 2026-10-11 next: The unchanged installed R startup test passed normally, failed under deferred subreaper ownership with an adopted zombie, and passed with active adopted-child reaping; a developer is checking the installed R suite before restarting the unchanged canonical source with corrected ownership.
- 2026-10-11 next: The corrected process owner passed unchanged installed R checks and fresh code review, and a new owner is restarting the canonical campaign on unchanged e90429a0f with separate retained logs before official test and lint.
- 2026-10-11 next: The completed e904 campaign retained 7130 passing cases and two buffered-output fixture failures, and the repair now covers the additional custom cancellation readers found by fresh review before another full run.
- 2026-10-11 next: The a920 campaign exited before parity because Ada fixture Python measured 551 against its unchanged 550 ceiling, so the developer is removing redundant setup and checking every affected binding cap before another run.
- 2026-10-11 next: Reviewed fixture repair 2c5b0105 restores Ada to 550 lines with every affected cap passing, and its canonical campaign is owned in the background with retained logs and no active polling before exact-revision closure gates.
- 2026-10-11 next: Canonical 2c5b0105 exited 1 without signals after all local parity consumers exited 0, because release smoke could not find the Python release wheel; retain the completed logs, repair package preparation in an isolated lane and review it before any new campaign or closure gates.
- 2026-10-11 next: Fresh review accepted the corrected Python retained-wheel preparation and stale-wheel cleanup, its evidence record is committed under sdlc/records, and one canonical campaign now owns frozen ad4b49b4a in the background without active polling before exact-revision closure gates.
