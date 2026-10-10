# Rust Polars

Ask `thinkthen` questions of a Polars `Series` or `DataFrame` from Rust. The door is the `polars` feature of the `thinkthen` crate. It is off by default, so a build that does not ask for it compiles no Polars.

```toml
thinkthen = { version = "0.1", features = ["polars"] }
```

The feature adds `thinkthen::PolarsEngine` to your own `thinkthen::Engine`. Eager methods read one column in one call. Lazy expressions make one call per collect on the default engine. Both take the same batch path as a slice of strings, at the same throttle, and return answers in input order. The trait's rustdoc holds a full eager example.

The feature turns on Polars' lazy API and not its streaming engine. The streaming engine brings Polars' cloud storage stack, about 140 more crates. To collect with `polars::prelude::Engine::Streaming`, add `polars = { version = "0.55", default-features = false, features = ["streaming"] }` to your own manifest. The expressions are ordinary column functions, so either engine runs them. The default engine holds the whole frame in memory and judges the column in one call. The streaming engine calls the expression once per morsel, so each morsel is its own call. Bounded memory for the frame itself needs the streaming opt-in or a batched read.

Named eager Series judgments and `annotate_frame` pass native records through shared Request admission and execution, then project joined native complete results into their output columns. These calls retain native results alongside the input and output columns. Use the typed `*_input_column_batch` methods for pull-based results. Whole-set rank/find and explicit source helpers materialize their complete logical collection.

With a cache or recording folder, a lazy frame collected again sends nothing, and a frame sliced before the expression asks only the kept rows that the store does not hold.

Build that engine with `max_request_bytes`, `max_requests_total`, `timeout`, `max_retries`, `profile`, `record`, or strict `replay` before passing it to Polars.

A file-backed Rust `Question` keeps its saved calibration profile when the engine asks a scalar or Series question. Read `Engine::details` for the pinned question digest and an optional mismatch warning. A profiled question cannot be inserted as a member of an annotate frame's `QuestionSet`; the builder refuses it before sending.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. The column calls use your own `thinkthen::Engine`, so turn it off with `EngineBuilder::no_cache` when you build that engine.

The door takes Polars 0.55. Polars changes its Rust API between minor versions. `thinkthen::polars` re-exports the Polars the door was built with, so it names the version your `Series` must come from. The throttle is the most requests in flight at once. It holds per loaded copy of the library, so a program that loads two copies can run up to twice the throttle.

Each asking method returns a `Call` containing the typed value in the table below. Read it with `call.value()` or take it with `call.into_value()`. `call.facts()` holds that completed invocation's record, request, cache, token, duration, and model facts, including when one column takes several requests. A started failure carries the same final account on its error. Caller-owned `Tally` joins completed call facts, including partial failures; absent token usage stays absent. Its seconds span the first start to the last finish, including concurrent calls. `Engine::usage()` remains a process total, and `Engine::details` still describes one asked text.

## Eager methods

| Method | Question | Result |
| --- | --- | --- |
| `decide_series` | a `Question` or `BandedQuestion` | `Boolean`, null for not sure |
| `choose_series` | a choose `Question` | `String`, null where nothing fits |
| `score_series` | a score `Question` | `Float64`, the position from 0 to one less than the number of levels |
| `tag_series` | a tag `Question` | `List(String)` |
| `annotate_frame` | a `QuestionSet` and the name of the text column | your frame plus one column per question, then `failed` |
| `probability_frame` | a decide or choose `Question` | a `DataFrame` with `value` and nullable `probability` columns |
| `filter_series` | a cut decide question | matching present texts in original order and name |
| `rank_series` | a rank question | a frame with original `index`, `record`, `probability` |
| `find_series` | a find question | every candidate's original `index`, `unit`, `probability`, `selected` |
| `recognize_series` | a `Recognize` specification | nullable complete native `Recognized` values with spans and relations |
| `relate_frame` | a `Relate` specification and entity name/kind columns | complete native relation edges |
| `input_column` | ordered `Option<QuestionInput>` and `PolarsCallOptions` | typed value/probability columns; explicit images on decide/choose/score |
| `input_details_column` | the same ordered explicit inputs and `CallOptions` | complete typed native details and all probabilities |
| `source_column` | explicit paths and native `InputReaderOptions` | value columns plus physical file and nullable text line coordinates |
| `plan_series` | a question and text `Series` | a no-send `PlanEstimate` using the engine's request planner |

Asking methods take `CallOptions`. `column_with` also accepts `PolarsCallOptions` for a call-level threshold, decide true/false meanings and an optional probability result. A probability request on score or tag is refused before sending. One deadline, cancel token, and interrupt check cover the whole column. The call runs on your thread and starts no worker of its own.

The named probability, recognition and relation calls use typed native Requests. Probability columns project value and probability from the same complete answer. Recognition preserves native spans and relations. Relation columns form one complete entity set; paired null fields omit an entity, and an unmatched null refuses before sending. These methods retain their existing signatures.

Null input cells are omitted from requests and restored as nulls in their original positions; non-null cells keep input order. `plan_series` validates the whole non-null input, discloses the first prepared body and full-input counts, and sends nothing. Its token range is a measured estimate of the prepared body, not billed usage.

`rank_lazy` and `find_lazy` collect the whole logical frame before asking its selected text column. They compare the complete collection once, including duplicate candidates. They are eager calls after collection; no morsel is treated as a whole candidate set. Ranking retains stable ties. Null candidates are omitted, and result indices refer to original column positions. `find_series` returns every candidate, including synthetic none with null index/unit, rather than discarding the probability vector. `selected` marks the native selected real unit; a none/abstained selection marks no real unit. Empty find input keeps the native count refusal.

