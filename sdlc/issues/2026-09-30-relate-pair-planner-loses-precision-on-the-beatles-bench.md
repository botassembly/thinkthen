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

## Evidence

Beatles Bench `results/runs/2026-09-30-relate-jev` and `results/runs/2026-09-30-recognize-jev`, with their recordings, and `reports/results.md`, "The function suite".
