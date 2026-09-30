# relate's pair planner loses precision on the Beatles Bench

Status: open. Measured by Beatles Bench tickets 0019 and 0018 on 2026-09-30, against main `c22512868`. The bench's `reports/results.md` publishes every figure below.

## The problem

Ticket 0167 replaced relate's choice planner with the pair planner, which asks one yes or no question for each pair a rule allows. On the same set of 182 songs, the four Beatles and the 13 core albums, at the 0.5 cut:

| Planner | Edge precision | Edge recall | Edge F1 |
| --- | --- | --- | --- |
| Choice planner, main `02dc0b96` | 0.867 | 0.614 | 0.719 |
| Pair planner, main `c22512868` | 0.420 | 0.693 | 0.523 |

Smaller sets show the same shape:

- On 16 sets where the right album is left out, relate still names an album for every song. Edge precision is 0.296.
- On composer and producer pairs from Wikidata, precision is 0.330 at recall 0.833.
- An audit-tuned cut helps little: held-half F1 is 0.689 on solo songs, 0.373 where the right album is missing, and 0.596 on the Wikidata pairs.

Stated relations read from text do much better. `recognize --relation` reads edges a sentence states at precision 0.973 and recall 0.783.

## What should happen

The queue owner decides whether relate's precision at the default cut is acceptable for 0.1. Options:
- Tune relate's default cut.
- Ask each song's single-answer relations as one choice with a way to say none of these.
- Publish the trade-off on the relate page and point users to `recognize --relation` for relations a text states.

## Recommendation for 0.1

This is a product default. The coordinator sets it, and Ian can overturn it. Drawn from the figures above and experiment 275, round R2.

| Option | Evidence | Cost | Risk |
| --- | --- | --- | --- |
| (a) Tune the default cut (`cli/relate.rs:74`, `unwrap_or(0.5)`) | An audit-tuned cut reached held-half F1 0.689, 0.373 and 0.596. The first two rows of the table show the choice planner at F1 0.719 on the same set. The cut never enters the request, and `--details` already prints each pair's probability, so saved runs give every cut with no new spend | Small: one default and the pages | A cut cannot help where the right album is missing: every candidate pair can pass it. The low precision stays |
| (b) Ask each source's single-answer relation as one `choice` that includes a none-of-these option; keep the pair planner for relations that allow many answers | The choice planner scored precision 0.867 against 0.420. In experiment 275 R2, a pick-one menu per pair kept 27 of 27 relations at 27 of 29 precision with 34 questions, against yes/no pairs at 26 of 27 and 26 of 28 with 56 questions, on 30 stated-relation cases. The none-of-these answer is new and unmeasured. R2 read relations from text, and `relate` asks about names alone | One medium ticket after 0304 slice 4: a way to mark a relation single-answer, a new question shape, new recordings, conformance cases 51 and 52 recorded again, and one paid bench run through `sdlc/scripts/live` under a token cap (ruling 13) | It changes the wire form, so it cannot share slice 4's identical-results proof. If the model rarely picks none, precision on the missing-album sets stays low |
| (c) Keep the pair planner; state the measured precision on the relate page, and point to `recognize --relation` for relations a text states | `recognize --relation` reached precision 0.973 at recall 0.783 | Pages only | 0.1 ships a known weak default |

Recommendation: (b) with (c), in the relate ticket after 0304 slice 4 that also carries the unordered both-ways edge shape (issue priorities, coordinator default 1). Keep the 0.5 cut. The relate page states the measured precision and points to `recognize --relation` in either case. Accept (b) as the default only if the paid bench run beats the pair planner's edge F1 of 0.523 on the 182 songs and its precision of 0.296 on the missing-album sets. Otherwise ship (c) alone for 0.1, and keep this issue open. Option (b) costs more than (a), but (a) cannot fix the missing-album case, and a changed default is cheaper before 0.1 than after.

## Evidence

Beatles Bench `results/runs/2026-09-30-relate-jev` and `results/runs/2026-09-30-recognize-jev`, with their recordings, and `reports/results.md`, "The function suite".
