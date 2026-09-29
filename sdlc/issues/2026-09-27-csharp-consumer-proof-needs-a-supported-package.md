# A C#/.NET 8 consumer works through C, but no supported package exists

Status: ticket 0249 landed the `libraries/csharp/` product source. The accepted [integration closure](../records/0249-integration-closure.md) and [build record](../records/0249-csharp-jvm-build.md) cover the Botassembly.ThinkThen wrapper and local NuGet package consumers on their recorded Linux pin. This issue remains open for a final release commit rebuild, actual `ubuntu-24.04` Actions build and release, checksummed native archives, NuGet distribution and other-host proof. No release dispatch or publication is claimed.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 290 at `experiments/290-thinkthen-csharp-c-interface/`. Stage one pinned main `2b08154b`; stage two pinned `5aec6402`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (P/Invoke; strict cancellation native via CancellationToken). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: NuGet-shaped ThinkThen.C 0.0.1, linux-x64, no bundled native bits; the versioned native archive is a documented separate contract.

## Handoff

The historical experiment copy instruction is complete and superseded by `libraries/csharp/` and [ticket 0249](../records/0249-integration-closure.md). Use the integrated source and its product `check.sh` for the next release work; keep the original experiment evidence below. The remaining requirements are a final-pin rebuild, actual Actions execution, checksummed native release assets, NuGet distribution and other-host proof.

Note: The stage-one gate initially built unlocked (dotnet); fixed to one locked entrypoint before acceptance.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.


## Pre-merge re-pin (2026-09-28, pin 71f25087)

ADAPTED-PASS (packing, envelope, 30-arrival multiset). Evidence: local experiment's `repin-71f25087-REPORT.md` with the unchanged-gate FAIL preserved as drift record, exact new multisets, and all planted negatives. Contract changes consolidated in `2026-09-28-batching-and-envelope-changed-the-c-door-contract-on-main.md`; merge input and steps in `2026-09-28-language-merge-runbook.md`.

## Landed product source, ticket 0249

The reviewed first batch imports `repin/package/` into `libraries/csharp/` with a product-local gate and the `Botassembly.ThinkThen` wrapper ID. The current C header has 30 declarations, verified against the rebuilt library. Local NuGet packaging, two isolated consumers, the exact 30-arrival matrix, named errors, constructor/facts behavior and the J1 public-binding corpus pass on Linux x86_64. The product build receipt is `sdlc/records/0249-csharp-jvm-build.md`. Ticket 0249 source integration landed. Registry publication, native release packaging and other hosts remain open.
