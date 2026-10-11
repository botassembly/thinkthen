# 0338: Test locks, long waits and test binaries

Status: COMPLETE.

Opened as: 2026-10-11. Follows 0304 slice 3d, which settles the send-limit statics, and 0335 slice 2, whose merge rows edit `tests/public_batches/`. Plan: `sdlc/planning/cleanup-2026-09-30.md`, "Next after the running work", the cleanup ticket from record 0305. Takes over 0335 slice 3.

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

- The statics stayed after 3d as one cell, `PROCESS_LIMITS` in `engine/limits.rs`, so both `SERIAL` locks stay and now name it. `public_cap` and `public_estimated` read the process send totals, so they stay their own binaries beside `public_controls` and `public_batches`.
- Binaries: 44 test executables became 11. The crate's 36 integration binaries became 6 (`backend` for the command line, `library`, and the four above), five Polars binaries became one `polars`, and the loopback backend's two became one `loopback`. Test count: 1323 listed and 1299 run before; 1326 and 1302 after, the same cases plus three from two splits. Nextest listed both sets by leaf name and found them equal before the splits.
- `sdlc/scripts/test`: 57 s wall and 30.9 s of nextest before (load 13 to 16); 38 s and 17.6 s after (load 16 to 17). 20 nextest runs of `public_controls` and `public_batches` passed. `cargo test --workspace`, `package`, `spec`, the Polars check and the C door tests passed.
- After rebasing over 0304 slice 5, which removed tests, `sdlc/scripts/test` ran 1273 tests in 54 s wall and 21.8 s of nextest at load 11 to 12. One run at load 22 failed `ca_bundle::genuine_tls_trust…`: its openssl responder exited before binding a port the test had reserved and freed. It passed 20 runs of its own and the rerun of the whole suite; it is a load race older than this ticket.
- The biggest cost was not waits. An unoptimized `serde_json` and `sha2` made the command's CPU-bound tests run 5 to 25 s; optimizing just those two in the test profile took the 255-line relate plan from 29 s to 1 s. Alone at load 19, two spawn-heavy tests still took 5.4 s; split by input, their parts take 2.4 to 3.4 s at load 13, so no routine test now runs over 5 s.
- The long waits: a held deadline reply waited out a 2 s release it had already consumed (4.6 s to under 1 s); the width drain released one answer per 150 ms quiet spell and now releases all held answers at once (3.9 s to 1.3 s).
- Merging exposed two `cargo test` races that nextest hides, and lint's `children` check refused the Polars rerun child until it built a cleared environment with the fake key named. Shared fixture files staged under a process id alone collide between threads, and a closed listener stays reachable while another test's child sits between fork and exec. Nextest runs each test alone, so only `cargo test` and `package` see them.
- Merged Polars tests that set a throttle rerun themselves alone in a fresh process, so one binary keeps the one-throttle-per-process rule under `cargo test`.
