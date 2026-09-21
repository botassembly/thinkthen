# Build review of ADR 0017 before the one-crate move

Date: 2026-09-21

Status: open

## Verdict

Revise ADR 0017 section 8 step 1 before writing the merge ticket. A behavior-preserving one-crate move is feasible, but the five named pieces do not move whole and the present gates do not prove the new boundary.

This review read current main at `85b9ba9` before ticket 0050 landed. It changed nothing and ran no test, network call, or paid call.

## Findings

### 1. Extract an engine boundary instead of moving five files whole

The earlier issue names `http.rs`, `schedule.rs`, `annotate_schedule.rs`, `recorder.rs`, and the request loop in `asking.rs`. Those pieces still depend on command arguments, environment and key reading, command output, exit codes, `Failure`, and `annotate::Judging`. `recorder.rs` also depends on `cache_lock.rs`.

The merge ticket must define typed engine inputs, typed results, and structured engine errors first. The command keeps argument parsing, credential lookup, input framing, output, and exit-code mapping. The engine takes the resolved values. Move `cache_lock.rs` with recording.

Evidence: `crates/thinkthen/src/asking.rs:482`, `crates/thinkthen/src/schedule.rs:81`, `crates/thinkthen/src/annotate_schedule.rs:11`, and `crates/thinkthen/src/recorder.rs`.

### 2. Preserve core purity inside one crate

The current check assumes two crates. `sdlc/scripts/policy.py:198` requires both, and `crates/thinkthen-core/src/lib.rs:7` applies crate-level rules that a source module cannot inherit unchanged. One crate also makes a reverse call from `core` into `engine` possible.

The merge needs a mechanical module-level purity check and a dependency-direction check. The accepted ban tables stay intact. A focused self-test must prove that a prohibited core call and a core-to-engine dependency fail the check.

### 3. Preserve command failures and retries

Six public error kinds do not carry enough detail to reproduce today's command messages. Recording misses, conflicts, malformed entries, transport failures, and ordered-stream stops have distinct diagnostics and stop counts. The engine error needs structured causes and stop metadata. The command maps those values to its current `Failure` messages and exit codes.

Step 1 must preserve today's retry behavior. `crates/thinkthen/src/http.rs:188` currently retries every transport failure. ADR 0017 section 3 changes refused connections later. That behavior change does not belong in the move.

Evidence: `crates/thinkthen/src/failure.rs:219` and `crates/thinkthen/src/schedule.rs:400`.

### 4. Test the package boundary

The current gates build all default features. They do not prove that a library build omits the command parser or that the package stands without the sibling crate.

The merge must make the command parser optional, require the `cli` feature for the binary, check a library build with default features off, and verify the package without `thinkthen-core` beside it. Core stays private and its doctests keep running. Repository instructions and Rust standards change to describe the new layout. Command behavior pages need no change.

Evidence: `sdlc/scripts/lint:16`, `sdlc/scripts/test:8`, and experiment 205 `FINDINGS.md:18`.

### 5. Keep blocking input readers out of the engine

Both schedulers detach an input-reader thread. A reader blocked in its iterator can survive the call. A short-lived command process hides that lifetime; a library host cannot.

Keep terminal and stream reading in the command, or join and bound every engine feeder. Add a test inside a process that stays alive after the call. ADR 0017's zero-thread promise must be proved at that boundary.

Evidence: `crates/thinkthen/src/schedule.rs:203` and `crates/thinkthen/src/annotate_schedule.rs:147`.

## Required order

1. Amend ADR 0017 section 8 step 1 with these boundaries and proofs.
2. Land Job 3's shared conformance cases.
3. Land Job 2's DuckDB interrupt proof inside Python.
4. Write and build the one-crate merge ticket.

Ian can overturn the order or any proposed boundary. The review recommends keeping the order because the cases and interrupt proof become the fixed evidence for the move.
