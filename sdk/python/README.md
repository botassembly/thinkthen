# ThinkThen for Python

Ask questions about text from ordinary Python code. Install the wheel with `pip install thinkthen`. The wheel includes its native engine; using it needs neither a compiler nor a separate command installation. The package includes type hints and `py.typed`.

Set your backend key in the shell, then call a function:

```python
import thinkthen

row = thinkthen.decide(
    "Does the customer ask for money back?",
    "Please refund my order.",
    threshold="0.2:0.8",
)
if row.status == "unsure":
    print("Please review this message.")
elif row.answer is True:
    print("Send this message to the refunds queue.")
else:
    print("This message does not request a refund.")
```

Module functions use one lazily created default `Client`. A client holds one engine and reuses its connections and settings across calls. Create a client to choose the backend or change its settings:

```python
client = thinkthen.Client(adapter="openai", model="gpt-6-luna", batch=16)
row = client.decide("Does the customer ask for money back?", "Please refund my order.")
with client.decide_stream("Does this need review?", ["Please check my order."]) as rows:
    for row in rows:
        print(row.answer)
```

`Client(mode=..., adapter=..., url=..., key_source=..., model=..., batch=...)` selects the backend and its settings. It has the same functions and stream methods as the module. Each call supplies only its question options, evidence pointer and cancellation handle. The engine reads its selected backend's key from the environment and owns defaults, validation and errors. See the [backend documentation](../../README.md#backends).

## Results

`Row[A]` is a frozen dataclass with a typed answer. For example, `decide` returns `Row[bool]`, `choose` returns `Row[str]` and `score` returns `Row[float]`. Its fields are `id`, `status`, `answer`, `p`, `error` and `detail`. `answer` is `A | None`, and `status` is `"answered"`, `"unsure"` or `"failed"`. Nested answers use plain dictionaries with `TypedDict` hints. `detail` contains ordinary Python JSON values: dictionaries, lists, strings, numbers, booleans and `None`. An absent optional field is `None`. Freezing the dataclass prevents field assignment; nested dictionaries and lists remain ordinary mutable Python values.

`p` carries the probability when the engine supplies one. An unsure row has `answer=None`. See the shared row contract for each function's answer shape. Text offsets work as Python string indices. A dictionary record's `id` passes through; otherwise the engine assigns its input position. A single-record argument can be a JSON object, array, number, boolean or `None`, as well as text. An array is one record; only an explicitly named stream function consumes it as multiple records. A Python string is text, including a string that looks like serialized JSON. Pass decoded JSON values to supply structured records.

## The functions

```python
import thinkthen

text = "Ada works for Acme.\nThe delivery arrives Friday."

choice = thinkthen.choose(
    "What is the message about?",
    text,
    options=["employment", "delivery", "other"],
)
tags = thinkthen.tag("Which topics occur?", text, labels=["employment", "delivery"])
urgency = thinkthen.score("How urgent is this?", text, levels=["routine", "urgent"])
delivery = thinkthen.find("When does the delivery arrive?", text)
names = thinkthen.recognize(text, kinds=["person", "company"])
relations = thinkthen.relate(
    text,
    rules=[
        {"name": "works_for", "source": "person", "target": "company"},
    ],
)

if delivery.answer is not None:
    print(delivery.answer["text"])
print(names.answer)
print(relations.answer)
```

`filter` and `rank` take an iterable. Filter yields only the kept records, preserving each original Python object and its type. Rank yields rows in rank order; its row IDs identify the original records. Other stream functions yield rows in input order. A generator, file iterator or list can supply records. Each record is text or a Python JSON value. Text stays text even when it looks like JSON. `on` selects evidence inside JSON records using a JSON Pointer.

```python
messages = [
    {"id": "a", "body": "Please refund my order."},
    {"id": "b", "body": "Where is my delivery?"},
]

with thinkthen.filter(
    "Does the customer ask for money back?", messages, on="/body"
) as kept:
    for message in kept:
        print("Refund request:", message["id"])

with thinkthen.rank(
    "Does this need immediate attention?", messages, on="/body", top=1
) as rows:
    for row in rows:
        print("Review first:", row.id)
```

