# Rust Polars

Ask `thinkthen` questions of a Polars `Series` or `DataFrame` from Rust. The door is the `polars` feature of the `thinkthen` crate. It is off by default, so a build that does not ask for it compiles no Polars.

```toml
thinkthen = { version = "0.1", features = ["polars"] }
```

The feature adds one trait, `thinkthen::PolarsEngine`, to your own `thinkthen::Engine`. Each method reads a text column in place and makes one engine call over the whole column. That call takes the same batch path as a slice of strings, at the same throttle, and the answers come back in input order. The trait's rustdoc holds a full example.

Build that engine with `max_request_bytes`, `max_requests_total`, `timeout`, `max_retries`, `profile`, `record`, or strict `replay` before passing it to Polars.

A file-backed Rust `Question` keeps its saved calibration profile when the engine asks a scalar or Series question. Read `Engine::details` for the pinned question digest and an optional mismatch warning. A profiled question cannot be inserted as a member of an annotate frame's `QuestionSet`; the builder refuses it before sending.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. The column calls use your own `thinkthen::Engine`, so turn it off with `EngineBuilder::no_cache` when you build that engine.

The door takes Polars 0.55. Polars changes its Rust API between minor versions. `thinkthen::polars` re-exports the Polars the door was built with, so it names the version your `Series` must come from. The throttle is the most requests in flight at once. It holds per loaded copy of the library, so a program that loads two copies can run up to twice the throttle.

Each asking method returns `Call<Series>` or `Call<DataFrame>`. Read its Polars value with `call.value()` or take it with `call.into_value()`. `call.facts()` holds that completed invocation's record, request, cache, token, duration, and model facts, including when one column takes several requests. A started failure carries the same final account on its error. Caller-owned `Tally` joins completed call facts, including partial failures; absent token usage stays absent. Its seconds span the first start to the last finish, including concurrent calls. `Engine::usage()` remains a process total, and `Engine::details` still describes one asked text.

## Eager methods

| Method | Question | Result |
| --- | --- | --- |
| `decide_series` | a `Question` or `BandedQuestion` | `Boolean`, null for not sure |
| `choose_series` | a choose `Question` | `String`, null where nothing fits |
| `score_series` | a score `Question` | `Float64`, the position from 0 to one less than the number of levels |
| `tag_series` | a tag `Question` | `List(String)` |
| `annotate_frame` | a `QuestionSet` and the name of the text column | your frame plus one column per question, then `failed` |
| `probability_frame` | a decide or choose `Question` | a `DataFrame` with `value` and nullable `probability` columns |
| `plan_series` | a question and text `Series` | a no-send `PlanEstimate` using the engine's request planner |

Asking methods take `CallOptions`. `column_with` also accepts `PolarsCallOptions` for a call-level threshold, decide true/false meanings and an optional probability result. A probability request on score or tag is refused before sending. One deadline, cancel token, and interrupt check cover the whole column. The call runs on your thread and starts no worker of its own.

Null input cells are omitted from requests and restored as nulls in their original positions; non-null cells keep input order. `plan_series` validates the whole non-null input, discloses the first prepared body and full-input counts, and sends nothing. Its token range is a measured estimate of the prepared body, not billed usage.

In a frame, a decide column is nullable `Boolean`, a choose column `String`, a score column nullable `Float64`, and a tag column `List(String)`. These dtypes stay fixed when a question fails. A failed answer is null in its question column; the final `failed` column holds a nullable Struct with one field per question and the full nested `failed: {kind, cause}` marker. A row with no failures has a null outer `failed` cell. A not-sure `decide` or `choose` has a null answer without a marker. Your own columns come back unchanged. A failed row in a series call ends the call with the engine's `Backend` error.

## Refusals

Each refusal happens before any request and returns `thinkthen::Error::Usage`:

- `the column {name} is {dtype}, not text`. Cast a `Categorical` column to `String` first.
- `the frame holds no column {on}`
- `the frame already holds a column named {name}`
- `{method} needs a {verb} question, and this one is a {kind} question`

An empty column returns a `Call` with an empty value of the method's type and zero records and sends. No message or `Debug` line holds a text from your column.

## Checks

This folder holds no code. The door lives in `crates/thinkthen/src/public/frame.rs`, and its tests in `crates/thinkthen/tests/polars/`. `check.sh` is the one lane that compiles Polars. It runs from the repository root with its own target folder, a fake key, and a closed loopback address. It runs Clippy, the tests, and the rustdoc example with the feature on. It prints "not run" and exits 77 only when cargo's cache lacks the locked crates. The root `deny.toml` holds the four license exceptions the Polars tree needs.
