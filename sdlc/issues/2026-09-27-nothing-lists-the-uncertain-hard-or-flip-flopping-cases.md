# Nothing lists the uncertain, hard, or flip-flopping cases a person should label

Status: Open. Filed 2026-09-27 from the GEPA tuning experiments 296 and 297. Evidence lives in the workspace at `experiments/297-gepa-loop-tests/`; the full write-up is `notes/2026-09-27-optimization-lessons.md`.

## What happens today

`audit` reports a whole question: counts, measures, a coverage curve, and a suggested cut. It never ranks the cases inside the question. `diff` names the cases that changed between two runs, which is a different question: it needs two runs, and it does not rank by how much a label would be worth.

## Why it matters

Every tuning round begins by choosing a few cases to label. The Optimizer ideal state says the round "picks the uncertain cases and a random audit sample." Nothing in ThinkThen does that, so each consumer writes its own selector, and a poor selector wastes the only expensive resource the loop has: a person's labels.

## Measured

Local experiment 297, `runs/labels.json`. The saved probabilities of the bench's 2026-09-26 run over all 228 yes/no questions. A fixed third, 76 cases, was held out. Policies picked labels from the other 152, the cut was tuned on the picked labels, and accuracy was measured on the fixed held set.

| Labels | Random | Uncertainty | Uncertainty plus a 20% audit share |
| --- | --- | --- | --- |
| 10 | 0.699 | 0.724 | 0.724 |
| 30 | 0.704 | 0.724 | 0.724 |
| 60 | 0.711 | 0.724 | 0.722 |
| 120 | 0.713 | 0.697 | 0.697 |
| All 152 | 0.697 | 0.697 | 0.697 |

Held at the default cut 0.5: 0.711. Ten to sixty uncertainty-picked labels beat random, beat the default, and beat tuning on all 152 labels. Beyond about sixty the extra labels pull the cut toward an overfit value.

The same experiment measured the flip side, in `runs/noise-summary.json`: fifty `multi-hop` same-month yes/no cases asked three times each with no cache produced one answer flip, on a case at p 0.49 to 0.52. The median probability spread across repeats was 0.020 and the maximum 0.070. A selector wants the cases near the cut and the cases that disagree between runs.

## What to change

A subcommand that ranks cases for labeling: distance from the cut, disagreement between two saved runs, and a flip across repeats, with a random share for quiet drift. It reads saved `--details` rows, sends nothing, and needs no key, like `audit` and `diff`.

The label simulation above is the contract to test it against: ten to sixty uncertainty-picked cases should beat random on a held set. A new command is additive.

## Factual preparation, 2026-09-28

At main `e58aceae`, `audit` groups scored cases and `diff` names changes; neither supplies the proposed label-order queue. Per-case parsing would help, but uncertainty, disagreement, hard-case and seeded audit-share criteria remain distinct. Experiment 297’s 10–60-label advantage was observed on one cohort and must not become a required empirical accuracy gate. A small saved-row table can prove deterministic ordering, tie handling and zero sends. See `sdlc/records/2026-09-28-tuning-loop-intake-preparation.md`.

## Added 2026-09-28: the shape of the miss, and the criterion

The follow-up reading adds three selection signals beyond distance from the cut.

- **The shape of the miss.** Cluster the misses before choosing a case to label. A wrong value one step off points at knowledge, an answer at the cut points at scale, a tie or null points at ambiguity. The chained album-year family's 38 misses are one or two years off in both directions, so the lever is evidence or a split chain, not a label.
- **The criterion, not only the record.** A decision model grades a whole rubric in one request (AutoRubric), and ThinkThen's `tag` and `annotate` return a probability for each label or each named question. The unit to select is the uncertain criterion as well as the uncertain record.
- **The shortlist.** The truth sits in Jev's top two on 70% of the chained album-year cases against 37% for the top pick, and 95% on the single-hop song-to-album question. A second-position answer is a case worth a label or a stronger judge.

Evidence: `~/workspace/experiments/297-gepa-loop-tests/LESSONS.md` sections 11 to 13.