`filter_series`, `rank_series` and `find_series` retain their existing arguments and output columns. Their native Request execution applies the same question admission, cancellation and call facts as the other Request callers. No caller-side request document, JSON result reader or new call name is needed. Existing code using these methods, including `rank_lazy` and `find_lazy`, keeps working.

Complete dataframe methods use the native result/2 calls and readers. Recognition collections now use one native collection call and its checked once-rounded cost. The shared native scheduler retains one deadline, cancellation scope and request budget for the whole collection.

## Lazy expressions

`decide_expr`, `choose_expr`, `score_expr` and `tag_expr` take a Polars `Expr` and return an `Expr`. The output keeps the eager dtype and null positions. `PolarsExprOptions::probability(true)` returns a `Struct` with `value` and nullable `probability` for decide or choose; score and tag refuse it before collection. The expression captures an owned question, engine and controls. A `Tally` passed in those options joins the real completed or partially failed facts from each morsel. A shared `CancelToken` stops later sends. `deadline_after(Duration)` starts a separate deadline at each morsel, so repeated collections do not share a spent deadline.

```rust
use thinkthen::polars::prelude::{col, IntoLazy};
use thinkthen::{CancelToken, PolarsEngine, PolarsExprOptions, Tally};

let tally = Tally::new();
let stop = CancelToken::new();
let judgment = engine.decide_expr(
    &question,
    col("body"),
    PolarsExprOptions::new().tally(tally.clone()).token(stop.clone()),
)?;
let judged = frame.lazy().with_columns([judgment.alias("decision")]).collect()?;
```

Apply cheap filters first, then add the judgment with `with_columns` and filter its result. Polars may push a judged `filter` into the scan, so `filter` or a later `head` does not bound requests to the first matching rows. Judge only the first matches through a batched or sliced read that stops once it has them. Collection errors from the UDF are Polars compute errors; an internal panic stays inside that boundary. No expression stores call facts on its output, so retain the explicit tally when facts matter.

In a frame, a decide column is nullable `Boolean`, a choose column `String`, a score column nullable `Float64`, and a tag column `List(String)`. These dtypes stay fixed when a question fails. A failed answer is null in its question column; the final `failed` column holds a nullable Struct with one field per question and the full nested `failed: {kind, cause}` marker. A row with no failures has a null outer `failed` cell. A not-sure `decide` or `choose` has a null answer without a marker. Your own columns come back unchanged. A failed row in a series call ends the call with the engine's `Backend` error.

## Refusals

Each refusal happens before any request and returns `thinkthen::Error::Usage`:

- `the column {name} is {dtype}, not text`. Cast a `Categorical` column to `String` first.
- `the frame holds no column {on}`
- `the frame already holds a column named {name}`
- `question kind does not match the requested function`
- `this function does not accept this threshold`

An empty row-wise or rank column returns a `Call` with an empty value of the method's type and zero records and sends. No message or `Debug` line holds a text from your column.

## Checks

The consumer under `consumer/` exercises actual frame associations through the public complete methods. The door lives in `crates/thinkthen/src/public/frame.rs` and its `frame/` modules, and its tests in `crates/thinkthen/tests/polars/`. `check.sh` is the one lane that compiles Polars. It runs from the repository root with its own target folder, a fake key, and a closed loopback address. It runs Clippy, the tests, and the rustdoc example with the feature on. It prints "not run" and exits 77 only when cargo's cache lacks the locked crates. The root `deny.toml` holds the reviewed license exceptions for the locked tree.

## Complete typed results

The additive `Engine` methods `decide_series_complete`, `choose_series_complete`, `tag_series_complete`, `score_series_complete`, `filter_series_complete`, `rank_series_complete`, `rank_set_series_complete`, `annotate_series_complete`, `recognize_series_complete`, `relate_series_complete` and `find_series_complete` read actual text Series. Each returns the native typed `Call` and a separate vector mapping compact native occurrence ordinals to original nullable frame rows. Null cells send nothing. Native answer IDs, original inputs, rank member details and call facts remain intact.

For explicit images, selected JSON, per-record context/candidates or physical source inputs, use the matching `*_input_column_complete` method with the actual Series and a parallel `Vec<Option<RecordInput<T>>>`. The method checks that the row count and null positions match before any send. The Series supplies presentation; the typed record carries exact admitted evidence and provenance. The six row-wise `*_input_column_batch` methods keep native pull behavior and return the same position map. `choose_dynamic_input_column_complete` and its batch route accept a whole ordered candidate list per record.

Use the existing native `read_inputs` and `RecordReading` for located inputs. Associate those typed inputs with the frame column; no frame method opens a second reader. Collect a whole logical lazy frame before rank or find, as the retained `rank_lazy` and `find_lazy` helpers do. A native compact ordinal is not a dataframe presentation index.

## Migration to native ownership

The named `PolarsEngine` methods retain their signatures and native column results. Complete methods retain typed originals separately from nullable presentation columns; their returned position maps preserve each original row. Replace a host `complete` request or handwritten result reader with these named typed methods. Native `Request` admission owns the question and record reading, and native complete values own answers and call facts.

The installed frame consumer now links only the unpacked `thinkthen` crate. It no longer depends on the legacy host complete dispatcher or a copied settings adapter. Its small caller fixture shares framing with the direct Rust consumer, while actual frame methods continue to test native Series associations. Routine checks select the shared routine cases; full parity and platform checks belong to the candidate.
