# A COBOL (GnuCOBOL 4) consumer works through C, but no supported package exists

Status: ticket 0249 landed the `libraries/cobol/` product source. The accepted [integration closure](../records/0249-integration-closure.md) and [build record](../records/0249-cobol-build.md) cover the GnuCOBOL 4 source package and installed consumers on their recorded Linux pin. This issue remains open for a final release commit rebuild, actual `ubuntu-24.04` Actions build and release, checksummed native archives, direct source installation and other-host proof. No release dispatch or publication is claimed.

Ticket 0249 landed a Linux GnuCOBOL 4 source package under `libraries/cobol/`, built against the current 30-export C header. Its public binding passed all 29 executable J1 cases, two installed consumers, copied failure facts and the 30-body historical matrix. Fresh Medium source review accepted `5209f934`; shared registration review and landing completed under ticket 0249. The historical handoff below is superseded by the landed source and accepted integration closure. COBOL proves pre-fired tokens and deadlines, while the strict held-call helper remains C-only. This issue stays open for a final-release native pin, `ubuntu-24.04` CI/release, distribution and untested hosts.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 292 at `experiments/292-thinkthen-cobol-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `26ae542e`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (BY VALUE SIZE IS 8 width discipline; engine-level strict proof via labeled ctypes helper). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: copybook facade plus dependency-free C JSON tokenizer, engine_new_with bound.

## Handoff

The historical experiment copy instruction is complete and superseded by `libraries/cobol/` and [ticket 0249](../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, direct source installation and other-host proof.

Note: Single-threaded CALL cannot fire in-flight tokens; pre-fired and deadlines are the COBOL-supported forms.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 34->30). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.
