# Python functions that answer questions about text

ThinkThen is a classifier you call like a function: it reads text, answers one bounded question, and returns a typed value. In Python, each result stands for its answer, ready for an `if`.

Use Python 3.10 or later. Run `python3 -m venv .venv`, then `. .venv/bin/activate`, then `python -m pip install thinkthen==0.2.0`. The wheel includes the native engine. Set `THINKTHEN_API_KEY` in the environment first; the client reads it from there.

## Judge a file of messages

Put one JSON record per line in `records.jsonl`, such as `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}`.

```python
import json
import thinkthen as tt

with open("records.jsonl", encoding="utf-8") as source:
    records = [json.loads(line) for line in source if line.strip()]
with tt.Client() as client:
    with client.decide_stream("Does the customer request a refund?", records, field="/text") as refund_checks:
        for is_refund in refund_checks:
            if is_refund.error is None:
                print(is_refund.position, "answered", is_refund)
            else:
                print(is_refund.position, "failed", is_refund.error.message)
```

<!-- output: capture -->

Look for one line per record, numbered from 1, with `True` or `False` for each message. A line that is `null`, or a record with no `text`, prints `failed` with `record.missing` in its place and sends nothing to the backend. A blank record, such as `""`, only spaces or `{}`, prints `failed` with `record.empty` the same way. The stream carries on past both.

[Where record text goes](service.md) covers keys, backends and connection settings.

## Branch on the answer

A one-record call returns the answer for that record. `if is_refund:` reads it as yes or no.

```python
import thinkthen as tt

with tt.Client() as client:
    is_refund = client.decide(
        "Does the customer request a refund?",
        "Where is my parcel?",
        threshold=(0.2, 0.8),
    )
try:
    print("refund queue" if is_refund else "inbox")
except tt.Unsure:
    print("a person reads it", is_refund.probability)
```

<!-- output: capture -->

`threshold=(0.2, 0.8)` is a band: no below 0.2, not sure from 0.2 up to 0.8, yes at 0.8 or above. Without a band, decide uses a cut at 0.5 and is never not sure. A number such as `threshold=0.7` sets a different cut, and `tt.Band(0.2, 0.8)` is the same as the tuple. Choose takes only a number and is not sure unless its best choice reaches it. Tag, find, recognize and relate take only a number.

Using an answer raises when there is no answer to use:

| Raised | When |
| --- | --- |
| `tt.Unsure` | The probability falls inside the band. `.row` holds the result. |
| `tt.RecordError` | ThinkThen could not judge the record. It carries `code` and its message. |

Truth, equality (`==` and `!=`), printing, `str()`, iteration over tag, recognize and relate answers, and `float()` on a score all raise this way. Send not sure to a person; it is not a failure. Check `.unsure` and `.failed` before comparing an answer, or inspect `.answer` directly to read its plain value.

To look without raising, read the fields: `is_refund.unsure`, `is_refund.failed`, `is_refund.ok` (answered), `is_refund.answer` (`None` unless answered), `is_refund.probability` and `is_refund.error`. Every result also has `function`, `status` (`tt.Status.ANSWERED`, `UNSURE` or `FAILED`), `position` from 1, `id` when the record has one, `source` and `detail`.

A one-record call given a blank record, such as `""`, only spaces or `{}`, raises `tt.RecordError` with code `record.empty` straight away and sends nothing to the backend.

Problems with the call itself raise straight away, as subclasses of `tt.ThinkThenError`: `UsageError`, `SetupError`, `InternalError` or `Cancelled`. Each carries `code`, its message and `http_status` when the service sent one.

## The ten functions

Each example below runs on its own and uses the same support inbox. Printing an answer raises `tt.Unsure` if ThinkThen is not sure, so a not-sure answer stops the script instead of printing.

### decide: answer a yes-or-no question

```python
import thinkthen as tt

with tt.Client() as client:
    is_refund = client.decide(
        "Does the customer request a refund?",
        "Please refund my order.",
    )
print(is_refund)
```

<!-- output: capture -->

### filter: keep the records that pass

```python
import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
with tt.Client() as client:
    refund_requests = list(
        client.filter("Does the customer request a refund?", messages)
    )
for refund_request in refund_requests:
    print(refund_request)
```

<!-- output: capture -->

