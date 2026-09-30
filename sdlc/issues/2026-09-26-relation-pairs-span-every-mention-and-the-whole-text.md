# Relation pairs span the whole text

Status: open, a post-0.1 candidate. Shortened 2026-09-30. Fix 1, merging mentions, landed: `core/relation/pairs.rs::asked_names` keeps one name per text and kind at its first mention (ticket 0167; `sdlc/records/2026-09-29-recognition-release-decision.md`). Fixes 2 and 3 remain. ADR 0056 keeps the whole text as relation evidence, so a default distance limit changes the contract and needs a ruling.

Priority: rank 21 of 25 in `../planning/issue-priorities-2026-09-30.md`. Owner: Ian rules on the default, then a future ticket.

## What happens today

`recognize --relation` pairs every recognized name with every other name the rule allows, across the whole text. Nothing checks where the two names sit. A person named in the first sentence is asked about a song named forty sentences later.

## Measured

Local experiment 275, model `jev-1.13.0`, three runs an arm, 28 generated paragraphs of about six sentences with 359 stated relations.

| Arm | Recall | Precision | Questions | Input tokens |
| --- | --- | --- | --- | --- |
| Every mention, every allowed pair | 359/359 each | 94.2 to 94.7% | 4,244 | 107,903 |
| Mentions merged by name and kind first | 358/359 each | 94.7 to 95.5% | 2,028 | 60,516 |
| Pairs inside one sentence only | 339/359 each | 96.0 to 96.9% | 820 | 42,355 |
| Pairs inside one sentence or two neighbouring sentences | 359/359 each | 94.7 to 95.2% | 2,072 | 66,339 |

The one-sentence limit lost the 20 relations that cross a sentence boundary. The neighbouring-sentence limit kept recall on these easy paragraphs. Natural text with a relation stated three sentences apart was not measured.

## Remaining fixes

2. Pair only names in the same sentence or in neighbouring sentences by default. A setting widens the limit or turns it off. The page states the measured loss of the one-sentence limit and why the default is wider.
3. Report pairs with and without the limit in the recognize plan, so a caller sees what the limit saves.

A single sentence, the common case, asks the same questions with or without the fix.
