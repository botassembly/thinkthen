# 0293 — Lazy native Python stream (F2)

Status: Draft preparation. Accepted ADR 0107 settles the outcome; implementation and issue closure remain open.

## Outcome

Expose core Batch through a bounded channel. The source advances only inside __next__ on the caller thread, while the worker never touches Python. Bound look-ahead to throttle+1 request ceilings (currently 5 × 96,000 bytes of text, at most 4,096 records per request); close, drop and context exit stop scheduling. Keep in-flight billing, single-reader and fork guards, token/deadline/Ctrl-C, frozen facts after join, decide/choose probability pairs, and exact Stream.value refusal. A slow half-full iterator waits as accepted by 0212; document batch=1 for a live source.

F1 and F2 share one reviewed green public API landing because a judge applied to an iterator must already return Stream; Stream.value belongs here.

## Prerequisites and proposed files

Prerequisite: F1 joint green landing; reuse core Batch from 0212. Proposed file families: `libraries/python/src/{lib.rs,input.rs,worker.rs,result.rs} plus one private stream module; Python package API/stubs and focused stream tests`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

SQLite cursor calling-thread proof, bounded take, exact distinct-record bytes, close/failure facts and cancellation children. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Lazy bounded native Stream over core Batch, jointly landed with F1.
- Proof: SQLite cursor calling-thread proof, bounded take, exact distinct-record bytes, close/failure facts and cancellation children. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
