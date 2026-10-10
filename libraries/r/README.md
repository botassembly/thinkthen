# The R surface

The development 0.2 package has one family of named calls: `tt_decide`, `tt_choose`, `tt_tag`, `tt_score`, `tt_filter`, `tt_rank`, `tt_find`, `tt_annotate`, `tt_recognize`, and `tt_relate`. Each takes `question`, `input`, `options = list()`, `deadline_ms = NULL`, and `completion = NULL`. R converts values; Rust admits questions, records, options, saved selectors, declarations, reading, cache behavior and execution.

```r
library(thinkthen)
call <- tt_decide("Does this ask for a refund?", "Please refund my order.")
call$results[[1]]$value
call$facts$requests_sent

choice <- tt_choose(list(choose = "Which category?", options = c("refund", "other")),
                    "Please refund my order.")
choice$results[[1]]$value
prioritized_messages <- tt_rank("Which message most clearly asks for a refund?",
                  c("Please refund my order.", "When will it arrive?"))
prioritized_messages$results[[1]]$index
```

An ordinary `NULL` input supplies an empty record collection. `tt_input("json", value = NULL)` retains explicit JSON null. A scalar character input is literal text. Other scalar or structured inputs become JSON evidence. Record functions take a vector, list, or data frame of original records; data frames convert by row. Character vectors also supply records to primitive calls. Ordinary atomic R columns retain missing slots separately from native evidence. Primitive calls and recognition submit only present values; `filter`, `rank`, `find`, `annotate` and `relate` refuse missing atomic records. Nested missing fields in structured JSON evidence retain JSON null. JSON null, an absent generated field, and R `NA` remain distinct values.

A question string is literal wording, including strings beginning with `@`. An authored question is an ordinary named list. Explicit authority uses `tt_question(file = "question.json")`, `tt_question(name = "support.refund")`, or `tt_question(reference = "@support.refund")`. `tt_files(paths, unit = "line", window = NULL, media = "text")` selects native file reading. See the [canonical request schema](../../specification/request.schema.json) for accepted selector fields and options. Rust rejects incompatible options before sending or reading selected sources.

`tt_input(kind, ...)` marks an explicit canonical input descriptor without validating it. Use this for ordered items with per-record context or options, image attachments, or a single JSON array that should remain one original. For example:

```r
input <- tt_input("records", items = list(list(
  original = list(kind = "json", value = list(body = "Please refund my order.")),
  context = list(queue = "support")
)))
call <- tt_decide(list(decide = "Does this ask for a refund?", on = "/body"), input)
```

Calls have class `thinkthen_Call` and contain `$results` and `$facts`. Result classes include `thinkthen_DecideResult`, `thinkthen_ChooseResult`, and the other generated result classes. Their fields support ordinary R `$`, `[[`, indexing, and comparison. Native indexes and span offsets stay zero-based. Ordinary atomic columns carry `$positions`, which maps compact native record indexes to original zero-based R slots, and `$length`, which retains the full column length. For `c("alpha", NA_character_, "beta")`, the map is `c(0, 2)` and the length is three; the two generated results retain native indexes zero and one and their original answer identities. Scalar primitive `$value` and decision/choice `$probability` views fill skipped slots with typed missing values; tag views retain empty character vectors for missing slots. Plans and batches retain the same map beside native plans or rows. These presentation positions do not rewrite native identities or represent completed missing records. Add one to a span start for `substr`. Generated optional fields use `thinkthen_absent` when missing and `NULL` when explicitly null. Results, inputs, identities, plans, and questions print summaries that withhold content. Retained values survive garbage collection.

A native failure raises a `thinkthen_error` condition with `$kind`, `$retryable`, `$complete`, and native facts when available. A joined streamed failure also carries its actual completed prefix in `$completed`. Pre-invocation refusals have no invented facts. R interruption cancels native work and raises R's interrupt condition. `tt_completion()` and `tt_completion_read()` retain asynchronous settlement evidence after the synchronous caller is interrupted.

`tt_plan(question, input, function_name = "decide", options = list())` uses native preview admission for primitive functions. It returns a safe-printing `thinkthen_Plan` with record and request counts, size estimates, and the first request body. It sends no request.

