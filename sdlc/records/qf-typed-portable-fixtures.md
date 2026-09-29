# Typed portable batch fixture Quick Fix

Candidate on `ticket/qf-integrated-lint-checkpoint` after merging main `ccba1931a`. The prior [integrated lint checkpoint](2026-09-29-integrated-lint-checkpoint.md) remains a historical receipt: its all-target Clippy run failed with eleven disallowed dynamic-JSON uses in `crates/thinkthen/src/core/batch/tests/portable.rs`. That failure and the two existing fixture tests are the red phase. This Quick Fix changes only that test file, this record and the shared measured `sdlc/ratchet.json`.

`PortableFixture` now deserializes the known compact spelling, hash heads, cut flags, member indices, close reasons and digests into typed vectors. `TextFixture` adds the text inputs and flattens the same expected fields. Both existing tests still compare each selected spelling, head, cut, member grouping, close reason, complete literal request body and digest; they still pin five texts/three batches, three structured rows/two batches, and the escaped-text counterexample. The fixture files and their bytes did not change. The typed member comparison replaces `json!`; no dynamic `Value` access, policy exemption, new test, listener, framework or production hook was added.

## Focused proof

- `cargo test --locked -p thinkthen --lib core::batch::tests::portable:: -- --nocapture`: **2 passed**, 407 filtered out; `thinkthen` test crate compiled in this lane's warm target.
- `cargo clippy --locked --workspace --all-targets -- -D warnings`: **pass**, visibly checked `thinkthen` and `conformance-backend` with default features and two jobs.
- `cargo fmt --all -- --check`: pass. Offline `python3 sdlc/scripts/policy.py`: pass, 189 resolved packages. `node sdlc/scripts/ratchet.mjs`: **99521/99521**. `git diff --check`: pass.
- The changed Rust test file lost **3 nonblank lines** after formatting; the shared ceiling fell from 99524 to 99521. The diff adds 44 lines and removes 45 physical lines. I checked whether the two fixture types duplicated the six expected fields: a shared `PortableFixture` with a flattened text wrapper kept one field list and left the distinct input grammar explicit. The old dynamic extraction and JSON member construction were removed rather than kept beside typed copies. No existing test was deleted or consolidated.

## What the build taught us

Serde's typed fixture decode preserves the literal oracle while satisfying the core boundary's ban on dynamic JSON in all-target test compilation. This correction qualifies only the Clippy segment found by the selected checkpoint. The original full `lint` private-name failure, held DuckDB child guard, versions checker, Dart named-answer scan and API inventory findings retain their separate statuses. No full lint, installer/consumer/package matrix, held SQL/DataFrame feature, provider or Actions check was run for this fix.

## Review and integration

Fresh Medium code review accepted `6e0df944` and independently verified unchanged fixture files, all retained assertions, policy, format and the exact measured ratchet. The coordinator integrated the reviewed code unchanged and retained the focused two-test and all-target Clippy results. No full-rung green result is claimed.
