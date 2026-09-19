# Record 0009: `choose` and `score`

- Ticket: `sdlc/tickets/0009-choose-and-score.md`
- Branch: `ticket/0009-choose-and-score`
- Landed: 2026-09-19

## What was built

`thinkthen choose QUESTION OPTION...` and `thinkthen score QUESTION LEVEL...` answer one question over one document. All three of the backend's question types are now in the shell. Four how-tos are green.

The core carries the judgment.

- `question.rs`: `Question` is an internally tagged enum over `verb`, with `Decide`, `Choose { options }`, and `Score { levels }`. `Labels` holds a checked list. `Labels::options` takes two to 255 labels and `Labels::levels` takes two to ten, and both refuse a blank label and a repeat. The old `Verb` enum and the `new_decide` constructor are gone, because a struct variant says the same thing with fewer lines.
- `answer.rs`: `Distribution` keeps every label with its probability in the order the labels were sent, and answers `leader`, `tied`, and `position`. `Answer` is one of three shapes, and `Answer::read` is the one place a threshold meets an answer. A pick resolves when its leader clears the cut and no second option ties it exactly. A score resolves always, because `score` takes no rule.
- `threshold.rs`: `is_cut` tells a single mark from a band, and `judge` takes a `Probability`.
- `result.rs`: `DecisionResult` carries the value and an optional threshold, so `score` prints `"threshold": null`.
- `systemone.rs` split into `systemone/request.rs` and `systemone/response.rs` when the one file passed the 500-line ceiling. The seam is encode and decode. `request.rs` writes `criteria` as an ordered map for a choice and as an array for a score. `response.rs` rebuilds each distribution in the order the labels were sent, by lookup, and refuses a reply that omits a probability.

The binary lost its duplication. `decide.rs` became `judge.rs`, and the whole flow from question to request to printed answer is one `run` function that all three verbs call. Each verb builds its `Question`, its `Option<Threshold>`, and its `View`, and hands them over in one `Asked` struct. That is where I looked for duplication before raising the ratchet.

`failure.rs` gained three refusals: `--raw` beside another view, a bad label list, and a band on `choose`.

## Red then green

The systemone tests were written first and failed to compile against seven names that did not exist yet: `Labels`, `new_choose`, `new_score`, `confidence`, `distribution`, `leader`, and `MissingProbability`. The demo pages were then written against the live answers and run until each block matched what the model really said.

## How each acceptance bullet is proven

| Bullet | Test |
| --- | --- |
| The cut and the exact-tie rule of `choose` | `answer::tests::a_pick_clears_the_cut_falls_under_it_or_ties_for_first`, a table over five distributions |
| The weighted score on known distributions | `answer::tests::a_score_is_the_weighted_position_on_the_levels_it_was_given` |
| The bare `choose` value is an option or `null` | `answer::tests::a_pick_is_always_an_option_that_was_sent_or_nothing`, a proptest |
| Bare values, `--raw`, `--quiet`, `--details`, refused replies | `backend/choosing.rs`, ten tests against a local listener |
| `--dry-run` and every list refusal and option refusal | `choose_and_score_edge.rs` |
| The pinned `decide` digest holds | `recording::tests::the_digest_of_the_fixture_request_is_the_name_the_entry_keeps` |
| No key and no evidence in any diagnostic | `decide_edge::no_diagnostic_ever_carries_the_key_or_the_evidence` |
| The spec rung prints the new green demos with no key and no network | `demo_runner::every_recorded_demo_runs_and_every_demo_still_red_is_skipped`, and `sdlc/scripts/spec` printing `demos: 5 green, 10 red` |
| The ratchet equals the measured total | `sdlc/scripts/ratchet.mjs`, run by `lint` |

## What the live responses really looked like

One probe went out first, before any fixture was finalized. A choice answer carries `type`, `choice`, `confidence`, and `probabilities`. A score answer carries `type`, `score`, `confidence`, `legend`, and `probabilities`.

Two things the specification did not say, which the wire proved:

- A choice answer's `probabilities` keys are the option names, and their order is not the order the options were sent. The probe sent `billing shipping account other` and the reply came back `billing other shipping account`. The adapter therefore rebuilds the distribution by lookup, in sent order, and the result's `answer.probabilities` is in sent order.
- A score answer's `probabilities` keys are the level's position as a string, counting from `"0"`, and `legend` maps those same keys back to the level text. The keys are not the level text and the probabilities are not an array.

