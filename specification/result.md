# The result

Status: **Settled** for the bare value, the object, the four answer kinds, the full distribution with `confidence`, and the two `annotate` digests. ADR 0010 accepted the last three. One open point is left at the foot of the page.

One internal result model feeds both views. The view never changes the request or the answer. Every probability and token count in an example here is illustrative.

## The bare value

| Command | Default standard output |
| --- | --- |
| `decide` | `true`, `false`, or `null` |
| `choose` | a JSON string, or `null`. `--raw` prints the bare label, as [choose.md](choose.md) describes |
| `tag` | a JSON array of every label that reaches the cut, including `[]` |
| `score` | a JSON number |
| `filter` | each kept record, byte for byte as it arrived, in input order |
| `rank` | each record as it arrived, most likely yes first |
| `annotate` | one JSON object per record |

Every value is compact and sits on one line, so one answer is also one record for `jq`, `grep`, and `wc -l`.

## `--details`

`--details` prints this object in place of the bare value.

```json
{"schema":"thinkthen.result/1","value":true,"question":{"verb":"decide","text":"Does this ask for a refund?"},"answer":{"kind":"yes_no","probability":0.92},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"982f...88","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}
```

- `value` is the bare value the command would have printed.
- `question` names the verb and the text the model received.
- `answer` is everything the backend said, in thinkthen's own words. No vendor field name appears in it.
- `threshold` is a number for a single cut, the string `"LOW:HIGH"` for a band, and `null` when none applies. `decide` never prints `null` here, because a rule always exists and the default is the cut of one half. [threshold.md](threshold.md) gives the rule.
- `meta` carries the run. Every field is always present, and `usage` alone may be absent.

## A detailed result keeps everything

Settled by ADR 0009 item 2, accepted in ADR 0010. `answer` carries the probability of every option or every level, and it carries the backend's own `confidence` when the backend reports one. A saved run can then be swept at another rule with no second request.

`confidence` is present only when the backend sends it. No cut is taken on it. [backends.md](backends.md) says why. The vendor sends no `confidence` on a yes/no answer, so a `yes_no` answer carries none. `sdlc/planning/interface-audit.md` found the page promising one, and this page no longer does.

## Four answer kinds

**`yes_no`**, from `decide`, `filter`, and `rank`. It carries `probability`, the probability of yes, and nothing else.

`find` fits none of the four kinds, and [find.md](find.md) holds that open point.

**`choice`**, from `choose`.

```json
{"schema":"thinkthen.result/1","value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02},"confidence":0.91},"threshold":0.8,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"5d5f...25","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":312,"output_tokens":48},"replayed":false}}
```

`answer.pick` is the option with the highest probability, before any threshold. `answer.probabilities` holds one entry per option sent, in the order the options were sent. `value` is the label that cleared the cut.

`value` is `null` when the answer is unresolved, and `answer.pick` still names the option that led. A script reads `value` and never `pick`. A person reading an unresolved row learns from `pick` what the model was leaning toward.

**`tag`**, from `tag`.

```json
{"schema":"thinkthen.result/1","value":["billing"],"question":{"verb":"tag","text":"Which topics?","labels":["billing","urgent"]},"answer":{"kind":"tag","probabilities":{"billing":0.91,"urgent":0.22}},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"00b0...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":208,"output_tokens":32},"replayed":false}}
```

`answer.probabilities` holds one entry per label in the order sent. `value` keeps every label whose probability reaches the one cut, in that order. An empty array is a complete successful answer.

**`score`**, from `score`.

```json
{"schema":"thinkthen.result/1","value":1.6,"question":{"verb":"score","text":"How much disruption does this report?","levels":["None.","Work continues with a workaround.","Work is blocked."]},"answer":{"kind":"score","level":"Work is blocked.","probabilities":{"None.":0.05,"Work continues with a workaround.":0.30,"Work is blocked.":0.65}},"threshold":null,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"005c...f5","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":208,"output_tokens":32},"replayed":false}}
```

`answer.level` is the level with the highest probability. `answer.probabilities` holds one entry per level, in the order the levels were given, lowest first. `value` is the weighted position on those levels, and [score.md](score.md) gives the arithmetic.

## `meta`

ADR 0010 dropped `profile` and `adapter` from this object. Profiles left version one, and one wire shape leaves nothing for an adapter name to tell a reader apart from.

| Field | Holds |
| --- | --- |
| `tool` | The name and version of the binary that made the row |
| `question_sha256` | The digest of the exact question the row answers, as [question-file.md](question-file.md) fixes it. The same question typed and read from a file gives one digest, and any override gives another |
| `url` | The URL that answered |
| `model` | The model that answered, as the backend reported it |
| `usage` | The token counts the backend reported. Absent when the backend reports none |
| `replayed` | `true` when a recording answered rather than a backend |

## Record rows

In record mode the object also carries `input`, the original record. `input` holds the whole record, including parts that were never sent. On `filter` and `rank`, `--details` prints these objects for the same records in the same order that the bare values would have taken.

```json
{"schema":"thinkthen.result/1","value":true,"input":{"id":"T-91","body":"Payouts have failed for 3 days."},"question":{"verb":"decide","text":"Does this report a payment failure?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":0.5,"meta":{"tool":"thinkthen 0.4.0","question_sha256":"a1e3...df","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":88,"output_tokens":12},"replayed":false}}
```

## `annotate`

`annotate --details` prints `schema`, `input`, `value` holding the named answers, `answers` holding the result for each name, and `meta`.

```json
{"schema":"thinkthen.result/1","input":{"id":"T-91","body":"Payouts have failed for 3 days."},"value":{"unresolved":true,"kind":"bug"},"answers":{"unresolved":{"value":true,"question":{"verb":"decide","text":"Is this still unresolved?"},"answer":{"kind":"yes_no","probability":0.97},"threshold":"0.1:0.9","request":"6b1f...c4"},"kind":{"value":"bug","question":{"verb":"choose","text":"Which kind of request is this?","options":["bug","feature","other"]},"answer":{"kind":"choice","pick":"bug","probabilities":{"bug":0.94,"feature":0.04,"other":0.02}},"threshold":0.8,"request":"6b1f...c4"}},"meta":{"tool":"thinkthen 0.4.0","questions_sha256":"9ad3...7e","url":"https://api.typesafe.ai/v1/systemone","model":"jev-1.13.0","usage":{"input_tokens":402,"output_tokens":60},"replayed":false}}
```

Each entry under `answers` carries the same `value`, `question`, `answer`, and `threshold` that a single judgment prints.

Settled by ADR 0008 item 3 and replaced in part by ADR 0027. `meta.questions_sha256` is the digest of the resolved canonical question set, so spacing, its path, and runtime backend settings do not change it. Each answer carries `request`, the digest that also names the recording entry. Two answers that rode in one request carry the same digest.

`meta.usage` is the sum over the record's requests.

## What a high probability does not mean

A decider model judges only the evidence it was shown. A probability of 0.98 says nothing about facts that were absent from the input. In one measurement the model approved every case at 0.98 while human reviewers had refused 23% of them. The only test of a question is a measurement against labeled cases.

Every reply behind one row must report the same model version. Different versions fail the record because one row cannot represent two measurements. The diagnostic safely names both short model identifiers when it can. It tells the user to pin `--model` and rerun with `--record` or `--cache`.
