# Backends

Status: **Settled** for profiles, keys, and the `systemone` adapter. **Draft** for `chat-logprobs` and the configuration file.

A backend is a URL and an adapter. Nothing in `thinkthen` is tied to one vendor.

## A profile

| Value | Meaning | Flag | Environment variable |
| --- | --- | --- | --- |
| URL | Where the request is posted | `--url` | `THINKTHEN_URL` |
| Adapter | The wire format the server speaks | `--adapter` | `THINKTHEN_ADAPTER` |
| Model | The model name sent in the request | `--model` | `THINKTHEN_MODEL` |
| Key variable | The name of the environment variable that holds the key | `--key-env` | `THINKTHEN_KEY_ENV` |

A flag value that is empty or holds only white space is a usage error. An environment variable that is set to the empty string counts as unset, the way most Unix tools read one. An environment variable that holds only white space is a usage error.

`--backend NAME` or `THINKTHEN_BACKEND` picks a named profile. A flag beats an environment variable, and an environment variable beats the profile.

An ad-hoc backend is a URL, an adapter, and a model given together. A URL names a server, an adapter names the language it speaks, and a model names what answers, so a new URL with a borrowed adapter or model is a guess. A URL without both of the others is a usage error, and so is an adapter without a URL. A model alone may replace a profile's model, which is how a user pins a version. An ad-hoc backend has no name, and results report its `backend` as `null`. Naming a profile with the `--backend` flag beside a URL is a usage error, because the profile would do nothing. A profile named only by `THINKTHEN_BACKEND` yields to an ad-hoc backend given by flags, because a flag beats an environment variable. That is the one pairing that yields. A name and a URL that both come from the environment beat nothing, and a name given by the `--backend` flag is refused beside a URL from either source. Named profiles come from a configuration file in a later version. One profile is built in:

| Name | URL | Adapter | Model | Key variable |
| --- | --- | --- | --- | --- |
| `jev` | `https://api.typesafe.ai/v1/systemone` | `systemone` | `jev-latest` | `TYPESAFE_API_KEY` |

The built-in profile is a row of data. It is the default when nothing else is named. A user who wants repeatable answers names an exact model version, and the result always reports the model that answered.

## Keys

- The key is read from the environment variable the profile names. No flag takes a key value.
- A key never crosses hosts. An ad-hoc backend takes nothing from a profile, its key variable included. The user names one with `--key-env` for the new host, or the request goes out with no key, which suits a local server.
- A key variable that is named and holds no value is exit code 4, for a profile and for an ad-hoc backend alike. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.

## The request

Every version-one adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`. `--timeout SECONDS` defaults to 30 and covers one attempt from connect to the last byte. `--max-retries N` defaults to 2. A retry happens after a transport failure or a status of 429, 500, 502, 503, 504, or 529. The wait doubles from one second, and no wait follows the last attempt. Any other error status fails at once. A failure after the last retry is exit code 4. The message gives the status code and never the response body, because a backend can quote the evidence back in an error. A fixed phrase follows the code for the common failures:

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

An adapter touches no network, no file, and no clock. Its tests are the fixture files under `fixtures/`. Decode refuses a reply that lacks an answer for a planned question, carries an answer of the wrong kind, or holds a probability outside zero to one. A refused reply is exit code 4.

## The `systemone` adapter

The first vendor's format. The request body:

```json
{
  "state": "Help! My payouts have been failing for 3 days.",
  "model": "jev-latest",
  "questions": {
    "q1": { "type": "noul", "instructions": "Does this convey urgency?" }
  }
}
```

The response body:

```json
{
  "model": "jev-latest",
  "answers": { "q1": { "type": "noul", "noul": 0.92 } },
  "usage": { "input_tokens": 312, "output_tokens": 48 }
}
```

| thinkthen | `systemone` |
| --- | --- |
| The evidence | `state`, as one string |
| A yes/no question and its condition | `type` `noul`, with the condition as `instructions` |
| A yes/no answer's probability | `noul` |
| Pick one from a list | `type` `choice`, with the options as the keys of `criteria` and each description as the value, or `null` |
| Rate on named levels | `type` `score`, with the levels as the `criteria` array |
| A pick-one answer's probability per option | Draft. One probability per option name, under the `choice` answer |
| A rating answer's probability per level | Draft. One probability per level, in level order, under the `score` answer |
| A rating's weighted value | Draft. Computed locally from the level probabilities. Nothing is read from the wire |
| Several questions over one evidence | One `questions` map with one entry per question. One request, one `state` |

Question names are `q1`, `q2`, and onward in plan order. The vendor does not show names to the model.

The names carry the order. The order of keys inside the `questions` object and the `answers` object carries no meaning. Decode ignores an answer whose name the plan lacks, and it ignores any field it does not use, such as the vendor's `confidence`. Decode refuses a response whose `model` is absent or blank, because a result must name the model that answered.

The four Draft rows above name what an adapter must produce for `decide which` and `decide how`. The exact response field names for a `choice` answer and a `score` answer land with their fixtures under `fixtures/systemone/`, and this table stops being Draft then. An adapter that cannot supply a probability per option or per level refuses the reply, and the exit code is 4. It never invents one.

## The `chat-logprobs` adapter

Draft. It will ask any server that speaks the common chat-completions format for a single constrained token and read the token probabilities. It makes a local model a backend. Its document lands before its ticket.
