# A Swift 6.4 consumer works through C, but no supported package exists

Status: closed 2026-09-30. Merged into `../2026-09-26-language-packages-need-a-release.md`, which keeps this language's open release items and limits.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 294. Stage one pinned main `5b90c13b`; stage two pinned `7a1ba6cc`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (Module-map import; strict cancellation from a separate Thread). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: SwiftPM with a system-library target and vendored module map.

## Handoff

The historical experiment copy instruction is complete and superseded by `libraries/swift/` and [ticket 0249](../../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, direct SwiftPM installation and macOS consumer proof.

Note: Tarball provenance recorded without an upstream sidecar hash.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.

## Post-J1 feedback run (2026-09-28, pin 6dbdf03f)

`thinkthen_engine_new_with` bound and tested through Swift: a settings-carried `base_url` selects the backend over the environment, `{}` matches the environment constructor by equal decisions, unknown-key and wrong-type objects refuse with null-engine EUSAGE on the calling thread before any request, and construction sends nothing (counted-fixture ledgers prove zero arrivals); sequential cache coalescing observed. Result-schema parity samples pass against the bare `$defs/annotate|recognize|relate` definitions — consumer guidance for J1/J8: the schema's bare verb definitions are what bindings validate against (not the detailed root), `null` and the failed marker need a tagged branch in any Decodable-style model, and offsets are Unicode scalars, not UTF-16 indices. No product defect found. Evidence: `post-j1/POST-J1-REPORT.md` in local experiment 294.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 36->30; packaged example fixed). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Landed product source (2026-09-28)

Ticket 0249's landed Swift source used experiment 294 `repin/package/` plus only the separately proven post-J1 settings constructor delta. This supersedes the earlier stage-two copy instruction above. `libraries/swift/check.sh` currently passes on Ubuntu 24.04.3 x86_64 with Swift 6.4 and the current 30-export C header/library: public J1 55 schema / 29 executable cases, 30 complete normalized request bodies in the matrix, settings precedence and invalid-settings zero-send, copied started-failure facts and six named kinds, strict held cancellation, and two isolated source/native archive consumers with two exact bodies each. The static request ledger, public TypeCase adapter, planted packed-parser and privacy refusals, and host ratchets are product-owned. The exact build pin and native hash are in `sdlc/records/0249-swift-zig-build.md`. Fresh source review and ticket 0249 integration closure accepted this package. The issue stays open for final release pin, `ubuntu-24.04` Actions build/release, checksummed native distribution, and an actual macOS or other-platform consumer proof. Local archives are disposable gate outputs, not published assets.
