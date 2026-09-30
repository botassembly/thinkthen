# 0317: Faster, leaner tests

Status: ready. Starts after ADR 0111 slice 1 lands. Plan: `sdlc/planning/cleanup-2026-09-30.md`, step 5.

## Outcome

The landing suite runs in under 30 seconds of test time on a warm build, with no routine test over 2 seconds. Duplicate and wording-only tests are gone. Behavior coverage stays.

## Evidence

- Starts from: record 0305 (1,262 tests, 39 s under nextest, cut list), ticket 0303's deletions, and tonight's landings.
- Keeps: parser, secrecy, cancellation, cache-miss, invalid-input and conflict regressions; command-line exact-output tests; the conformance cases.
- Changes: split the 518-spawn secrecy sweep; keep one 16 MiB edge test instead of four; merge duplicate batch-limit, audit-refusal and retry tests; delete exact-sentence tests below the command line that the command line or `conformance/cases.json` already pins; cut fixed sleeps and long timeouts to the shortest that still proves the behavior.
- Proof: nextest time and count before and after, under stated load; each deletion names the test that still pins its behavior.
- Defers: the shared test locks in `public_controls` and `public_batches`, which go with the process-wide statics after ADR 0111 slice 3; merging test binaries; surface package tests replaying `conformance/cases.json`.
