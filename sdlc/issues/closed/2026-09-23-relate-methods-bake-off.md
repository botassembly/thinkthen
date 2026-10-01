# relate: the methods bake-off and the recommended algorithm

Status: Closed on 2026-09-25 as superseded. The landed relate design (0081, 0088, relate-design.md) uses a choice across kinds and yes/no within one kind. Earlier status: Open, for the build team and for Ian's ruling. Filed 2026-09-23 by the product side at Ian's request. Evidence: local experiment 237 (live, recorded, $0.058 charged in caps, $0.027 of reported input). `scripts/rerun.sh` there replays every call with no key and rebuilds every table. `relate-design.md` is unchanged. Ian rules on any change to it.

## The recommendation

Use one algorithm for `relate` and for the relation step of `recognize`:

1. The rules and the kinds give the legal pairs, as the design says today.
2. **Ask one yes/no (`noul`) question per legal pair per relation.** A both-ways relation asks each unordered pair once. A one-way relation asks each direction as its own question, so a pair can hold both directions.
3. **Put the shared wording in the state, not in each question.** The state carries the numbered records plus one preface line: what the list is, and any note such as "count only a direct cause". Each question carries only the statement, for example `Does this hold: Item 3 and Item 7 describe the same problem?`. The vendor reads the state once per request and bills each question by its own tokens (`notes/jev/docs/models.md`, `primitives.md`).
4. An edge is a yes probability at or above `--threshold`, default 0.5. The printed number is that probability.
5. Split requests under the size budget. This is already required.

## The evidence

Seven labeled sets, with the truth written before any run: duplicates (N = 24 and N = 50), contradictions, a one-way cause chain, pairs holding two relations, names inside one text, and Beatles songs to singers and albums (N = 25).

| Method | True edges found | False edges | Input tokens, all sets |
| --- | --- | ---: | ---: |
| Lean yes/no (recommended) | 89 of 89 | 11 | 53,776 |
| Yes/no as ruled today | 88 of 89 | 12 | 80,674 |
| Per-record choice, every option above the cut | 79 of 89 | 4 | 73,634 |
| D, then yes/no on its candidates | 88 of 89 | 10 | 84,260 |
| Score per pair (six sets) | 76 of 76 | 9 | 64,986 |
| Pick-one over relations (the old ruling, five sets) | 64 of 72 | 14 | 66,831 |
| Three-way choice (the one-way set only) | 4 of 4 | 0 | 3,933, against 1,899 for lean yes/no |

- Lean yes/no was the most accurate method, and it used the fewest tokens on every set. It won six of seven sets outright. Moving the wording into the state saved 33% against the ruled yes/no questions, and accuracy stayed the same or improved.
- **N questions does not mean fewer tokens.** Each choice option costs about 18 input tokens. At N = 50, per-record choice asked 50 questions holding 2,500 options, which came to 45,309 tokens. Lean yes/no asked 1,225 questions for 32,324 tokens. The vendor bills tokens, not questions.
- **One choice question can name several targets, but they split one probability.** Choice probabilities add up to 1. In a cluster of three duplicates, each report's two true matches shared the probability, so three true pairs fell below 0.5 (recall 0.77). The method works only when each record has one true target.
- **Where each source has exactly one target, per-record choice is more precise.** On the Beatles set it had precision 0.90 and F1 0.89, against precision 0.74 and F1 0.85 for yes/no at 0.5. Yes/no said yes to the right album and also to a wrong one the model half-believed. At the best cut the two were level: 0.91 to 0.94 for yes/no at 0.7, and 0.92 for choice at 0.6. With 32 edges, this is not enough to add a mode.
- Pick-one over relations again lost the second relation (7 of 14 on the two-relation set). The three-way choice matched yes/no on the cause chain but cost 2.1 times as much, and it cannot return both directions of one pair.
- The two-stage methods saved nothing at these sizes. A gate helps only when most records relate to nothing.
- A repeat of every arm moved 7 edges across the cut, all on the Beatles set.

## Cost at N = 10, 50, 255 (one both-ways relation)

This comes from dry-run plans and the measured rates: lean yes/no 25.1 tokens per question, ruled yes/no 37.1, choice 17.8 per option, score 85.3, pick-one 99.5.

| N | Lean yes/no | Ruled yes/no | Per-record choice | Score | Pick-one |
| ---: | --- | --- | --- | --- | --- |
| 10 | 45 questions, 1 request, 1,326 tokens | 1,845 | 1,958 | 4,014 | 4,655 |
| 50 | 1,225 questions, 1 request, 31,690 tokens, $0.0013 | 47,310 | 45,472 | 107,289 | 124,745 |
| 255 | 32,385 questions, 27 requests, 944,559 tokens, $0.040 | 1,391,126 | 1,211,897 | 3,088,410 | 3,627,888 |

For one-way relations between two kinds (songs to albums), per-record choice runs at about 0.7 of lean yes/no, because it asks each option once. At 255 that saving is about half a cent.

## Should today's ruling change

- **The both-ways half stands.** Yes/no per pair was right. The build team should adopt the lean wording, because it is the same question at a third less cost.
- **The one-way half should change** from a three-way choice to one yes/no per direction. The accuracy was the same, and the cost was 2.1 times lower on the measured set. It also gives one question type for every relation, lets a pair hold both directions, and needs no separate path. When the kinds fix the direction (person to company), the three-way choice has only one legal direction anyway. The measured set is small (7 events, 4 edges), and Ian rules.
- `relate-design.md` open item 3, the one-question-per-subject form, stays open. The runs support it only as a later option for relations with one target per source. It is not the default.

## What Ian can overturn

All of it. The recommendation changes the one-way half of the 2026-09-23 ruling and adds the lean wording. Neither lands until Ian rules.
