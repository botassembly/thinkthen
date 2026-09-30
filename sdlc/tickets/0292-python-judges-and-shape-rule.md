# 0292 — Python judges and shape rule (F1)

Status: Corrected implementation candidate from High-accepted preparation `8bf14799`; same High code re-review and issue closure remain open.

## Outcome

Build immutable Judge from omitted input on module verbs and Engine methods. Bind question settings at construction and deadline_ms/token at application. A judge pickles without a key into spawn; a judge bound to a tally refuses pickling. Call.__bool__ refuses filter(judge,xs). Lists, tuples, range, numpy arrays, dict views and pandas Index are eager; only a true iterator (`iter(x) is x`) is lazy; set and frozenset refuse because order cannot align with answers. Remove four *_many module and Engine methods with exact messages, and add tt.plan(judge,column_or_list). Implement the Stream return and Stream.value refusal in F2 under one joint public landing, never a temporary public return.

## Implemented paths and dependencies

F1/F2 land together. The implementation uses `libraries/python/thinkthen/{__init__.py,judge.py,stream.py,__init__.pyi}`, native `src/{asked,engine,input,result,stream,worker}.rs` and their cohesive nested helpers, selected tests, README and measured Python ratchets. The shared core remains unchanged.

## Smallest meaningful proof

Curry/partial, shape table, bool refusal, spawn pickle, packed pipe row and exact removals with F2 in one candidate. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0107, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Immutable judges, shape rule, safe pickling, plan and many-name removals, jointly landed with F2.
- Proof: Curry/partial, shape table, bool refusal, spawn pickle, packed pipe row and exact removals with F2 in one candidate. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The accepted preparation `8bf14799` correctly joined Judge and Stream. The prior Series-only pandas Index refusal is intentionally superseded: an Index is an eager ordered input. The omitted-input sentinel differs from explicit `None`; a partial creates the same Judge as a direct call. Plan and execution read one validated native question, so later mutation of the original options list leaves both body and send unchanged. High review found that plan accepted an unordered set even though Judge application refused it; the corrected plan rejects both set forms before a send. Source and installed proof, plus package/release limits, are recorded in [the build](../records/0292-python-judges-build.md).
