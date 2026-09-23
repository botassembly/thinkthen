# relate and recognize: the call math, the waste, and the real jobs

Status: Open, for the build team. Filed 2026-09-23 by the product side from Ian's deep-dive request. Evidence: `experiments/236-relate-deep-dive/` (live, recorded, $0.0014 charged) and `experiments/225-recognize-harvest-package/relate/measurements/`. Ian can overturn every recommendation here.

## What exists today, by command

- **Main and `review/mainline-2026-09-23`** are the same commit, `f1bc273`. The command has `decide`, `filter`, `rank`, `choose`, `find`, `score`, `tag`, `annotate`, `status`, `cache`, and `prune`. It has no `relate` and no `recognize`. `specification/` has no page for either.
- **`surfaces` at `f942e06`** lists both functions in `functions.toml`, and all nine library and database surfaces expose them. Every surface reaches the stand-in engine, and `standin/src/replay.rs` answers both functions only from `standin/data/recognize-replay.json`. A text the recordings lack is a usage error. The replay also refuses a kind field for `relate`. No surface makes a live `relate` or `recognize` call.
- **The only live path** is a hand-built question set run through `annotate`: one state holds every record, and one `choice` question per pair goes to `POST https://api.typesafe.ai/v1/systemone` (experiments 225, 230, 233, and 236).

## The backend

Every step uses the one System One endpoint with typed questions (`notes/jev/docs/api.md`, `primitives.md`).

| Step | Question type | Options |
| --- | --- | --- |
| recognize, find the spans | `choice` IN/OUT for each word, with a five-word window | 2 |
| recognize, label the kinds | `choice` over the caller's kinds for each word | kinds |
| recognize and relate, link a pair | `choice` over the rule directions the pair allows, plus `NO_RELATION` | 1 + both-ways rules + 2 × one-way rules |
| 225 arm (a), the alternative | `noul` for each relation | none |

The docs say every question sharing a state belongs in one request, and that the limit is the request's token budget of "around 32,000 tokens, roughly 150,000 characters". `choice` returns a full probability map, and the chosen option's probability is the calibrated signal (RECOGNIZE-PRODUCT-SPEC). The choice of endpoint and question type is sound.

**Bug 1, a stale limit.** `sdlc/planning/handoff-to-the-architect-2026-09-21.md` and the 225 verdicts say a question holds at most 100 options. On 2026-09-23, `choose` with 101 options and with 255 options both answered (`236/probe-101`, `236/probe-255`). The request that failed in 225 held 165,154 bytes, so it was over the size budget. Correct the handoff. Keep the tool's 255 ceiling.

## The call math

### recognize, over W words (punctuation counts), with S spans holding w words in all, and L legal pairs

- The specification runs three passes: W span questions, then w kind questions, then L pair questions. That makes 3 requests for a short text when rules are given. Without rules it makes 2.
- The recorded demo (232) folds the first two passes into one: it asks 2W questions and then L. For "Maria Chen joined Northwind Freight, a company in Chicago." W = 11, S = 3, and L = 2 of the 3 pairs. That came to 22 + 2 questions in 2 requests and 4,986 input tokens. The two `--dry-run` plans confirm 22 and 2 questions.
- L ≤ S(S−1)/2. The number of rules R never adds questions. R adds options to a pair question, and R decides which pairs are legal.
- A question costs about 197 tokens (4,341 over 22), so one request holds about 150 questions. A text past about 75 words (folded form) or 150 words (three passes) needs more requests: ⌈2W/150⌉ + ⌈L/250⌉.
- So "recognize makes 3 calls" holds only for the three-pass form on a short text with rules. The recorded method makes 2. The question count is W + w + L, or 2W + L.

### relate, over N records

| | N = 10 | N = 50 | N = 255 |
| --- | --- | --- | --- |
| Pairs, N(N−1)/2 | 45 | 1,225 | 32,385 |
| Pick-one questions, either rule or one-way (the ruled design) | 45 | 1,225 | 32,385 |
| Yes/no questions per both-ways rule / per one-way rule, both directions | 45 / 90 | 1,225 / 2,450 | 32,385 / 64,770 |
| Planned request by `annotate --dry-run` (pairs) | 19,788 bytes | 527,035 bytes | 14,007,163 bytes |
| Requests needed at the vendor budget | 1 | about 6 | about 145 |
| Input tokens (pairs, about 124 per question plus the state per request) | about 5,700 | about 155,000 | about 4.5 million |
| Cost at $0.042 per million input tokens (output is free) | $0.0002 | $0.007 | $0.19 |
| Per-record form: N questions, each against the other records | 10 | 50 | 255 |
| Planned request, per-record form | 4,944 bytes | 64,499 bytes | 1,454,203 bytes |
| Per-record input tokens (about 0.55 a byte, measured) | about 2,700 | about 36,000 | about 0.9 million |
| Per-record cost | $0.0001 | $0.0015 | $0.04 |

