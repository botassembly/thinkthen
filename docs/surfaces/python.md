# ThinkThen in Python

ThinkThen judges evidence with ten functions. Use Python 3.10 or later. Install the supplied 0.2.0 wheel in a virtual environment; replace the wheel path with the supplied filename:

Run `python3 -m venv .venv`, then `. .venv/bin/activate`, then `python -m pip install /path/to/thinkthen-0.2.0-platform.whl`.

The wheel includes the native engine. Once 0.2.0 is published, `python -m pip install thinkthen==0.2.0` installs it from the package index. Set `THINKTHEN_API_KEY` through your environment or secret manager before creating a normal client. Keep keys out of scripts, output and reports. `Client()` selects the default backend. `Client(backend="stub")` needs no key or network and produces deterministic synthetic answers for installation checks.

A whole JSON null record or a record without the selected evidence field returns a failed `record.missing` row locally and sends no HTTP request for that record. A selected field containing JSON null, such as `{"text":null}` with `/text`, sends that null value to the backend. Invalid records return failed `record.*` rows. Pass every record to the collection call to retain false answers and failed rows.

Create `records.jsonl` with one JSON value per line. For example, put `{"id":"a1","text":"Please refund my order."}` and `{"id":"a2","text":"Where is my parcel?"}` on separate lines. Add `null`, `{"id":"a4"}` and `{"id":"a6","text":null}` on separate lines to compare the null cases. The example first judges `["first", "second"]` as one JSON array record, then reads the file as a collection of separate records. Both calls use the stub backend. Keep every collection outcome, including answered false and failed rows.

```python
import json
import thinkthen as tt

with open("records.jsonl", encoding="utf-8") as source:
    records = [json.loads(line) for line in source if line.strip()]
with tt.Client(backend="stub") as client:
    array = client.decide("Does the customer request a refund?", ["first", "second"])
    print("array", array.status.value)
    with client.decide_stream("Does the customer request a refund?", records, evidence="/text") as rows:
        for row in rows:
            print(row.position, row.status.value, row.answer, row.error.message if row.error else "")
```

`decide`, `choose`, `tag`, `score`, `find`, `recognize`, `relate` and `annotate` return one row. Their `_stream` methods take many records and return streams. `filter` and `rank` already return streams. Rows need no closing. Use a `with` block on a stream to close it, including after an early break. With `AsyncClient`, await single-record calls and use `async with` on streams.

Use a normal client for meaningful judgments. Reuse one client for multiple calls and close it with a `with` block. Client options include `api_key`, `batch` and `retry=tt.RetryPolicy(...)`. Retry time values use seconds. Prefer environment keys to literal `api_key` values.

## Records and rows

A record is a string or a JSON-compatible Python value. A list passed to a single-record call is one JSON-array record. Object records can carry a string or integer `id`. `evidence="/text"` selects a field with a JSON Pointer. Without a selector, ThinkThen uses the whole record. A pointer uses `/` between keys, `~1` for a literal slash and `~0` for a literal tilde.

A pointer selects one value. To judge several fields together, place them under one object, such as `{"id": "a1", "evidence": {"subject": "Invoice refund", "body": "Please refund my order."}}`, and use `evidence="/evidence"`. This includes both fields and preserves the outer ID. Build the evidence object from the fields your question needs; selecting one field does not include its siblings.

Rows are frozen dataclasses. Each row has `function`, `position`, `id`, `source`, `status`, `answer`, `probability`, `error` and `detail`. Positions start at 1. Compare `status` with `tt.Status.ANSWERED`, `UNSURE` or `FAILED`. An unsure or failed row has `answer=None`. An answered find row can also have `answer=None` when no line answers the question. `row.ok` means answered, so it can be true for a negative decision or a missing find result.

A rank row's `answer` is its rank and `position` is its input order.

```python
with tt.Client() as client:
    row = client.decide(
        "Does the customer request a refund?",
        {"id": "a1", "text": "Please refund my order."},
        evidence="/text",
        threshold=(0.2, 0.8),
    )
    row.raise_for_error()
    if row.status is tt.Status.UNSURE:
        print("unsure", row.probability)
    else:
        print(row.id, row.answer, row.probability)
```

