# One rule for every number the tool prints

Written 2026-09-21 by the product side. Ian's instruction: the words must match what the model really reports, "not making something up, and not turning it into something that it's not."

## What the model reports

From the vendor's own documents (`notes/jev/docs/primitives.md` and the three pages under it):

| Question kind | The model returns |
| --- | --- |
| Yes or no | One number, the probability that the answer is yes. The vendor states it has no separate confidence |
| Pick one | The picked option, a probability for every option that sums to 1, and `confidence` |
| Place on a scale | A position, a probability for every level, and `confidence` |

The vendor defines its `confidence` as a number "computed from how `probabilities` is spread". It is a summary of the shape of the distribution. It is no probability that the answer is right. The recognize experiments measured it as a gate and it failed (AUC 0.22).

## The rule

1. **`probability`** means a number the model reported, passed through unchanged. Nothing we compute carries that word.
2. **`threshold`** is the user's bar. It is ours, it is an input, and it never appears as if the model said it.
3. **The vendor's `confidence`** passes through under `--details`, under the vendor's own name, exactly as reported. No function gates on it, no bare output prints it, and no page of ours uses the word for anything else.
4. **A number we compute is named as ours and shows its parts.** Today that is two numbers: the position `score` prints, which is the mean of the level probabilities, and the number on a recognized name.

## What this changes

- A relation's number is `probability`. It is the model's probability for the picked relation. This agrees with the build team's recommendation.
- The number on a recognized name cannot be called `confidence`. That word belongs to the vendor and means something else. The number is ours: the least of the word probabilities times the mean of the kind probabilities.
- The cleanest end state has no computed number at all on a name. The name carries `probability`, the lowest probability among the model's answers that produced it. It is a real reported number and it reads plainly: "the answer we were least sure of, behind this name". Whether it gates as well as the computed number is a measurement. The recognize team holds recordings of every run, so the comparison costs nothing. It is item 5 on their closing list.
- If the plain number gates worse, the computed number stays, under a name that says it is computed, with its parts under `--details`. The product side picks that name then.

## The comparison came back, 2026-09-21: the computed number stays, and its name is `strength`

The recognize team ran the free comparison on both full corpora from recordings (`experiments/227-name-number/report.md`). How well each number separates right names from wrong ones:

| Number | WikiGold | CoNLL04 |
| --- | --- | --- |
| The computed number | 0.840 | 0.761 |
| The lowest reported word probability | 0.761 | 0.722 |
| The lowest of all reported probabilities | 0.824 | 0.761 |

The computed number wins on one corpus and ties on the other. No plain model probability gates as well, so a name cannot carry `probability`.

The product side rules the field name: **`strength`**. It is a plain word, it claims nothing about chance, and the vendor does not use it. The manual defines it once: the lowest word probability behind the name times the average kind probability, with connector words left out. `--details` prints those parts beside it. `--threshold` on `recognize` is the bar a name's strength must reach.

What changes: the `recognize` output object, the harvest package's expected objects and its rules six and seven, the demo output, the method page, and the library team's conformance cases. The recognize team's report lists the files. The vendor's own `confidence` field keeps its name wherever it is the literal field.

## What Ian can overturn

All of it.
