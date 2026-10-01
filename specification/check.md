# Check

Status: **Settled** by ticket 0121.

`thinkthen check` answers one question: does the backend at an address you name work with this tool? It sends four fixed requests to `BASE/systemone`, one after another. Each finding is critical or a warning. The check exits 0 only when nothing is critical, so a script or a list of backends can trust the exit code. The report opens with the address, the provider, and the models, and it prints each decoded reply.

Each request is built by the production question grammar and written by the production encoder. The engine sends it with the production transport, retry rule, and key rule. The production decoder reads the reply. A critical finding therefore means what a real run would meet, and it carries the sentence a real run would print.

## Command line

```text
thinkthen check [--url BASE] [--backend NAME] [--model NAME] [--timeout SECONDS] [--plan]
```

- The address comes from `--url` or `--backend`, then `THINKTHEN_BASE_URL` or `THINKTHEN_BACKEND`, then the configuration file's `url` or `backend`, by the tiers of [backends.md](backends.md#named-backends). A named backend counts as a named address. The rules of [backends.md](backends.md) apply unchanged. The built-in default address is refused, because the check would otherwise spend requests at the hosted service when you named nothing. The refusal exits 2 before the key is read and sends nothing. Its whole standard error line reads `thinkthen: check needs an address you name: give --url or --backend, set THINKTHEN_BASE_URL or THINKTHEN_BACKEND, or set url or backend in the configuration file`.
- `--model` resolves as every command resolves it: the option, then the configuration file's `model`, then `jev-1.13.0`. `model asked` prints the option or the file's value, or `unspecified` when neither names one. `model sent` prints the resolved value, which every request carries.
- The key comes only from `THINKTHEN_API_KEY`, or from a named backend's own key variables. An unset or blank key exits 4 with the sentence every command prints, before any request, unless the address is `localhost`, `127.0.0.1`, or `[::1]`. There the probes go out with no `Authorization` header, as on every command.
- `--timeout` works as it does everywhere. `--max-retries` keeps its default of 3 and is not accepted. Four probes send at most sixteen attempts. Each attempt takes at most the timeout, and each wait before an attempt at most the lesser of the timeout and 60 seconds, because a longer server wait fails the probe ([backends.md](backends.md)). Twelve retry waits and three waits between probes give fifteen waits, so at `--timeout 90` a check ends within 16 × 90 + 15 × 60 = 2,340 seconds, plus at most one second for another process's usage lock. Without retry headers the waits double from one second, and the bound is 16 × 90 + 4 × 7 + 3 × 8 = 1,492 seconds.
- The check reads no standard input and no cache, recording, replay, or profile. `--cache`, `--no-cache`, `--record`, `--replay`, and `--profile` are unknown options and exit 2.
- Its attempts and reported tokens count in the usage totals, as every live request does.
- `THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL` binds each probe as it binds every live request, by [backends.md](backends.md). `check` has no flag for it. A probe the cap cannot admit stops the check before that probe is sent: the command prints the asking commands' sentence, such as `thinkthen usage: max_estimated_input_tokens_total=1 (encoded-body-bytes-908-v1) would be exceeded before this call's first request`, prints no report, and exits 2.

## The four probes

Every probe asks about the same made-up parcel delivery. The text evidence is `The parcel arrived on Tuesday and the box was intact.`

| Probe | Serves | Field forms it covers |
| --- | --- | --- |
| `noul` | `decide`, `filter`, `rank`, and yes/no questions of `recognize` and `relate` | Text `state`. String `instructions`. `criteria.true` as a string. A present null `false` input is omitted from the wire |
| `choice` | `choose`, `find`, and choice questions of `recognize` | `criteria` as a map with a `null`, a string, and an object description |
| `score` | `score` | `criteria` as an array holding an empty object for a `null` description, a string, and an object |
| `mixed` | `tag`, `annotate`, and many-question requests | Object `state`. Five wire questions of three types. A `noul` with no `criteria`. Bare level names. The structured tag form: array `instructions` and a `criteria.true` object |

The four bodies at the default model, in probe order, as [fixtures/check/requests.jsonl](fixtures/check/requests.jsonl) holds them:

```json
{"state":"The parcel arrived on Tuesday and the box was intact.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Did the parcel arrive undamaged?","criteria":{"true":"The text says the box or its contents were intact."}}}}
{"state":"The parcel arrived on Tuesday and the box was intact.","model":"jev-1.13.0","questions":{"q1":{"type":"choice","instructions":"Which day did the parcel arrive?","criteria":{"Monday":null,"Tuesday":"The second working day of the week.","Wednesday":{"what":"The middle of the week","examples":["midweek"]}}}}}
{"state":"The parcel arrived on Tuesday and the box was intact.","model":"jev-1.13.0","questions":{"q1":{"type":"score","instructions":"How well was the parcel packed?","criteria":[{},"The box was dented but the contents were fine.",{"what":"The box was intact","not_for":"a dented box"}]}}}
{"state":{"note":"The parcel arrived on Tuesday.","box":"intact"},"model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Did it arrive on time?"},"q2":{"type":"choice","instructions":"Which day?","criteria":{"Monday":null,"Tuesday":null}},"q3":{"type":"score","instructions":"How was the packing?","criteria":["poor","good"]},"q4":{"type":"noul","instructions":["Which labels fit?",{"label":"on_time","description":{"what":"It arrived when promised"}}],"criteria":{"true":{"what":"It arrived when promised"}}},"q5":{"type":"noul","instructions":["Which labels fit?",{"label":"damaged","description":"The box or its contents were harmed."}],"criteria":{"true":"The box or its contents were harmed."}}}}
```

Under `--backend ollama`, the four bodies carry model `nimble` and each description as text, the Ollama workaround in [backends.md](backends.md#named-backends). [fixtures/check/requests-text.jsonl](fixtures/check/requests-text.jsonl) holds them. `choice`, `score` and `mixed` each send a description object as text, so a live check under `ollama` adds one warning to each of those rows, and `--plan` prints the same sentence once on standard error after `thinkthen: `. Neither changes the exit code.

## Findings

The report has eight rows, always in this order: `connection`, `key`, `endpoint`, `noul`, `choice`, `score`, `mixed`, `usage`. A row with no finding prints `ok ROW`. A row the check did not reach prints `unchecked ROW`. A finding prints `critical ROW: SENTENCE` or `warning ROW: SENTENCE`, one line each, in wire order.

| Row | Level | When | Sentence |
| --- | --- | --- | --- |
| `connection` | critical | The first probe meets a transport failure | The transport sentence every command prints |
| `key` | critical | The first probe meets status 401, 402, or 403 | The status sentence every command prints |
| `endpoint` | critical | The first probe meets status 404 | `the backend answered with status 404: nothing answers at this address` |
| probe | critical | Any other error status after the allowed attempts | The status sentence every command prints |
| probe | critical | The decoder refuses the whole reply | `the reply was refused: ` and the decoder's sentence |
| probe | critical | The reply passes its request's limit | The reply-limit sentence every command prints |
| probe | critical | The reply fails one logical question and keeps another | ``the answer to questions `qA` to `qB` failed as `CAUSE` ``. A one-wire question reads ``the answer to question `qA` failed as `CAUSE` ``. The range is the wire questions that logical question covers. CAUSE is the failure cause a result names |
| probe | critical | A later probe meets a transport failure or status 401 to 404 | The same sentence as the first-probe rows |
| probe | warning | A `choice` or `score` answer carries no `confidence` | ``the answer to question `qN` carries no confidence`` |
| `usage` | warning | Any decoded reply carries no `usage` | `a reply carries no token counts, so results and usage totals leave them out` |
| probe | warning | The `ollama` backend sent one of the probe's description objects or lists as text | ``backend `ollama` sends each description object as its `what` text, a temporary workaround for an Ollama bug, so its other fields are left out`` |

A transport failure or status 401, 402, 403, or 404 stops the check, because every later request would meet it too. Any other failure belongs to its probe, and the check goes on. The first probe decides `connection`, `key`, and `endpoint`. A reply of any status marks `connection` ok, and any status but 401 to 404 also marks `key` and `endpoint` ok. After a stop, every row not yet decided prints `unchecked`. So a 404 at the first probe prints `ok connection`, `unchecked key`, and `critical endpoint`. `usage` is ok when every decoded reply carried it, and unchecked when no reply was decoded.

A finding never quotes a reply. It names a question by its wire name and nothing the backend sent back. The check does not compare the model a reply names with the model sent, because an alias such as `jev-latest` answering as a version is normal. A reply line prints the decoded reply as JSON and never its raw bytes. It names the model that reply names.

## Output and exit codes

A full pass prints exactly these lines, with the resolved address and model:

```text
url http://127.0.0.1:PORT/arm/full/v1/systemone
provider systemone
model asked unspecified
model sent jev-1.13.0
reply noul {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}
reply choice {"model":"jev-1.13.0","answers":[{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.05,"Wednesday":0.05},"confidence":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}
reply score {"model":"jev-1.13.0","answers":[{"kind":"score","level":"fair","probabilities":{"fair":0.9,"good":0.05,"excellent":0.05},"confidence":0.9}],"usage":{"input_tokens":1,"output_tokens":1}}
reply mixed {"model":"jev-1.13.0","answers":[{"kind":"yes_no","probability":0.9},{"kind":"choice","pick":"Monday","probabilities":{"Monday":0.9,"Tuesday":0.1},"confidence":0.9},{"kind":"score","level":"poor","probabilities":{"poor":0.9,"good":0.1},"confidence":0.9},{"kind":"tag","probabilities":{"on_time":0.9,"damaged":0.9}}],"usage":{"input_tokens":1,"output_tokens":1}}
ok connection
ok key
ok endpoint
ok noul
ok choice
ok score
ok mixed
ok usage
critical 0, warning 0
```

`provider` names the wire interface the tool speaks, `systemone`. It does not name who runs the server, because the tool knows nothing about that.

Each `reply PROBE JSON` line holds one decoded reply: `model` is the model that reply names, `answers` lists each logical answer in plan order as a result prints `answer`, and `usage` is the reply's token counts or `null`. A failed logical question prints the failure marker a result prints. The JSON writer escapes every control character in a model name.

| Probe outcome | Reply line | Finding row |
| --- | --- | --- |
| Decoded, every answer good | yes | `ok` or a warning |
| Decoded, one logical question failed | yes, with that answer as a failure | `critical` |
| The decoder refused the reply | no | `critical` |
| An error status or a transport failure | no | `critical` |
| Not reached after a stop | no | `unchecked` |
| The reply names another model than the one sent | yes, with the reply's model | no finding |

The last line counts the finding lines. The report prints once, after the last probe. On SIGINT or SIGTERM the check prints no report and follows the interrupt rule of [channels.md](channels.md).

| Exit | When |
| --- | --- |
| 0 | The report holds no critical line. Warnings may stand |
| 2 | No named address, an address the rules refuse, a blank model, a timeout outside 1 to 86400, or an unknown option. Nothing is sent |
| 4 | The report holds a critical line, or the key is unset or blank at an address other than loopback. An unset key prints nothing on standard output |
| 5 | Standard output could not be written |
| 70 | A defect |

`--plan` prints the `url`, `provider`, `model asked`, and `model sent` lines, then one `request PROBE BODY` line per probe and a JSON count line for the four prepared requests. It prints no reply line. The bodies come from the same split the live check sends. It inspects the optional configured key for an address collision first. It requires no key, sends nothing, and exits 0 for a safe address.

## What the check cannot see

One reply cannot show whether a backend reads a description, a criterion, or the option order. A backend that accepts a description and ignores it passes. The check does not probe limits on questions, options, or request bytes. A backend profile states those.
