# 0484: Split the release tests out and remove test sediment

Status: OPEN.

Milestone: 0.2

Reviews: revision b9027d08b, accept

Reviews: revision 085aac3ab, reject

Reviews: revision 610ac150f, accept

Reviews: revision 5df81fd366c77332f10891e3f443caf212a30a72, accept

Reviews: revision b431cfba212c270a90deafe29a04d4078e4edbf1, accept

## Outcome

The routine suite holds only outside-in behavior tests through the public Rust API, the CLI and each installed library, and no routine test runs longer than 5 seconds. Large-input boundary cases and nested package builds run only in the release suite. In-source unit tests that an outside-in test already covers are gone.

## Evidence

- Starts from: Ian's ruling [two test suites](../decisions/2026-10-09-two-test-suites.md) and the architecture/test-sediment PM message of 2026-10-08, ask 1. On 2026-10-09 the routine suite held cases of 16 to 121 seconds, and Rust test code was about 1.3 times Rust product code, with about 510 tests in 111 in-source test modules.
- Keeps: Parser negatives, secrecy, cancellation and concurrency, memory safety, threshold edges and cache or version compatibility vectors that catch real behavior drift. Do not delete a digest vector merely because it pins a string. Every large-input boundary keeps its test; it moves to the release suite. Never delete the only test of a behavior.
- Changes: Two slices, in order.
  - Slice A, now: mark the release-only cases with one mechanism that `sdlc/scripts/test-full-cases --run` runs and `sdlc/scripts/test` skips. They include the multi-megabyte cases in `crates/thinkthen/tests/images/shared_admission.rs`, `crates/thinkthen/tests/images/admission.rs` and `crates/thinkthen/tests/images/local.rs`, the 16 MiB record cases in the files under `crates/thinkthen/tests/**` that build a 16 MiB value (fourteen on 2026-10-09, including `crates/thinkthen/tests/backend/limits.rs`, `crates/thinkthen/tests/backend/source_limits.rs` and `crates/thinkthen/tests/request_contract/projections.rs`), and the nested Python build in `conformance/consumer/consumer/tests/public/native.rs`. Keep one routine case per limit when it runs under the timeout. `test-full-cases --run` runs `sdlc/scripts/test` first, so the release run must also include the cases the routine run skips. Add `.config/nextest.toml` with a slow-test timeout that fails a routine test over 5 seconds. Claim `crates/thinkthen/tests/**`, `conformance/consumer/consumer/tests/**`, `.config/nextest.toml`, `sdlc/scripts/test` and `sdlc/scripts/test-full-cases`.
  - Slice B, after 0512 lands: for each in-source test module under `crates/thinkthen/src/**`, delete tests that an outside-in test in `crates/thinkthen/tests/**` or a library check already covers. Add a small outside-in case first when none exists. Work module by module and lower `sdlc/ratchet.json` after each removal. Consolidate duplicate assertions without hiding distinct edge cases. Apply the same pass to `libraries/**/tests/**` for tests that repeat a claim the shared cases make. Record the result in `sdlc/records/0484-prune-proven-unit-sediment.md`.
- Proof: The routine `sdlc/scripts/test` passes with no case over the timeout. `test-full-cases --run` runs the release cases once at the end of slice A. One reviewer checks each deletion against the retained behavior. The landing record gives test and source line counts before and after, with the counting method. No per-test receipts and no counting gate.
- Defers: Load and timing stay in `test-stress --run` unchanged. Per-language parity suites stay with each migration ticket.

## Progress

- 2026-10-10 landed d05def2bb; next: Routine Clippy now passes after row-count setup errors use Result. The existing large-case separation is landed; finish bounded removal of proven redundant source tests after active core changes settle.
