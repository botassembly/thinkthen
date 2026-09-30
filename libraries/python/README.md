# The Python surface

`import thinkthen as tt` calls the real engine through the public `thinkthen` crate. The binding is the unpublished crate `thinkthen-python` in its own Cargo workspace (ADR 0047). The design page is `sdlc/planning/libraries/python.md`.

```python
import thinkthen as tt

call = tt.decide("Does the customer ask for a refund?", text)
call.value  # True, False, or None
call.facts.requests_sent  # attempts for this call
refund = tt.question(decide="Does the customer ask for a refund?", threshold=(0.2, 0.8))
tt.decide(refund, "I was charged twice. Can you fix this?").value
complaints = tt.filter("Is this a complaint?", reviews).value
rows = tt.annotate("form.json", reviews).value  # one dict per review

engine = tt.Engine(throttle=8, batch="max", cache=False)
engine.decide_many(refund, reviews).value
engine.decide_many(refund, reviews, batch=1).value  # old one-record requests
engine.choose_many("Which team?", reviews, options=["billing", "shipping"]).value
```

`None` means "not sure". A call reads one `str`, or a list, tuple, or other iterable of `str`, whole before its first request. `examples.json` holds one checked example for each function.

### Typed labels and annotation rows

`options=`, `labels=`, `levels=`, and recognition `kinds=` take the existing ordered list/map forms, an `Enum` class, or a direct standard-library `Literal[...]`. A string-valued Enum member supplies its value; another member supplies its name. Canonical declaration order is kept. An Enum member's `description` or distinct member docstring supplies its meaning. Put explicit meanings in the ordered members map, such as `options={"billing": "Invoices"}`. Recognition keeps `descriptions=`. Unknown labels and observable Enum aliases are usage errors before a request. Python can collapse a repeated `Literal` argument before runtime inspection; a resulting one-label `choose` set is refused by the native question grammar.

For typed `score` levels, a bare member keeps its name as its criterion even when another level has a description. An explicit map or override value of `None` still means the native empty criterion `{}`. A Pydantic question-set field uses `bool | None` for decide, `Literal[...] | None` for choose, bare `float` for score, or bare `list[Literal[...]]` for tag; nullable scores and tags are refused before a request.

```python
from enum import Enum
from typing import Literal

class Team(Enum):
    billing = "billing"
    shipping = "shipping"

tt.choose("Which team?", text, options=Team).value
tt.tag("Which tags?", text, labels=Literal["billing", "shipping"]).value
tt.recognize(text, kinds=Team, descriptions={"shipping": {"what": "Delivery team"}}).value
```

The stub types an annotation row as `dict[str, AnnotatedValue]`, where a value may be a bool, chosen string, numeric score, tag list, unresolved `None`, or a distinct `{"failed":{"kind","cause"}}` marker. `Call.value`, `Call.facts`, and `Call.details` keep their existing meaning. `Question.kind` and errors use the finite `QuestionKind` and `ErrorKind` stub aliases.

`thinkthen[pydantic]` is optional. Import `thinkthen.pydantic` only to use a `BaseModel` class with ordered `Field(description=...)` labels or question fields, `Annotated[Literal[...], Field(...)]` labels, or `row_model(question_set)`. A question-set model field uses `bool | None`, `Literal[...] | None`, `float`, or `list[Literal[...]]`; its Field description is the question text, and `json_schema_extra` carries existing question-file settings. `row_model` accepts a validated model, version-one dict, or file and returns a strict Pydantic model for explicit `Row.model_validate(call.value[0])`. It does not alter the returned row. Ordinary `import thinkthen` and core calls import no Pydantic package.

`decide`, `decide_many`, `choose`, `score`, and `tag` also read a Polars `Series` or another Arrow column in place, with no copy, in one engine call. A null input stays null in its original place and sends no request. A `Series` gets a `Series` back, and any other column gets a list. `annotate` and `recognize` read a Polars `DataFrame` with `on=`, the name of the text column:

```python
df = tt.annotate("form.json", df, on="body").value  # one new column per question
names = tt.recognize(df, kinds=["product"], on="body").value  # row, text, start, end, length, kind, strength
```

In a Polars or pandas frame, each answer column keeps its question type: nullable boolean for `decide`, string for `choose`, float for `score`, and a list of strings for `tag`. The last column, `failed`, is null when every question answered. A partial row holds a map from each failed question to its full `{"failed":{"kind":"backend","cause":...}}` marker. pandas uses a sparse dict; Polars uses a fixed-field Struct with null sibling fields. A null answer with no marker means not sure for `decide` or `choose`. A successful empty tag is `[]`. A question named `failed` or an input frame with that column is refused before any request.

