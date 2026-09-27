# A COBOL (GnuCOBOL 4) consumer works through C, but no supported package exists

Status: the local package experiment (local experiment 292) is complete through both stages, parent-verified and review-accepted. The queue owner decides the integration ticket (batch J8) and release; nothing is published. Ian authorized the work on 2026-09-27.

## Evidence

Beelink, Ubuntu 24.04.3, glibc 2.39, x86_64; local experiment 292 at `experiments/292-thinkthen-cobol-c-interface/`. Stage one pinned main `5b90c13b`; stage two pinned `26ae542e`. Native libraries rebuilt offline from each pin under the shared heavy lock with experiment-owned registry copies; hashes, exports, soname and archive members verified in `artifacts/manifest.json`. Each stage ran the full ten-verb matrix against an independently counted loopback fixture with exact arrival multisets, reverse bulk completion, twice-per-name recognition, and the strict post-0166 cancellation contract (BY VALUE SIZE IS 8 width discipline; engine-level strict proof via labeled ctypes helper). Every stage was rerun independently by the parent and accepted by a fresh read-only review (`stage*/reviews/`, `stage*/FINDINGS.md`).

Package: copybook facade plus dependency-free C JSON tokenizer, engine_new_with bound.

## Handoff

Copy only `stage2/package/` into the J8 integration ticket under `libraries/cobol/`; never ship rehearsal archives. The ticket applies the type contract (issue `2026-09-27-one-type-contract-for-every-surface`) — Copybook with 88-level condition names for outcome and all six error kinds, unresolved/failed/resolved annotate fields, mixed-label rejection by name, thinkthen_engine_new_with bound and tested, schema-sample parity at gate time. Fully type-contract compliant. — runs J1's result-schema parity corpus now that `specification/result.schema.json` has landed, binds `thinkthen_engine_new_with` where the stage-two pin predates it, rebuilds on the final release pin, and adds the GitHub Actions `ubuntu-24.04` build/release path with native archives on GitHub Releases and direct or language-registry distribution.

Note: Single-threaded CALL cannot fire in-flight tokens; pre-fired and deadlines are the COBOL-supported forms.

## Limits

One host, one toolchain version per language, synthetic loopback replies, shared-C library linkage, no runtime ABI identity handshake, no sanitizer coverage of Rust allocations, no live-model quality. See each stage's FINDINGS for the full list.
