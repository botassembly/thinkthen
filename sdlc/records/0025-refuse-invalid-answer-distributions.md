# 0025: Refuse invalid answer distributions

Branch `ticket/0025-refuse-invalid-distributions`. Built 2026-09-20.

## What landed

`choose` and `score` now accept only one complete distribution over the labels the question sent. The common answer type checks that the probability total differs from one by no more than `member count × f64::EPSILON`. The System One adapter rejects an extra option or level key and still reports a missing requested key first.

The reported probabilities remain unchanged in detailed output. A score divides its weighted sum by the accepted measured total before the existing twelve-decimal rounding, so admitted floating-point residue cannot move the score outside its level scale. Yes/no answers, confidence, thresholds, requests, recordings, and digests do not change.

`backends.md` states the distribution and exact-key rules. `score.md` states the measured-total division. ADR 0019 records the decision and its numeric bound.

## Red then green

The first focused cases showed that a choice or score reply with every member equal to `1.0` decoded successfully. Three score members of `1.0` produced `3.0` on a scale whose valid positions are 0 through 2. A reply could also add a probability for a label the question never sent, and the decoder ignored it.

The generic tests now pin totals immediately inside and outside the derived tolerance on both sides of one. They put accepted residue at the lowest and highest score positions and keep the result from 0 through the highest position. An exact serialization check proves an accepted non-unit total keeps every reported member.

Adapter tests cover high and low invalid totals for both question types, extra choice and score keys, and missing-before-extra precedence. The complete error sentence is pinned. A sentinel extra label and its value do not appear in the error.

The full test rung first exposed an older fake response in `from_record.rs`: its four-option form totaled 1.05. The helper now gives the winner 0.9 and divides the remaining 0.1 across the other options. It serves the same winning-label purpose with a real distribution.

## Review

The design reviewer rejected two drafts. The first required a derived tolerance and arithmetic that keeps a score inside its scale. The second required `score.md` to state the changed formula. It then accepted the design and the level 2 Luna High route.

The code reviewer accepted the production behavior and rejected the first proof. It required upper tolerance boundaries, both score endpoints, exact serialized preservation, and a planted extra label in the safe-error test. The final pass accepted the whole diff with no remaining blocker.

## Choices made where the pages were silent

Ian can overturn these choices.

- **The tolerance grows with member count.** Binary parsing and addition can each leave representation residue. The limit is about `5.7e-14` at 255 choice labels and about `2.3e-15` at ten score levels. All 493 distributions in recordings plus four standalone fixtures are closer to one than `1.2e-16`.
- **The common answer type owns the total.** Every adapter must produce the same valid internal value. The System One adapter owns its wire keys because another adapter may name labels differently.
- **Accepted probabilities are retained while score arithmetic uses their measured total.** Rewriting the members would make the result claim the backend reported different values. Dividing only the weighted sum preserves the evidence and the score range.
- **Bad total and extra label have separate safe errors.** Both name the question and the broken rule. Neither repeats a label or value from the reply.

## Gates and size

The source ceiling is 15,436 measured Rust lines, up from 15,183. Most growth is the decoder matrix and the tolerance, endpoint, serialization, precedence, and secrecy tests. The generic distribution tests moved into `answer_distribution_tests.rs` to keep `answer.rs` below 500 lines.

| Rung | Result |
| --- | --- |
| `sdlc/scripts/install` | exit 0 |
| `sdlc/scripts/lint` | exit 0, ratchet `15436/15436` |
| `sdlc/scripts/test` | exit 0 |
| `sdlc/scripts/spec` | exit 0 |

The coordinator ran the ladder with the key and base address unset. No live call ran.

## What proved wrong

The first proposed tolerance was one billionth. The repository evidence showed compatibility but did not justify that number, and any nonzero raw-total tolerance could put a score beyond its documented scale. ADR 0019 now derives the tolerance from binary precision and member count, and score arithmetic divides by the measured accepted total.
