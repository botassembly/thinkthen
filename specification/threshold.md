# The threshold

Status: **Settled** for version one, by ADR 0007.

One option sets the rule. `--threshold` takes a single cut or a band, and no second policy option exists.

Every probability on this page is illustrative.

## The rule

For `decide`, let p be the probability of yes. The other verbs that accept a cut use the same inclusive boundary on the quantity named in the table below. Only `decide` accepts a band and returns yes, no, or not sure from it.

| Form | Accepted values | Yes | No | Not sure |
| --- | --- | --- | --- | --- |
| none given | | p ≥ 0.5 | p < 0.5 | never |
| `--threshold T` | 0 < T ≤ 1 | p ≥ T | p < T | never |
| `--threshold LOW:HIGH` | 0 ≤ LOW < HIGH ≤ 1 | p ≥ HIGH | p < LOW | LOW ≤ p < HIGH |

The high boundary is inclusive. The low boundary belongs to the not sure side, so a value below LOW is no and a value at LOW is not sure.

A cut of 0 is refused because every probability would reach it and every answer would be yes. A band low of 0 is accepted because a probability of exactly 0 is not sure unless it also reaches HIGH.

A value is a decimal fraction. A percent such as `90`, a reversed band such as `0.9:0.1`, an empty side, and a number that is not finite are usage errors before any request goes out. `--threshold 0.5` and no threshold at all name the same rule.

The rule has a second home. A question file holds it under `threshold`, and a `--threshold` typed beside `@FILE` replaces it. [question-file.md](question-file.md) gives the precedence and what each source is named in a message.

## Worked boundaries

| p | none given | `--threshold 0.9` | `--threshold 0.1:0.9` |
| --- | --- | --- | --- |
| 0 | no | no | no |
| 0.1 | no | no | not sure |
| 0.5 | yes | no | not sure |
| 0.9 | yes | yes | yes |
| 1 | yes | yes | yes |

Under `--threshold 0.1:0.9`, p of 0.1 is not sure because it reaches the low edge, and p of 0.5 is not sure because it sits inside the band.

## A single cut never says the model is sure

Under a single cut, "no" means the answer did not reach the mark. It does not mean the model is sure of no. `--threshold 0.9` calls p of 0.88 a no, and it calls p of 0.02 a no, and the two are not the same evidence. The help says so.

Measurement of the first System One model showed answers inside a not sure band flipping between identical runs 5% to 14% of the time. Answers outside such a band flipped 0.5% to 2%. Section 5 of `sdlc/planning/design-study.md` records both rates. A band names the region where a second look pays. A single cut hides it.

Repeated calls can move an answer far enough to cross a cut. In experiment 212, a borderline answer moved by up to 0.08 between two identical requests. The offline [0163 drift record](../sdlc/records/0163-answer-drift.md) found a largest gap of 0.45 and 430 repeated digests crossing 0.5 in the pinned benchmark; the repository probes had no repeats, and its other recordings had three repeats but no crossing. Size a band from the gaps that record reports.

## Which verbs take which form

| Command | Single cut | Band | The cut applies to |
| --- | --- | --- | --- |
| `decide` | yes | yes | the probability of yes |
| `choose` | yes, optional | no | the highest option probability; an exact top tie stays unresolved even when it reaches the cut |
| `tag` | yes | no | each label's independent probability of yes; every label that reaches the cut is included |
| `filter` | yes | no | the probability of yes for each record |
| `recognize` | yes | no | each name's printed strength, P(kind) times P(span) rounded to four decimals |
| `recognize --relation-threshold` | yes, under its separate flag | no | each stated relation edge's probability of yes |
| `relate` | yes | no | each relation edge's probability of yes |
| `score` | no | no | its value is a weighted position on levels, with no command threshold |
| `rank` | no | no | it orders by probability of yes and takes no threshold |
| `annotate` | no command flag | no command flag | each saved `decide`, `choose`, or `tag` question keeps its own rule; `score` has none |
| `find` | no | no | it selects one unit and takes no threshold |

`--threshold` on a command that takes none is a usage error. A question inside an `annotate` file carries its own threshold, and [annotate.md](annotate.md) gives the rule there.

On `choose`, no cut means the sole top option can be returned at any probability. `decide`, `tag`, `filter`, `recognize`, and `relate` default to a cut of 0.5. A tuned cut therefore means a mark on that row's quantity: yes probability, leading option probability, each label's yes probability, printed name strength, or each edge's yes probability. `audit` tunes `score`'s weighted position with level boundaries instead of writing a command threshold; [audit.md](audit.md) gives that separate rule.

An exact tie for first place in `choose` is not sure with or without a threshold. With no threshold and one strict winner, `choose` returns that label.

## The value travels

`--details` prints the rule as `threshold`: a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. A value copied out of a result works on the command line and in a saved question file.

## `--min-prob` maps to a band

The landed code carries `--min-prob P`, a symmetric pass mark above 0.5. It is gone, and `--threshold (1−P):P` is the same rule.

| Old | New |
| --- | --- |
| `--min-prob 0.9` | `--threshold 0.1:0.9` |
| `--min-prob 0.8` | `--threshold 0.2:0.8` |
| no mark, and an `unassessed` result | `--threshold 0.5`, or nothing at all |

The old rule could not express an uneven band, and it could not express a single cut. The new rule expresses both. The `unassessed` outcome is gone, because a rule always exists.
