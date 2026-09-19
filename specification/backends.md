# Backends

Status: **Settled** for the wire shape, the key, the address, the request, and the `systemone` adapter. **Draft** for profiles and the ad-hoc flags. **Draft** for the two adapter rows that wait on their fixtures.

`thinkthen` speaks one wire shape, System One, by ruling 1 of ADR 0010. Another model is reached by a server that presents that shape at another address. Every token count and probability in an example here is illustrative.

## The key

Settled by ADR 0010.

The key is read from `THINKTHEN_API_KEY` unless the hidden `--key-env` names another variable.

- A key variable that is absent or empty is exit code 4. An empty variable counts as absent. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.
- A key never crosses hosts. A run pointed at another address sends the key of the variable it was told to read, and nothing else.

## The address

Settled by ADR 0010.

The address comes from the hidden `--url`, then `THINKTHEN_BASE_URL`, then the default base `https://api.typesafe.ai/v1`. The tool posts to `BASE/systemone`. A base with a trailing slash is accepted. A base that is not an `http` or `https` address is a usage error before any request. An empty variable counts as absent.

`--dry-run` shows the address the run would use, and it reads no key.

## Profiles and ad-hoc backends

Draft. ADR 0010 proposes that the configuration file, profiles, `--profile`, `--adapter`, `--key-env`, and `--config` leave version one, because two variables and `--model` already say everything a profile held. [config.md](config.md) waits on Ian's answer, and nothing here is built until he gives it.

A backend profile is four values: the URL, the adapter, the model, and the name of the key variable. `--profile NAME` picks a named profile out of the configuration file. One profile is built in, and it holds the default address, the `systemone` adapter, the model `jev-latest`, and `THINKTHEN_API_KEY`.

An ad-hoc backend is a URL, an adapter, and a model given together on the command line. A new URL with a borrowed adapter or model is a guess, so a URL without both of the others is a usage error, and so is an adapter without a URL. A model alone may replace a profile's model. That is how a user pins a version.

An ad-hoc backend has no name, and a result reports its `meta.profile` as `null`. `--profile` beside `--url` is a usage error, because the profile would do nothing. A flag value that is empty or holds only white space is a usage error.

## The request

The adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`.

`--timeout SECONDS` covers one attempt from connect to the last byte. `--max-retries N` bounds the retries after the first attempt. The configuration file sets the default for each one, and `timeout_seconds` of 30 and `max_retries` of 2 apply when the file names neither.

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

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Version one compiles one adapter, `systemone`.

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
| A choice answer's probability per option | Draft. One probability per option name, under the `choice` answer |
| A score answer's probability per level | Draft. One probability per level, in level order, under the `score` answer |
| The backend's own confidence | `confidence`, kept and never cut on |
| A score's number | Computed locally from the level probabilities. Nothing is read from the wire |
| Several questions over one evidence | One `questions` map with one entry per question. One request, one `state` |

Question names are `q1`, `q2`, and onward in plan order. The vendor does not show names to the model.

The names carry the order. The order of keys inside the `questions` object and the `answers` object carries no meaning. Decode ignores an answer whose name the plan lacks. Decode refuses a response whose `model` is absent or blank, because a result must name the model that answered.

The two Draft rows name what the adapter must produce for `choose` and `score`. The exact response field names land with their fixtures under `fixtures/systemone/`, and those rows stop being Draft then.

### What the adapter keeps

Settled by ADR 0009 item 2, accepted in ADR 0010. The adapter keeps the full distribution and the vendor's `confidence` field. Both reach `answer` in the result, as [result.md](result.md) describes. A saved run can then be swept at another rule with no second request.

The cut on `choose` stays on the winning option's probability. That number exists on every backend, and a reader can say what it means. Most of the vendor's own pages cut on `confidence` instead, and the formula behind `confidence` is unpublished. A live sweep of both against labels settles whether the rule changes.
