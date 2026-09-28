# The Python surface

`import thinkthen as tt` calls the real engine through the public `thinkthen` crate. The binding is the unpublished crate `thinkthen-python` in its own Cargo workspace (ADR 0047). The design page is `sdlc/planning/libraries/python.md`.

```python
import thinkthen as tt

tt.decide("Does the customer ask for a refund?", text)  # True, False, or None
refund = tt.question(decide="Does the customer ask for a refund?", threshold=(0.2, 0.8))
tt.decide(refund, "I was charged twice. Can you fix this?")
complaints = tt.filter("Is this a complaint?", reviews)
rows = tt.annotate("form.json", reviews)  # one dict per review

engine = tt.Engine(throttle=8, cache=False)
engine.decide_many(refund, reviews)
```

`None` means "not sure". A call reads one `str`, or a list, tuple, or other iterable of `str`, whole before its first request. `examples.json` holds one checked example for each function.

`decide`, `decide_many`, `choose`, `score`, and `tag` also read a Polars `Series` or another Arrow column in place, with no copy, in one engine call. A `Series` gets a `Series` back, and any other column gets a list. `annotate` and `recognize` read a Polars `DataFrame` with `on=`, the name of the text column:

```python
df = tt.annotate("form.json", df, on="body")  # one new column per question
names = tt.recognize(df, kinds=["product"], on="body")  # row, text, start, end, length, kind, strength
```

In a Polars or pandas frame, each answer column keeps its question type: nullable boolean for `decide`, string for `choose`, float for `score`, and a list of strings for `tag`. The last column, `failed`, is null when every question answered. A partial row holds a map from each failed question to its full `{"failed":{"kind":"backend","cause":...}}` marker. pandas uses a sparse dict; Polars uses a fixed-field Struct with null sibling fields. A null answer with no marker means not sure for `decide` or `choose`. A successful empty tag is `[]`. A question named `failed` or an input frame with that column is refused before any request.

A pandas `Series` works in the same five verbs and comes back as a pandas `Series` with the caller's index and name. `decide` and `decide_many` give `boolean`, `score` gives `Float64`, `choose` gives `string`, and `tag` gives `object` with one list of labels per row. "Not sure" is `pd.NA`, or `None` in `tag`. A pandas `DataFrame` with `on=` comes back from `annotate` with one new column per question and `failed`, and from `recognize` with a new `names` column: one list per row of `dict` with `text`, `start`, `end`, `length`, `kind`, and `strength`. The index stays the caller's. A null in the text column, a repeated or missing `on` label, `MultiIndex` columns, and a question named as a column are refused before any request. The answer keeps the input's name, so rename it to add it as a column:

```python
df = df.join(tt.decide("Is it late?", df["body"]).rename("late"))
```

What is fast: a pandas 2 column crosses at list speed and still makes one engine call at the full throttle. A pandas 3 `str` or `string[pyarrow]` column crosses zero-copy. A categorical column crosses at list speed on both versions. `astype("string[pyarrow]")` makes a pandas 3 object column zero-copy, and it gives pandas 2 no fast path. The gate runs pandas 3.0.6 and 2.3.3. The oldest pandas the 2026-09-21 checks ran is 2.2.3. The package never imports pandas.

`filter`, `rank`, `find`, and `relate` read lists only. The optional extra `thinkthen[polars]` names the tested Polars floor. The package never imports Polars itself. A warm Polars pool hangs a forked child, so start children with `spawn`.

The module functions use one engine that the environment configures: `THINKTHEN_BASE_URL`, `THINKTHEN_CACHE`, and the rest. `tt.Engine` takes `base_url`, `model`, `throttle`, `max_requests`, `max_request_bytes`, `cache`, `timeout`, `max_retries`, `profile`, `record`, and `replay` as keywords and reads the environment for the rest. The key comes only from `THINKTHEN_API_KEY`. The throttle is the number of requests in flight. It is one per loaded copy of this package: the first explicit throttle sets it, and a second, different one raises `UsageError`.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. `cache prune` is the only thing that removes entries. Turn it off with `tt.Engine(cache=False)`.

Every verb takes `deadline`, in seconds from the call, and `token`, a `CancelToken` any thread can set. No deadline is spelled ``deadline=None`` or ``deadline=-1`` (ADR 0041). Zero is a spent deadline: the call sends nothing and raises `DeadlineError`. Any other negative, a bool, and a non-number raise `UsageError`.

Every call runs on its own worker thread. Ctrl-C or the caller's token stops the wait within 50 ms and raises `Cancelled`, a subclass of both `KeyboardInterrupt` and `ThinkThenError`. No new request starts after a stop, and a request already sent ends on its own. A signal handler's own error, such as `SystemExit`, passes through unchanged.

## Run facts

`details(question, text)` returns the command's `--details` line for one text as a `dict`, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. No call reports cost or time yet.

A question read with `tt.question(file=...)` keeps its saved calibration `profile` in `meta.question_sha256`. When the engine selects a different runtime profile, details include `meta.profile_warning` with `tuned_for` and `running`. A profiled question cannot become a grouped column member for `choose`, `score`, or `tag`; those calls raise `UsageError` before sending.

`usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens. A call over a column or a frame returns values only, so read its facts with `details` for one text and `usage()` for the totals.

`check.sh` is this folder's gate. It needs Python 3.12 or later, `uv`, and `maturin`, and it installs the pinned test packages offline from uv's cache. A missing piece reports "not run". Its last step runs the pandas tests and the secrecy test again under pandas 2.3.3 (`requirements-pandas2.txt`), over the same built extension. `build-wheel.sh` builds the release wheel and checks its contents. `NOTES.md` records the port's decisions.
