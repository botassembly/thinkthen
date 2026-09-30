# 0338: Test locks, long waits and test binaries

Status: in progress. Follows 0304 slice 3d, which settles the send-limit statics, and 0335 slice 2, whose merge rows edit `tests/public_batches/`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, "Next after the running work", the cleanup ticket from record 0305. Takes over 0335 slice 3.

## Outcome

No routine test waits on a fixed wall-clock limit longer than it needs. The crate's integration tests build as a few binaries, not 35, plus one Polars binary and one conformance backend binary. The shared test locks in `public_controls` and `public_batches` guard only process state that the engine still holds, and a comment names that state.

## Evidence

- Starts from: record 0305's cut list and "Consolidating binaries" (the `SERIAL` locks cost 17 s under `cargo test`; 37 integration binaries then, 35 now in `crates/thinkthen/tests/*.rs`, plus five `polars_*` feature binaries and two in `conformance/backend/tests/`); ticket 0317's Defers; 0335 slice 3; `sdlc/planning/after-slice-3-prep.md` section 4, item 6. Ticket 0340 took five load flakes: it replaced the 400 ms sleep in `public_controls` `a_stop_during_a_batch…` (now `public_controls/stopped.rs`) and the 2 s spin in `backend/scheduling.rs` with events, and left every lock alone. Ticket 0317 already split the secrecy sweep into 17 `secrecy::no_leak_on_*` tests, and 0317 and 0335 slice 1 deleted the prose-only tests, so this ticket does neither.
- Keeps: every test case and every retained regression the inventory lists; one throttle, pacer, 429 gate and request total per process, the coordinator default for 0304 slice 3d under Ian's 0077 ruling; lesson 5's write-and-rename for shared files; `sdlc/scripts/test` under nextest, where each test runs in its own process.
- Changes: three steps.
  - Recheck on main after 3d which process-wide statics remain (`engine/mod.rs` `PROCESS_WIDTH`, `engine/backoff.rs` `PROCESS_GATES`, `engine/budget.rs` `BUDGET` today). Under nextest the locks cost the gate nothing. They matter under `cargo test`, in `sdlc/scripts/package`, and in any merged binary. If the statics stay, the locks stay and name the state they guard. If Ian overturns the 3d default and the statics go, the locks go with them.
  - Cut the long waits in `engine::deadline_tests` (17 tests, 14.7 s summed) and `cli::schedule::width_tests` (3.8 s) with settable limits or signals in place of sleeps, or move a case to `test-stress`.
  - Merge the integration binaries along record 0305's lines: a command-line binary and a library binary; one Polars binary; one conformance backend binary. Tests that touch process state keep a lock inside the merged binary or stay in their own.
  - Added in the build: the test profile optimizes `serde_json` and `sha2` (`[profile.dev.package]` in the workspace manifest). Unoptimized, they spent 29 of 30 s planning a 255-line relation, so the CPU-bound command tests ran 5 to 25 s. Two spawn-heavy tests split by input so the runner spreads them. Three shared-file writers stage under a process-and-thread name.
- Proof: `sdlc/scripts/test` wall time and test count before and after, with the 1-minute load; nextest's binary count; the same test count before and after the merge; `cargo test --workspace` and `sdlc/scripts/package` passing; 20 repeated nextest runs of each file whose lock changed, with no failure; no routine test over 5 s.
- Defers: removing the process-wide statics themselves, which waits for an Ian overturn of the 3d default; `transforms/sweep/test.sh`'s 5.1 s; the per-surface matrices, which move when the release suite runs bindings.

## What the build taught us

Pending.
