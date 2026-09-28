# A C#/.NET 8 consumer works through C, but no supported package exists

Status: the local package experiment (local experiment 290) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized the work on 2026-09-27.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 290 at `experiments/290-thinkthen-csharp-c-interface/`. Stage one pinned main `2b08154b`; stage two pinned `5aec6402`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (P/Invoke; strict cancellation native via CancellationToken). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: NuGet-shaped ThinkThen.C 0.0.1, linux-x64, no bundled native bits; the versioned native archive is a documented separate contract.

## Handoff

Copy only `stage2/package/` into the J8 integration ticket under `libraries/csharp/`; never ship rehearsal archives. The ticket applies the type contract (issue `2026-09-27-one-type-contract-for-every-surface`) — NuGet-shaped `ThinkThen.C` 0.0.1 (linux-x64, no bundled native bits; a separately supplied versioned native archive is the documented contract). Typed `Outcome` present; deltas: named error kinds, map-form label sets with structured descriptions, annotate failure typing, offset naming plus the shared non-BMP case. — runs J1's result-schema parity corpus now that `specification/result.schema.json` has landed, binds `thinkthen_engine_new_with` where the stage-two pin predates it, rebuilds on the final release pin, and adds the GitHub Actions `ubuntu-24.04` build/release path with native archives on GitHub Releases and direct or language-registry distribution.

Note: The stage-one gate initially built unlocked (dotnet); fixed to one locked entrypoint before acceptance.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.


## ABI preflight drift note (2026-09-28, from the Dart port)

The C header now exports twenty-one symbols at recent pins: `thinkthen_error_facts_json` joined `thinkthen_engine_new_with` at main `5f069321`. Any consumer ABI preflight this issue recorded with a fixed count (nineteen or twenty at its stage pins) is stale at final pins. J8 must derive the export list from the sealed header at each rebuild instead of pinning a count; the Dart stage-two gate (`exports.py` comparing header declarations against `nm -D`) is the pattern to copy. Evidence: local experiment 300, `stage2/FINDINGS.md`.
