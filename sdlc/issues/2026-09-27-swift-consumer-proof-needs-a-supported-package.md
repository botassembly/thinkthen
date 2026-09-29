# A Swift 6.4 consumer works through C, but no supported package exists

Status: the local package experiment (local experiment 294) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized the work on 2026-09-27.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 294 at `experiments/294-thinkthen-swift-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `7a1ba6cc`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (Module-map import; strict cancellation from a separate Thread). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: SwiftPM with a system-library target and vendored module map.

## Handoff

Copy only `stage2/package/` into the J8 integration ticket under `libraries/swift/`; never ship rehearsal archives. The ticket applies the type contract (issue `2026-09-27-one-type-contract-for-every-surface`) — SwiftPM package with CThinkThen system-library target and vendored module map. Typed Outcome: Int32 present; same deltas as C# plus J1 parity corpus. — runs J1's result-schema parity corpus now that `specification/result.schema.json` has landed, binds `thinkthen_engine_new_with` where the stage-two pin predates it, rebuilds on the final release pin, and adds the GitHub Actions `ubuntu-24.04` build/release path with native archives on GitHub Releases and direct or language-registry distribution.

Note: Tarball provenance recorded without an upstream sidecar hash.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.

## Post-J1 feedback run (2026-09-28, pin 6dbdf03f)

`thinkthen_engine_new_with` bound and tested through Swift: a settings-carried `base_url` selects the backend over the environment, `{}` matches the environment constructor by equal decisions, unknown-key and wrong-type objects refuse with null-engine EUSAGE on the calling thread before any request, and construction sends nothing (counted-fixture ledgers prove zero arrivals); sequential cache coalescing observed. Result-schema parity samples pass against the bare `$defs/annotate|recognize|relate` definitions — consumer guidance for J1/J8: the schema's bare verb definitions are what bindings validate against (not the detailed root), `null` and the failed marker need a tagged branch in any Decodable-style model, and offsets are Unicode scalars, not UTF-16 indices. No product defect found. Evidence: `post-j1/POST-J1-REPORT.md` in local experiment 294.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 36->30; packaged example fixed). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Product integration candidate (2026-09-28)

Ticket 0249's Swift source candidate uses experiment 294 `repin/package/` plus only the separately proven post-J1 settings constructor delta. This supersedes the earlier stage-two copy instruction above. `libraries/swift/check.sh` currently passes on Ubuntu 24.04.3 x86_64 with Swift 6.4 and the current 30-export C header/library: public J1 55 schema / 29 executable cases, 30 complete normalized request bodies in the matrix, settings precedence and invalid-settings zero-send, copied started-failure facts and six named kinds, strict held cancellation, and two isolated source/native archive consumers with two exact bodies each. The static request ledger, public TypeCase adapter, planted packed-parser and privacy refusals, and host ratchets are product-owned. The exact build pin and native hash are in `sdlc/records/0249-swift-zig-build.md`. This candidate awaits fresh code review. The issue stays open for final release pin, `ubuntu-24.04` Actions build/release, checksummed native distribution, and an actual macOS or other-platform consumer proof. Local archives are disposable gate outputs, not published assets.