A pandas `Series` works in the same five verbs and comes back as a pandas `Series` with the caller's index and name. `decide` and `decide_many` give `boolean`, `score` gives `Float64`, `choose` gives `string`, and `tag` gives `object` with one list of labels per row. "Not sure" is `pd.NA`, or `None` in `tag`. A pandas `DataFrame` with `on=` comes back from `annotate` with one new column per question and `failed`, and from `recognize` with a new `names` column: one list per row of `dict` with `text`, `start`, `end`, `length`, `kind`, and `strength`. The index stays the caller's. A null in the text column yields null answer cells without a request for that row. A repeated or missing `on` label, `MultiIndex` columns, and a question named as a column are refused before any request. The answer keeps the input's name, so rename it to add it as a column:

```python
df = df.join(tt.decide("Is it late?", df["body"]).value.rename("late"))
```

What is fast: a pandas 2 column crosses at list speed and still makes one engine call at the full throttle. A pandas 3 `str` or `string[pyarrow]` column crosses zero-copy. A categorical column crosses at list speed on both versions. `astype("string[pyarrow]")` makes a pandas 3 object column zero-copy, and it gives pandas 2 no fast path. The gate runs pandas 3.0.6 and 2.3.3. The oldest pandas the 2026-09-21 checks ran is 2.2.3. The package never imports pandas.

`filter`, `rank`, `find`, and `relate` read lists only. `decide_many`, `filter`, `rank`, `choose_many`, `score_many`, and `tag_many` accept `batch=` and shared nonblank `context=`. Eligible column calls accept `batch=`; `decide` over a text column also accepts `context=`. `annotate` accepts `batch=` for lists and frames but no shared context. A scalar text, `find`, `recognize`, and `relate` do not take shared context. The optional extra `thinkthen[polars]` names the tested Polars floor. The package never imports Polars itself. A warm Polars pool hangs a forked child, so start children with `spawn`.

The module functions use one engine that the environment configures: `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`, `THINKTHEN_BATCH`, and the rest. `tt.Engine` takes `base_url`, `model`, `throttle`, `batch`, `max_requests`, `max_requests_total`, `max_request_bytes`, `cache`, `timeout`, `max_retries`, `profile`, `record`, and `replay` as keywords and reads the environment for the rest. The key comes only from `THINKTHEN_API_KEY`. The throttle is the number of requests in flight. It is one per loaded copy of this package: the first explicit throttle sets it, and a second, different one raises `UsageError`.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `tt.Engine(cache=False)`.

Every verb takes `deadline_ms`, in whole milliseconds from call start, and `token`, a `CancelToken` any thread can set. Omit `deadline_ms` or pass -1 for no deadline. Zero is spent: the call sends nothing and raises `DeadlineError`. A bool, fraction, out-of-range count, and the old `deadline=` spelling raise `UsageError` before a send.

Every call runs on its own worker thread. Ctrl-C or the caller's token stops the wait within 50 ms and raises `Cancelled`, a subclass of both `KeyboardInterrupt` and `ThinkThenError`. No new request starts after a stop, and a request already sent ends on its own. After a worker starts, `Cancelled.completion` gives a receipt: `completion.done` is a nonblocking check, and `completion.result(timeout=0)` polls or raises `TimeoutError`. A later `result()` returns final facts and details without a value. A failed completion also has `kind`, `message`, and `retryable`; these are `None` for success or an unaccounted panic. A signal handler's `SystemExit` keeps its type and exit code and has the same completion receipt.

## Run facts

Every successful verb returns `Call[T]`. `Call.value` is the former answer, including a Series or frame. `Call.probability` carries the yes probability for decide and the chosen option's probability for choose from the same answer; an unresolved choose has `None`, and score and tag have `None`. `Call.facts` has this call's `records`, `requests_sent`, `cache_answers`, optional tokens and model, and elapsed seconds. `Call.details` is an ordered tuple of immutable question observations with answer or failure, probabilities, request digests and per-question shares. Started ordinary failures carry final `error.facts` and `error.details`; refusals before the worker starts have neither. A caught worker panic has no account. These are Rust call observations, not differences in process counters.

`details(question, text).value` is the command's `--details` line for one text as a `dict`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. No call reports cost.

A question read with `tt.question(file=...)` keeps its saved calibration `profile` in `meta.question_sha256`. When the engine selects a different runtime profile, details include `meta.profile_warning` with `tuned_for` and `running`. A profiled question cannot become a grouped column member for `choose`, `score`, or `tag`; those calls raise `UsageError` before sending.

`usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens. Column and frame calls carry their own facts outside the value's columns.

`check.sh` is this folder's gate. It needs Python 3.12 or later, `uv`, and `maturin`, and it installs the pinned test packages offline from uv's cache. A missing piece reports "not run". Its last step runs the pandas tests and the secrecy test again under pandas 2.3.3 (`requirements-pandas2.txt`), over the same built extension. `build-wheel.sh` builds the release wheel and checks its contents. `NOTES.md` records the port's decisions.
