# Backends

Status: **Settled** for profiles, keys, and the `systemone` adapter. **Draft** for `chat-logprobs`.

A backend is a URL and an adapter. Nothing in `thinkthen` is tied to one vendor. Every token count and probability in an example here is illustrative.

## A profile

A backend profile is four values.

| Value | Meaning | Flag |
| --- | --- | --- |
| URL | Where the request is posted | `--url` |
| Adapter | The wire format the server speaks | `--adapter` |
| Model | The model name sent in the request | `--model` |
| Key variable | The name of the environment variable that holds the key | `--key-env` |

`--profile NAME` picks a named profile. Profiles come from the configuration file, and [config.md](config.md) gives the file, the selection order, and the two environment variables the tool reads. The five `THINKTHEN_*` backend variables of ADR 0004 are gone.

A flag value that is empty or holds only white space is a usage error.

One profile is built in.

| Name | URL | Adapter | Model | Key variable |
| --- | --- | --- | --- | --- |
| `jev` | `https://api.typesafe.ai/v1/systemone` | `systemone` | `jev-latest` | `TYPESAFE_API_KEY` |

The built-in profile is a row of data. It answers when nothing else is named, and a file profile named `jev` replaces it. A user who wants repeatable answers names an exact model version, and the result always reports the model that answered.

## An ad-hoc backend

An ad-hoc backend is a URL, an adapter, and a model given together on the command line. A new URL with a borrowed adapter or model is a guess, so a URL without both of the others is a usage error, and so is an adapter without a URL. A model alone may replace a profile's model. That is how a user pins a version.

An ad-hoc backend has no name, and a result reports its `meta.profile` as `null`. `--profile` beside `--url` is a usage error, because the profile would do nothing.

## Keys

- The key is read from the environment variable the profile names. No flag takes a key value.
- A key never crosses hosts. An ad-hoc backend takes nothing from a profile, its key variable included. The user names one with `--key-env` for the new host, or the request goes out with no key. No key suits a local server.
- A key variable that is named and holds no value is exit code 4, for a profile and for an ad-hoc backend alike. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error. The configuration file never holds a key.

## The request

Every version-one adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`.

`--timeout SECONDS` covers one attempt from connect to the last byte. `--max-retries N` bounds the retries after the first attempt. The configuration file sets the default for each one, and `timeout_seconds` of 30 and `max_retries` of 2 apply when the file names neither.

A retry happens after a transport failure or a status of 429, 500, 502, 503, 504, or 529. The wait doubles from one second, and no wait follows the last attempt. Any other error status fails at once.

A failure after the last retry is exit code 4. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code for the common failures.

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
- **decode** takes the response body as bytes and returns the model that answered, one answer per named question, and the usage if the backend reports it.

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. The set of adapters is an enum compiled into the binary.

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
| A score's number | Computed locally from the level probabilities. Nothing is read from the wire |
| Several questions over one evidence | One `questions` map with one entry per question. One request, one `state` |

Question names are `q1`, `q2`, and onward in plan order. The vendor does not show names to the model.

The names carry the order. The order of keys inside the `questions` object and the `answers` object carries no meaning. Decode ignores an answer whose name the plan lacks, and it ignores any field it does not use. Decode refuses a response whose `model` is absent or blank, because a result must name the model that answered.

The two Draft rows name what the adapter must produce for `choose` and `score`. The exact response field names land with their fixtures under `fixtures/systemone/`, and those rows stop being Draft then.

## The `chat-logprobs` adapter

Draft. It asks any server that speaks the common chat-completions format for one constrained token and reads the token probabilities. It makes a local model a backend, and it lets a user run with no hosted service at all. It costs no vendor credits, so it moves ahead of the record verbs in the plan.

A yes/no question sends the question as the user message, asks for one token, and requests the top token probabilities. The probability of yes is the mass on the yes token, normalized over the yes token and the no token alone.

A choice sends the options as a numbered list and constrains the answer to one digit per option. The probability per option is the mass on that option's digit, normalized over the digits offered.

A score sends the levels as a numbered list and reads them the same way. Nothing else is read from the reply, and no free text is parsed.

### Open points

- Does a choice read digits, or the first character of each label? Digits work for any label and stay one token. Labels that share a first letter break the other reading. Recommendation: digits.
- What happens when a constrained token is missing from the returned probabilities? Recommendation: treat the missing mass as zero and refuse the reply when every offered token is missing.
- How does one request carry several questions, given that a chat completion answers one? Recommendation: one request per question, and the tool sums the usage. `annotate` therefore costs more against this adapter than against `systemone`.
- Which fields name the probabilities? They land with the fixtures under `fixtures/chat-logprobs/`.
