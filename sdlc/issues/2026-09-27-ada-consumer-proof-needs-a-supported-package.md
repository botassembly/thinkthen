# A Ada (GNAT 13) consumer works through C, but no supported package exists

Status: the local package experiment (local experiment 293) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized the work on 2026-09-27.

Ticket 0249 now has a Linux GNAT source package candidate under `libraries/ada/`, built against the current 30-export C header. Its public binding passed all 29 executable J1 cases, two installed consumers, copied failure facts, the 32-body historical matrix and Ada-task cancellation. Fresh High source review accepted `31fa3f78`; final shared registration review and landing remain in progress. The historical handoff below is superseded by the accepted re-pin preparation. This issue stays open for a final-release native pin, `ubuntu-24.04` CI/release, distribution and untested hosts.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 293 at `experiments/293-thinkthen-ada-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `0d8a7df4`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (Tasking runs strict in-flight cancellation natively). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: GNAT .gpr, controlled owners, full type contract.

## Handoff

Copy only `stage2/package/` into the J8 integration ticket under `libraries/ada/`; never ship rehearsal archives. The ticket applies the type contract (issue `2026-09-27-one-type-contract-for-every-surface`) — Controlled owning types with exception-safe release, Outcome representation clause No/Yes/Not_Sure, six named Failure kinds, typed relation endpoints, all-bare-or-all-described Label_Set. Fully type-contract compliant (section 2). — runs J1's result-schema parity corpus now that `specification/result.schema.json` has landed, binds `thinkthen_engine_new_with` where the stage-two pin predates it, rebuilds on the final release pin, and adds the GitHub Actions `ubuntu-24.04` build/release path with native archives on GitHub Releases and direct or language-registry distribution.

Note: Review found three typed-decode defects; all fixed with adversarial regressions and re-reviewed ACCEPT.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 38->32; Call_Value/Call_Facts added in repin package). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.