`tt_batch(function_name, question, input, options = list())` owns a native request session. `$next_result()` blocks in R's synchronous idiom, `$poll()` returns a ready result or `NULL`, `$cancel()` signals cancellation, `$close()` stops consumption, and `$facts()` returns actual terminal facts after settlement. Dropping the last handle cancels through native finalization. Aggregate functions retain native aggregate output. Deadline and attempt controls for a session go in `options`.

| Previous call | Development 0.2 call |
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

The previous simplified frames, judge closures, `_complete` functions, and named `_batch` functions are no longer public. Old native compatibility modules remain private while other hosts still consume them. `tt_engine()` selects the session engine, and `tt_usage()` reports its running usage totals.

For bounded records, `tt_feed(next_item, name = "records")` supplies an R producer. The producer returns `tt_record(original, location = NULL, ...)`, ordinary evidence, or `NULL` at end of input. Use `tt_record(NULL)` for a present JSON null. Record fields such as context, options and images pass to native admission. An explicit `tt_reader_failure("io")`, `"utf8"` or `"invalid_input"` finishes input with a native failure. Rust preserves the completed prefix and final facts. Producers run only on R's thread; polling retains one pending record while the native input cell is full. A producer's `close` callback releases its reader at input end, failure, cancellation or garbage collection.

```r
at <- 0L
feed <- tt_feed(function() {
  at <<- at + 1L
  if (at > length(messages)) return(NULL)
  tt_record(messages[[at]])
})
call <- tt_decide("Does this ask for a refund?", feed)
```

Manual producers use `tt_batch(..., input = tt_feed())`, `$push(record)` and `$finish(failure = NULL)`. Push returns `"accepted"`, `"full"` or `"closed"`; retry the same record after polling when full, and stop advancing input when closed. `$poll()` remains nonblocking and `$next_result()` waits while checking R interrupts. `$cancel()` stops intake promptly; final facts arrive after already-sent native work settles. A named call joins native result packets into its normal `thinkthen_Call` result. Its `deadline_ms` maps to the native request option. Completion receipts are unavailable for feed calls and refuse before producer access. Append `framing`, `reading` and `images` to `tt_feed` when the native feed needs them; the existing `next_item`, `name` and `close` positional arguments keep their order. Omitted fields use native defaults. With `framing = "jsonl"`, the producer supplies one raw JSON line per item. With `framing = "lines"`, it supplies one text line. Rust decodes and skips blank records, retains caller locations, and refuses unsupported framing or projection combinations before producer access. Use the named call's `options` for `field`, `context_field` and `options_field`; `tt_record` continues to supply already composed values.

With `framing = "csv"` or `"tsv"`, supply one complete encoded header first, then at most one complete encoded data row per item. A quoted multiline cell belongs in the same item. Rust decodes the table and applies the named call's projections; `tt_record` can retain physical locations and supply explicit row context or options.

```r
reader <- file("rows.jsonl", open = "r")
line <- 0L
feed <- tt_feed(function() {
  raw <- readLines(reader, n = 1L)
  if (!length(raw)) return(NULL)
  line <<- line + 1L
  tt_record(raw, list(file = "rows.jsonl", first_line = line, last_line = line))
}, close = function() close(reader), framing = "jsonl")
ownership <- tt_choose(list(choose = "Which team owns this?"), feed,
                    options = list(field = list("/body"), context_field = "/policy", options_field = "/teams"))
```

Each record supplies `body`, `policy` and a `teams` list. The result retains that whole original record and its physical location. A later malformed line raises a native condition with the completed results and final facts.

## Install on Linux

Public installation remains **0.1.2** until 0.2 is published. The current source checkout is a **0.2.0 development build**. Installing the command or the C library alone does not install the R package.

Use R 4.2 or later and jsonlite 2.0.0 or later. A source install also needs R development headers, a C compiler and linker, make, and Rust's `cargo` and `rustc` on PATH. Public 0.1.2 requires Rust 1.95.0 or later; this development checkout pins Rust 1.95.0 in `rust-toolchain.toml`. On Debian/Ubuntu, the R/compiler prerequisites are `r-base-dev` and `build-essential`; other distributions use their corresponding development packages. dplyr, dbplyr, purrr, tidyr and igraph are optional for application code, and required by the complete surface check.

### R-universe

```r
install.packages("thinkthen", type = "source",
  repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
library(thinkthen)
packageVersion("thinkthen")
```

The source route downloads the exact package version's Rust engine from crates.io and resolves its Cargo dependencies during installation, so it needs network access as well as the source prerequisites. It is different from the vendored offline tarball below. Check `packageVersion()` after installing: R-universe serves its current indexed version, not a permanently pinned 0.1.2 archive.

