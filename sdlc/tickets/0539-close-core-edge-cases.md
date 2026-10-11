# 0539: Close the remaining core edge cases

Status: OPEN.

Milestone: 0.2

## Outcome

No public Rust call drops a usage write failure. The CLI reader behaves the same as the library reader, or a test states each difference. Plain rank never returns an internal defect for a released unit, and a failed reader lock reports an error.

## Evidence

- Starts from: gaps 5 and 6 of [the 0.2 closure review](../records/2026-10-11-0-2-closure-review.md).
- Keeps: the typed usage status calls from 0468, the 0512 fixes for truncation, annotate admission and rank with details, and the frozen 0.1 C exports.
- Changes:
  - Usage: delete `Engine::finish_usage()` (`crates/thinkthen/src/public/engine.rs:234`) or make it return the failure. Delete the C JSON usage-status routes (`libraries/c/src/call.rs:92`), which are not 0.1 exports, and map them to the typed exports in the C README.
  - Reader: each `cli_reader` branch at `public/pull.rs:186,303,378` and `public/complete/recognize/streaming.rs:78` is deleted, or kept with one CLI test that states the behavior. `native_reader.rs:55,71` returns an error on a poisoned lock.
  - Rank: plain rank at `cli/asking/native.rs:427,443` keeps the original it needs, or a test proves the released unit can never be a result.
- Proof: CLI and Rust API tests for each kept branch, for the usage failure and for plain rank. Routine checks pass.
- Defers: nothing.
