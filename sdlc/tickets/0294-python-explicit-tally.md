# 0294 — Python explicit tally (F3)

Status: Corrected joint implementation candidate from High-accepted preparation `8bf14799`; same High code re-review and issue closure remain open.

## Outcome

Wrap the T7 Arc-shared core Tally as tt.Tally and accept tally= for judges, streams and frame idioms. Aggregate finished-call records, sent requests, cache answers, tokens and first-start-to-last-finish seconds. Keep tt.usage as process totals; no mutable last-call slot.

## Implemented paths and dependencies

T7 is already in the shared Rust core. `libraries/python/src/tally.rs`, the engine/frame/stream call paths, public package/stub and selected tests use that one Arc-shared Tally. The Python claim contains the measured host ratchets; priced host aggregation is a later claim.

## Smallest meaningful proof

Two calls at a held listener before release, then a small completion/cache/missing-usage table against observed request bodies. Count exact accepted loopback requests and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Explicit Python Tally over the Arc-shared core fact sum.
- Proof: Two held arrivals before release plus completed-call, identical replay cache and missing-usage accounts against captured bodies. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

A two-arrival held listener proves actual overlap before release. A separate small table proves two completed sends, one identical cached replay counted as one cache answer, then a no-usage reply leaving token totals absent. The stream first-failed-row case retains records 0 and requests 1 in the same core Tally. The corrected SIGINT case proves that a terminal stream yields no later value while the completion receipt and Tally both retain one admitted record and request. No mutable last-call slot, sixteen-thread campaign or copied accounting algorithm was added. See [the build](../records/0294-python-tally-build.md) for the proof and remaining limits.
