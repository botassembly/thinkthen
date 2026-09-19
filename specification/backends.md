# Backends

Status: **Settled** for the wire shape, the key, the address, the request, the `systemone` adapter, and every adapter row.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

A backend is two values: the address and the model. `THINKTHEN_BASE_URL` names the address, `THINKTHEN_API_KEY` holds the key, and `--model` names the model. ADR 0010 took the configuration file and the four options that served it out of version one. [roadmap.md](roadmap.md) names each one, says what the file held, and says what would bring it back.

## The key

Settled by ADR 0010.

The key is read from `THINKTHEN_API_KEY`. No option names another variable.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.
- **The key goes to the address the user named.** That is the whole rule. Naming an address is the user's own act, so a run pointed at another address carries the key of `THINKTHEN_API_KEY` and nothing else.

## The address

Settled by ADR 0010.

The address comes from the hidden `--url`, then `THINKTHEN_BASE_URL`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

`--url` names a base and takes no companion option. It is the command-line spelling of `THINKTHEN_BASE_URL`, it outranks the variable, and the key rule above does not change when it is given. It stays hidden from the short help, because a variable in front of the command is the everyday way to point a run somewhere else.

Space around a base is dropped. A scheme is read without regard to case and written back in lower case, so one exchange keeps one recording digest whatever case the caller typed. A base carrying user information, a query, or a fragment is a usage error, because the address is printed in a plan and kept in a recording. The refusal message names the rule and never the base it refused. A base that is empty or holds only white space is a usage error too.

`--dry-run` shows the address the run would use, and it reads no key.

## The model

`--model NAME` names the model the request carries, and it defaults to `jev-latest`. That is how a run is pinned to one version. A model name that is empty or holds only white space is a usage error.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`.

`--timeout SECONDS` covers one attempt from connect to the last byte, and it defaults to 30. `--max-retries N` bounds the retries after the first attempt, and it defaults to 2.

A retry happens after a transport failure or a status of 429, 500, 502, 503, 504, or 529. The wait doubles from one second, and no wait follows the last attempt. Any other error status fails at once.

A failure after the last retry is exit code 4. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code.

| Status | Phrase |
| --- | --- |
| 401 | the key was refused |
| 402 | the account has no credit |
| 403 | the key may not use this model or address |
| 404 | nothing answers at this address |
| 422 | the backend refused the request as malformed or too large |
| 429 | the backend's rate limit was reached |

## The adapter contract

An adapter is two pure functions.

- **encode** takes a plan and returns the request body as bytes. A plan holds the evidence, the model name, and an ordered list of named questions.
- **decode** takes the response body as bytes and returns the model that answered, one answer per named question, and the usage the backend reported.

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Version one compiles one adapter, `systemone`, and nothing selects it. Ruling 1 of ADR 0010 left one wire shape, so there is nothing to choose between. A recording entry still names `systemone`, so an entry written today says which shape it recorded.

Decode refuses a reply that lacks an answer for a planned question, carries an answer of the wrong kind, or holds a probability outside zero to one. A refused reply is exit code 4. An adapter that cannot supply a probability per option or per level refuses the reply. It never invents one.

## The `systemone` adapter

The first vendor's format. The request body:

```json
{"state":"Help! My payouts have been failing for 3 days.","model":"jev-latest","questions":{"q1":{"type":"noul","instructions":"Does this convey urgency?"}}}
```

The response body:

```json
{"model":"jev-latest","answers":{"q1":{"type":"noul","noul":0.92}},"usage":{"input_tokens":312,"output_tokens":48}}
```

| thinkthen | `systemone` |
| --- | --- |
| The evidence | `state`, as one string |
| A yes/no question and its text | `type` `noul`, with the question as `instructions` |
| A yes/no answer's probability | `noul` |
| Pick one from a list | `type` `choice`, with the options as the keys of `criteria` and each description as the value, or `null` |
| Place on named levels | `type` `score`, with the levels as the `criteria` array |
| A choice answer's probability per option | `probabilities` under the `choice` answer, keyed by option name. The key order carries no meaning, and the adapter rebuilds the distribution in the order the options were sent |
| A score answer's probability per level | `probabilities` under the `score` answer, keyed by the level's position as a string, counting from `"0"`. The adapter maps each key back to the level text it sent |
| The backend's own confidence | `confidence`, kept and never cut on |
| A score's number | Computed locally from the level probabilities. Nothing is read from the wire |
| Several questions over one evidence | One `questions` map with one entry per question. One request, one `state` |

Question names are `q1`, `q2`, and onward in plan order. The vendor does not show names to the model.

The names carry the order. The order of keys inside the `questions` object and the `answers` object carries no meaning. Decode ignores an answer whose name the plan lacks. Decode refuses a response whose `model` is absent or blank, because a result must name the model that answered.

The vendor also sends `choice`, `score`, and `legend` beside the probabilities. Each one is derivable from the distribution and the question that was asked, so the adapter computes them and reads none of them. The fixtures under `fixtures/systemone/` hold a real response of each kind.

### What the adapter keeps

Settled by ADR 0009 item 2, accepted in ADR 0010. The adapter keeps the full distribution and the vendor's `confidence` field. Both reach `answer` in the result, as [result.md](result.md) describes. A saved run can then be swept at another rule with no second request.

The cut on `choose` stays on the winning option's probability. That number exists on every backend, and a reader can say what it means. Most of the vendor's own pages cut on `confidence` instead, and the formula behind `confidence` is unpublished. A live sweep of both against labels settles whether the rule changes.
