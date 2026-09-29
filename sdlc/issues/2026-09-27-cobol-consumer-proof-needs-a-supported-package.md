# A COBOL (GnuCOBOL 4) consumer works through C, but no supported package exists

Status: the local package experiment (local experiment 292) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized the work on 2026-09-27.

Ticket 0249 now has a Linux GnuCOBOL 4 source package candidate under `libraries/cobol/`, built against the current 30-export C header. Its public binding passed all 29 executable J1 cases, two installed consumers, copied failure facts and the 30-body historical matrix. Fresh Medium source review accepted `5209f934`; final shared registration review and landing remain in progress. The historical handoff below is superseded by the accepted re-pin preparation. COBOL proves pre-fired tokens and deadlines, while the strict held-call helper remains C-only. This issue stays open for a final-release native pin, `ubuntu-24.04` CI/release, distribution and untested hosts.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 292 at `experiments/292-thinkthen-cobol-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `26ae542e`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (BY VALUE SIZE IS 8 width discipline; engine-level strict proof via labeled ctypes helper). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: copybook facade plus dependency-free C JSON tokenizer, engine_new_with bound.

## Handoff

Copy only `stage2/package/` into the J8 integration ticket under `libraries/cobol/`; never ship rehearsal archives. The ticket applies the type contract (issue `2026-09-27-one-type-contract-for-every-surface`) — Copybook with 88-level condition names for outcome and all six error kinds, unresolved/failed/resolved annotate fields, mixed-label rejection by name, thinkthen_engine_new_with bound and tested, schema-sample parity at gate time. Fully type-contract compliant. — runs J1's result-schema parity corpus now that `specification/result.schema.json` has landed, binds `thinkthen_engine_new_with` where the stage-two pin predates it, rebuilds on the final release pin, and adds the GitHub Actions `ubuntu-24.04` build/release path with native archives on GitHub Releases and direct or language-registry distribution.

Note: Single-threaded CALL cannot fire in-flight tokens; pre-fired and deadlines are the COBOL-supported forms.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 34->30). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.