A numeric decide or filter threshold yields true at or above its cut and false below it. A `(low, high)` tuple or `tt.Band(low, high)` yields false below the lower bound, unsure inside the band and true at or above the upper bound. Choose accepts the same forms and yields unsure unless its winning choice reaches the upper cut. Tag, find, recognize and relate accept only a numeric cut.

## The ten functions

This example shows call shapes. Each line invokes a separate function. Choose, tag, score, rank, recognize and relate require their named lists.

```python
import thinkthen as tt

text = "Morgan works for Example Group.\nPlease refund my order."
records = [{"id": "a1", "text": text}, {"id": "a2", "text": "Where is my parcel?"}]
record = {"id": "a1", "text": text}
rules = [tt.RelationRule(source="person", target="organization", relation="works_for")]
questions = tt.AnnotationQuestions({
    "refund": tt.DecideQuestion("Does the customer request a refund?"),
    "topic": tt.ChooseQuestion("What is the message about?", choices=["billing", "shipping", "other"]),
    "urgency": tt.ScoreQuestion("How urgent is the request?", levels=["routine", "urgent"]),
})

with tt.Client() as client:
    decision = client.decide("Does the customer request a refund?", record, evidence="/text")
    choice = client.choose("What is the message about?", record, choices=["billing", "shipping", "other"], evidence="/text")
    tags = client.tag("Which topics appear?", record, labels=["billing", "shipping"], evidence="/text")
    score = client.score("How urgent is the request?", record, levels=["routine", "urgent"], evidence="/text")
    found = client.find("Which line states the requested action?", record, evidence="/text")
    names = client.recognize(record, kinds=["person", "organization"], evidence="/text")
    relations = client.relate(record, rules=rules, evidence="/text")
    annotation = client.annotate(questions, record, evidence="/text")
    for result in (decision, choice, tags, score, found, names, relations, annotation):
        result.raise_for_error()
    assert annotation.answer is not None
    for outcome in annotation.answer.values():
        outcome.raise_for_error()
    with client.filter("Does the customer request a refund?", records, evidence="/text") as rows:
        for row in rows:
            row.raise_for_error()
            print(row.id, row.item, row.kept, row.status)
    with client.rank("How urgent is the request?", records, levels=["routine", "urgent"], evidence="/text", top=5) as rows:
        for row in rows:
            row.raise_for_error()
            print(row.id, row.answer)
```

| Function | Answer when answered |
| --- | --- |
| decide | Boolean |
| filter | Boolean; `item` holds the original record and `kept` selects true answers |
| choose | One choice string |
| tag | Tuple of all matching labels, possibly empty |
| score | Number from 0 to 1 on the ordered scale |
| rank | Rank starting at 1, highest score first |
| find | `Span(text, start, end)`, or `None` |
| recognize | Tuple of names with `text`, `start`, `end`, `kind` and `probability` |
| relate | Tuple of relations with `source`, `target`, `relation`, `probability` and `either` |
| annotate | Mapping of question names to typed outcomes |

Score and rank levels run from lowest to highest. Score uses the probability-weighted scale position. Rank reads the whole collection before returning ranked answers; `top` limits answered ranks and retains failed records. Find chooses one whole nonblank line; preserve line breaks in the evidence. Spans use Python string offsets with an inclusive start and exclusive end: `text[span.start:span.end]`. Relation rules can set `reads` to a wording and `either=True` for an undirected relation. The source and target in a returned relation are typed spans.

`AnnotationQuestions` accepts per-record question objects, including filter, find, recognize and relate. Rank cannot be an annotation entry. The outer annotation can be answered while an entry is unsure or failed. Inspect each entry's status, or call its `raise_for_error()`. An entry can carry its own evidence selector; otherwise it uses the selector passed to annotate.