Use `decide_stream`, `choose_stream`, `tag_stream`, `score_stream`, `find_stream`, `annotate_stream`, `recognize_stream` or `relate_stream` for the other functions over an iterable. Their question arguments match the single-record function. Streams return a `Stream[T]`, which is an iterator and a context manager. Filter streams contain the input record type; the other streams contain `Row` values. Leaving its context or calling `close()` cancels pending work and releases resources. Use the context manager when you may stop iteration early. An input iterator's own exception propagates to the caller and cancels the stream.

```python
with thinkthen.choose_stream(
    "Which team should handle the message?",
    (message["body"] for message in messages),
    options=["billing", "shipping"],
) as rows:
    for row in rows:
        print(row.id, row.answer)
```

One call or stream enters the engine once. Streams use the engine's batching and retain bounded input and output queues. Python's lock is released while the engine works and while the caller waits for results. Input iteration reacquires it to read Python records. Ranking follows the engine's retention behavior. Changing the Python input into a list is unnecessary.

## Toolboxes

Admit a toolbox from a dictionary. Select a named question and call it, or run the whole toolbox with `annotate`. `question()` selects the only question when the toolbox holds one. The engine parses toolbox contents and owns every question rule.

```python
import thinkthen

toolbox = thinkthen.Toolbox(
    {"questions": {"refund": {"decide": "Does the message ask for money back?"}}}
)
refund = toolbox.question("refund")
row = refund("Please refund my order.")

with refund.stream((message["body"] for message in messages)) as rows:
    for row in rows:
        print(row.id, row.answer)

annotated = thinkthen.annotate(toolbox, {"body": "Please refund my order."})
print(annotated.answer)
```

`annotate` also accepts a toolbox dictionary directly. Question strings stay literal, including text ending in `.json`. The library reads no question files. Selected questions are immutable callables. Toolboxes carry question thresholds and evidence pointers. Model and batch settings belong to the client. `Question.stream` uses the same stream and error behavior as the named stream functions. To rank by a toolbox's decide or score question, pass that selected question to `rank`, such as `thinkthen.rank(toolbox.question("priority"), messages)`.

## Failures and cancellation

Usage, backend and local failures raise `UsageError`, `BackendError` or `LocalError`, all under `ThinkThenError`. `error.code` is the engine's stable dotted code, and `str(error)` is its message. Ordinary Python calling mistakes, such as an unknown keyword, raise `TypeError`. Records use Python's standard JSON serialization; objects it cannot serialize raise `TypeError`.

A single-record call raises when its top-level row fails. A stream raises an admission failure before yielding values. Row streams deliver every row, including failed rows, then raise the first engine failure. Filter continues through record failures and yields every passing original record. After exhausting its input, filter raises the exception for the first failed record in input order, if any. Rank yields its ranked rows, then raises the exception for the first failed record after input is exhausted, including when `top` leaves that failed row out. A failed row has `error.code` and `error.message`. An answered annotate row retains named failures inside its plain JSON answer. An unsure result is a result, not an exception.

```python
try:
    row = thinkthen.decide("Is this a refund request?", "Please refund my order.")
except thinkthen.ThinkThenError as error:
    print(error.code, str(error))
```

Ctrl-C cancels the engine call and raises `KeyboardInterrupt`. The binding owns no process-wide signal handler. For cancellation from another thread, pass a `Cancel` handle and call its `cancel()` method. Cancellation uses the engine's existing cancellation behavior. Closing a stream or interrupting it stops new work; cleanup waits for active requests to return or reach the engine's attempt timeout. Closing a stream leaves its caller-supplied handle usable by other calls. A handle remains cancelled once fired.

```python
from threading import Timer
import thinkthen

cancel = thinkthen.Cancel()
timer = Timer(5, cancel.cancel)
timer.start()
try:
    row = thinkthen.decide(
        "Does this need review?", "Review this order.", cancel=cancel
    )
finally:
    timer.cancel()
```

