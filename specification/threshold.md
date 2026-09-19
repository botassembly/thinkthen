# The threshold

Status: **Settled** for version one, by ADR 0007.

One option sets the rule. `--threshold` takes a single cut or a band, and no second policy option exists.

Every probability on this page is illustrative.

## The rule

Let p be the probability of yes.

| Form | Accepted values | Yes | No | Unresolved |
| --- | --- | --- | --- | --- |
| none given | | p ≥ 0.5 | p < 0.5 | never |
| `--threshold T` | 0 < T ≤ 1 | p ≥ T | p < T | never |
| `--threshold LOW:HIGH` | 0 ≤ LOW < HIGH ≤ 1 | p ≥ HIGH | p ≤ LOW | LOW < p < HIGH |

Boundaries are inclusive. A value meets its mark when it reaches it.

A cut of 0 is refused because every probability would reach it and every answer would be yes. A band low of 0 is accepted because the low side is inclusive, so a probability of exactly 0 is still a no.

A value is a decimal fraction. A percent such as `90`, a reversed band such as `0.9:0.1`, an empty side, and a number that is not finite are usage errors before any request goes out. `--threshold 0.5` and no threshold at all name the same rule.

## Worked boundaries

| p | none given | `--threshold 0.9` | `--threshold 0.1:0.9` |
| --- | --- | --- | --- |
| 0 | no | no | no |
| 0.1 | no | no | no |
| 0.5 | yes | no | unresolved |
| 0.9 | yes | yes | yes |
| 1 | yes | yes | yes |

Under `--threshold 0.1:0.9`, p of 0.1 is a no because the low side is inclusive, and p of 0.5 is unresolved because it sits strictly inside the band.

## A single cut never says the model is sure

Under a single cut, "no" means the answer did not reach the mark. It does not mean the model is sure of no. `--threshold 0.9` calls p of 0.88 a no, and it calls p of 0.02 a no, and the two are not the same evidence. The help says so.

Measurement of the first decider model showed answers inside an unresolved band flipping between identical runs 5% to 14% of the time. Answers outside such a band flipped 0.5% to 2%. A band names the region where a second look pays. A single cut hides it.

## Which verbs take which form

| Command | Single cut | Band | The cut applies to |
| --- | --- | --- | --- |
| `decide` | yes | yes | the probability of yes |
| `choose` | yes | no | the winning option's probability |
| `filter` | yes | no | the probability of yes for each record |
| `score` | no | no | |
| `rank` | no | no | |
| `annotate` | no | no | |

`--threshold` on a command that takes none is a usage error. A question inside an `annotate` file carries its own threshold, and [annotate.md](annotate.md) gives the rule there.

An exact tie for first place in `choose` is unresolved with or without a threshold. With no threshold `choose` returns the winning label.

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
