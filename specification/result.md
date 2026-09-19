# The result

Status: **Settled** for `decide if`. **Draft** for the other verbs.

A single judgment prints one JSON document and a newline. The document is compact and sits on one line, so a result is also one record for `jq`, `grep`, and `wc -l`. The example below is spread out for reading. Every field in the example is always present. An empty field holds `null`, and only `usage` may be absent. Token counts are whole numbers of zero or more. `meta.adapter` is an adapter name from [backends.md](backends.md).

```json
{
  "schema": "thinkthen.result/1",
  "question": { "verb": "if", "condition": "asks for a refund" },
  "answer": { "kind": "yes_no", "probability": 0.92 },
  "assessment": { "status": "accepted", "value": true, "min_prob": 0.9 },
  "meta": {
    "backend": "jev",
    "url": "https://api.typesafe.ai/v1/systemone",
    "adapter": "systemone",
    "model": "jev-1.13.0",
    "usage": { "input_tokens": 312, "output_tokens": 48 }
  }
}
```

## Three layers

- **answer** is what the backend said, in thinkthen's own words. No vendor field name appears in it.
- **assessment** is what local policy made of the answer.
- **meta** names the backend profile, the URL that answered, the adapter, the model that answered as the backend reported it, and the usage the backend reported. `backend` is `null` for an ad-hoc backend. `usage` is absent when the backend reports none.

## Four outcomes

| Outcome | How it shows |
| --- | --- |
| Yes | `status` is `accepted` and `value` is `true` |
| No | `status` is `accepted` and `value` is `false` |
| Unsure | `status` is `unsure` and `value` is `null` |
| Error | No result prints. Standard error explains, and the exit code is 4, 5, or 70 |

A fifth status, `unassessed`, means the user gave no pass mark. `value` is `null`, `min_prob` is `null`, and nothing is accepted. A yes, a no, an unsure, and an error never share a representation.

## The pass mark

`--min-prob P` sets a symmetric pass mark. `P` is above 0.5 and at most 1.

- The answer is yes when the probability is at or above `P`.
- The answer is no when one minus the probability is at or above `P`.
- Anything else is unsure.

A probability exactly on the mark is accepted. `--min-prob 0.9` accepts 0.9 as yes and 0.1 as no.

No pass mark is built in. A pass mark is a measurement for one model, and a default would be a guess. A backend profile may carry a measured mark in a later version. An asymmetric pair of marks is a later option.

## A pick-one answer

Status: **Draft**. `decide which` prints this shape.

```json
{
  "schema": "thinkthen.result/1",
  "question": { "verb": "which", "by": "the primary purpose of this issue", "options": ["bug", "feature", "question", "other"] },
  "answer": {
    "kind": "choice",
    "pick": "bug",
    "probabilities": { "bug": 0.94, "feature": 0.03, "question": 0.02, "other": 0.01 }
  },
  "assessment": { "status": "accepted", "value": "bug", "min_prob": 0.8, "min_gap": 0.3 },
  "meta": {
    "backend": "jev",
    "url": "https://api.typesafe.ai/v1/systemone",
    "adapter": "systemone",
    "model": "jev-1.13.0",
    "usage": { "input_tokens": 312, "output_tokens": 48 }
  }
}
```

- `answer.pick` is the option with the highest probability. It is what the backend said, before any policy.
- `answer.probabilities` holds one entry per option sent, in the order the options were sent. `--none` appears here as `none`.
- `assessment.value` is the accepted option name, or `null`.
- `min_prob` and `min_gap` are the marks the user set. Each is `null` when the user set none.

The pass mark for a pick is not symmetric. `--min-prob P` applies to the winning option's probability alone, and `P` may sit below 0.5, because a four-way choice is not a two-sided decision.

| Outcome | How it shows |
| --- | --- |
| A pick was accepted | `status` is `accepted` and `value` is the option name |
| Unsure | `status` is `unsure` and `value` is `null` |
| No mark was set | `status` is `unassessed` and `value` is `null` |

The answer is unsure when the winning probability falls under `min_prob`, when the winner's lead over the runner-up falls under `min_gap`, when the top two tie exactly, or when the winner is an option the user named with `--abstain-on`. The reason appears as `assessment.reason` with one of `below_min_prob`, `below_min_gap`, `tie`, or `abstain_option`.

A pick of `none` is an accepted answer. It says no option fits. It is not an unsure answer.

## A rating answer

Status: **Draft**. `decide how` prints this shape.

```json
{
  "schema": "thinkthen.result/1",
  "question": { "verb": "how", "property": "handoff completeness", "levels": ["missing essentials", "usable with follow-up", "ready for review"] },
  "answer": {
    "kind": "rating",
    "level": "usable with follow-up",
    "probabilities": { "missing essentials": 0.15, "usable with follow-up": 0.62, "ready for review": 0.23 },
    "value": 0.54
  },
  "assessment": { "status": "unsure", "value": null, "min_prob": 0.8 },
  "meta": {
    "backend": "jev",
    "url": "https://api.typesafe.ai/v1/systemone",
    "adapter": "systemone",
    "model": "jev-1.13.0",
    "usage": { "input_tokens": 208, "output_tokens": 32 }
  }
}
```

- `answer.level` is the level with the highest probability.
- `answer.probabilities` holds one entry per level, in the order the levels were given, lowest first.
- `answer.value` is the weighted position of the levels, from 0 at the lowest level to 1 at the highest. With `K` levels, it is the sum of each level's probability times its zero-based index, divided by `K` minus one. The example gives `(0 × 0.15 + 1 × 0.62 + 2 × 0.23) / 2`. That is 0.54.
- `assessment.value` is the accepted level name, or `null`.

`answer.value` is a position on the levels the user named. It is not a probability that the property holds, and it is not a confidence in the answer. A script that compares it across two different rubrics is comparing two different scales.

The answer is unsure when the winning level's probability falls under `min_prob`. `answer.value` still prints, because it is what the backend said.

## What a high probability does not mean

A decider model judges only the evidence it was shown. A probability of 0.98 says nothing about facts that were absent from the input. The only test of a question is a measurement against labeled cases.
