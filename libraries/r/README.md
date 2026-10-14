# The R surface

The package has one family of named calls: `tt_decide`, `tt_choose`, `tt_tag`, `tt_score`, `tt_filter`, `tt_rank`, `tt_find`, `tt_annotate`, `tt_recognize`, and `tt_relate`. Each takes `question`, `input`, `options = list()`, `deadline_ms = NULL`, and `completion = NULL`. R converts values; Rust admits questions, records, options, saved selectors, declarations, reading, cache behavior and execution.

```r
library(thinkthen)

question <- "Does the customer ask for a refund?"
broken <- "Please refund my order. It arrived broken."
refund <- tt_decide(question, broken)
is_refund <- refund$value
stopifnot(identical(is_refund, TRUE))
stopifnot(identical(refund$probability, 0.99))
```

An ordinary `NULL` input supplies an empty record collection. `tt_input("json", value = NULL)` retains explicit JSON null. A scalar character input is literal text. Other scalar or structured inputs become JSON evidence. Record functions take a vector, list, or data frame of original records; data frames convert by row. Character vectors also supply records to primitive calls. Ordinary atomic R columns retain missing slots separately from native evidence. Primitive calls and recognition submit only present values; `filter`, `rank`, `find`, `annotate` and `relate` refuse missing atomic records. Nested missing fields in structured JSON evidence retain JSON null. JSON null, an absent generated field, and R `NA` remain distinct values.

A question string is literal wording, including strings beginning with `@`. An authored question is an ordinary named list. Explicit authority uses `tt_question(file = "question.json")`, `tt_question(name = "support.refund")`, or `tt_question(reference = "@support.refund")`. `tt_files(paths, unit = "line", window = NULL, media = "text")` selects native file reading. See the [canonical request schema](../../specification/request.schema.json) for accepted selector fields and options. Rust rejects incompatible options before sending or reading selected sources.

`tt_input(kind, ...)` marks an explicit canonical input descriptor without validating it. Use this for ordered items with per-record context or options, image attachments, or a single JSON array that should remain one original.

Calls have class `thinkthen_Call` and contain `$value`, `$probability`, `$results` and `$facts`. Result classes include `thinkthen_DecideResult`, `thinkthen_ChooseResult`, and the other generated result classes. Their fields support ordinary R `$`, `[[`, indexing, and comparison. Native indexes and span offsets stay zero-based. Ordinary atomic columns carry `$positions`, which maps compact native record indexes to original zero-based R slots, and `$length`, which retains the full column length. For `c("alpha", NA_character_, "beta")`, the map is `c(0, 2)` and the length is three; the two generated results retain native indexes zero and one and their original answer identities. Scalar primitive `$value` and decision/choice `$probability` views fill skipped slots with typed missing values; tag views retain empty character vectors for missing slots. Plans and batches retain the same map beside native plans or rows. These presentation positions do not rewrite native identities or represent completed missing records. Add one to a span start for `substr`. Generated optional fields use `thinkthen_absent` when missing and `NULL` when explicitly null. Results, inputs, identities, plans, and questions print summaries that withhold content. Retained values survive garbage collection.

A native failure raises a `thinkthen_error` condition with `$kind`, `$retryable`, `$complete`, and native facts when available. A joined streamed failure also carries its actual completed prefix in `$completed`. Pre-invocation refusals have no invented facts. R interruption cancels native work and raises R's interrupt condition. `tt_completion()` and `tt_completion_read()` retain asynchronous settlement evidence after the synchronous caller is interrupted.

`tt_plan(question, input, function_name = "decide", options = list())` uses native preview admission for primitive functions. It returns a safe-printing `thinkthen_Plan` with record and request counts, size estimates, and the first request body. It sends no request.

`tt_batch(function_name, question, input, options = list())` owns a native request session. `$next_result()` blocks in R's synchronous idiom, `$poll()` returns a ready result or `NULL`, `$cancel()` signals cancellation, `$close()` stops consumption, and `$facts()` returns actual terminal facts after settlement. Dropping the last handle cancels through native finalization. Aggregate functions retain native aggregate output. Deadline and attempt controls for a session go in `options`.

| Previous call | Current call |
| --- | --- |
| `tt_decide(question, input, threshold = cut)` | `tt_decide(question, input, options = list(threshold = cut))` |
| `tt_choose(question, input, options = labels)` | `tt_choose(list(choose = question, options = labels), input)` |
| `tt_score(question, input, levels = levels)` | `tt_score(list(score = question, levels = levels), input)` |
| `tt_tag(question, input, labels = labels)` | `tt_tag(list(tag = question, labels = labels), input)` |
| `tt_details(question, input)` | `tt_decide(question, input)$results[[1]]` |
| `tt_*_complete(question, input)` | `tt_*(question, input)` with ordinary values or explicit selectors |
| `tt_*_batch(question, input)` | `tt_batch("*", question, input)` |
| `tt_files(question, paths)` | `tt_*(question, tt_files(paths))` |
| `tt_plan(judge, input)` | `tt_plan(question, input)` |

The previous simplified frames, judge closures, `_complete` functions, and named `_batch` functions are no longer public. The old R native endpoints and handwritten compatibility readers have been deleted. `tt_engine()` selects the session engine, and `tt_usage()` reports its running usage totals.

