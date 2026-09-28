# Relation requests carry one rule and every entity

Status: closed as already fixed by0147/0167. Fresh proof review798cbaef confirms the implemented shared-state outcome; see `sdlc/records/qf-recognition-remainder-proof.md`. The separate whole-text split cost remains under register115. Historical report follows. Filed 2026-09-26 from the ten-function efficiency audit under ADR 0054, local experiment 275. Ticket 0147 built the shared state for `recognize`. Ticket 0167 carries it to `relate`. Owner: ticket R5, which builds ADR 0050 items 5 and 6. It absorbs severity 3 title 11 of report 10 in `2026-09-26-architect-review-severity-3-findings.md`. Does not block 0.1.

## What happens today

`relate` and `recognize --relation` build one request state for each concrete relation.

- `crates/thinkthen/src/core/relation.rs` lines 304 to 336: `relation_evidence` writes `{evidence, entities, relation}`. `entities` lists every entity of the set or text, whatever its kind. `relation` names one concrete rule.
- `crates/thinkthen/src/engine/prepared_request.rs` line 119 builds each concrete relation's request from that state.
- `crates/thinkthen/src/engine/facade/relate.rs` line 62 and `crates/thinkthen/src/engine/facade/recognize.rs` line 147 plan one request group for each rule. `recognize` then sends the groups one after another (line 165).
- The pair question reads "Does the relation hold from i1 to i2?" (`relation.rs` line 278). It learns which relation from the state, so two rules cannot share one state.

So each rule's request repeats the whole text and the whole entity list. A place or an organisation that no rule names still rides in every relation request. Ticket 0147 item 5 keeps this shape and only changes the question.

## Why it breaks ADR 0054

- Item 1. Entities of kinds no rule names can never reach an edge, yet every request carries them.
- The audit pattern "sequential requests that could share one". Rules over the same text send separate requests with the same evidence, and `recognize` waits for each in turn.
- The audit pattern "evidence sent more than once". The text and the entity list are billed once for every rule.

## Measured

Local experiment 275, model `jev-1.13.0`. The relation step was given the key's names, so recognition errors play no part. Rules: `sang=person:song`, `wrote=person:song`, `appears_on=song:album`, and on the hand-written set also `recorded_at=album:place`. Every arm asks the pair questions of ticket 0147 items 5 and 6.

- **Per rule**, as planned by 0147: one request per concrete rule, with today's state and "Does the text state that the relation holds from i1 to i2?".
- **Shared**: one request per text. The state holds the text and only entities of a kind some rule names. Each question names its relation: "Does the text state that i1 sang i2?".
- **Menu**, the pick-one proposal in `relate-design.md`'s superseded history: as shared, but a person and song pair gets one pick-one question over `sang`, `wrote`, `sang and wrote` and `none`.

| Set | Arm | Recall | Precision | Requests | Questions | Input tokens |
| --- | --- | --- | --- | --- | --- | --- |
| 140 generated sentences, 3 runs | per rule | 100% each | 100% each | 340 | 840 | 176,029 |
| | shared | 100% each | 100% each | 140 | 840 | 72,203 |
| | menu | 100% each | 100% each | 140 | 460 | 98,423 |
| 30 hand-written sentences, 5 runs | per rule | 26/27 each | 26/28 three times, 26/27 twice | 45 | 56 | 19,501 |
| | shared | 26/27 each | 26/28 each | 24 | 56 | 9,559 |
| | menu | 27/27 each | 27/29 each | 24 | 34 | 11,077 |

- The shared request cut input tokens by 59% and 51%, and requests by 59% and 47%. Its answers matched the per-rule arm within the noise on both sets. The generated set is easy and saturates, so it measures cost more than accuracy.
- The menu asks fewer questions but bills 36% and 16% more tokens than the shared yes/no questions, because every option carries a description. Its one extra edge on the hand-written set sits inside that set's noise. The menu does not earn a place.

## Recommended fix

In R5, build one relation state per text or entity set. It holds the text, when there is one, and only the entities whose kind some rule names. Every rule's pair questions ride in the same requests, up to 0147's cap of 400 questions a request, and each question names its relation by its `reads` text. `recognize` then sends its relation requests at once, which settles severity 3 title 11. Keep yes/no questions and drop the menu idea.

The change alters every relation request body, so old recordings and cache entries miss. R5 already changes them for items 5 and 6, so the two changes cost one re-record.

## Blocks 0.1

No. R5 is the right place, because it re-records every relation request anyway.
