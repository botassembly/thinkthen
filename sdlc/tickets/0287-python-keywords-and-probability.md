# 0287 — Python keywords and probability (T5)

Status: Accepted design at preserved preparation `8bf14799`; implementation candidate for fresh code review. ADR 0105 settles the outcome; issue closure remains open.

## Outcome

Keywords on every verb; `.probability` on `Call` (decide and choose; None on
score and tag); `Engine(max_requests_total=)`; `true_`/`false_` → `true`/
`false`; `deadline=` seconds → `deadline_ms=` milliseconds with the plain
rename message; `descriptions=` folded into the members map on choose, score
and tag (recognize keeps its own).

## Prerequisites and files

T1 and T7 are landed; F1/F2 follow this ticket. The change is confined to `libraries/python/src/{asked.rs,engine.rs,engine/settings.rs,frame/recognition.rs,input.rs,result.rs,worker.rs}`, `thinkthen/{__init__.py,__init__.pyi}`, selected tests, README, check script and both measured host ratchets. The shared core parser and process reservation are reused without edits.

## Smallest meaningful proof

Selected Python replay pins one reply value/probability, nulls, rename messages and real cap. Count exact accepted loopback request bodies and pin exit codes and refusal sentences where applicable; record source and installed-host receipts separately. Run only the affected functional cases, measured ratchets, focused format/policy/pages/tickets/diff. [Shared proof routes](../records/2026-09-29-sql-frame-redesign-proofs.md) name optional stress and later package qualification.

## Evidence

- Starts from: Main `869710193`, accepted ADR 0105, experiment 2038 HANDOFF and saved spikes, and the [preparation](../records/2026-09-29-sql-frame-redesign-preparation.md).
- Keeps: Existing successful values, owned facts, NULL/not-sure, cache identity, six error kinds, offline replay and privacy except the accepted changes.
- Changes: Python uniform keywords, Call probability, active engine cap and exact renames.
- Proof: Selected Python replay pins one reply value/probability, nulls, rename messages and real cap. Use the [independent corpus](../records/2026-09-29-sql-frame-redesign-corpus.md) and captured wire bodies.
- Defers: Unrelated package/release qualification, provider work, marketing site, token cap and per-record cache; named prerequisites remain.

## What the build taught us

The seven original Python package failures had six causes. The Arrow case had a stale private fixture: its seven-argument call and `Call.value` now exercise the retained owned batch release. The slide case expected no `failed` column; the corrected assertion pins all answer values and a null failure marker. A pandas `Index` escaped as `AttributeError`; the public wrapper now gives the existing Series-only Usage refusal before a send. The request-cap and held-throttle cases assumed implicit one-record batches. An explicit batch-one cap witness and eight independent three-record packed groups retain those separate properties; default packing has its own exact-body proof. The two Pydantic failures came from a Python 3.14 beta paired with a pin expecting the final typing API. The check script selects a stable cached interpreter and compares the reused venv's executable identity and version. Its three selected Pydantic cases pass under stable 3.13.5.

The Rust binding calls `Settings::parse` for the public question keywords, converts millisecond deadlines once for the whole operation, and forwards the unsigned process cap into existing reservations. `Call.probability` reads the owned observation of the same answer; no companion request exists. The new proof pins yes, chosen, unresolved and banded probabilities, score/tag `None`, exact packed versus batch-one bodies, and unknown/repeated/wrong-key zero-send refusals. Explicit null description fields remain distinct inputs to the shared parser; the local refusal table does not mislabel them invalid. The old `descriptions=` overrides for choose and score were removed from the label test because their direct member-map equivalents already have exact wire comparisons. Recognition keeps its own description keyword. The 200-allocation RSS case moved to the existing opt-in stress selector; the distinct one-call Arrow release and moved-child functional witness remains.

The implementation uses the existing Python wrapper and `Call` carrier, not a second settings grammar or provider call. The source and one locally installed non-release wheel have selected Linux proof; a full package gate, release wheel, pandas 2 lane, all platforms, stress, and provider acceptance remain unqualified. The complete original package issue therefore remains open for its later checkpoint.
