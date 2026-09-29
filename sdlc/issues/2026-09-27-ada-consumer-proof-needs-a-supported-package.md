# A Ada (GNAT 13) consumer works through C, but no supported package exists

Status: ticket 0249 landed the `libraries/ada/` product source. The accepted [integration closure](../records/0249-integration-closure.md) and [build record](../records/0249-ada-objc-cobol-build.md) cover the GNAT source package and installed consumers on their recorded Linux pin. This issue remains open for a final release commit rebuild, actual `ubuntu-24.04` Actions build and release, checksummed native archives, direct source installation and other-host proof. No release dispatch or publication is claimed.

Ticket 0249 landed a Linux GNAT source package under `libraries/ada/`, built against the current 30-export C header. Its public binding passed all 29 executable J1 cases, two installed consumers, copied failure facts, the 32-body historical matrix and Ada-task cancellation. Fresh High source review accepted `31fa3f78`; shared registration review and landing completed under ticket 0249. The historical handoff below is superseded by the landed source and accepted integration closure. This issue stays open for a final-release native pin, `ubuntu-24.04` CI/release, distribution and untested hosts.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 293 at `experiments/293-thinkthen-ada-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `0d8a7df4`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (Tasking runs strict in-flight cancellation natively). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: GNAT .gpr, controlled owners, full type contract.

## Handoff

The historical experiment copy instruction is complete and superseded by `libraries/ada/` and [ticket 0249](../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, direct source installation and other-host proof.

Note: Review found three typed-decode defects; all fixed with adversarial regressions and re-reviewed ACCEPT.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 38->32; Call_Value/Call_Facts added in repin package). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.