Programmatic cancellation also raises `KeyboardInterrupt`. An already-cancelled handle raises before a call yields any row and sends no request. Cancelling a running stream stops further yields; the caller keeps rows it already consumed. Python translates the engine's cancellation outcome into this exception and invents no cancellation row.

The complete API is in [thinkthen/__init__.pyi](thinkthen/__init__.pyi).

Rows decode `detail` when the caller first reads it. Detail remains an ordinary JSON value. String offsets in answers and detail use the language’s own string indexing. Single-record calls raise the library error when the record fails; streams deliver failed rows before raising the first failure. Filters yield their passing records before raising the first failure.


## pandas and Polars

Use the typed `ThinkThenAccessor` and `ThinkThenNamespace` wrappers to ask about a whole column in one engine operation. The examples below are checked with mypy against the installed package.

Install `thinkthen[pandas]` or `thinkthen[polars]`, then import the matching namespace module. Importing `thinkthen` alone loads neither library and registers no accessor. The dependency floors are pandas 2.2 and Polars 1.0. The base package keeps its Python requirement. An optional library may require a newer Python for its newest release; the installer selects a compatible release. Each extra adds only its named library. Neither extra adds Arrow, a second native engine or a dataframe plugin.

### pandas example

```python
import pandas as pd
import thinkthen
from thinkthen.pandas import ThinkThenAccessor

messages = pd.DataFrame(
    {"body": ["Please refund my order.", "Where is my delivery?"]},
    index=pd.Index(["mail-b", "mail-a"], name="message"),
)
client = thinkthen.Client(adapter="openai")
result = ThinkThenAccessor(messages["body"]).decide(
    "Does the customer ask for money back?", client=client
)
messages["refund"] = result["answer"]
messages["refund_status"] = result["status"]
```

Every accessor method returns a pandas DataFrame of result columns. `result["answer"]` is the answer Series with the source index. The frame keeps the source index, including its name, duplicate labels and MultiIndex levels, and keeps the input order. Assembly uses source positions; labels and record IDs never serve as join keys. The caller owns any subsequent assignment or join.

### Polars example

```python
import polars as pl
import thinkthen
from thinkthen.polars import ThinkThenNamespace

messages = pl.DataFrame({"body": ["Please refund my order.", "Where is my delivery?"]})
client = thinkthen.Client(adapter="openai")
result = ThinkThenNamespace(messages["body"]).decide(
    "Does the customer ask for money back?", client=client
)
messages = messages.with_columns(
    result["answer"].alias("refund"),
    result["status"].alias("refund_status"),
)
```

The Polars Series namespace returns a Polars DataFrame in source order. Its answer Series has the source length, so `with_columns` attaches it by position. Evaluation is eager and blocking. Collect a LazyFrame before calling the namespace. There is no expression namespace: Polars documents that Python expression callbacks must be pure and may run with arbitrary inputs. A model call has effects and consumes requests. An eager Series method also gives a clear boundary for one operation over the whole column. Both namespaces support eager Series only. They register no DataFrame, GroupBy or expression interface. If a caller explicitly invokes a Series method for each group, each invocation starts a separate engine operation.

### Methods and answer columns

The imports also register `series.thinkthen` as an untyped shorthand. Static type checkers do not know these dynamic accessors; use the explicit wrappers for checked calls.

Both wrappers expose the same method signatures. The source Series replaces the record argument. All question arguments, keyword options and selected `Question` values follow the existing Python methods. Saved choose, tag and score questions supply their own options, labels and levels when the call omits them. `client=` selects a reusable `Client`; omitting it selects the existing default client. Every method also accepts the existing `cancel=` handle. Backend settings stay on the client. An accessor stores only its source Series and never owns a client, installs a handler or changes process settings.

