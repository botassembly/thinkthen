# 0222 Linux DuckDB batching build

Status: Linux x86-64 source implementation and focused Rust checks complete; installed package validation is in progress. The branch merged main `79244123` before the native package build. This record does not claim Linux ARM64, Intel macOS, actual macOS 15 or the full 0128 rehearsal.

## Shared source and focused checks

The shared one-split facade now calls the existing planner's `halves_with_questions` once after an eligible refusal. A stopping B12a caller examines the left result before the right send; native recovery continues after recoverable left failures. Fatal errors leave the worker as outer errors, so `workers::ordered` halts admission while joining work already in flight. The native carrier uses the existing planner, ordered worker and profile-aware member constructor. The existing detail writer takes optional record input: SQL scalar details omit it while retaining the saved profile, question digest, actual request digests and member batch metadata.

The two new outside-in public batch regressions pass: a refused original with a failed left logical member or failed left request contrasts stopping `(2 sends, 1 or 0 finished records)` with native `(3 sends, 4 returned records)` and pins full request bodies and digests; an invalid local member preserves the already closed prefix and later good member. Strict all-target/all-feature Clippy passes for `thinkthen` and `thinkthen-duckdb-bridge`; focused Rust tests pass 2/2; DuckDB source checks and Python compilation pass. The first interrupted Clippy run did **not** pass: it reported the nested reconstruction in `public/native_batch.rs`; subsequent runs exposed split type complexity, oversized test placement and nested bridge decoding. Those were corrected before the passing reruns.

The old `splits.rs` test file exceeded the 500-line policy after the new regressions; their cases moved intact to sibling `native.rs`, and its existing digest helper is shared. No prior test was deleted. The shared Rust total is 92,581 versus merged-main 91,819, a 762-line increase across the native carrier, shared split policy and outside-in tests; the DuckDB Rust total is 6,168 versus 5,916 (+252), C++ is 1,526 versus 1,478 (+48), and Python is 3,101 versus 3,016 (+85). Existing `core::Batcher`, `workers::ordered`, member/profile writer and harness were reused; no second planner, result writer or test-only production hook was added. The four ratchets equal these measured totals pending any final corrections.

## Installed artifact

Pending a committed source SHA, exact bridge/package/extracted SHA-256 values, stock 1.5.5 load, 1.5.4 and changed-footer refusal, and selected installed verbs/settings/conformance results. Native Linux x86-64 is the only artifact in this slice. The earlier 0231 M5 artifact predates this source and proves none of these changes.