For bounded records, `tt_feed(next_item, name = "records")` supplies an R producer. The producer returns `tt_record(original, location = NULL, ...)`, ordinary evidence, or `NULL` at end of input. Use `tt_record(NULL)` for a present JSON null. Record fields such as context, options and images pass to native admission. An explicit `tt_reader_failure("io")`, `"utf8"` or `"invalid_input"` finishes input with a native failure. Rust preserves the completed prefix and final facts. Producers run only on R's thread; polling retains one pending record while the native input cell is full. A producer's `close` callback releases its reader at input end, failure, cancellation or garbage collection.

Manual producers use `tt_batch(..., input = tt_feed())`, `$push(record)` and `$finish(failure = NULL)`. Push returns `"accepted"`, `"full"` or `"closed"`; retry the same record after polling when full, and stop advancing input when closed. `$poll()` remains nonblocking and `$next_result()` waits while checking R interrupts. `$cancel()` stops intake promptly; final facts arrive after already-sent native work settles. A named call joins native result packets into its normal `thinkthen_Call` result. Its `deadline_ms` maps to the native request option. Completion receipts are unavailable for feed calls and refuse before producer access. Append `framing`, `reading` and `images` to `tt_feed` when the native feed needs them; the existing `next_item`, `name` and `close` positional arguments keep their order. Omitted fields use native defaults. With `framing = "jsonl"`, the producer supplies one raw JSON line per item. With `framing = "lines"`, it supplies one text line. Rust decodes and skips blank records, retains caller locations, and refuses unsupported framing or projection combinations before producer access. Use the named call's `options` for `field`, `context_field` and `options_field`; `tt_record` continues to supply already composed values.

With `framing = "csv"` or `"tsv"`, supply one complete encoded header first, then at most one complete encoded data row per item. A quoted multiline cell belongs in the same item. Rust decodes the table and applies the named call's projections; `tt_record` can retain physical locations and supply explicit row context or options.

With projected JSON records, each record can supply `body`, `policy` and a `teams` list. The result retains that whole original record and its physical location. A later malformed line raises a native condition with the completed results and final facts.

## Install

Use R 4.2 or later and jsonlite 2.0.0 or later. Install the source package from R-universe:

```r
install.packages("thinkthen", type = "source",
  repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
```

A source install needs R development headers, a C compiler, make and Rust on PATH. The published source package downloads its matching Rust engine and dependencies. A vendored source archive carries those dependencies for offline installation. The package links its Rust engine into `libs/thinkthen.so`; it needs no separate C archive or runtime library path.

From a checkout, run `CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2 R CMD INSTALL --library=rlib libraries/r/thinkthen` after creating `rlib`. Set `R_LIBS_USER` to that installed library directory. `tools/make-tarball.sh OUT_DIR` creates a vendored source archive inside a `git archive` tree. The [backend guide](https://thinkthen.dev/install/backends/) explains keys and local models; the [recording guide](https://thinkthen.dev/learn/recording/) explains offline replay.

## Checking

From `libraries/r`, `./check.sh` installs the vendored source archive and runs the routine outside-in checks. `THINKTHEN_ARTIFACT` selects an existing archive. `THINKTHEN_TEST_PROFILE=full` selects all shared cases and the release-only interruption and concurrency campaigns at the candidate. An explicit `CARGO_TARGET_DIR` belongs to the installer and is retained for reuse; otherwise package-local build output is removed after installation. It exits 77 and reports "not run" if R, a tested R dependency or a cached crate is missing. `tools/setup.sh` prepares pinned R dependencies on a networked machine. From the repository root, `sdlc/scripts/smoke libraries/r` installs into owned scratch, loads the native package and replays a saved answer; its loopback counter proves the consumer adds no requests.

## Live usage durability

`tt_usage_persistence()` observes without waiting; `tt_finish_usage_status()` drains current usage deltas. Both return a `thinkthen_UsageStatus` named list with `state` and optional `advice`. They observe the engine already selected by `tt_engine()` or the process default; they introduce no separate engine owner.

Written covers this engine's current deltas, not later calls or other engines. Only usage-lock acquisition has a deadline; other filesystem work can take longer. Failed reports incomplete durable totals while answers and in-memory counts remain usable. These live observations contain no facts-snapshot timestamp, credentials or filesystem paths, and add no model requests.

## Typed questions and answers

Use `tt_options()` for request controls, `tt_recognize_question()` for recognition and `tt_relate_question()` for relation rules. Each successful named call returns the existing `thinkthen_Call` with `$value`, `$probability`, `$results` and `$facts`. Read the value in one line, as in `tt_decide(question, text)$value`. Failures raise `thinkthen_error`.

Primitive text input returns a scalar; atomic columns return aligned columns with missing positions retained. Tag returns label vectors per record. Filter returns a list of kept inputs, and rank returns a list of zero-based indexes; their probability lists follow the retained order. Find returns the selected input or null. Annotate returns a list of named field values per record and a null probability.

Scalar recognition returns an entity or boundary-proposal carrier. Record recognition returns a list of those carriers. Recognition probabilities are lists from retained span proposals or boundary proposals, never entity strength. Relate returns typed edges; its probability list contains each edge's probability. Complete native rows remain in `$results`, and `$facts` retains actual settlement evidence. The [ten-function example](examples/native.R) runs from the installed package.

A bounded feed joins into the same call fields. Use `tt_batch` to consume packets as they arrive; retain delivered packets yourself. `tt_batch` supplies no ordinary call-result accessor. Completion handles retain final facts after interruption for supported calls.
