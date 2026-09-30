# ADR 0107: Functional and lazy ThinkThen in 0.1

- Status: **accepted** 2026-09-29 by Ian's approval relayed by the coordinator, after the review history in experiment 2038's `review.md` (rounds 1-7 fresh reviewers, Opus rounds 8-9, the marketing lead's round 10 with its diff-checked fix) and Ian's recorded rulings in that folder's `questions.md` and `feedback-round-1.md`. Proposed 2026-09-29, from experiment 2038's `proposal-functional-and-lazy.md`, whose probes live in the experiment's `probes-functional/`. Nothing here is built. Ian's change to ruling 4 (2026-09-29, end of the experiment's `questions.md`) put the functional and lazy interfaces in 0.1; his three answers on the proposal are recorded below.
- Relation to ADR 0105: this ADR shares ADR 0105's **T1 core settings schema** — every keyword here parses through the one Rust-core parser T1 builds, and the F tickets follow ADR 0105's T5 and T6, whose keywords and `.probability` they reuse. ADR 0105's frame spellings are the base this design extends; its item 10 gains this ADR's removals.
- Ian's answers of 2026-09-29, all yes: **(1)** frame idioms carry facts through an explicit `tally=`; **(2)** chaining uses `pipe` and plain judges, with no `tt.seq` class; **(3)** the Rust Polars door gets lazy expressions in 0.1 — option B, a `decide_expr` family, one M ticket — overriding the proposal's defer recommendation.

## Context

`filter(judge, xs)` shapes and `toolz.pipe` stages send one request per item when a judge is applied per item, while the core already owns a lazy batching path (ticket 0212, ADR 0089's caller-owned iterators). MotherDuck-style vector engines and Polars' streaming morsels show the same need on frames. The proposal measured the shapes with a local stub and the pinned libraries (Polars 1.44.2, pandas 3.0.6, narwhals 2.26.0, toolz 1.1.0, dplyr with dbplyr; no network, no key; each script's output is committed). Key measurements: 306 titles cost 88,933 input tokens unbatched and 11,468 packed (ADR 0055's experiment 275); the stream stub peaked at 13 MB for both 10,000 and 1,000,000 records; `take(5)` over 1,000 lines pulled 48 items and sent 3 requests; Polars streaming over 200,000 rows made 16 engine calls of 12,500 rows across 16 threads.

## Decision

### 1. Judges: curry and partial application

```python
complaint = tt.decide("Is this a complaint?", threshold="0.3:0.7")
complaint("The parcel never came.").value      # True, one request
complaint(["Thanks!", "Where is my refund?"])  # Call, .value == [False, True], one packed request
complaint(tickets["body"])                     # Call, .value is a Series
curry(tt.decide)("Is this a complaint?")        # a judge (probed)
functools.partial(tt.decide, threshold=0.7)(q)  # a judge (probed)
```

- The question comes first, the input second; a missing input returns an immutable `Judge`. Engine methods follow the same rule.
- ADR 0105's keywords bind when the judge is built — `threshold`, `true`, `false`, `options`, `levels`, `labels`, `context`, `batch` — plus this ADR's `probability` and `tally`. Two run controls bind at application: `deadline_ms` and `token`. Any other keyword there is a `UsageError`.
- A judge is thread safe, and pickles as its question JSON, settings and engine settings; the key comes from each process's environment and never enters a judge. A judge with no explicit engine uses the lazy default engine of the process it runs in, so it works in a `spawn` child — whose engine settings give it its own `max_requests_total`. **A judge bound to a tally refuses to pickle** with a plain message, because a `spawn` child would add to its own copy and silently short the parent's total.
- **The per-item hazard**: `filter(judge, xs)` would send one request per item and keep every item, because any object is true. `Call.__bool__` raises a `TypeError` naming `tt.filter(q)(xs)`. `map(judge, xs)` still sends per item; the docs put `judge(xs)` beside it.

### 2. The input sets the output

| Input | Answer | Sends |
| --- | --- | --- |
| `str` | `Call[value]` | one request |
| `list`, `tuple` | `Call[list]`, eager | packed |
| pandas, Polars or Arrow column | `Call[column]`, same index and name | packed, one engine call |
| an iterator (`iter(x) is x`) | `Stream`, lazy | packed in windows |
| `polars.Expr` | `polars.Expr` | packed per chunk at `collect()` |

A **sized collection is eager and an iterator is lazy**: `list`, `tuple`, `range`, numpy arrays, dict views and pandas `Index` are eager `Call`s; `set` and `frozenset` are refused as `usage`, because their iteration order cannot be aligned with the answers; only a true iterator (`iter(x) is x`) gives a `Stream`. F1 pins the edge table. `Stream.value` raises "a stream has no value; iterate it, or pass a list" (ADR 0105 item 10). A list already packs through `decide`, so `decide_many`, `choose_many`, `score_many` and `tag_many` add nothing and are removed (ADR 0105 item 10). `rank`, `find`, `annotate`, `recognize` and `relate` need their whole input and stay eager. `tt.plan` takes a list or column, because counting a stream would consume it.

### 3. Lazy streams

```python
kept = tt.filter("Does this ask for money back?")(read_lines("tickets.txt"))
for line in kept: print(line)
kept.facts   # frozen per-call facts once the stream ends

pipe(read_lines("tickets.txt"), tt.filter("Does this ask for money back?"),
     team, take(20), list)   # stops sending once 20 are taken
```

- **How it runs**: the binding exposes the core's lazy `Batch` (ticket 0212), which keeps order, packs, shares duplicates, bounds work in flight and stops at the first failed record. One bridge: the source is advanced **only inside `__next__`, on the calling thread** — there is no background reader — and each pull pushes owned texts into a bounded channel; the worker runs `Batch` over the channel and never touches Python. F2 proves it with a `sqlite3` cursor generator, which `check_same_thread` confines to one thread. `__next__` waits in the existing 50 ms ticks, tops up the look-ahead, returns a ready row and runs signal handlers.
- **Bounded look-ahead**: the channel holds at most (throttle + 1) request ceilings of text — 5 × 96,000 bytes today, at most 4,096 records a request. Measured flat: the stub peaked at 13 MB at both 10,000 and 1,000,000 records.
- **Abandonment**: a stream sends only while someone pulls it. Measured: `take(5)` over 1,000 lines pulled 48 items, sent 3 requests; `islice(stream, 3)` over 100,000 lines pulled 40, sent 2. Dropping the last reference stops the stream and leaves no thread (probed; the worker holds only its channels, never the stream object).
- **Close and cancel**: `stream.close()` and `with` stop pulling and scheduling; a request on the wire finishes and is billed. Ctrl-C raises `Cancelled` within one tick with its completion receipt; a `CancelToken` stops the stream from any thread; `deadline_ms` counts from the first `next()`.
- **Facts without a global**: `stream.facts` is `None` while the stream runs and holds the frozen per-call facts after exhaustion, failure or close, after its worker joins. A failure raises after the rows before it, carrying the same facts. A caller inside `pipe` who never holds the stream uses the tally (item 6).
- `probability=True` makes a stream yield `(value, probability)` pairs and a Polars expression return `Struct{value, probability}` — on decide and choose; on score and tag it is `usage`, per ruling 5A.
- **Slow sources**: a half-full batch waits on a blocking iterator, as Ian approved for Rust in ticket 0212; the docs give `batch=1` for a live source.

### 4. No chain class (Ian's answer 2)

`tt.seq(xs).filter(q).map(q2).take(5)` would copy `toolz.pipe`, `itertools` and `more-itertools`. Judges already are the stages, and each stage packs (the composed stub over 60 lines sent 11 requests at 8 records each):

```python
pipe(emails, tt.filter("Is this a complaint?"), team, take(100), list)
compose(list, team, tt.filter("Is this a complaint?"))(emails)
list(more_itertools.chunked(tt.filter("Is this a complaint?")(emails), 50))
```

### 5. Frames

**Polars** (opt-in `import thinkthen.polars`; the base package imports neither Polars nor pandas):

```python
(pl.scan_parquet("tickets.parquet")
   .filter(pl.col("lang") == "en")                       # cheap filters first
   .with_columns(flag=pl.col("body").tt.decide("Does this ask for money back?"))
   .filter(pl.col("flag"))
   .with_columns(team=pl.col("body").tt.choose("Which team owns this?",
                                              options=["billing", "shipping"]))
   .sink_parquet("refunds.parquet"))
```

The `.tt` namespace holds `decide`, `choose`, `score` and `tag`, each a `map_batches(judge, return_dtype=..., is_elementwise=True)` over the existing Arrow column door; `tt.decide(q)(pl.col("body"))` returns the same expression. **A judge inside `filter` may run on every row the scan reads**: Polars merges it with cheaper filters and pushes both to the scan — one probe measured 200,004 of 200,000 rows judged for `filter(cheap).filter(judge)` and another measured 20,000 on the same version with a different frame shape (`probes/polars-pushdown-option`), so the order is not something the pages rely on. `map_batches` offers no not-pushable switch (its parameters are `function`, `return_dtype`, `agg_list`, `is_elementwise`, `returns_scalar`). `head` after a judged filter does not stop the sends in the worst shape, so it is never relied on either. The pages and F4 lead with the `with_columns` form after the cheap filters and state this limit plainly — including that **any `head` after a filter on the judged column still judges the whole scan** (measured: 200,000 of 200,000 rows); the first N matches come from a stream over `iter_slices` or batched reads, which stop sending. The methods take `token=` and `deadline_ms=` at build time; **`deadline_ms` on an expression bounds each morsel call**, as a per-call `max_requests` does, because Polars gives a `map_batches` function no query-begin hook and a plan is often collected more than once — the whole-query bounds are the token, the process total and the tally. A `LazyFrame` is previewed by selecting the judged column and calling `tt.plan`. Measured on 200,000 rows: eager or lazy `collect()` makes 1 engine call; `collect(engine="streaming")` makes 16 calls of 12,500 rows across 16 threads; scan-to-sink makes 17. Each morsel packs under the one process throttle; a morsel edge closes a batch, costing about one partial request per morsel (about 5% at 100,000 short rows; F4 measures it). `is_elementwise=False` is never set. **No pyo3-polars plugin**: `map_batches` is stable public API, enters Python once per morsel, crosses Arrow with no copy and releases the interpreter; a plugin would couple the wheel to Polars' Rust crate and pyo3 version for the same packed requests. Reject the plugin until a measured run shows `map_batches` overhead matters beside the network.

**pandas** has no lazy engine, and the docs say so. Opt-in `import thinkthen.pandas` registers `series.tt.decide(q)` and siblings — a Series back with the caller's index through one packed call. Laziness comes from chunked readers (`pd.read_csv(..., chunksize=50_000)` with `to_parquet` per chunk). **No narwhals**: it ran on pandas and eager Polars only through a private call and refused a Polars `LazyFrame` and DuckDB (probed); the Arrow PyCapsule protocol already lets one door read pandas 3, Polars and PyArrow columns.

**R**: `complaint <- tt_decide("Is this a complaint?", threshold = "0.3:0.7")` returns a function (the judge form, matching Python); plain vectors in `mutate` keep one packed call per column or per group; `purrr::partial` works; dbplyr passes `thinkthen_decide` through to SQL unchanged, so a lazy table reaches the DuckDB or PostgreSQL extension (probed with `show_query`). E9 pins dbplyr to DuckDB, whose vectors pack; on PostgreSQL the passed-through call is the scalar per-row form, and the example states its count.

**DuckDB and SQL**: DuckDB's vector already packs; PostgreSQL and SQLite pack through ADR 0105's `_many` form.

### 6. The tally (Ian's answer 1)

A Polars expression, a pandas accessor and a stream inside `pipe` cannot carry `.facts`. A judge built with `tally=t` adds each finished call's facts — including every morsel call — to `t`. `t.facts` sums records, requests, cache answers and tokens, and its `seconds` spans first start to last finish. **The tally is one core type** — an `Arc`-shared facts sum built in ADR 0105's T7 — and Python's `tt.Tally` wraps it, so Rust's `decide_expr` family takes the same tally and there is one implementation. The tally is thread safe and explicit; no hidden global exists, and without one `tt.usage()` still gives process totals.

```python
t = tt.Tally()
lf.with_columns(pl.col("body").tt.decide(q, tally=t)).collect(engine="streaming")
t.facts.requests_sent
```

### 7. Rust Polars lazy expressions (Ian's answer 3)

The Rust door gains a `decide_expr` family — `decide_expr`, `choose_expr`, `score_expr`, `tag_expr`, with the `probability` Struct on decide and choose only (ruling 5A; score and tag have none) — over Polars' Rust lazy expression API, taking the core tally of item 6 at build time, with `token=` and a deadline that bounds each morsel call (Rust keeps `Duration` inside `CallOptions`; milliseconds are named only where a number crosses). This is the one place the design tracks a changing Polars Rust UDF interface rather than the stable Python `map_batches`; F7 holds that coupling, and the door's pinned Polars re-export (0.55) keeps the version honest. Python carries the same demand through F4; F7 serves Rust-native pipelines. F7 carries the same `filter`-judges-every-row statement and proof as F4 (item 5): judge after the cheap filters, in a `with_columns`-shaped select.

2026-09-30: Ticket 0307 dropped `polars/streaming` from the `polars` feature. The coordinator made that call under Ian's "lighter" ruling. This repo no longer proves the streaming engine or morsel cuts. Streaming is the user's opt-in. The lazy proof stands.

### 8. Process safety

- **Threads.** Judges and tallies are shared freely. A stream has one reader; a second thread calling `next()` mid-pull gets a `UsageError`. Polars pool threads each start an engine call under the one process throttle.
- **Fork.** An open stream belongs to its process and refuses in a forked child (plain message, probed). The standing rules stay: fork before the first call or use `spawn`; a warm Polars pool hangs a forked child. `multiprocessing` pickles judges; a stream refuses pickling.
- **The GIL.** The reader holds it only to pull a source item; waiting, packing and sending run without it; `map_batches` holds it for the Arrow hand-off only.
- **Ctrl-C.** `PyErr_CheckSignals` does nothing off the main thread, and streaming `map_batches` calls run on Polars pool threads, so the binding chains a C-level SIGINT handler on the **first expression call, never at import**, following ADR 0081's pattern: the handler increments a **SIGINT sequence counter**, and every engine call started inside `map_batches` (and every F7 closure) snapshots the counter at its start and stops when the counter moves — no flag to reset, so one Ctrl-C cancels the running query and a later query runs normally. The chain is scoped to those calls: ordinary main-thread and library calls keep ADR 0047's rule unchanged, and an application handler that does not raise leaves them running. Each expression call start re-checks the disposition with `sigaction` and re-chains if `signal.signal` or `asyncio.run` replaced the chain; a process that started with SIGINT ignored keeps `SIG_IGN`, a `SIG_DFL` chain is restored and re-raised, and the process id is checked after a fork (ADR 0038's rules, adopted by citation). The user sees `Cancelled` — a `KeyboardInterrupt` — once, with the handler's pending interrupt folded into it; on Windows, where `sigaction` does not exist, the chain installs best-effort on first call and ordinary `KeyboardInterrupt` behavior is unchanged. Polars aborts the query on the first UDF error, so later morsels do not start, and a morsel call that would start within one tick after the signal refuses. F4 proves: SIGINT during a streaming collect on loopback starts no request after one tick (0.63–0.70 s measured against 5.3 s without; the counter semantics — an in-flight call stops, the next runs clean with no reset, a non-raising handler leaves work running — are probed in `probes/ctrlc-counter-pattern`); a second collect after a Ctrl-C runs; a non-raising application handler leaves ordinary calls running; a chain replaced by `signal.signal` is restored by the next call. F7 proves a cancelled token stops later morsels.
- **Panics.** A stream worker panic surfaces as `defect` on the next `__next__`, with facts; F7's closures use `catch_unwind`, mapping a panic to a Polars compute error so it never unwinds through Polars' pool.
- **Generators left open.** A dropped stream stops at once; an unread one idles after its look-ahead; at exit a detached worker starts no new request.
- **The key.** Only `THINKTHEN_API_KEY`; no judge, stream, tally, pickle, `repr` or error holds it.

### 9. Rejected options

Always-lazy (breaks `tt.decide(q, ["a","b"]).value`); a bare value for one text (drops facts, invites the per-item hazard); `tt.seq`, a pyo3-polars plugin and narwhals (above); pure-Python chunking over the eager list path (each edge closes a batch, stalls the pipe, and an inserted record shifts later chunks off the cache — the core's content cuts avoid both); idle flush for slow sources (deferred, consistent with ticket 0212).

### 10. Tickets, in landing order

All follow ADR 0105's T5 and T6 and reuse T1's parser. Count: one L, three M, three S.

| Ticket | Size | Scope | Proof |
| --- | --- | --- | --- |
| **F1 judges and the shape rule** | M | `Judge`, omit-the-input on verbs and Engine methods, the keyword split, pickling with the tally refusal, the `Call.__bool__` refusal, `Stream.value`'s plain message, **`tt.plan(judge, column_or_list)`** (moved here from ADR 0105's T5, which needs judges), removal of the four `*_many` names and Engine methods | `curry` and `partial` build judges (they inspect signatures); one `pipe` row proves packing; the iterator-vs-sized edge table (generator, `range`, set, numpy array, dict view, pandas `Index`); `filter(judge, xs)` refuses; a judge pickled into a `spawn` child answers under replay; each removed name fails with a plain message |
| **F2 lazy stream** | L | native `_Stream` over the core `Batch`, bounded feeder, `close`, `with`, drop, process and reader guards, `probability=True` pairs, `.facts` | `take(5)` over a selective 100,000-line generator sends at most the look-ahead; stream request bodies equal the eager list's byte for byte **over distinct records**; peak RSS at 10,000 and 1,000,000 records within a pinned margin; Ctrl-C and token children; no thread after a drop; a forked child refuses; `.facts` equals the listener count; a `sqlite3` cursor generator works (`check_same_thread`), proving the source advances only on the calling thread |
| **F3 tally** | S | `tt.Tally`, `tally=`, thread-safe sums | 16 threads add to one tally; totals equal the listener count |
| **F4 Polars namespace**, with the SIGINT chain of item 8 | M | `thinkthen/polars.py`, `judge(pl.Expr)`, the `probability` Struct on decide and choose, `token=`/`deadline_ms=` at build, the `thinkthen[polars]` extra; pages lead with `with_columns`-then-`filter` | eager, lazy, streaming and scan-to-sink runs replay to identical answers; rows judged pinned for `filter(cheap).filter(judge)` versus `with_columns(flag).filter(col("flag"))`; a child-process SIGINT during streaming collect starts no request after one tick, a second collect after a Ctrl-C runs, a non-raising application handler leaves ordinary calls running, and a chain replaced by `signal.signal` is restored by the next call; a second collect of one plan is not refused by the per-morsel deadline; the score/tag `probability=True` refusal; the tally matches the listener; runs at the Polars floor and at current |
| **F5 pandas accessor** | S | `thinkthen/pandas.py`, chunked-reader docs | pandas 2.3.3 and 3.0.6 lanes; index and name kept; one engine call per Series |
| **F6 R judge form** | S | verbs return a function when the input is missing; **`tt_plan` takes the judge** (moved here from ADR 0105's T6); README on grouped `mutate` and dbplyr | `purrr::partial` and the judge agree; grouped call count; a dbplyr `show_query` snapshot with no database |
| **F7 Rust Polars lazy expressions** (after ADR 0105's T7; Ian's answer 3) | M | the `decide_expr` family with the `probability` Struct on decide and choose, the core tally at build, `token=` and the first-morsel deadline, `catch_unwind` closures, the `filter`-judges-every-row statement and its `with_columns`-shaped lead | lazy and streaming parity with the eager door on shared cases; the score/tag probability refusal; a cancelled token stops later morsels; the streaming and eager tallies equal the listener count; rows judged pinned beside the eager door; the pinned Polars re-export compiles and runs |

Order: F1; then F2 and F3 in parallel; then F4, F5 and F7 (F7 after ADR 0105's T7); then F6. ADR 0105 holds the one combined order across both ADRs.

### 11. Changes to ADR 0105, applied with this ADR

- Section 2's "module verbs only" parenthetical is replaced: this ADR adds judges, streams, the Polars `.tt` namespace and the pandas accessor in 0.1.
- Item 10's removal table gains the four Python `*_many` module functions and their Engine methods (replaced by judge application over lists).
- T8's corpus gains three replayed examples: E7 `pipe` and `take` over a generator, E8 a Polars streaming scan to sink, E9 R `mutate` with dbplyr.
- The relations section names `Stream.facts` and the tally beside the run-facts issue.
- T7 stays as written; F7 extends the Rust Polars door inside this ADR's tickets.

## What Ian can overturn

Every item above. The recorded trade-offs he may revisit: the `Call.__bool__` refusal (a bare truthiness would silently keep every item); no chain class (a `tt.seq` remains one small ticket away); `map_batches` over a native plugin (revisit only with a measured overhead case); the tally as the only facts carrier on frame idioms; and F7's coupling to Polars' Rust UDF API, which he chose over deferral.
