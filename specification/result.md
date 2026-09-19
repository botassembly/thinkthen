# The result

Status: **Settled** for the bare value, the object, and the three answer kinds. **Draft** for `confidence`, `meta.tool`, and the two `annotate` digests.

One internal result model feeds both views. The view never changes the request or the answer. Every probability and token count in an example here is illustrative.

## The bare value

| Command | Default standard output |
| --- | --- |
| `decide` | `true`, `false`, or `null` |
| `choose` | a JSON string, or `null`. `--raw` prints the bare label, as [choose.md](choose.md) describes |
| `score` | a JSON number |
| `filter` | each kept record, byte for byte as it arrived, in input order |
| `rank` | each record as it arrived, most likely yes first |
| `segment` | one JSON object per segment with `start_line`, `end_line`, `start_unit`, `end_unit`, and `text` |
| `annotate` | one JSON object per record |
| `report` | one JSON object |

Every value is compact and sits on one line, so one answer is also one record for `jq`, `grep`, and `wc -l`.

## `--details`

`--details` prints this object in place of the bare value.

```json
{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Does this ask for a refund?"},"answer":{"kind":"yes_no","probability":0.92,"confidence":0.88},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}
```

- `value` is the bare value the command would have printed.
- `question` names the verb and the text the model received.
- `answer` is everything the backend said, in thinkthen's own words. No vendor field name appears in it.
- `threshold` is a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. `decide` never prints `null` here, because a rule always exists and the default is the cut of one half. [threshold.md](threshold.md) gives the rule.
- `meta` carries the run. Every field is always present, and `usage` alone may be absent.

## A detailed result keeps everything

Draft, from Proposed ADR 0009. `answer` carries the probability of every option or every level, and it carries the backend's own `confidence` when the backend reports one. A saved run can then be swept at another rule with no second request.

`confidence` is present only when the backend sends it. No cut is taken on it. [backends.md](backends.md) says why.

## Three answer kinds

**`yes_no`**, from `decide`, `filter`, `rank`, and `segment`. It carries `probability`, the probability of yes.

`find` fits none of the three kinds, and [find.md](find.md) holds that open point.

**`choice`**, from `choose`.

```json
{"schema":"thinkthen.result/1","value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02},"confidence":0.91},"threshold":0.8,"meta":{"tool":"thinkthen 0.4.0","profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}
```

`answer.pick` is the option with the highest probability, before any threshold. `answer.probabilities` holds one entry per option sent, in the order the options were sent. `value` is the label that cleared the cut.

`value` is `null` when the answer is unresolved, and `answer.pick` still names the option that led. A script reads `value` and never `pick`. A person reading an unresolved row learns from `pick` what the model was leaning toward.

**`score`**, from `score`.

```json
{"schema":"thinkthen.result/1","value":1.6,"question":{"verb":"score","text":"How much disruption does this report?","levels":["None.","Work continues with a workaround.","Work is blocked."]},"answer":{"kind":"score","level":"Work is blocked.","probabilities":{"None.":0.05,"Work continues with a workaround.":0.30,"Work is blocked.":0.65}},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":208,"output_tokens":32},"replayed":false}}
```

`answer.level` is the level with the highest probability. `answer.probabilities` holds one entry per level, in the order the levels were given, lowest first. `value` is the weighted position on those levels, and [score.md](score.md) gives the arithmetic.

## `meta`

| Field | Holds |
| --- | --- |
| `tool` | Draft, from Proposed ADR 0008. The name and version of the binary that made the row |
| `profile` | The backend profile's name, or `null` for an ad-hoc backend |
| `url` | The URL that answered |
| `adapter` | The adapter name, from [backends.md](backends.md) |
| `model` | The model that answered, as the backend reported it |
| `usage` | The token counts the backend reported. Absent when the backend reports none |
| `replayed` | `true` when a recording answered rather than a backend |

## Record rows

In record mode the object also carries `input`, the original record. `input` holds the whole record, including parts that were never sent. On `filter` and `rank`, `--details` prints these objects for the same records in the same order that the bare values would have taken.

```json
{"schema":"thinkthen.result/1","value":true,"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":{"verb":"decide","text":"Does this report a payment failure?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"replayed":false}}
```

## `annotate`

`annotate --details` prints `schema`, `input`, `value` holding the named answers, `answers` holding the result for each name, and `meta`.

```json
{"schema":"thinkthen.result/1","input":{"id":"T-91","body":"Payouts have failed for 3 days."},"value":{"unresolved":true,"kind":"bug"},"answers":{"unresolved":{"value":true,"question":{"verb":"decide","text":"Is this still unresolved?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":"0.1:0.9","request":"6b1f...c4"},"kind":{"value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02}},"threshold":0.8,"request":"6b1f...c4"}},"meta":{"tool":"thinkthen 0.4.0","questions_sha256":"9ad3...7e","profile":"jev","url":"https://api.typesafe.ai/v1/systemone","adapter":"systemone","model":"jev-1.13.0","usage":{"input_tokens":402,"output_tokens":60},"replayed":false}}
```

Each entry under `answers` carries the same `value`, `question`, `answer`, and `threshold` that a single judgment prints.

Draft, from Proposed ADR 0008. `meta.questions_sha256` is the digest of the definition file, so a saved row names the definition that made it. Each answer carries `request`, the digest that also names the recording entry, so a row can be traced to the exchange that produced it. Two answers that rode in one request carry the same digest.

`meta.usage` is the sum over the record's requests.

## What a high probability does not mean

A decider model judges only the evidence it was shown. A probability of 0.98 says nothing about facts that were absent from the input. In one measurement the model approved every case at 0.98 while human reviewers had refused 23% of them. The only test of a question is a measurement against labeled cases.

## Open points

- How does `meta` report an `annotate` record whose requests were answered by different model versions? A run that mixes versions is two measurements. Recommendation: `meta.model` becomes the list of distinct versions when more than one answered, and `report` names such a run.
