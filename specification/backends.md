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

`--backend NAME` or `THINKTHEN_BACKEND` picks a named profile. A flag beats an environment variable, and an environment variable beats the profile. Named profiles come from a configuration file in a later version. One profile is built in:

| Name | URL | Adapter | Model | Key variable |
| --- | --- | --- | --- | --- |
| `jev` | `https://api.typesafe.ai/v1/systemone` | `systemone` | `jev-latest` | `TYPESAFE_API_KEY` |

The built-in profile is a row of data. It is the default when nothing else is named. A user who wants repeatable answers names an exact model version, and the result always reports the model that answered.

## Keys

- The key is read from the environment variable the profile names. No flag takes a key value.
- A key never crosses hosts. When `--url` or `THINKTHEN_URL` replaces a profile's URL, the profile's key variable is dropped. The user names one with `--key-env` for the new host, or the request goes out with no key, which suits a local server.
- A missing key for a profile that names one is exit code 4. The message names the variable and never a value.
- No key appears in a plan, a result, a recording, a log line, or an error.

## The request

Every version-one adapter sends one `POST` with `Content-Type: application/json`. When a key is present it adds `Authorization: Bearer KEY`. `--timeout SECONDS` defaults to 30 and covers the whole exchange. `--max-retries N` defaults to 2. A retry happens after a transport failure or a status of 429, 500, 502, 503, 504, or 529. The wait doubles from one second. Any other error status fails at once. A failure after the last retry is exit code 4.

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

The four Draft rows above name what an adapter must produce for `decide which` and `decide how`. The exact response field names for a `choice` answer and a `score` answer land with their fixtures under `fixtures/systemone/`, and this table stops being Draft then. An adapter that cannot supply a probability per option or per level refuses the reply, and the exit code is 4. It never invents one.

## The `chat-logprobs` adapter

Draft. It will ask any server that speaks the common chat-completions format for a single constrained token and read the token probabilities. It makes a local model a backend. Its document lands before its ticket.
