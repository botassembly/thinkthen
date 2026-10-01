# A PHP 8.3 FFI consumer works through C, but no supported package exists

Status: closed 2026-09-30. Merged into `../2026-09-26-language-packages-need-a-release.md`, which keeps this language's open release items and limits.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 291. Stage one pinned main `5b90c13b`; stage two pinned `2b08154b`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (FFI::cdef; list-or-dict kinds/options/labels/levels already idiomatic). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: Composer-shaped with php >=8.3 + ext-ffi and an absolute-library-path native contract.

## Handoff

The historical experiment copy instruction is complete and superseded by `libraries/php/` and [ticket 0249](../../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, Packagist/direct distribution, other-host proof and PHP-driven in-flight cancellation.

Note: PHP cannot fire a token while a blocking FFI call holds the VM — release work retains the pre-fire-only documentation and any worker-process design as an explicit open choice.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 48->40; FFI deprecation noted for J8). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Landed product source (2026-09-28)

Ticket 0249's landed PHP source is in `libraries/php/`; [preflight](../../records/0249-php-preflight.md) and [build proof](../../records/0249-php-build.md) name its current-source gate and completed review. This does not close the issue. The final release pin, `ubuntu-24.04` build/release path, native archives, and Packagist/direct distribution remain explicit release criteria. The local Linux x86_64 source gate does not claim other hosts or PHP-driven in-flight cancellation.