On **Ubuntu 26.04 (Resolute), R 4.6, x86-64**, the existing binary route avoids compiling ThinkThen:

```r
install.packages("thinkthen", repos = c(
  "https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/",
  "https://cloud.r-project.org"))
library(thinkthen)
```

Use `resolute-arm64` for the corresponding ARM64 build. Do not use these binaries as a general Linux route: match the distribution, architecture and R series. CRAN dependencies may still build from source, and `install.packages()` can fall back to source when a binary is missing. Keep the source prerequisites in that case. See [R-universe's Linux binary requirements](https://docs.r-universe.dev/install/binaries.html).

On 2026-10-06, the [source index](https://botassembly.r-universe.dev/src/contrib/PACKAGES) and the [x86-64 R 4.6 binary index](https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/src/contrib/PACKAGES) listed 0.1.2; the ARM64 R 4.6 index also listed 0.1.2. These index checks establish availability, not a public installation rehearsal on every host. No public 0.2 package is claimed here.

### Source package or development checkout

For an existing vendored source archive produced by `tools/make-tarball.sh`, install into a directory you own (replace the archive path with yours):

```sh
mkdir -p rlib
R CMD INSTALL --library=rlib /path/to/thinkthen_0.1.2.tar.gz
R_LIBS_USER="$PWD/rlib" Rscript --vanilla -e 'library(thinkthen); packageVersion("thinkthen")'
```

The vendored archive carries the Rust engine and registry dependencies. Cargo builds it with `--locked --offline`, including with an empty Cargo home. R, jsonlite and the source compiler prerequisites must already be installed; the tarball does not vendor R packages.

From the **current development checkout**, run at the repository root:

```sh
mkdir -p libraries/r/rlib
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=2 R CMD INSTALL \
  --library=libraries/r/rlib libraries/r/thinkthen
R_LIBS_USER="$PWD/libraries/r/rlib" Rscript --vanilla -e 'library(thinkthen); packageVersion("thinkthen")'
```

This builds 0.2.0 from the local Rust engine. It needs the pinned toolchain and cached dependencies for `libraries/r/thinkthen/src/rust/Cargo.lock`. A missing cached crate is a prerequisite failure; on a networked preparation machine, `cargo fetch --locked --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml` fills that cache. `tools/make-tarball.sh OUT_DIR` creates the vendored package only inside a `git archive` tree, not a checkout.

`library(thinkthen)` loads the installed package's `libs/thinkthen.so` and its registered native routines. The Rust engine is linked into that shared object; no separate `libthinkthen.so`, C archive or `LD_LIBRARY_PATH` setting is needed. If loading fails, check `.libPaths()`, `find.package("thinkthen")`, the installed jsonlite version and the host/binary compatibility before retrying.

### Replay a saved answer without a key

Download and unpack the public 0.1.2 sample once:

```sh
curl -fsSL https://github.com/botassembly/thinkthen/releases/download/v0.1.2/thinkthen-first-run.tar.gz | tar -xz
```

Start a fresh R session with the installed package on its library path, in the directory containing `thinkthen-first-run`. The following call uses only the saved answer and sends no model request, including on a replay miss:

```r
library(thinkthen)
root <- "thinkthen-first-run"
tt_engine(replay = file.path(root, "recording"), cache = FALSE)
question <- "Does this report say what the person did before the problem appeared?"
text <- readChar(file.path(root, "report.txt"),
                 file.info(file.path(root, "report.txt"))$size)
call <- tt_decide(question, text)
stopifnot(identical(call$results[[1]]$value, TRUE), call$facts$requests_sent == 0)
call$results[[1]]$value
# [1] TRUE
```

No key is needed. Select replay before any asking call because R keeps one engine per session. The download needs network access; replay does not. For the development checkout, the same saved report and recording live in `demos/27-test-with-no-network` (use that path as `root`).

## Checking

From `libraries/r`, `./check.sh` runs the complete offline surface check. It exits 77 and reports "not run" if R, a tested R dependency or a cached crate is missing. `tools/setup.sh` prepares pinned R dependencies on a networked machine. From the repository root, `sdlc/scripts/smoke libraries/r` installs into owned scratch, loads the native package and replays a saved answer; its loopback counter proves the consumer adds no requests.
