# Portable batch fixtures fail core Clippy

Status: Closed. Fixed by the typed portable fixture Quick Fix at `6e0df944`, accepted by fresh Medium code review. Both existing tests and all-target Clippy pass.

The default workspace all-target Clippy check reports eleven disallowed dynamic-JSON uses in `crates/thinkthen/src/core/batch/tests/portable.rs`. Its two fixture tests decode literal corpus files through `serde_json::Value` and construct JSON member arrays, despite the existing core prohibition. These are test code failures; no product request or answer defect is established.

## Outcome and proof

Decode the small known fixture grammar into typed test structures and compare ordinary typed fields and member arrays. Preserve every existing literal spelling, hash head, cut, membership, close reason, complete body and digest assertion, the five/three input counts and escaped-text counterexample. Keep the fixture files and expected values unchanged. Do not exempt the tests from the core policy, remove proof or add another harness.

Run the two existing portable tests, relevant all-target Clippy, format, policy and measured ratchet. This is a bounded test-only Quick Fix; no provider, package, stress, SQL or DataFrame run is needed. The checkpoint log is `target/codex-builds/integrated-lint-checkpoint/clippy.log` in codex-2.
