# 0460: Bound initial requests across repeated input

Native, command-line and annotate plans now bound initial sends by admitted wire-question occurrences. Closed preview batches drain immediately. Bytes and tokens describe uninterrupted uncoalesced packing; annotate retains separate packed-body counts. Runtime coalescing, caching, retries, refusal splits, original positions and the C ABI are unchanged. ADR 0123 supersedes the old global once-per-call promise.

Qualified source: 607c3688b8bf42c1e146ae8486e9a9a392eda1f9. Fresh whole-change review accepted production source 710bd20ec; subsequent test-oracle and established gate-fixture corrections were accepted without changing production. Full tests passed: 1738 workspace, 336 library-only and 23 consumer cases, doctests, supporting checks and 19 smokes. Full lint passed; the unchanged executable specification retained its passing result. Focused tests completed all 186 original rank positions and exercised C, Python and Polars forwarding. The measured Rust total is 161603: 53 production and 167 test lines added. Earlier runs found stale count expectations and load-sensitive deadline, TLS and packing fixtures; the final complete run passed. Core freeze waits for 0444; this does not claim final cross-surface parity.

## What the build taught us

Packed bodies cannot bound sends after pending duplicates complete or input pauses. Count wire-question occurrences for admission, keep preview bytes separate, and collect contract-test failures together. Deterministic loopback packing fixtures must retain the existing debug input-pause control; live behavior stays unchanged.
