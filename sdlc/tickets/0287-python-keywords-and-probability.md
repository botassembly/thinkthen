# 0287 — Python keywords and probability (T5)

Status: Accepted design at preserved preparation `8bf14799`; implementation candidate for fresh code review. ADR 0105 settles the outcome; issue closure remains open.

## Outcome

Keywords on every verb; `.probability` on `Call` (decide and choose; None on
score and tag); `Engine(max_requests_total=)`; `true_`/`false_` → `true`/
`false`; `deadline=` seconds → `deadline_ms=` milliseconds with the plain
rename message; `descriptions=` folded into the members map on choose, score
and tag (recognize keeps its own).

## Prerequisites and files

T1 and T7 are landed; F1/F2/F3 follow this ticket in one coordinated Python boundary. The change is confined to `libraries/python/src/{asked.rs,engine.rs,engine/settings.rs,frame/recognition.rs,input.rs,result.rs,worker.rs}`, `thinkthen/{__init__.py,__init__.pyi}`, selected tests, README, check script and both measured host ratchets. The shared core parser and process reservation are reused without edits.

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

The first High code review found that the old pandas guard and Arrow reader still refused a column with any null input. ADR 0105 T5 requires null-in, null-out. The corrected Arrow reader checks the producer's validity bitmap inside its owned memory snapshot, borrows only present text, and retains sliced and chunked offsets. A single worker call compacts present rows, then aligns typed values, probabilities, observations and frame cells with the original positions. Missing inputs make no request and no failure marker; an unresolved valid answer and a failed call remain separate states. The pandas list fallback normalizes missing cells without replacing the Arrow ownership path. The review also confirmed that the descriptions keyword removal follows ADR 0105. The [build record](../records/0287-python-keywords-and-probability-build.md) separates historical passing receipts from the corrected source and wheel proofs.

The High re-review of the first null-input correction found a separate malformed-producer gap. Utf8 and LargeUtf8 offsets must remain ordered and within range across null slots, even though the null payload bytes are never read. The corrected reader validates the entire offset sequence before borrowing present text; the separate Utf8 View layout retains its own rules. The selected boundary witness rejects validity `101` with offsets `[0, 3, 1, 4]` as Usage before any send. Valid mixed and all-null inputs retain the accepted null-in/null-out behavior. The matching wheel and exact proof are in the build record. A new nullable representation must retain structural validation for ignored payloads; the joint 0292/0293/0294 Judge, Stream and Tally work carries that lesson at its public entrypoints.

Pricing stage 0300 landed after the original 0287 source proof and was merged before code review. Python's closed question-settings parser still refuses its new pricing and estimated-token keys as Usage; the Python engine constructor maps neither into a silent setting. The merged wheel rebuilt against the new core and its installed consumer retained values, probabilities, facts and the active request cap. Python price controls and cost fields remain a later host claim. The original 41 selected cases and eight conformance cases remain evidence pinned to the pre-merge source; the focused merged-artifact results are recorded in the [build record](../records/0287-python-keywords-and-probability-build.md).
