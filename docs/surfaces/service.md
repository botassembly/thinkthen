# The JSON request and the result row

ThinkThen is a classifier you call like a function. It reads text, answers one bounded question, and hands back a result you can branch on.

Under every library and the command there is one JSON contract. A **request** holds one question and the records to ask it about. The answer is one **row** per record, each a JSON object on a line of its own. Read this page when you write or check a library, save rows to a file, or read rows another program wrote. A scalar SDK call promotes a failed parent result to its host error instead of returning it. This does not change the JSON contract: a request with one message still uses the same array and result shapes. The exact field lists are the generated schemas: `request.schema.json`, `row.schema.json` and `contract.json`.

Every row is in exactly one of three states, named by its `status`:

- **`answered`**: there is an answer, and `answer` is it.
- **`unsure`**: the model's number fell between the two bars you set, or no choice was likely enough.
- **`failed`**: the record could not be judged, or the service did not respond.

## What every row guarantees

A library turns a row into its own result type, and it relies on these rules to do that. They hold for every function on this page.

1. **`function`, `position`, `source` and `status` are on every row.** `status` is exactly `"answered"`, `"unsure"` or `"failed"`, and nothing else.
2. **`answer` is present exactly when `status` is `"answered"`.** On a not-sure or failed row the key is left out. It is never written as `null`, `false`, `0`, `""` or `[]`, so a missing answer can't be read as "no" or as an empty value.
3. **The only `null` answer is find's.** `"answer": null` on an answered find row means "no line answers". That is an answer, not a missing one.
4. **`probability` is never on a failed row and always on a not-sure row.** On an answered row it is present for decide, filter, choose and find, and left out for tag, score, rank, recognize, relate and annotate, which have no single number.
5. **`error` is present exactly when `status` is `"failed"`.** It always has a `code` and a `message`. The `code` is one of the eight record and service codes listed under [Errors](#errors). `http_status` is there only when the service answered with one.
6. **Only decide, choose and find can be `"unsure"`,** and decide only when the request sets two bars. A filter, tag, score, rank, recognize, relate or annotate row is answered or failed.
7. **`position` counts the request's records from 1,** in the order you sent them. It is the record's place in your input even when the rows come back in another order, as rank's do.
8. **`id` is present exactly when the record has one,** and it is the value you sent: a string stays a string, and a whole number stays that number.
9. **Every record gets one row, in the order of the records.** A bad record gets a failed row and the rows after it still come. rank is the one exception: its rows come in rank order, and with `top` set the records ranked below the top get no row.
10. **A key with no value is left out, not set to `null`.** Rule 3 is the only `null` in a row. A row has no keys beyond the ones on this page. The `thinkthen` command adds one of its own when it writes the rows of a many-record call with `--json`: `item`, the record itself. The service never sends it.
11. **A mistake in the request never comes back as a row.** It comes back as one error for the whole call, and no row carries a call's error code.
12. **`start` and `end` count characters from 0,** and `end` is one past the last character. That holds for every saved row, whichever library or command wrote it.

The answer has one JSON type per function:

| `function` | `answer` on an answered row | `detail`, when present |
| --- | --- | --- |
| `decide` | `true` or `false` | none |
| `filter` | `true` or `false` | none |
| `choose` | a string: exactly one of the request's `choices` | `labels`: every choice and its probability |
| `tag` | an array of strings from the request's `labels`, in the request's order, possibly empty | `labels`: every label and its probability |
| `score` | a number from 0 to 1 | `levels`: one probability per level, in the request's order |
| `rank` | a whole number from 1 | `levels`, as for score |
| `find` | an object with `text`, `start` and `end`, or `null` | none |
| `recognize` | an array of names, possibly empty | `pieces` and `names`: the probabilities behind the names |
| `relate` | an array of relations, possibly empty | `names`: every name found. `pairs`: every pair that was checked |
| `annotate` | an object: one entry per named question | none |

## decide

### The simplest if

Send one question and one record.

```json
{
  "question": {
    "function": "decide",
    "question": "Does the customer request a refund?"
  },
  "records": ["Please refund my order."]
}
```

One row comes back. Read its status before its answer.

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

`source` identifies the actual question, service and model. It is on every row, including failed ones. Preserve it when saving results.

Answered no:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

Failed:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A failed row has no `answer` key, so it can never read as no. With one bar, a decide row is never not sure.

### Yes, no, or something else

Set two bars and decide can also be not sure. `status` tells you whether there is a yes or no to read.

```json
{
  "question": {
    "function": "decide",
    "question": "Does the customer request a refund?",
    "threshold": {"low": 0.2, "high": 0.8}
  },
  "records": ["Where is my parcel?"]
}
```

The row is one of these:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

When `status` is `"answered"`, `answer` is `true` for the refund queue and `false` for the inbox. Any other `status` means there is no answer, and a person reads it.

### Not sure or failed

The two ways of having no answer are told apart by `status` alone.

| `status` | `answer` | `probability` | `error` | Where it goes |
| --- | --- | --- | --- | --- |
| `"answered"` | `true` | present | left out | refund queue |
| `"answered"` | `false` | present | left out | inbox |
| `"unsure"` | left out | present | left out | a person reads it |
| `"failed"` | left out | left out | present | try again later |

Yes and no are the two answers. Not sure and failed are the two ways of having none. `status` has three values and no others, so a `switch` over the three names every state.

### Did it work

A row worked unless its `status` is `"failed"`. You can tell without looking for an answer.

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

The command and every library print a failed result as `failed:`, its stable code and the actual service reason. They print a not-sure row as `unsure`, and an answered row as its answer, in the words of the table at the top of the command's guide. Not sure counts as working: ThinkThen did its job, and the answer was "not sure".

### Raise if it failed

JSON can't raise. A library's explicit raise-if-failed step on bulk results, forms or their entries reads `status` and, when it is `"failed"`, `error`. On an answered annotate form it also checks each entry: the step raises if the form failed or any entry failed, but not for a not-sure entry. Otherwise it returns the same form. Scalar annotate promotion checks only the parent; an answered parent can still contain failed entries. Rule 5 holds for each entry as well as the outer row, so every failed entry carries its own code and message.

### Both bars and the probability

`threshold` takes one of two forms.

```json
{"function": "decide", "question": "Does the customer request a refund?", "threshold": 0.7}
```

```json
{"function": "decide", "question": "Does the customer request a refund?", "threshold": {"low": 0.2, "high": 0.8}}
```

- No `threshold`: one bar at 0.5. The answer is always yes or no.
- `"threshold": 0.7`: one bar. Yes at 0.7 or above, otherwise no.
- `"threshold": {"low": 0.2, "high": 0.8}`: two bars. No below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above.

A bar is a number above 0 and at most 1, and `low` must be below `high`. Anything else is refused with `usage.threshold`.

`probability` is the number ThinkThen compared with your bars: the model's probability of yes. A confident no has a low probability, such as `0.05`. On a decide row it is missing only when the row failed. It is written at full precision, so read it as a 64-bit float.

The number is the model's own. To choose bars for your data, send examples you have already labelled in one decide request, save each row's `probability`, and compare with your labels.

## choose with your own choices

Give your choices as an array of strings. The answer comes back as one of them, spelled exactly as you sent it.

```json
{
  "question": {
    "function": "choose",
    "question": "What is the message about?",
    "choices": ["billing", "shipping", "other"]
  },
  "records": ["I was charged twice for one invoice."]
}
```

Answered, not sure and failed:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

choose can be not sure even when you set no bar. Its bar is 0.5 on the best choice, and with three choices all three can fall below it. So check `status` before reading `answer`. `"threshold": 0.3` moves the bar. choose takes one bar only.

The strings in `choices` are the words the model reads, so make them plain words or short phrases, not codes. A library that offers its language's enumerated type sends the members' text here and turns `answer` back into a member. That works because `answer` is always exactly one of the strings in the request.

`probability` is the best choice's. `detail.labels` holds every choice with its probability.

## Many records

`records` is an array, and one request can hold as many as you like. Every function takes many records the same way.

```json
{
  "question": {
    "function": "decide",
    "question": "Does the customer request a refund?",
    "threshold": {"low": 0.2, "high": 0.8}
  },
  "records": [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice."
  ]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

One failed record does not stop the rows. Had the second record been `null`, its row would be this, with the first and third unchanged:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A row does not repeat your record. It carries `position`, counted from 1, and the record's `id` when it has one. A library keeps the records it sent and attaches each to its row by `position`, so its caller never matches by hand. The command does the same and writes the record into each row as `item`.

Send the whole collection in one request so ThinkThen can send the records to the service in batches. An empty `records` array gives no rows and sends nothing.

To save every result, write each row as a line of JSON. It includes the state, so nothing is lost.

## filter

filter means "give me the yeses". On the wire it is decide with one bar: every record gets a row, and the records to keep are the ones whose row is answered `true`.

```json
{
  "question": {
    "function": "filter",
    "question": "Does the customer request a refund?"
  },
  "records": [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice."
  ]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A library hands its caller records 1 and 3, in order.

filter takes one bar, so every record is a yes or a no and none is left out for being not sure. `"threshold": 0.7` moves the bar. Two bars are refused. This request gives no rows and one error:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

If a record fails, its row is a failed row in its place, and the other rows still come:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A library gives its caller every kept record first, then reports the failed rows in one error.

## tag

Labels work as choices do. The answer is an array of the labels that fit, in the order you gave them, and it can be empty.

```json
{
  "question": {
    "function": "tag",
    "question": "Which topics appear?",
    "labels": ["billing", "shipping"]
  },
  "records": ["I was charged twice and my parcel is late again."]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

tag is never not sure. `threshold` sets how likely a label must be to count, and takes one bar only. An empty array is an answer: no label fit. A tag row has no `probability`. Each label's own number is in `detail.labels`.

## score

Levels name a scale, lowest first. The answer is a number from 0 to 1 along it.

```json
{
  "question": {
    "function": "score",
    "question": "How urgent is the request?",
    "levels": ["routine", "urgent"]
  },
  "records": ["Please refund my order."]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

The score is a position between your levels, not one of them, so it is a number and never a level's name. To get exactly one level back, ask with choose. score is never not sure, takes no `threshold`, and its row has no `probability`. `detail.levels` holds one probability per level, in the order of the request.

## rank

rank puts many records in order along your levels, highest first.

```json
{
  "question": {
    "function": "rank",
    "question": "How urgent is the request?",
    "levels": ["routine", "urgent"]
  },
  "records": [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice."
  ]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

`answer` is the rank, starting at 1, and `position` says which of your records it is. Here the third message ranks first. The rows come in rank order, and none comes until every record has been read. Records that tie keep the order you sent them in.

`"top": 5` selects the five highest-ranked messages after judging the collection, and lower-ranked messages get no result. A failed record comes back after the ranked ones, as a failed row:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

rank is never not sure, takes no `threshold`, and its row has no `probability`.

## find

find points to the one line that answers.

```json
{
  "question": {
    "function": "find",
    "question": "Which line states the requested action?"
  },
  "records": ["Morgan works for Example Group.\nPlease refund my order."]
}
```

A line answers, no line answers, not sure, and failed:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

The answer is an object with `text`, `start` and `end`, or `null` when no line answers. `start` and `end` count characters from 0 in the text that was judged, and `end` is one past the last character. A library changes them to the unit its own strings use, and changes them back when it saves a row.

`"answer": null` with `"status": "answered"` means ThinkThen looked and no line answers. It is the only `null` answer in the contract. A row with no `answer` key at all is not sure or failed. Keep the two apart when you decode.

Like choose, find can be not sure with no bar set. `probability` is the best option's, where "no line" is one of the options. find takes one bar only.

## recognize

recognize takes kinds and no question text.

```json
{
  "question": {
    "function": "recognize",
    "kinds": ["person", "organization"]
  },
  "records": ["Morgan works for Example Group."]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

Each name has `kind`, `text`, `start`, `end` and its own `probability`. The answer can be empty. recognize is never not sure, and the row itself has no `probability`. `threshold` sets how likely a name must be to count.

An answered row also carries `detail`, left out above to keep the example short. `detail.pieces` and `detail.names` hold the probabilities behind each name.

## relate

relate takes rules and no question text.

```json
{
  "question": {
    "function": "relate",
    "rules": [
      {"source": "person", "target": "organization", "relation": "works_for"}
    ]
  },
  "records": ["Morgan works for Example Group.\nPlease refund my order."]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

Each relation has `source`, `target`, `relation` and `probability`. It has `"either": true` when its rule does, and no `either` key otherwise. A rule may also give `reads`, the wording the model sees, and `either`, for a relation with no direction.

An answered row also carries `detail`, left out above. `detail.names` holds every name found, including names in no relation. `detail.pairs` holds every pair that was checked, with its probability. relate is never not sure, and the row itself has no `probability`.

## annotate

annotate asks several questions of each record and gives back one form per record.

```json
{
  "question": {
    "function": "annotate",
    "questions": {
      "refund": {"function": "decide", "question": "Does the customer request a refund?"},
      "topic": {"function": "choose", "question": "What is the message about?", "choices": ["billing", "shipping", "other"]},
      "urgency": {"function": "score", "question": "How urgent is the request?", "levels": ["routine", "urgent"]}
    }
  },
  "records": [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice."
  ]
}
```

One row per record. The row for the third message, with its topic not sure:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A form whose topic question failed while the other questions answered:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

A form whose record could not be judged at all:

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

Each entry of a form is a row of its own function, with its own `status`. It follows every rule at the top of this page, and leaves out only `position`, `id` and `source`, which belong to the form. One entry can be not sure, or failed, while the others answer. The form itself is never not sure and has no `probability`. A consumer checks each entry it uses before reading that entry's answer. An answered outer form does not establish that all its entries answered; raise-if-failed checks the entries too.

Any function except filter, rank and annotate can be an entry. filter and rank are about many records, and a form is about one. The entries come back under the names you gave.

## Say what each answer means

When a short question is not enough, say what the answers mean.

```json
{
  "function": "decide",
  "question": "Does the customer request a refund?",
  "yes_means": "the customer wants money back, including for a duplicate charge",
  "no_means": "the customer wants anything else"
}
```

```json
{
  "function": "choose",
  "question": "What is the message about?",
  "choices": [
    {"name": "billing", "description": "charges, invoices and refunds"},
    {"name": "shipping", "description": "parcels and delivery"},
    "other"
  ]
}
```

decide and filter take `yes_means` and `no_means`: what a yes means and what a no means. choose, tag, score and rank take a description on any of their choices, labels or levels. Write that one as an object with `name` and `description` in place of the plain string. The answer is still the name alone.

## Records and fields

A record is a JSON string, or an object or other JSON value. A string is judged as its text. Any other value is judged as its JSON text.

`"field": "/text"` in the question picks one field of an object. It is a JSON Pointer. Without it, ThinkThen judges the whole record.

```json
{
  "question": {
    "function": "decide",
    "question": "Does the customer request a refund?",
    "field": "/text"
  },
  "records": [
    {"id": "a1", "text": "Please refund my order."},
    {"id": 42, "text": "Where is my parcel?"}
  ]
}
```

The field table describes this state. Actual captured responses are shown in the [JSON reference](https://thinkthen.dev/reference/json-output/).

Give object records an `id`: a string or a whole number. Each row carries it back unchanged. A row for a record with no `id` has no `id` key. Whole numbers of any size up to 64 bits come back exactly, so read `id` with a parser that keeps them.

There is no separate one-record request. One record is an array of one. A record that can't be judged always comes back as a failed row, and nothing about it is sent to the service:

| The record | The failed row's `code` |
| --- | --- |
| Blank: an empty string, only spaces, `{}`, `[]`, or an object with nothing but its `id` | `record.empty` |
| `null`, or a `field` that points at nothing or at `null` | `record.missing` |
| An `id` that is `null`, a fraction, `true`, an object or an array | `record.invalid` |
| Too large to send on its own | `record.too-large` |

This holds for a request with one record too. A blank record is a failed row, not a refused call.

A request may also carry `"name"` beside `question` and `records`. It comes back in each row's `source.name`, so saved rows from different runs can be told apart.

## Saved questions

The `question` object is the saved question. It holds the function, the question text and every option, so a library can keep it, load it, and send it unchanged with new records.

A toolbox file keeps named questions together:

```json
{
  "entries": {
    "refund": {
      "question": {
        "function": "decide",
        "question": "Does the customer request a refund?",
        "threshold": {"low": 0.2, "high": 0.8},
        "field": "/text"
      }
    }
  }
}
```

A request never names a toolbox entry. The library looks the entry up and sends its `question`.

## Settings

Settings are not part of a request. They belong to the client that sends it, and have their own schema, `client.schema.json`.

```json
{
  "backend": "systemone",
    "batch": 16,
  "retry": {"max_retries": 5, "initial_backoff_ms": 500, "attempt_timeout_ms": 120000}
}
```

The fields are `backend`, `url`, `model`, `api_key`, `batch` and `retry`. Times are in milliseconds. `api_key` is accepted but never written back out. Keep the key in the environment, not in a file.

## Errors

Errors come back either as a call error or as a failed record row.

| What happened | How it comes back |
| --- | --- |
| A mistake in the request, such as a bad threshold or an unknown key | One error for the call. No rows |
| The client could not start, such as no API key | One error for the call. No rows |
| The call was cancelled | One error for the call, `local.cancelled`, after any rows already given |
| A remote request was refused | HTTP 401/403 always end the call. If every request in the first batch fails and any failure is HTTP 4xx, the call ends with that backend error. These are call errors, not failed record rows |
| A record could not be judged, or the service failed | A failed row for that record. The other rows still come |
| Not sure | An `"unsure"` row. It is not an error and has no `error` |

A call's error has the same three fields as a row's: `code`, `message` and, when the service gave one, `http_status`. It is not a row: it has no `status`, `function` or `position`.

A failed row's `code` is always one of these eight:

| `code` | Meaning |
| --- | --- |
| `record.empty` | The record's text is blank, so it was not sent |
| `record.missing` | The record is `null`, or the `field` points at nothing |
| `record.invalid` | The record or its `id` can't be read without loss |
| `record.json` | The record is not JSON |
| `record.too-large` | The record is too large to send on its own |
| `backend.transport` | The service could not be reached |
| `backend.status` | The service answered with an error. `http_status` has the number |
| `backend.reply` | The service's reply did not answer the request |

The listed backend codes can also end a call under the remote-refusal conditions above; the error comes back outside any row in that case. Every other code belongs to a call: the `usage.` codes for a mistake in the request or the settings, the `local.` codes for cancelling and for local files, and `internal.bug`. `contract.json` lists every code with its family. Branch on `code`, never on `message`, which is written for people and can change.

Two groupings let a library sort rows by meaning:

- There is no answer when `status` is not `"answered"`. That covers not sure and failed.
- Something went wrong only when `status` is `"failed"`. Not sure is a result and not an error.

## Upgrading from 0.1

The [published 0.1.2 source](https://proxy.golang.org/github.com/botassembly/thinkthen/@v/v0.1.2.zip) uses call envelopes with `value`, `facts` and `details`. The current row schema replaces that envelope with per-record `function`, `status` and an answer only when answered. Do not load an old call envelope as a current saved row; preserve its original inputs and regenerate rows through the current API. The `yes_means`/`no_means` request keys replace `true`/`false`.

---