Use `FilterQuestion` for a saved filter definition, including inside `AnnotationQuestions`; a `DecideQuestion` is a different function. The live filter row has a computed `kept` property. Its `to_dict()` export includes `status`, `answer` and original `item`, but does not include that property. Add it explicitly when your saved table needs it:

```python
refund_filter = tt.FilterQuestion("Does the customer request a refund?", evidence="/text")
with tt.Client() as client, client.filter(refund_filter, records) as rows:
    filter_outcomes = [{**row.to_dict(), "kept": row.kept} for row in rows]
```

The same flag can be derived from exported data as `outcome["status"] == "answered" and outcome["answer"] is True`. Keep false, unsure and failed outcomes in the saved result, then inspect their statuses before starting another call.

Question classes such as `DecideQuestion`, `ChooseQuestion` and `RecognizeQuestion` let you construct a question once and reuse it. Pass that object in place of the string question; recognize and relate then take `(question, record)`. A `Toolbox` stores named question objects, and methods such as `toolbox.decide("refund")` retrieve a typed question. It does not execute shell commands.

## Export results

Call `row.to_dict()` to export a row, including its answer, details, source and error, to independent plain Python values. Nested dataclasses and mappings become dictionaries; tuples become lists and enums become strings. Optional fields remain present as `None`. Filter exports also include the original `item`, so use JSON-compatible records when exporting filter rows. The export contains no client credentials or raw backend responses. Probabilities keep their full Python float precision, and integer record IDs keep their exact values.

```python
import json
import thinkthen as tt

with tt.Client(backend="stub") as client:
    row = client.choose(
        "Which team?",
        {"id": "a1", "text": "Please refund my order."},
        choices=["billing", "shipping"],
        evidence="/text",
    )
    row.raise_for_error()
    exported = row.to_dict()
    text = json.dumps(exported, ensure_ascii=False)
    assert json.loads(text) == exported
```

Annotation outcomes also have `.to_dict()`. The row export already converts every nested answer and detail. Preserve `row.id` when saving results or joining them to input records; rank can change row order. Use `row.position - 1` to find an input record when it has no ID.

Give each question one explicit meaning. Name the evidence field, allowed values and any date rule instead of relying on phrases such as “currently available.” Reuse a question object when several calls must apply the same definition.

## Streams and batching

Streams take iterables of records and are iterators themselves. `.to_list()` collects and closes a stream. Filter streams also have `.kept_items()`, which yields only original kept records; inspect full rows when you need failures and unsure outcomes.

```python
import json
import thinkthen as tt

with open("messages.jsonl", encoding="utf-8") as source:
    records = [json.loads(line) for line in source if line.strip()]
results = []
try:
    with tt.Client() as client, client.annotate_stream(questions, records, evidence="/text") as rows:
        for row in rows:
            results.append(row)
finally:
    with open("outcomes.jsonl", "w", encoding="utf-8") as destination:
        for row in results:
            destination.write(json.dumps(row.to_dict(), ensure_ascii=False) + "\n")
print("rows:", len(results), "IDs:", [row.id for row in results])
for row in results:
    row.raise_for_error()
    if row.answer is not None:
        for outcome in row.answer.values():
            outcome.raise_for_error()
```

The example retains every returned row and its original ID, including false, unsure and failed outcomes. It writes consumed rows even if a call-level error interrupts traversal. After saving, it raises for failed outer or nested outcomes before later work starts. Do not replace IDs with enumeration indexes or save only positive answers. JSON Lines holds one result per line, not one JSON array.

Pass the whole collection to one stream so the engine can batch questions. A list or tuple supplies a finite collection. A generator supplies records incrementally. Avoid a loop of single-record calls when you can stream. `Client(batch=...)` controls packing; the engine also applies request size and backend limits. One record may require several backend questions, so batch size does not equal network request count.

Top-level functions such as `tt.decide(...)` use a shared default client. They also accept `client=client`. An explicit client gives your application clear resource ownership.

## Tables and async code

The optional pandas and Polars integrations add `.thinkthen` accessors to Series and DataFrames. Install `thinkthen[pandas]` or `thinkthen[polars]` when using an indexed package, or install the optional dependency alongside the supplied wheel. Import `thinkthen.pandas` or `thinkthen.polars` to register its accessor. The SDK does not read spreadsheet files itself; use your table library to load them.

