# Relation pairs span every mention and the whole text

Status: Open. Filed 2026-09-26 from the ten-function efficiency audit under ADR 0054, local experiment 275. Owner: ticket R5 for mention merging, and the recognize window work of ADR 0050 item 7 for the sentence limit. It absorbs severity 3 title 9 of report 10 in `2026-09-26-architect-review-severity-3-findings.md` and adds a measurement to item 5 of `2026-09-25-recognize-and-relate-scale-and-shape.md`. Does not block 0.1.

## What happens today

`recognize --relation` pairs every recognized name with every other name the rule allows, across the whole text.

- `crates/thinkthen/src/engine/facade/recognize.rs` line 124 passes every recognized name to the relation step. Each mention is its own name, so a name the text repeats is paired again at each mention.
- `crates/thinkthen/src/core/relation.rs` line 179, `matching`, takes every entity of a kind. `pair_plan`, from line 259, asks every source and target pair. Nothing checks where the two names sit in the text.
- Ticket 0147 item 5 makes every concrete relation ask these pairs as yes/no questions, so the question count grows with the product of both sides.

A person named in the first sentence is asked about a song named forty sentences later. Report 10 priced the bound on a 1,000-word text at 1,998,000 pair questions.

## Why it breaks ADR 0054

- Item 1. A repeated name asks the same pair again, and its answer adds nothing an earlier mention's answer lacks.
- Item 2. Distance in the text is a cheap earlier filter. The planner could rule out far pairs before it asks.

## Measured

Local experiment 275, model `jev-1.13.0`, three runs an arm. 28 paragraphs of about six generated sentences each, with the 359 relations they state as the key. The Beatles members recur across sentences, so names repeat. Rules `sang=person:song`, `wrote=person:song` and `appears_on=song:album`. One generated case in seven states its album in the next sentence: "X wrote S, and Y sang it. The song appears on A." Every arm uses one shared request per paragraph and the pair questions of ticket 0147 item 6.

| Arm | Recall | Precision | Questions | Input tokens |
| --- | --- | --- | --- | --- |
| Every mention, every allowed pair, as 0147 plans | 359/359 each | 94.2 to 94.7% | 4,244 | 107,903 |
| Mentions merged by name and kind first | 358/359 each | 94.7 to 95.5% | 2,028 | 60,516 |
| Pairs inside one sentence only | 339/359 each | 96.0 to 96.9% | 820 | 42,355 |
| Pairs inside one sentence or two neighbouring sentences | 359/359 each | 94.7 to 95.2% | 2,072 | 66,339 |

- Merging mentions halved the questions and cut input tokens by 44% within the noise.
- A one-sentence limit cut questions by 81% and lost the 20 relations that cross a sentence boundary. That loss is beyond the noise.
- A limit of neighbouring sentences cut questions by 51% and tokens by 39% within the noise. On six-sentence paragraphs it saves half. On a long text it turns a count that grows with the square of the names into one that grows with their number.

The paragraphs are generated and easy. They measure cost well and accuracy only roughly. Natural text with a relation stated three sentences apart was not measured.

## Recommended fix

1. Merge mentions with the same name and kind before pairing. Keep each mention's offsets in the printed name, and give an edge found for the merged name to the first mention of each end.
2. Pair only names in the same sentence or in neighbouring sentences by default. A setting widens the limit, or turns it off, for a caller whose relations span further. Under ADR 0054 item 4 the page states the measured loss of the one-sentence limit and why the default is wider.
3. Report pairs with and without the limit in `recognize --dry-run`, so a caller sees what the limit saves.

## Blocks 0.1

No. A single sentence, the common case in the key, asks the same questions with or without the fix.
