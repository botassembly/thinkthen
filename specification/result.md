# The result

Status: **Settled** for `decide if`. **Draft** for the other verbs.

A single judgment prints one JSON document and a newline.

```json
{
  "schema": "thinkthen.result/1",
  "question": { "verb": "if", "condition": "asks for a refund" },
  "answer": { "kind": "yes_no", "probability": 0.92 },
  "assessment": { "status": "accepted", "value": true, "min_prob": 0.9 },
  "meta": {
    "backend": "jev",
    "adapter": "systemone",
    "model": "jev-1.13.0",
    "usage": { "input_tokens": 312, "output_tokens": 48 }
  }
}
```

## Three layers

- **answer** is what the backend said, in thinkthen's own words. No vendor field name appears in it.
- **assessment** is what local policy made of the answer.
- **meta** names the backend, the adapter, the model that answered as the backend reported it, and the usage the backend reported. `usage` is absent when the backend reports none.

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

## What a high probability does not mean

A decider model judges only the evidence it was shown. A probability of 0.98 says nothing about facts that were absent from the input. The only test of a question is a measurement against labeled cases.
