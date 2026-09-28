# 0214 Python batching and run facts build

Status: Candidate on `ticket/0214-python-batching-and-run-facts`; fresh High interpreter/API review and landing remain with the coordinator. No provider, stress, full surface or paid run was made.

## Implementation

The Python extension now returns `Call.value`, `Call.facts` and owned immutable `Call.details` from every asking route, including scalar, eager list, Series, frame, `details()` and module delegates. Rust observations supply question detail; the existing planner supplies per-call facts. Eligible `batch` and `context` controls reach the Rust path, and new eager `choose_many`, `score_many` and `tag_many` use 0230's dynamic-question bridge. The account stays with Arrow/frame conversions and aggregated per-text recognition; a returned post-call conversion error keeps it. A caught worker panic has no invented account.

ADR 0091's approved completion receipt lets `Cancelled` and a signal handler's `SystemExit` raise promptly while the existing worker finishes. The receipt owns one terminal state, supports a retryable timeout, and returns final facts/details after joined success or returned failure. It does not return a value discarded by cancellation or perform another request. Six existing exception kinds, pandas index/name, Polars dtype, and 0209 frame columns remain the value contract.

`libraries/python/src/result.rs` contains the carrier and snapshot conversion. Private `engine/{settings,operations}.rs` and `frame/{names,recognition}.rs` children keep each edited Rust file under 500 nonblank lines; they extract existing settings, planner dispatch and frame recognition instead of copying the scheduler. Rust source rose from 5,302 to exactly 6,278 nonblank lines for the carrier, controls, receipt and frame accounting. Python source plus tests rose from 2,557 to exactly 2,810 for wrappers, signatures and one focused outside-in case file. Those are the new measured ceilings. No dependency or shared Rust runtime file changed.

## Focused proof

- `tests/test_call.py`: six offline listener cases pass for default three-row/one-request packing versus explicit `batch=1` three sends, dynamic label lists, zero-send invalid controls, immutable ordered detail, returned failure facts, pandas/Polars/frame accounts and indexes, prompt receipt/timeouts, `SystemExit`, and post-call reconstruction error.
- Selected stopping, held-request and pandas parity cases: 12 passed together with `test_call.py`. Selected public-surface, door, Arrow release and secrecy cases: six passed. Earlier selected conformance IDs `17-annotate-mixed` and `17-annotate-partial` passed 2/2, and the examples runner passed 14/14 with `.value` migration.
- Three focused `worker::tests` pass. `cargo fmt --all -- --check`, `cargo clippy --locked --offline --all-targets --features probe -- -D warnings`, `git diff --check`, and both Python ratchets pass. `sdlc/scripts/policy.py` exits 1 on unchanged files outside this candidate: `libraries/python/src/arrow/probe/diagnostics_tests.rs` (unsafe placement and early test return), R/Ruby/TypeScript diagnostic tests (early returns), and `crates/thinkthen/src/core/batch/groups.rs` (glob import). Those paths are outside 0214's edits and claims; this candidate does not claim a clean repository-wide policy gate.

The old held cases expected eight requests before one held reply. The landed lazy Rust planner now has one held request even with `batch=1`; the corrected tests pin that exact count, prompt stop with no later send, and all 20 rows after release. `test_throttle_eight_holds_exactly_eight_in_flight` and `test_a_token_stops_a_held_batch_at_the_throttle` were renamed to state their actual behavior, not deleted. The historical per-row pandas fixture uses `batch=1` so it still checks the old body/count purpose. The new default-packing case provides the distinct batching proof. No test-only export, flag or hook was introduced.

## Owner routing

The library tests and examples are migrated. Direct Python consumers under marketing-owned `site/examples/functions/{choose,decide,filter,find,question-file,rank,recognize,relate,score,tag}/` and `site/examples/install/{python,polars}/first-call.py` still expect bare values; this ticket makes no claim that they run against `Call`. The coordinator must route their `.value` migration through a release-priority site issue and keep that issue open until its owner proves the examples. The existing C site issue is separate. B12d–B12f, SQL per-query facts, and any public release decision remain open.
