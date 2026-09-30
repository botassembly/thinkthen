# 0292 — Python judges and shape rule (F1)

Status: Draft preparation. Accepted ADR 0107 settles the outcome; implementation and issue closure remain open.

## Outcome

Build immutable Judge from omitted input on module verbs and Engine methods. Bind question settings at construction and deadline_ms/token at application. A judge pickles without a key into spawn; a judge bound to a tally refuses pickling. Call.__bool__ refuses filter(judge,xs). Lists, tuples, range, numpy arrays, dict views and pandas Index are eager; only a true iterator (`iter(x) is x`) is lazy; set and frozenset refuse because order cannot align with answers. Remove four *_many module and Engine methods with exact messages, and add tt.plan(judge,column_or_list). Implement the Stream return and Stream.value refusal in F2 under one joint public landing, never a temporary public return.

## Prerequisites and proposed files

Prerequisite: T5; joint green landing with F2. Proposed file families: `libraries/python/src/{lib.rs,asked.rs,input.rs,result.rs,engine.rs}; libraries/python/thinkthen/{__init__.py,__init__.pyi}; selected Python tests/ratchets`. Refresh exact nested helpers, package member inventories, nonblank source headroom and current Lanes claims before implementation. No source file is claimed by this preparation draft.

## Smallest meaningful proof

Curry/partial, shape table, bool refusal, spawn pickle, packed pipe row and exact removals with F2 in one candidate. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Immutable judges, shape rule, safe pickling, plan and many-name removals, jointly landed with F2.
- Proof: Curry/partial, shape table, bool refusal, spawn pickle, packed pipe row and exact removals with F2 in one candidate. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

Pending implementation: record corrected assumptions, preparation misses, proof adjustments and remaining limits before landing.
