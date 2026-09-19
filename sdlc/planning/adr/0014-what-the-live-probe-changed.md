# ADR 0014: What the live probe changed

- Status: Decided by the agent on 2026-09-19 under the rulings of ADR 0010. Ian accepted item 1, `find`, the same day, and ADR 0015 records it. Ian can overturn each item cheaply, because no item has code behind it yet
- Date: 2026-09-19

ADR 0010 ruled that a live probe runs before more of the design is trusted, and that `find` stays Draft until a live run compares it with `rank --top 1`. Ticket 0011 ran six probes through `sdlc/scripts/live`. `sdlc/records/0011-the-live-probe.md` holds every number, and an independent reviewer recomputed each one from the committed rows. Every case is made up, and every count is small. The probes spent 214,997 input tokens, and the ledger stands at 247,839 of 476,000,000.

## 1. `find` is built, with a `none` option, and its reach is stated

Over sixteen made-up documents with one answering line, one `choose` over the line numbers picked the right line 16 times, and one `decide` per line with a local sort picked it 15 times. Over four documents with no answering line, a `none` option said none 4 times and never refused a document that had an answer. `find` sent 20 requests and 11,063 tokens. The other form sent 239 requests and 69,143 tokens.

`find.md` leaves Draft. `find` enters the plan after `rank`, as slice 11. The `none` option closes the page's first open point.

The documents held 11 to 14 lines. The page allows 255 units, and nothing measured that. The ticket that builds `find` first repeats the comparison on documents of 100 to 250 lines. The page then states the largest size that held, and the limit drops to that size when the larger documents fail.

## 2. The cut stays on the winning probability

Sixty labeled picks gave 58 right answers. Neither `confidence` nor the winning probability separates the two wrong ones. Two errors cannot settle the question, so the reopening clause of ADR 0009 item 2 stays open. A later eval with more errors can reopen it.

## 3. `score` is stronger than the pages say

Over forty trusted levels, `score` matched the level 31 times and came within one level 40 times. Its unrounded value tracked the trusted order with a rank correlation of 0.9703. `choose` over the same labels matched 30 times with 0.9507. All nine `score` misses were one level too high. `score.md` and `rank.md` lose the sentence that calls rating the weakest thing the model does. `score.md` says instead that the order held well, that the absolute level ran one step high on nine of forty, and that a cut on the number should be tuned on labeled cases.

## 4. Option order matters a little, and only at the catch-all

Reversing the options changed 2 of 60 picks, and shuffling them changed 1 of 60. Every change involved the catch-all `other`. An added irrelevant option changed 0 of 60 and received a probability of 0.0 every time. `choose.md` and the long help say: keep the option order fixed once a cut is tuned, and put the catch-all last.

## 5. A planted claim moves the judge, and a command aimed at the judge does not

Twenty clean cases were judged right 20 times. With hostile text added, 18 answers held, 2 became unresolved under the band, and none flipped. A direct command to the judge moved the probability by 0.04 or less. A false claim planted in the evidence moved it by as much as 0.57, and one planted claim moved it by 0.02. The tool cannot tell a planted claim from a true one, because both are evidence. `decide.md` says so in its cautions and names the defenses a pipeline has: `--field` to keep untrusted parts out, a band to send the moved cases to a person, and an eval with hostile cases. The safety fixture of slice 9 holds both kinds.

## 6. The recipes need to read a pick before `report` is judged again

No recipe reads a `choose` row or a `score` row. The probes wrote a twin of `compare.jq` and a twin of `sweep.jq` for picks. This is the second argument that comparison is clumsy as a file. Ian's five outcomes in ADR 0010 stay open. Slice 10b first makes `compare` and `sweep` read any `value`, so one recipe serves all three verbs, and the second verdict on `report` follows that work. No command is built on this evidence alone.

## Consequences

The specification pages named above change in one pass. `plan.md` marks slice 6 done and slice 11 as accepted work. The cap on questions in one request is still unmeasured and waits for `annotate`.

## Clarifications from the coherence reading, 2026-09-19

The agent ruled on these, and Ian can overturn each.

- `find` takes no `--threshold` in version one. The `none` option is the whole rule for "nothing fits". The test on documents of 100 to 250 lines also reports whether a floor on the winning probability would have caught any wrong pick, and the page changes only on that evidence.
- The list of everyday options in `channels.md` names every everyday option, `--options POINTER` and `--none` included. `--jobs N` and `--cache DIR` join the advanced list with ticket 0013.
- A slice row in `plan.md` records what the slice delivered on the day.
- Slice 10b starts with the work of item 6.
- No check holds a size budget on a specification page. The agent's briefs said one did, and they were wrong. The pages are short today, and a budget enters only if one grows past use.