`specification/backends.md` said both of those wrong. It held two Draft rows, "A choice answer's probability per option" and "A score answer's probability per level", and the second asserted "One probability per level, in level order". I rewrote both rows to what the wire shows, dropped **Draft** from the status line, and replaced the paragraph that promised the rows would settle with one naming `choice`, `score`, and `legend` as fields the adapter computes and never reads. Those are the only specification edits in this ticket.

The vendor's own `score` and the weighted sum of the vendor's own probabilities disagree. The probe returned `score: 1.86` beside probabilities of 0.0, 0.13, and 0.87, whose weighted sum is 1.87. `backends.md` already ruled that a score's number is computed locally and nothing is read from the wire, so the local number stands. It keeps the printed `value` consistent with the printed `answer.probabilities`, which a vendor number would not.

## Choices made where the pages were silent

- A score's weighted sum is rounded at 1e12 before printing. Float summation left `1.9000000000000001` on a distribution that means 1.9. The rounding drops summation remainder and nothing else.
- `--raw` beside `--details` or `--quiet` is a usage error. `channels.md` makes an option that cannot act in the chosen mode a usage error, and `--raw` prints a bare label, so it acts in neither other view.
- `score` refuses `--threshold`, `--quiet`, and `--raw`. It has no rule and no label to print bare.
- `choose` with no `--threshold` resolves on the leader alone, and `"threshold": null` appears in the detailed result. A `decide` with no `--threshold` still gets the default band, because `threshold.md` fixes one.
- The vendor's `choice`, `score`, and `legend` fields are ignored. Each is derivable from the distribution and the question that was asked.
- A `Labels` list is checked before anything is sent, so every list refusal costs no request.

## The demos

Four pages are in the ADR 0011 how-to form, and each block prints what the model really answered.

- Demo 02, "How to branch on a label with `choose` and `case`". The `--input FILE` blocks were out of scope, so each command now reads one document with `<`. The `--jsonl` block is gone.
- Demo 05, "How to sort files into folders by label". The page had asserted `defect=2 process=2 other=0 review=1` at `--threshold 0.8`. The real answers give `defect=2 process=0 other=0 review=3` at 0.8, and `defect=2 process=2 other=1 review=0` with no mark. The page's point survives, so it now runs three blocks: the quick loop with no mark, the same loop at 0.8, and a margin rule in `jq` that gives `defect=2 process=1 other=0 review=2`. The strict mark sends most of a mixed folder to a person, and the page says that is the cost the desk is choosing.
- Demo 17, "How to rate on a scale, sort by it, and test it with `jq -e`", is new. The four reports sort the way a person would sort them. Two of them sit a hundredth apart at 1.01 and 1.0 and do not mean the same thing, which is the page's argument for reading `--details`.
- Demo 20, "How to tell 'not stated' from 'false'", is new. `decide` answers `false` at 0.01 on a notice that contradicts the claim and `false` at 0.04 on a notice that never mentions it. `choose supported contradicted not_stated ambiguous` answers `contradicted` and `not_stated`. The page ends on the branch the yes/no version could not reach.

Every notice, report, note, and ticket in those folders is invented text.

## The live calls

Every call went out through `sdlc/scripts/live`. 6,222 input tokens in total: 658 for the probe, 348 for demo 02, 1,670 for demo 05, 1,438 for demo 17, and 2,108 for demo 20. `sdlc/live-tokens` moved from 1,811 to 8,033 of 476,000,000. The probe script was deleted after it ran.

The key reached `curl` through standard input alone, as a `-K -` configuration file, so it was never an argument and never printed.

Searching the committed recordings for `apikey_`, for `authorization`, and for `bearer`, ignoring case, returns zero, zero, and zero.

## Left for the reviewer

`demos/README.md` holds two sentences that this ticket did not touch, because the ticket limits its edits there to the rows of demos 02, 05, 17, and 20. "Demo 01 holds the only recording so far, and it is the only green page" is now false. "Every page closes with 'What this demo decides'" is false for the four pages in the ADR 0011 form, which close with "What can go wrong". Both sentences belong to whoever reconciles the shared files.
