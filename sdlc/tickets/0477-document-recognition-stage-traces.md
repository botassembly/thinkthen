# 0477: Document recognition questions and stage observations

Status: COMPLETE. Fresh ticket review accepted the design; implementation has not started.

Milestone: 0.2
Owner: builder.
Signed: queue owner, 2026-10-08.
Review: accept. Fresh read-only Sol review accepted the design after the signoff correction.

Reviews: revision b6970338e, accept

Reviews: revision 95fade3863f777142ecd49d4abca0efcf1790cce, accept

Reviews: revision eeee618da3d0b0fd6d8a53b6510af24267535ce3, accept

Landed: 892e0ab

Reviews: revision 1d131b130, accept

## Outcome

Document a working way to read each recognize logical question and its answer by original record and stage using the existing native observer. Add no trace API or storage.

## Evidence

- Starts from: PM inbox `2026-10-08-pm-thinkthen-pm-scope-rulings-on-the-tcga-recognize-asks.md`, ask 5. `CallOptions::observe` on native `recognize_records_complete_with` emits `RecordObservation::Question { index, stage, position, detail }` before each completed row. `QuestionDetail::question()` gives actual normalized text and declared options; `value()`, `probabilities()`, `failure()`, `requests()`, and `to_owned()` expose answers and durable snapshots. Stages are `boundary`, `kind`, `edge`, `relation`. `RecordObservation` JSON serialization omits question text; CLI `--details` carries question digests and stage distributions, not an indexed question transcript. Recordings retain bodies but do not by themselves present a record/stage join.
- Keeps: Existing result, recording and cache privacy rules; no SDK or CLI coverage claim beyond the native Rust route. The observer sees supplied original records and may expose sensitive question/evidence text to the caller.
- Changes: Add a concise native Rust recipe beside recognition documentation. Show `CallOptions::new().observe(&callback)`, match `Question` events, copy or `to_owned()` inside the callback, group by zero-based `index`, `stage`, then `position`, and read the logical text/options and answer/failure/probabilities. Mention that `Row` marks a completed record, that a failed request yields no complete result, and that cache/replay can produce the same logical observations with source metadata. State the CLI/serialized-event limitation plainly.
  Claim `specification/recognize.md`, `libraries/rust/**` and `site/**`.
- Proof: Replay a repository saved recognition fixture with no key and no send; compile/run the documented example through the docs example gate. Assert at least one boundary question and its answer, record/stage/position mapping, and no claim that every kind/edge/relation stage occurs on every input.
- Also changes: Document the actual step-1 boundary and step-2 kind/edge question shapes, with a runnable plan example showing which rendering is exposed by `--plan`. Say explicitly that question wording is subject to change and is not a stable interchange format. The tagged-example input is the stable interface for teaching recognition once its conditional implementation lands. Use generic software fixtures.
- Defers: CLI trace export and a store/query framework. Any confirmed SDK gap remains in 0.2; 0.3 is the proxy.