```python
import pandas as pd
import thinkthen as tt
import thinkthen.pandas

frame = pd.read_csv("messages.csv")
with tt.Client() as client:
    outcomes = frame.thinkthen.annotate(questions, evidence="/text", client=client)
    print(outcomes[["topic", "topic_status", "topic_probability"]])
```

DataFrame annotation returns the input columns followed by `name`, `name_status`, `name_probability` and `name_detail` for each child. pandas preserves the original index, including duplicates. Polars returns the same column layout with native column types. Existing column names must not overlap the added names. Series annotation and stream conversion retain nested outcomes. Other pandas operations return an outcome DataFrame with the original index. A filter accessor returns outcomes and a `kept` column. It does not discard input rows. Streams can also collect with `.to_pandas()` or `.to_polars()`. These collection methods retain all result rows in memory.

`tt.AsyncClient` supports `async with`. Await its single-record methods and iterate its streams with `async for` inside `async with`. Async streams offer `.to_list()` and `.aclose()`. The async API accepts ordinary iterables and async iterables of records. `thinkthen.aio` supplies top-level async functions. Cancelling an asyncio task keeps Python's normal task cancellation behavior and closes the underlying operation.

## Save and load questions

A saved toolbox uses named question entries. Save this as `toolbox.json`:

```json
{"entries":{"ready":{"question":{"function":"filter","question":"Does the customer request a refund?","evidence":"/text"}},"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"evidence":"/text"}}}}
```

Load the entries once, select a typed question and pass the whole collection to its matching call. Saving the toolbox preserves question definitions and their evidence selectors. SDK toolboxes contain questions; command shell steps and runs belong to the command's richer saved-run format.

```python
from pathlib import Path
import thinkthen as tt

box = tt.Toolbox.from_json(Path("toolbox.json").read_text(encoding="utf-8"))
Path("toolbox-copy.json").write_text(box.to_json(), encoding="utf-8")
with tt.Client(backend="stub") as client:
    filtered = client.filter(box.filter("ready"), records).to_list()
    for row in filtered:
        row.raise_for_error()
    kept = [row.item for row in filtered if row.kept]
    rows = client.annotate_stream(
        tt.AnnotationQuestions({"topic": box.choose("topic")}), kept
    ).to_list()
    for row in rows:
        row.raise_for_error()
        if row.answer is not None:
            for outcome in row.answer.values():
                outcome.raise_for_error()
        print(row.id, row.answer)
```

`box["topic"]` retrieves a general question; selectors such as `box.choose("topic")` return the matching typed question. The example retains every filter outcome and checks failures before selecting answered true input items or starting annotation. Annotation takes `AnnotationQuestions`; use `annotate_stream` for a collection.

Call `relate` or `relate_stream` alone when you need names and relations from the same evidence. `row.detail.names` contains all admitted names in the existing `Name` shape, including names with no returned relation. `row.detail.pairs` contains the scored relation pairs; `row.answer` contains the accepted relations. Do not run recognize first for that task.

## Failures and cancellation

A bad record or backend answer returns a failed row. It does not raise automatically, even for one record. Call `row.raise_for_error()` if your application wants a `tt.RecordError`. Call-level problems raise `tt.ThinkThenError` subclasses: `UsageError`, `SetupError`, `InternalError` or `Cancelled`. These carry `code`, their library message and optional `http_status`.

Pass `cancel=tt.Cancel()` to a call when another thread or callback must interrupt it. Calling that handle's `.cancel()` stops the operation; reading it raises `tt.Cancelled` with code `local.cancelled`. A stream also has `.cancel()`. Close streams after cancellation. Ctrl-C during a native call maps to `tt.Cancelled`. Catch that exception separately from failed rows.

Each annotation child has its own status. For decide and filter, probability means the probability of yes, including when the answer is false. For choose, it means the chosen label's probability. Score and tag have no single probability; their distributions remain in detail.