The measured anchors: 12 records gave 66 pair questions and 8,162 tokens against 12 per-record questions and 3,449 tokens (236). 225 measured 94,938 against 23,033 tokens at 50 records, and 1,646,214 tokens over 34 requests for 10,000 pairs at 200.

**The two-set form.** With S sources and T targets, a kinds-aware pair run asks S × T questions. Run as one set without kinds, it asks every pair among S + T records, and the same-side pairs are about half of them. Choose-per-source asks S questions with T + 1 options. At 128 × 127: 16,256 pair questions (about 2 million tokens, $0.09) against 128 choose questions (about 0.35 million tokens, $0.015). `choose` on main already does this: `236/match-choose` sent 6 requests of about 400 tokens each and matched 6 of 6.

## The waste

1. **The confirmations are the bill.** In `236/dup-pairs`, 61 of 66 pair questions confirmed no relation at 1.0. The work grows with N², and each question resends about 124 tokens of instruction and options.
2. **Nothing splits the request.** `annotate` plans one request per record whatever the size (`specification/annotate.md`: over the limit, "the backend refuses, and the exit code is 4"). Hand-run relate breaks at about 25 records. The design promises splitting that no build has.
3. **Pick-one per pair drops a second relation.** 225 arm (a): 11 of 24 at 2,449 tokens, against 22 of 24 at 1,714 tokens for yes/no per relation.
4. **False edges grow with the pair count.** 225 counted 5, 14, and 43 false edges at 10, 50, and 200 records. Between two runs at 50 records, 8 of 625 pair answers flipped. Per-record: 0 of 25.
5. **The two-set jobs pay for same-side pairs** when run as one set, and the stand-in refuses the kind field that would drop them.
6. **Recognize repeats its instructions in every word's question.** The specification's own fix, instructions once per request, is worth five times.

## Recommended changes, by value

1. **Split requests to the budget, in the engine, for every many-question function.** `--dry-run` prints the question count, request count, and a token estimate. Without this, neither function runs on real sizes.
2. **Ask per record against the candidates.** For each record, ask one `choice` over the other records (or the target set) plus none, and keep every candidate at or above a cut. Edges come from the probability map. 236 got every edge from 12 questions where pairs took 66, at 42 percent of the tokens. The pairs form stays for small N and for rules where every pair is independent.
3. **Take two sets.** `relate --to FILE` compiles to choose-per-source. The how-to shows `choose --options` until then.
4. **One question per pair and rule.** Ask a both-ways rule as `noul`. Ask a one-way rule as a three-way `choice`: A to B, B to A, or neither. This keeps several relations per pair, and the direction keeps its probabilities. Ian ruled this on 2026-09-23, with packing required this round and no recognition knobs (`sdlc/planning/relate-design.md`, commit bbe8b8d). The 236 arms each ask one both-ways rule with two options, so they read as this yes/no form.
5. **Block before pairing when N is large.** A `choose` or `tag` pass puts records in coarse groups, and pairs are asked only within a group.
6. **Print the records on each edge**, per `2026-09-22-relate-output-must-read-on-its-own-and-take-two-sets.md`.
7. **Recognize: instructions once per request.**

## The real jobs

Relate earns its place where the answer is a link inside one pile and no list of options exists:

1. **Contradictions** among policy rules, contract clauses, or requirements. Nothing else finds them. (236: 2 of 2 found, none extra.)
2. **Duplicates** in one pile: bug reports, tickets, contact records. (236: 5 of 5.)
3. **What blocks what** among tasks. The edges are directed, and code sorts them into an order.
4. **Cause chains** among the events in an incident timeline (slide 10c, 233).
5. **Which claims each piece of evidence supports.** This is many-to-many across two sets, so it borders on `tag`.

The honest part: most jobs pitched as relate are choose-per-record. A new ticket to its open incident, a payment to its invoice, a résumé to a job. Ian's Beatles case, each song to its lead singer and its album, is two sets and many-to-one, so it is `choose` twice per song. The answer also comes from the model's own knowledge. The input does not hold it, so it fails the obvious-example rule.

## Relate and recognize

The two share one pair-asking path in the engine and do two jobs. Recognize links a few names inside one sentence, and the sentence is the evidence. Relate links records in a pile, and the pairs grow with N². The record-graph job matches the published idea of asking every unordered pair once, as a three-way directed choice, and letting code build the graph. Keep them as two functions over one engine path.