| Method | Arguments beyond the source column | Answer column |
| --- | --- | --- |
| `decide(question, *, threshold=None, on=None, client=None, cancel=None)` | A predicate or selected question | Nullable boolean |
| `filter(question, *, threshold=None, on=None, client=None, cancel=None)` | A predicate or selected question | Nullable boolean; separate `kept` column |
| `choose(question, *, options=None, threshold=None, on=None, client=None, cancel=None)` | Choice labels | Nullable string |
| `tag(question, *, labels=None, threshold=None, on=None, client=None, cancel=None)` | Tag labels | List of strings |
| `score(question, *, levels=None, on=None, client=None, cancel=None)` | Ordered level labels | Nullable floating point |
| `rank(question, *, levels=None, top=None, on=None, client=None, cancel=None)` | A predicate or selected decide/score question | Nullable integer rank position |
| `find(question, *, threshold=None, on=None, client=None, cancel=None)` | A search question | Found-unit dictionary or native struct |
| `annotate(toolbox, *, on=None, client=None, cancel=None)` | A toolbox or toolbox dictionary | Plain JSON object |
| `recognize(*, kinds=(), threshold=None, on=None, client=None, cancel=None)` | Name kinds or selected question | List of name dictionaries or native structs |
| `relate(*, rules, threshold=None, on=None, client=None, cancel=None)` | Relation rules or selected question | List of relation dictionaries or native structs |

Every result frame has `answer`, `status`, `p`, `error` and `detail` columns. `error` contains the existing code and message as a dictionary in pandas and a struct in Polars. `detail` contains the engine's plain JSON value. Engine IDs remain available in an `id` object column; an integer `position` column identifies the original source offset even when IDs or pandas index labels repeat. Offsets inside found units, names and relations remain Python string indices. The existing result contract defines the nested answers and probability fields; the adapter does not invent result types or scores.

Pandas uses nullable `boolean`, `string`, `Float64` and `Int64` for scalar answers, nullable strings for status and object columns for nested JSON. An unsure answer uses `pd.NA`; object answer columns also use `pd.NA` for an unsure row. Polars uses `Boolean`, `String`, `Float64` and `Int64`, with null for an unsure answer. Fixed nested shapes use `Struct` and `List` with explicit dtypes so an empty or entirely unsure result still has the same schema. The optional relation field `either` is true for an unordered relation and null for a directed relation in a Polars struct. Arbitrary JSON in `annotate`, `detail` and `id` uses Polars `Object` to preserve heterogeneous values and arbitrary object keys without serializing them to JSON text. The API promises Python objects in those columns, with Polars' normal restrictions on Object operations. It performs no inference-and-retry conversion or Arrow round trip.

A failed row makes the operation raise the engine error after the stream completes. An unsure row has `status="unsure"`. An answered JSON null remains distinguishable by its answered status. An input missing value has missing result fields, including status, and a populated `position`. The adapter fabricates no status for it. Top-level `None` is missing in both namespaces. Pandas scalar missing sentinels, including `pd.NA`, `NaN` and `NaT`, and Polars nulls are omitted from the engine input, then restored by position. A Polars floating-point `NaN` follows the existing Python serialization behavior rather than becoming a skipped null. To submit JSON null as evidence, put it inside a nonmissing JSON record and select it with `on`, such as `{"body": None}` with `on="/body"`. Nested JSON values are never passed to a broad missing-value predicate that would produce an array of booleans. Polars List and Array cells become Python lists before JSON serialization. Other values use the existing Python JSON serialization and engine validation. The accessor never stringifies values or changes evidence selection.

### Filter and rank keep frame identity

Filter returns a decision for every nonmissing source value when the operation completes successfully, including any failed and unsure rows the engine delivers without a final error. Its `kept` column carries the engine's existing keep flag. It does not shrink or mutate the caller's frame. Select passing records with the result mask:

```python
# pandas; positional selection also handles duplicate index labels.
result = ThinkThenAccessor(messages["body"]).filter("Does this need review?")
kept = messages.iloc[result["kept"].fillna(False).to_numpy(dtype=bool)]
failures = result[result["status"] == "failed"]
```