`filter` gives back your own records, the ones answered yes, in their input order. Not-sure records are left out. `filter` and `filter_rows` take `yes_means` and `no_means`, as decide does; see [Say what each answer means](#say-what-each-answer-means). [Keep what filter drops](#keep-what-filter-drops) shows failures and the full view.

### choose: pick one option

```python
import thinkthen as tt

with tt.Client() as client:
    topic = client.choose(
        "What is the message about?",
        "Please refund my order.",
        choices=["billing", "shipping", "other"],
    )
print(topic)
```

<!-- output: capture -->

### tag: label every topic that fits

```python
import thinkthen as tt

with tt.Client() as client:
    topics = client.tag(
        "Which topics appear?",
        "I was charged twice and my parcel is late again.",
        labels=["billing", "shipping"],
    )
print(list(topics))
```

<!-- output: capture -->

The answer can be empty when no label fits.

### score: place a message on your levels

```python
import thinkthen as tt

with tt.Client() as client:
    urgency = client.score(
        "How urgent is the request?",
        "Please refund my order.",
        levels=["routine", "urgent"],
    )
print(urgency)
```

<!-- output: capture -->

Levels run from lowest to highest. The answer is a number from 0 to 1 along them, weighted by each level's probability. Use it to order a queue a person reads; `urgency > 0.5` works.

### rank: order records, highest first

```python
import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
with tt.Client() as client:
    by_urgency = client.rank(
        "How urgent is the request?",
        messages,
        levels=["routine", "urgent"],
    ).to_list()
for ranked in by_urgency:
    print(ranked, ranked.item)
```

<!-- output: capture -->

Each result's answer is its rank from 1, and `ranked.item` is your record. Rank reads the whole collection before it returns. `top=5` limits how many records get a rank; failed records still come back. `descriptions` says what each level means, as in `descriptions={"urgent": "the customer needs an answer today"}`.

### find: point to the line that answers

```python
import thinkthen as tt

message = "Morgan works for Example Group.\nPlease refund my order."
with tt.Client() as client:
    action = client.find("Which line states the requested action?", message)
print(action)
```

<!-- output: capture -->

Find chooses one whole nonblank line, so keep the line breaks in the field. The answer is a `tt.Span` with `text`, `start` and `end`, or `None` when no line answers. Offsets count Python characters, with an inclusive start and exclusive end: `message[action.start:action.end]`.

### recognize: pull out each name

```python
import thinkthen as tt

with tt.Client() as client:
    names = client.recognize(
        "Morgan works for Example Group.",
        kinds=["person", "organization"],
    )
for name in names:
    print(name.kind, name.text)
```

<!-- output: capture -->

Each name also has `start`, `end` and `probability`.

### relate: link the names you allow

```python
import thinkthen as tt

message = "Morgan works for Example Group.\nPlease refund my order."
rules = [
    tt.RelationRule(
        source="person", target="organization", relation="works_for"
    )
]
with tt.Client() as client:
    relations = client.relate(message, rules=rules)
for relation in relations:
    print(relation.source.text, relation.relation, relation.target.text)
```

<!-- output: capture -->

A rule can set `reads` to a wording, and `either=True` for a relation with no direction. Each relation has `source`, `target`, `relation`, `probability` and `either`. Call relate alone when you need names and relations from the same text: `relations.detail.names` holds every name it found, including names in no relation, and `relations.detail.pairs` holds every scored pair. [Relation rule fields](service.md#relation-rule-fields) lists the rule fields.

### annotate: fill in a form for each record

```python
import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
questions = tt.AnnotationQuestions({
    "refund": tt.DecideQuestion("Does the customer request a refund?"),
    "topic": tt.ChooseQuestion(
        "What is the message about?",
        choices=["billing", "shipping", "other"],
    ),
    "urgency": tt.ScoreQuestion(
        "How urgent is the request?",
        levels=["routine", "urgent"],
    ),
})
with tt.Client() as client:
    forms = client.annotate_stream(questions, messages).to_list()
for form in forms:
    print(form["refund"], form["topic"], form["urgency"])
```

<!-- output: capture -->

Each entry in a form is its own answer, with its own status, so one entry can be not sure while the others answer. Entries can be any per-record question, including filter, find, recognize and relate; rank cannot be an entry. An entry can carry its own `field`; otherwise it uses the one passed to annotate.

## Say what each answer means

Decide, filter and filter_rows take `yes_means` and `no_means`. Choose, tag, score and rank take `descriptions`, which maps each choice, label or level to what it means. A key that is not one of them is refused with `tt.UsageError`. The question classes take the same, as in `tt.FilterQuestion("Does the customer request a refund?", yes_means="the customer wants money back")` and `tt.RankQuestion("How urgent is the request?", levels=["routine", "urgent"], descriptions={"urgent": "the customer needs an answer today"})`.

```python
import thinkthen as tt

with tt.Client() as client:
    is_refund = client.decide(
        "Does the customer request a refund?",
        "I was charged twice for one invoice.",
        yes_means="the customer wants money back, including for a duplicate charge",
        no_means="the customer wants anything else",
    )
    topic = client.choose(
        "What is the message about?",
        "I was charged twice for one invoice.",
        choices=["billing", "shipping", "other"],
        descriptions={
            "billing": "charges, invoices and refunds",
            "shipping": "parcels and delivery",
        },
    )
print(is_refund, topic)
```

<!-- output: capture -->

Choices, labels and levels can also be an `Enum` class or a `Literal` type, and the answer then comes back as that type. Key `descriptions` by the Enum members, as in `{Topic.BILLING: "charges, invoices and refunds"}`. Name the field, the allowed values and any date rule in the question rather than relying on phrases such as "currently available".

## Keep what filter drops

When a record fails, `filter` still gives you every kept record first. Then it raises one `tt.RecordError`, whose `code` is the first failure's code and whose `rows` holds every failed result in input order.

```python
import thinkthen as tt

messages: list[tt.Record] = [
    "Please refund my order.",
    None,
    "I was charged twice for one invoice.",
]
with tt.Client() as client:
    try:
        for refund_request in client.filter("Does the customer request a refund?", messages):
            print("refund queue:", refund_request)
    except tt.RecordError as failure:
        for failed in failure.rows:
            print("failed:", failed.position, failed.error.code)
```

<!-- output: capture -->

Look for the kept messages first, then `failed: 2 record.missing` for the `None`.

`errors="coerce"` yields each failed `tt.FilterRow` in its record's place instead of raising:

```python
import thinkthen as tt

messages: list[tt.Record] = [
    "Please refund my order.",
    None,
    "I was charged twice for one invoice.",
]
with tt.Client() as client:
    for refund_request in client.filter("Does the customer request a refund?", messages, errors="coerce"):
        if isinstance(refund_request, tt.FilterRow):
            print("failed:", refund_request.position)
        else:
            print("refund queue:", refund_request)
```

<!-- output: capture -->

`filter_rows` gives one `tt.FilterRow` for every record, kept or not. Each has `item`, your record, and `kept`, which is true only for an answered yes:

```python
import thinkthen as tt

messages = [
    "Please refund my order.",
    "Where is my parcel?",
    "I was charged twice for one invoice.",
]
with tt.Client() as client:
    refund_checks = client.filter_rows(
        "Does the customer request a refund?", messages, threshold=(0.2, 0.8)
    ).to_list()
for refund_check in refund_checks:
    print(refund_check.position, refund_check.status, refund_check.kept, refund_check.item)
```

<!-- output: capture -->

## Records

A record is a string or a JSON-compatible Python value. `field="/text"` picks one field with a JSON Pointer; without it, ThinkThen judges the whole record. A pointer uses `/` between keys, `~1` for a literal slash and `~0` for a literal tilde. To judge several fields together, put them under one object, such as `{"id": "a1", "message": {"subject": "Invoice refund", "body": "Please refund my order."}}`, and use `field="/message"`.

Give object records a string or integer `id`. Each result carries it back in `.id`, so keep it to join answers to your data; `position` counts only within one call.

A one-record call refuses a list, a tuple, a pandas Series or a DataFrame with a `TypeError` that names the `_stream` form. To judge a JSON array as one record, wrap it in an object, such as `{"items": [...]}`.

## Judge many records at once

`decide`, `choose`, `tag`, `score`, `find`, `recognize`, `relate` and `annotate` each have a `_stream` form, such as `decide_stream`, that takes many records and yields one result per record. `filter`, `filter_rows` and `rank` always take many. A stream carries on past a not-sure or failed record, so each result keeps its own status.

Pass the whole collection to one stream so the engine can batch the questions. A list or tuple is a finite collection; a generator supplies records as you produce them. Use a `with` block on a stream to close it, including after an early `break`. `.to_list()` collects and closes a stream. `Client(batch=...)` sets how many records go in one request; the engine also applies request size and backend limits.

To save every result, write `.to_dict()` for each one. It turns a result, with its answer, details, source and error, into plain Python values: tuples become lists and enums become strings. It holds no credentials or backend responses.

```python
import json
import thinkthen as tt

with open("records.jsonl", encoding="utf-8") as source:
    records = [json.loads(line) for line in source if line.strip()]
with (
    tt.Client() as client,
    client.decide_stream("Does the customer request a refund?", records, field="/text") as refund_checks,
    open("refund-checks.jsonl", "w", encoding="utf-8") as destination,
):
    for is_refund in refund_checks:
        destination.write(json.dumps(is_refund.to_dict(), ensure_ascii=False) + "\n")
```

The file keeps one line per record with its `id`, including the no, not-sure and failed ones. A filter result's `.to_dict()` includes `item` but not `kept`.

## Tables and async code

The optional pandas and Polars integrations add `.thinkthen` accessors to Series and DataFrames. Install `thinkthen[pandas]` or `thinkthen[polars]` from PyPI. Import `thinkthen.pandas` or `thinkthen.polars` to register its accessor. The SDK does not read spreadsheet files itself; use your table library to load them.

The CSV file must have a `text` column.

```python
import pandas as pd
import thinkthen as tt
import thinkthen.pandas

frame = pd.read_csv("messages.csv")
with tt.Client() as client:
    is_refund = frame.thinkthen.decide(
        "Does the customer request a refund?", field="/text", client=client
    )
print(frame.assign(is_refund=is_refund))
```

Each accessor method returns one answer per input row by default, as a Series on the input's index. A Series keeps its name; a DataFrame's answers are named `answer`. pandas answers are `boolean` for decide, `string` for choose, `Float64` for score and `Int64` for rank, and Polars answers use native Polars types. Not sure is `<NA>` in pandas and `null` in Polars. On older Polars, such as 1.0, a missing `find` or annotate answer reads back as a struct of nulls, so check with `is_null()`, which works on every supported version. `filter` returns the kept subset of your own Series or DataFrame, and leaves out the rows that are not sure. `details=True` returns the outcomes instead: pandas gives an outcome DataFrame on the input's index, and Polars gives a struct of outcomes. With `details=True`, DataFrame annotation returns the input columns followed by `name`, `name_status`, `name_probability` and `name_detail` for each question name. pandas keeps the original index, including duplicates, and Polars uses the same column layout with native column types. Existing column names must not overlap the added names. By default (`errors="raise"`), a failed row, such as a missing value, raises `RecordError` after every row is judged, and the error's `outcomes` attribute holds every outcome already paid for. Streams can also collect with `.to_pandas()` or `.to_polars()`. These collection methods retain all result rows in memory.

To keep going past a failure, pass `errors="coerce"`. The answers stay a plain column with a missing value for not sure and for a failure, and `filter` leaves failed rows out of the subset. The call warns once with `tt.FailedRowsWarning`, which names every failed code, and `.thinkthen.failures()` on the result lists each failure, so you can tell it from not sure:

```python
import pandas as pd
import thinkthen as tt
import thinkthen.pandas

frame = pd.DataFrame({"text": ["Please refund my order.", None, ""]})
with tt.Client() as client:
    is_refund = frame.thinkthen.decide(
        "Does the customer request a refund?",
        field="/text",
        threshold=(0.2, 0.8),
        client=client,
        errors="coerce",
    )
failed_messages = is_refund.thinkthen.failures()
print(failed_messages[["position", "error_code"]])
```

Each row of `failures()` has the failure's `position` in your input, counted from 1, then `id`, `question`, `error_code`, `error_message` and `error_http_status`. `question` names the annotate question. The result is a pandas or Polars DataFrame, matching the input. A result with no failure gives no warning and zero rows. The warning's `codes` and `rows` hold the same failures, and `warnings.filterwarnings("ignore", category=tt.FailedRowsWarning)` silences it. With `details=True` the outcomes already show each status, so the call does not warn, and `failures()` still lists the codes. To keep status in your own table, use `details=True`.

Call `failures()` on the object the call returned, because the list does not always follow your data. pandas keeps it in that object's `.attrs`. A subset keeps the whole list, including failures outside the subset. `pd.concat` keeps it only when every input's `.attrs` are identical and not empty. `frame.assign(is_refund=is_refund)` loses it, because the list sits on the answer column and not on `frame`. Polars has no place to keep it, so every Polars operation that makes a new object loses it. Where the list was lost, `failures()` gives zero rows, so an empty table there does not mean the call had no failures.

`tt.AsyncClient` supports `async with`. Await its one-record methods, and iterate its streams with `async for` inside `async with`. Its `filter` yields kept records the same way, then raises one `tt.RecordError` for failures. Row streams offer `.to_list()` and `.aclose()`. The async calls accept ordinary iterables and async iterables of records, and `thinkthen.aio` supplies top-level async functions. Cancelling an asyncio task keeps Python's normal cancellation behavior and closes the underlying operation.

```python
import asyncio
import thinkthen as tt


async def main() -> None:
    async with tt.AsyncClient() as client:
        is_refund = await client.decide(
            "Does the customer request a refund?",
            "Please refund my order.",
        )
    print("refund queue" if is_refund else "inbox")


asyncio.run(main())
```

<!-- output: capture -->

`thinkthen.aio.set_default_client(client)` makes the `thinkthen.aio` calls use your `tt.AsyncClient`, as `tt.set_default_client` does for the plain calls. Call it inside your main coroutine, because each running event loop has its own default and a call outside a running loop raises `RuntimeError`. The default ends with its loop, so the next `asyncio.run` starts fresh. Setting a new default leaves the old one open, and `aio.aclose_default_client()` closes the one that is set.

```python
import asyncio

import thinkthen as tt
import thinkthen.aio as aio


async def main() -> None:
    retry = tt.RetryPolicy(max_retries=5)
    aio.set_default_client(tt.AsyncClient(retry=retry))
    is_refund = await aio.decide(
        "Does the customer request a refund?",
        "Please refund my order.",
    )
    print(is_refund)
    await aio.aclose_default_client()


asyncio.run(main())
```

## Save and load questions

Question classes such as `tt.DecideQuestion`, `tt.FilterQuestion` and `tt.ChooseQuestion` hold a question, its options and its field selector, so several calls apply the same definition. Pass one in place of the question text; recognize and relate then take `(question, record)`. A `tt.Toolbox` stores named questions and saves them as JSON. It never runs shell commands; command shell steps and runs belong to the command's own saved-run format.

Save this as `toolbox.json`:

```json
{"entries":{"refund_request":{"question":{"function":"filter","question":"Does the customer request a refund?","field":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}
```

```python
import thinkthen as tt

messages: list[tt.Record] = [
    {"id": "a1", "text": "Please refund my order."},
    {"id": "a2", "text": "Where is my parcel?"},
    {"id": "a3", "text": "I was charged twice for one invoice."},
]
toolbox = tt.Toolbox.load("toolbox.json")
with tt.Client() as client:
    refund_requests = list(client.filter(toolbox.filter("refund_request"), messages))
    forms = client.annotate_stream(
        {"topic": toolbox.choose("topic")}, refund_requests
    ).to_list()
for form in forms:
    print(form.id, form["topic"])
```

<!-- output: capture -->

`toolbox["topic"]` returns a general question; `toolbox.choose("topic")` and its siblings return the typed one, and refuse an entry of another function. `toolbox.to_json()` gives the saved form back, with every field selector. `tt.Toolbox.from_json(text)` reads it from a string.

## Cancel a call

Pass `cancel=tt.Cancel()` to a call when another thread or callback must stop it. Calling that handle's `.cancel()` stops the operation, which then raises `tt.Cancelled` with code `local.cancelled`. A stream also has `.cancel()`; close the stream afterwards. Ctrl-C during a call raises `tt.Cancelled` too. Catch it apart from record failures.

## Client settings

Reuse one client for many calls and close it with a `with` block. `tt.Client()` reads `THINKTHEN_BACKEND` when you pass no `backend`, as the command does, and otherwise uses System One. Its other options are `url`, `model`, `api_key`, `batch` and `retry=tt.RetryPolicy(...)`, whose times are in seconds. Prefer an environment key to a literal `api_key`.

Top-level functions such as `tt.decide(...)` use a shared default client, and also accept `client=client`. An explicit client makes it clear who owns and closes it.

For decide and filter, `probability` is the probability of yes, even on a no. For choose, it is the chosen option's probability. Score and tag have no single probability; their distributions are in `detail`.
