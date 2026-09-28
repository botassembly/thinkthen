# Verified preparation for tag, score and annotate batching

Status: bounded source inventory, not accepted tickets or completed features. The coordinator checked local experiment 2030 against its frozen main `8c15eb6b` and the accepted ADRs. Current main `fde46b37` includes later 0167, 0209, 0201 and 0208 work. Ticket 0213 is still building; refresh its affected files after it lands. No build, test or provider call ran for this inventory.

## Useful findings retained

| Boundary | Verified handoff |
| --- | --- |
| B9 tag and score | At the frozen revision, `cli/args.rs` lacks the batching group for these verbs, `cli/judge.rs` passes no batch setting, and `cli/asked.rs` drops the question-file tier. Trace all three together when extending the existing planner. |
| Grammar | `core/question_file.rs::parse_top`, the closed `specification/question-file.schema.json`, shared `specification/fixtures/question-file/corpus.json`, and both corpus consumers must agree. The old choose rejection belongs to 0213; add distinct tag/score cases after that ticket lands. |
| B10 annotate | Current adapters use `core/question_set.rs`, `cli/annotate.rs`, `cli/annotate_schedule.rs` and `engine/facade.rs::groups`. Trace record and on-group identity through preparation, dispatch and assembly. A per-group request does not prove serial transport. Existing scheduling, splitting, request-identity and partial-failure cases protect distinct behavior. |
| Capacity | At `8c15eb6b`, nonblank counts were asking 500, batch 496, args 492, batched 490 and question_set 469. These are historical warnings, not current headroom. 0213 already needs private extraction; remeasure its landed files instead of planning another copy. |
| Speed fixture | `probes/speed/functions.jsonl` carries B9/B10 under each function's `list.ticket`. Remove the settled exception marker when its ticket lands; retain the function and its measurement fixture. |

## Corrections to the preparation report

The Pi report is useful as a path inventory but must not be used unchanged as a design brief.

1. It says the nested annotate refusal at `questions.ok.batch` should be replaced. ADR 0048 item 4 permits at most one **top-level** batch setting. Keep the nested refusal and add the accepted top-level form.
2. It calls the environment tier an undecided B9 choice. ADR 0048 item 4 already fixes typed, environment, question-file, default precedence. Implement that contract unless a reviewed amendment changes it.
3. It infers tag cannot use the batch path from the error text that a batched verb asks one question. The guard checks `Asks::Fixed`; `judge::tag` also builds `fixed(&settled)`. A logical tag question can expand to several wire questions. Trace the actual variant, expansion, member counts and answer assembly before proposing a different scheduler.
4. It identifies the choose-file rejection as a B9 test to replace. 0213 owns that case and schema extension. B9 builds on the landed result and needs its own tag/score acceptance coverage.
5. It proposes changing `EVERY_KEY` merely to recognize batch, although the table already names it. Trace the per-verb support gate and permitted keys before listing edits.
6. It calls per-verb digest proofs duplicates without tracing the changed question shapes. Reuse the common proof where equivalent; retain a distinct boundary when the new verb changes encoding or answer assembly.
7. It describes annotate transport as serial based on the record scheduler. Its group runner uses concurrent callbacks. Request units, record ordering and simultaneous sends are separate facts.

The raw session contains **46 bash tool calls and one report write**, despite a 40-command stop rule and a report claiming exactly 40. The coordinator verified this from the session JSON. Do not treat a successful launcher or self-reported budget as compliance. Two failed launches preceded the completed one; the concrete second failure was Node 18 lacking the installed Pi runtime's required `enableCompileCache`. Launching with the installed Node 22 fixed that environment failure. No provider used by ThinkThen was called.

## Next builder brief

Prepare B9 after reading this corrected inventory, accepted ADRs 0048, 0051, 0053 and 0055, and the final 0213 source. Preserve top-level grammar, settled precedence, single-record bytes and each verb's existing output. Trace one tag and one score from input through logical question, wire expansion, complete request identity, 413 halves, ordered result and facts. Mark each old negative case keep or replace and cite the accepted clause. Record concrete unknowns instead of reopening settled choices. Use the existing parser/schema corpus and listener harness. Claim only the design files until independent design review and exact runtime holds are ready.

The 0213 build exposed a second reusable lesson: `--max-request-bytes` is a soft split threshold for an oversized singleton under ADR 0051 item 1; an explicit backend profile supplies the hard refusal. A proposed fix that made the soft threshold refuse a singleton was withdrawn before landing. A late-refusal fixture must use the explicit profile rather than redefine accepted behavior.

For paid proof, inspect the benchmark's grouping constraints before promising a request count. The 0213 builder found that experiment 262's 200 questions cannot be sent as one common-question stream. Its exact grouping and proposed bounded proof still need review. A generic replacement question would change the measured task.

This pass prepares two future families but adds **zero** independently reviewed numbered tickets and closes **zero** product items. Its useful paths and rejected inferences are recorded separately so the next builder can evaluate whether the corrected brief actually prevents rework.
