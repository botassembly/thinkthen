---
date: 2026-10-13
status: draft PRD
release: 0.4
source: Ian's rulings of 2026-10-13 in `functions.md` and `data.md`; PRD 00; ThinkThen 0.2's `specification/rank.md`, `find.md`, `choose.md`, `score.md`, `backends.md` and `thinkthen help rank` and `help find`; Ian's idea note on pairwise ranking and the jsort and jev-reranker clippings it cites; published work on pairwise, setwise, listwise and tournament ranking. The 26 worked questions and every source are in [03-rank-and-find-questions.md](03-rank-and-find-questions.md)
---

# PRD 03: Rank and find, with a cost and accuracy dial

## Who and what's in it for them

- **The builder ranking records** (U3, U37) gets one setting that buys a better order when it matters. Proof needed: the ranking experiment's ordering score against today's `rank` on TREC Deep Learning 2019 and 2020 and BEIR, at a stated cost.
- **The builder setting a cut** gets a 0-to-1 score that means the same in a strong pool and a weak one. Proof needed: a threshold chosen on half the queries holds on the other half at least as well as today's yes probability.
- **The agent searching by meaning** (U5, U38) can `find` the best of thousands of passages, or learn that nothing fits. Proof needed: top-one accuracy on 1,000 candidates, and right "none" answers on queries stripped of relevant passages.
- **Everyone who pays** (U19, U20) sees each setting's cost first. Proof needed: `--plan` bounds that hold on every experiment run.
- **The agent compacting a document** (U41) reuses the same dial through `rank`, with no extra function.

## The design

**One option, four settings, cheapest first.** `--compare` names how much of the set the model sees at once.

| Setting | What the model sees | Questions for 100 items | Default for |
| --- | --- | --- | --- |
| `alone` | One item. 0.2's `rank` | 100, packed into few requests | `rank` |
| `window` | Overlapping groups of up to 20, each a pick-one over the group | About 15 | `find` (one group holding the whole set, 0.2's `find`) |
| `tournament` | Two items at a time, about ten comparisons per item, paired by current standing | About 500 pairs, plus the `alone` pass | Neither |
| `all-pairs` | Every pair, in both orders | 9,900 | Neither; refused past 100 items |

**A comparison is a `find` of two.** 0.2's `find` sends the caller's question as written, with the candidates as a pick-one. A pair uses that wire shape with two candidates, so ThinkThen adds no wording.

**One fit turns every answer into a score.** Luce's choice rule reads picks from pairs and windows alike; with two items it is Bradley-Terry. The fit corrects for opponents, so beating strong items counts for more. The `alone` pass then sets the level: ThinkThen maps the fitted order onto its yes probabilities. The order comes from the comparisons and the level from the single questions. The experiment tests this against maximum, minimum, mean, median and Elo.

**Position and ties.** `tournament` shows each item first in half its comparisons and fits the first-position lean; `all-pairs` asks both orders. Probabilities count as partial wins, cycles lower the reported reliability, and exact ties keep input order.

**`find` past 255 items.** `window` carries each group's leaders to a final group, and only the final offers `none`. `tournament` compares the finalists in pairs before that final. Without `--compare`, 256 items refuse and name the setting, as PRD 00 says.

**Surfaces.** The command line, every library (`compare="tournament"`), SQL settings, tool files and MCP share the `compare` name and values. Under `tournament`, `--top N` stops asking about items that cannot reach the top. `--threshold` cuts the score under every setting. `--plan` prints the setting, rounds and upper bounds on requests and tokens. The loop runs in the caller's SDK, so through a proxy each round is an ordinary call. Each pair answer is stored like any answer, its evidence the ordered set of two items (PRD 01). A seeded schedule makes a rerun free.

## What must be true

| Risk | What retires it |
| --- | --- |
| Comparisons do not pay for themselves | Every setting against `alone` with a yes/no and a ten-level `score` question, on two models |
| Two-item picks are too sure to fit, as jsort found | An arm with a yes/no wording around the caller's question |
| The score does not carry a threshold across pools | Threshold transfer between query halves, and score drift for one item in strong and weak pools |
| Packing many pairs per request moves answers | Packed against one pair per request, on the same pairs |
| Items overflow small models | Request sizes against each backend's limit; one local run |

## Out of 0.3

Pairwise right answers and a pairwise answer key. Saved scales that place new items later, as jsort does. Tunable window size and comparisons per item. Comparisons for question sets. Truncating long items. A built-in compaction command.

## Questions for Ian

Ian's open questions for every PRD are in one list, ordered by importance, at the start of the PRD PDF. This PRD's are numbers 19, 20, 21, 23 and 24.
