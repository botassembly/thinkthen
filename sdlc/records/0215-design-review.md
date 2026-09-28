# 0215 design review handoff

Status: ready for a fresh independent design review of the proposed [ticket](../tickets/0215-tag-and-score-record-batching.md) and [preflight](0215-batching-preflight.md); no reviewer verdict or runtime acceptance is asserted. Review the pushed branch commit as the frozen candidate. Source pinned to main `ed3de23f`; 0213 codex-3 source at `f1a56b25` was dirty and unlanded when inspected.

## Review the decisions and risk boundaries

- Check that ADRs 0048/0051/0053/0055 and the batching issue already settle top-level batch grammar, typed/environment/file precedence, records-list evidence for tag/score, soft singleton versus profile hard refusal, one 413 halving and the B9 paid comparison. No ADR 0092 or invented setting should be necessary.
- Follow tag from `cli/judge.rs::tag` and `cli/asking.rs::fixed` through `core/plan.rs::wire_question_count`, `core/adapters/systemone/{request,response}.rs`, `core/batch.rs::Batch.questions` and `cli/asking/batched.rs::answer_rows`. The first-wire offset cannot be used as the index of the one logical decoded tag answer. Check duplicates and split halves as well as ordinary rows.
- Review every grammar copy: parser, closed schema, shared corpus and both consumers. Keep nested `questions.ok.batch` negative under ADR 0048 item 4; replace only tag/score top-level batch negatives. 0213 owns choose.
- Verify the proposed listener proof has an independent expected body/digest, real no-send and send counts, description/order, `q9` to `q10` byte effect, partial tag failure versus `[]`, and final facts. Check that it adds distinct cases without repeating the planner matrix or losing existing regressions.
- Inspect benchmark feasibility before any future paid run: existing 158/171 suites fall short; the proposed 219 shared source rows need offline exact IDs/truth/hashes and grouping. `live --max-tokens` reserves before execution, so conservative output as well as input budget and per-arm stop must be explicit. No provider call was made for this design.

## Source, overlap and open work

0213's question-aware planner and extraction remain pending. A final landed-source refresh and exact runtime claims are required before implementation. Current 500-line parents have no reliable spare capacity; the proposed extraction must preserve their public and private consumers. Accepted 0212 Rust facts design is separate, and B10 annotate and other ports remain outside B9. The only future public behavior here is the accepted B9 outcome; no new Ian decision has been identified. Fresh review may identify a concrete unresolved choice. This record does not self-approve the design or close B9/S1.

## What preparation taught us

The corrected B9/B10 inventory prevented a tag shape and nested-grammar mistake. Direct source tracing found logical answer indices differ from wire offsets, and external benchmark counts forced a feasible at-least-200 row plan. After implementation, compare the final planner and measured proof with these notes rather than treating this handoff as product evidence.
