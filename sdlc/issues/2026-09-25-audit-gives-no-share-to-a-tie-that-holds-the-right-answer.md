# audit gives no share to a tie that holds the right answer

Status: Open. Filed on 2026-09-25 by the Beatles Bench owner for Beatles Bench ticket 0011, which moves the bench's grading onto `thinkthen audit`.

Ian ruled on 2026-09-25 that everything is in 0.1 (`sdlc/planning/one-line-plan-2026-09-25.md`, commit `82336e9a`). Ticket 0131 (`sdlc/tickets/0131-audit-matches-the-bench.md`) settles this issue after ticket 0125 lands.

## What happens

`specification/audit.md`, "The answer under a rule": a `choose` answer whose top probability is shared by two or more options is `"tied"` under every rule. "The outcome" keeps `"tied"` as its own state. It is never right. Audit prints only the count of tied answers. It does not say whether the right option was among the tied ones.

Beatles Bench grades a tie differently. `scripts/score/score.py`, `credit()`: a tie at the top that holds the truth earns one over the number of tied options. That share is the chance a random pick among the tied options is right. A tie that misses the truth earns nothing. The bench's accuracy tables are built on this rule. In `results/tables/accuracy.tsv`, GLM-5.3 Flash scores 1451.33 of 1,501 overall, and the fraction comes from one shared tie. The word-overlap and BM25 baselines tie on 982 and 905 questions.

## Why it blocks the bench

Ticket 0011 grades every run through `thinkthen audit`, except named exceptions. Audit's counts cannot rebuild the bench's right answers when a run has ties, because the output does not say which ties hold the key. The bench must either change its published rule or keep a Python grader for ties.

## Options

1. audit adds a `tied_holding_key` count and a `tie_credit` sum per group: the share each tie holding the key would earn at random. Existing members keep their values.
2. audit prints one line per graded answer under a flag, with its outcome and its tied options, so a caller can apply its own rule.
3. No change. Callers that want the share keep their own grader.

Recommendation: option 1. It adds two members, keeps every current value, and gives the chance-corrected score that a benchmark with ties needs. Ian can overturn it.