```python
# Polars
result = ThinkThenNamespace(messages["body"]).filter("Does this need review?")
kept = messages.filter(result["kept"].fill_null(False))
failures = result.filter(pl.col("status") == "failed")
```

Rank returns its integer answer in source order. Exclude rows with a missing answer, then sort by `answer` to place selected records in engine rank order. `top` remains the engine's option; rows omitted by it have missing answer and status fields. Failed rows delivered by the engine stay visible. Rank does not recompute scores, sort ties itself or derive positions from record IDs. The result's `position` column supplies the source offsets for `messages.iloc[...]` in pandas or positional row selection in Polars. Source missing values also have missing rank answers. This makes a rank column safe to attach before the caller chooses whether to reorder the frame.

```python
# pandas
result = ThinkThenAccessor(messages["body"]).rank(
    "Does this need immediate attention?", top=1
)
selected = result.loc[result["answer"].notna()].sort_values("answer")
ranked = messages.iloc[selected["position"].to_numpy(dtype=int)]
```

```python
# Polars
result = ThinkThenNamespace(messages["body"]).rank(
    "Does this need immediate attention?", top=1
)
selected = result.filter(pl.col("answer").is_not_null()).sort("answer")
ranked = messages[selected["position"].to_list()]
```

### Calls, errors and ownership

One method starts one engine operation for the whole column and drains its row stream in a context manager. The engine owns batching and every ask within annotate, recognize and relate. The adapter maps source positions and translates output columns. It never calls a single-record function once per row or once per question. Conversion retains the source for the duration of the call and allocates one output frame; it adds no background task or persistent store.

The existing Python binding releases the interpreter lock while it waits. Ctrl-C and `Cancel.cancel()` retain the existing `KeyboardInterrupt` behavior. A `finally` block closes the stream on success, an exception or cancellation. Admission failures and terminal operation failures raise the existing Python exceptions with unchanged engine codes and messages. No partial frame escapes after a terminal exception. Dataframe calls collect the shared raw row stream before the Python filter projection. The narrow Python bridge exposes the existing engine source ordinal and keep flag. Every engine final error raises, even when the same error also appeared in a failed row. The adapter never matches codes or messages to suppress a final error. Every failed row contributes to the shared terminal result. There is no async facade; callers who need another event loop can schedule the blocking call through their existing executor.

### Packaging and checks

The namespace modules and their type stubs ship in the existing wheel. Registration uses the documented pandas Series accessor and Polars Series namespace decorators, under `thinkthen`. Namespace imports follow each library's ordinary name-conflict warning behavior. Type hints cover the named methods and native DataFrame returns. The base package still imports and runs when both optional libraries are absent.

Installed-wheel tests run the shared stub cases through each namespace and compare existing row answers, statuses, errors and detail. They check duplicate and nonmonotonic pandas indexes, MultiIndex names, Polars row order, unsure rows, terminal failures, null inputs, empty and all-missing columns, nested answers, arbitrary JSON keys, filter masks and rank positions. A request-count assertion verifies one column operation with engine batching, including annotate with several questions. Existing cancellation tests exercise the namespace call through the held-request fixture. The optional dependency floors and the current compatible versions use the same small installed-consumer cases. No live requests are needed.

The optional API uses two registration modules and one shared column-to-row translation helper. It reuses the existing client, raw stream, row decoder and cancellation handle. Every method returns a native DataFrame whose answer column aligns with the source. The shared Python bridge exposes existing engine source ordinals and raw rows before filter/rank presentation, and adds no engine rule.

The API choices follow [pandas' accessor documentation](https://pandas.pydata.org/docs/reference/api/pandas.api.extensions.register_series_accessor.html), [pandas' missing-value documentation](https://pandas.pydata.org/docs/user_guide/missing_data.html), [Polars' Series namespace documentation](https://docs.pola.rs/api/python/stable/reference/api/polars.api.register_series_namespace.html) and [Polars' expression callback requirements](https://docs.pola.rs/api/python/stable/reference/expressions/api/polars.Expr.map_batches.html).
