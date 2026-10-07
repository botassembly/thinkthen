# The R surface

The package `thinkthen` binds R to the public `thinkthen` Rust API (ticket 0108). Its asking verbs carry the `tt_` prefix: `tt_decide`, `tt_choose`, `tt_score`, `tt_tag`, `tt_filter`, `tt_rank`, `tt_find`, `tt_annotate`, `tt_recognize`, `tt_relate`, and `tt_details`. Each returns a `thinkthen_call` list with `$value`, `$probability`, `$facts`, and `$details`. Decide carries the yes probability and choose carries the selected label's probability in the same call; an unsure choice has `NA` probability. Score and tag have `NULL` probability. The original column, frame, or scalar answer is in `$value`; `NA` still means not sure. The six error kinds arrive as R conditions, such as `thinkthen_usage`, and each carries `retryable`. `tt_usage`, `tt_question`, and `tt_engine` are not asking verbs.

```r
library(dplyr)
library(thinkthen)
tickets |>
  filter(tt_decide("Is this a complaint?", body)$value) |>
  mutate(team = tt_choose("Which team owns this?", body, c("billing", "shipping", "account"))$value)
```

An omitted `input` returns a function judge with its question and call settings checked once. Apply it to a scalar or vector; pass `deadline_ms` or `completion` when applying it. An explicit `NULL` or `NA` input remains an eager call. `tt_plan(judge, input)` previews the same packed work with `records`, prepared `requests`, `estimated_bytes`, a lower/upper `estimated_input_tokens` band, `upper_bound`, and `first_body`. Planning uses the judge's bound batch and context; those controls cannot be overridden at plan time. It needs no key, reads no cache, and sends no request. Its request count precedes cache answers, refusal splits, and retries.

```r
complaint <- tt_decide("Is this a complaint?", threshold = "0.3:0.7")
preview <- tt_plan(complaint, tickets$body)
answer <- complaint(tickets$body)           # one packed call over the vector
answer$probability                         # yes probabilities from that call

library(dplyr)
tickets |>
  group_by(team) |>
  mutate(complaint = tt_decide("Is this a complaint?", body)$value)
# One packed call per group. purrr::partial(tt_decide, "Is this a complaint?")
# can be applied to a vector in the same way.
```

For a lazy table, dbplyr passes `thinkthen_decide` through to SQL without translating its name: `mutate(tbl, complaint = thinkthen_decide('Is this a complaint?', body, '{"threshold":0.7}'))`. DuckDB's vector execution packs rows. PostgreSQL's scalar `thinkthen_decide` calls once per row; use its `thinkthen_decide_many` keyed form when packing matters. `show_query()` previews SQL only and sends no judgment request.

- `tt_choose`, `tt_score`, and `tt_tag` over a column use the loaded question's dynamic-label many-record path. They retain label order and saved profiles. A structured question text uses one record per request; plain text can pack compatible records.
- `tt_question(file = path)` keeps a saved calibration `profile` on a single question. A built profiled question is refused by `tt_rank` and `tt_find` before a request; pass plain text to those verbs. The constructor also accepts structured question text and description values. On a decide question, `true = NULL` writes an explicit JSON null while an omitted `false` writes no key.
- `tt_recognize(...)$value` holds one data frame per text, with the columns `text`, `start`, `end`, `length`, `kind`, and `strength`. The `start` and `end` columns count characters from one, so `substr(text, start, end)` gives the name, and `length` counts its characters. With no kinds, every name has the kind `ENTITY`. Relations ride in the `relations` attribute, a frame whose last column, `either`, is `TRUE` for a relation of a both-ways rule, whose ends are then in the order the names were found.
- `tt_relate` takes a data frame with `name` and `kind` columns, such as `tidyr::unnest()` of `tt_recognize(...)$value`. A frame with `text` and no `name` column is named by its `text`. Its `$value` holds edges whose first two columns `igraph::graph_from_data_frame` reads. Its last column, `either`, is `TRUE` for an edge of a both-ways rule, whose ends are then in input order.
- `tt_decide`, `tt_choose`, `tt_score`, and `tt_tag` take `input`, `threshold`, `true`, `false`, `context`, `batch`, and `deadline_ms` by name. Each verb refuses keys it cannot use. A threshold can be a numeric cut or a string such as `"0.2:0.95"` for a decide band; choose and tag also accept a string cut. The old `evidence` name is `input`; the old `deadline` name raises `deadline was renamed deadline_ms, in milliseconds`. A deadline is a whole number of milliseconds, `-1` for none, or `NULL`.
- `tt_engine()` sets the backend, base URL, model, throttle, request cap, process request cap (`max_requests_total`), request-byte ceiling, cache, timeout, retries, profile, record, strict replay, and batch setting once per R session, over the environment's own settings. `batch = "max"` packs compatible records; a positive whole number caps records per request. Eligible record calls accept `batch =` and literal nonblank `context =`. `tt_annotate` accepts `batch =` only. Rust captures keys from the environment when the engine is constructed.
- The throttle caps the requests in flight in one loaded copy of the engine. A process that loads two copies, such as this package and a database extension, can run up to twice the throttle (ADR 0047 item 5).
- Ctrl-C stops a call at the next 100 ms tick and raises R's own `interrupt` condition (ADR 0042). Requests already sent finish on the backend. Supply `completion = tt_completion()` to an asking call when its final post-interrupt account matters; `tt_completion_read(handle)` reports unused, claimed, running, then terminal. The same handle belongs to one call.

`tt_engine(backend = "typesafe")` selects a built-in backend or a backend in the Rust configuration. The built-ins are `liquid`, `ollama`, `openrouter`, `perplexity`, and `typesafe`. Their keys come from `LIQUIDAI_API_KEY`, `OLLAMA_API_KEY`, `OPENROUTER_API_KEY`, `PERPLEXITY_API_KEY`, and `TYPESAFE_API_KEY`, respectively. Omit `backend` or use `NULL` to retain environment selection. Supply the full keyword name; existing positional arguments keep their positions. `base_url` and `model` override the selected backend's address and model. Its path, question form, prices, and profile remain selected unless their supported settings override them. Equal engine settings can be repeated. Changing them requires a new R session, and the refusal names the backend in force without its key. No R setting accepts a key. Backend selection belongs to `tt_engine`, not an asking verb.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `tt_engine(cache = FALSE)`.

## Indexes and source coordinates

These conventions describe the existing named R calls. They do not establish the result/2, image or typed source-carrier parity owned by 0431.

| Field or result | Convention |
| --- | --- |
| Column answers and recognition frames | Input order, including permitted `NA` positions |
| `tt_rank(...)$value$place` | One-based **original input position**, not rank number; output rows are best first |
| `tt_find(...)$value$place` | One-based selected original unit position |
| Asking call `$details[[i]]$index` | Zero-based original input index, including gaps for omitted `NA` evidence |
| Observation `$details[[i]]$position` | Zero-based question position within its member or recognition stage; not an input index or rank |
| `tt_recognize` frame `start`, `end` | One-based inclusive Unicode character positions; `length = end - start + 1` |
| `tt_files` located rank `index` | Native zero-based input record index; no R `place` conversion |
| `tt_files` recognition span `start`, `end` | Native zero-based Unicode scalar start and exclusive end, local to the record |
| `tt_files` `first_line`, `last_line` | Physical one-based inclusive source lines, on records and spans |

For the saved shared rank case with native indexes `1, 2, 0`, R returns `place = c(2L, 3L, 1L)`. Its first output row is rank one but came from input position two. `top` truncates those output rows without renumbering `place`. Ties retain input order, and duplicate text records retain distinct positions. `tt_filter` returns the original selected records in input order, without adding an index column. Use the call observations when original indexes are needed; they also include judged records that were filtered out.

The saved `18-find-second` case selects native input index `1` from `c("First passage.", "Second passage.", "Third passage.")`. The public `tt_find("Which passage answers the question?", units, none = TRUE)` result has `place = 2L`, `unit = "Second passage."`, and `probability = 0.8`. The native choice label is `u002`; a find observation's input `index` describes the whole comparison, not the selected candidate. The saved none case returns `NA_integer_` place, `NA_character_` unit and `NA_real_` probability. It is a successful absence. An invalid call raises a condition rather than returning a selected position. Fewer than two find units, including an empty vector, is usage. Empty rank returns a zero-row frame; neither empty call sends a request.

For recognized `"Maria Chen"` at the beginning of ASCII text, native `[0, 10)` becomes R `start = 1`, `end = 10`, `length = 10`. In the saved multibyte text `"Le café 😀 Maria Chen arrived."`, native `[10, 20)` becomes R `start = 11`, `end = 20`, `length = 10`:

```r
text <- "Le café 😀 Maria Chen arrived."
substr(text, 11, 20)                         # "Maria Chen"
nchar(substr(text, 1, 10), type = "bytes")   # 14, not the native start 10
```

Offsets count Unicode scalar values, not UTF-8 bytes or displayed grapheme clusters. Convert only the native start by adding one for R `substr`; the exclusive native end already equals the inclusive R end. Do not use these numbers to slice raw bytes. An `NA` recognition input has an empty frame; an empty text or text with no recognized names also has no span rows. Missing permitted column inputs have no observation and send no request: input positions one and three remain observation indexes `0` and `2`, not `0` and `1`. An accounted failure can retain earlier observations; those do not assert a successful answer for the failed input.

`tt_files` returns located native JSON values decoded into R lists, rather than the recognition data-frame conversion above. Slice a located span with `substr(row$record, span$start + 1, span$end)`. Blank line units are skipped but still count as physical lines: records on lines two and four have rank indexes `0` and `1`, while their `first_line` values remain `2` and `4`. Window/file spans remain local to their retained record text; their physical line ranges refer to the file. An exclusive end maps to the last included character's physical line. Find returns a located selection or `NULL` for none; a whole-call source failure returns no successful located result. See the [native reader contract](../files.md). Generic JSON access is compatibility access, not a claim of typed R parity.

## Run facts

Every asking call's `$facts` gives final call-scoped records, requests sent, cache answers, elapsed seconds, and provider tokens or model when reported. `$details` holds owned, ordered question observations with zero-based original input indexes and per-row request shares. An accounted error carries the facts and details on its named R condition. A refusal before Rust accounting has no facts. An all-`NA` permitted column has a measured zero-work account.

`tt_details(question, text)$value` is the command's `--details` line for one text, schema `thinkthen.result/1`. Its `meta` holds the model, reported usage, request digests, cache flag, and backend URL. A field the backend did not report is absent. No call reports cost or provider server time yet.

`tt_usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens.

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
stopifnot(identical(call$value, TRUE), call$facts$requests_sent == 0)
call$value
# [1] TRUE
```

No key is needed. Select replay before any asking call because R keeps one engine per session. The download needs network access; replay does not. For the development checkout, the same saved report and recording live in `demos/27-test-with-no-network` (use that path as `root`).

## Checking

From `libraries/r`, `./check.sh` runs the complete offline surface check. It exits 77 and reports "not run" if R, a tested R dependency or a cached crate is missing. `tools/setup.sh` prepares pinned R dependencies on a networked machine. From the repository root, `sdlc/scripts/smoke libraries/r` installs into owned scratch, loads the native package and replays a saved answer; its loopback counter proves the consumer adds no requests.

The [index conventions](#indexes-and-source-coordinates) distinguish R positions, native offsets and physical source lines. Explicit files and folders use the [library reader contract](../files.md) and [`tt_files` helper reference](thinkthen/man/tt_files.Rd), with line, window or whole-file units and located results. Existing text, record and column methods retain their arguments. The complete calls below expose typed native image and located result carriers.

## Complete native calls

`tt_decide_complete`, `tt_choose_complete`, `tt_tag_complete`,
`tt_score_complete`, `tt_filter_complete`, `tt_rank_complete`,
`tt_find_complete`, `tt_annotate_complete`, `tt_recognize_complete` and
`tt_relate_complete` return typed `results`, native final `facts`, original
`inputs` and `ordinals`. Known fields are validated classes with ordinary named
accessors; arbitrary caller payloads remain lists/JSON.

```r
question <- list(role="atomic", body=list(decide="Is this a complaint?"))
input <- list(kind="records", records=list(
  list(content=list(kind="text", value="Please refund."))))
call <- tt_decide_complete(question, input, attempts=TRUE)
probability <- call$results[[1L]]$answer$probability
requests <- call$facts$requests_sent
```

Question sources choose one body, raw JSON, path, name or reference, with an
explicit atomic, dynamic, rank, set, find, recognize or relate role. Rank sets
use set. Native loaders preserve structured descriptions, author names, wording
versions, declarations and annotation members.

File sources use `kind="files"`, `paths=list("report.txt")` and
`options=list(reading=list(unit="file"), media="text")`. Line/window readings
retain physical coordinates; `jsonl=TRUE` reads records and `media="image"`
reads explicit image files. Records may carry ordered PNG/JPEG byte attachments,
explicit context and described options. Images are admitted for decide, choose
and score and refused before sending for the other seven functions.

The six `tt_decide_batch`, `tt_choose_batch`, `tt_tag_batch`, `tt_score_batch`,
`tt_filter_batch` and `tt_annotate_batch` constructors return native lazy batches.
`next_row()` yields a typed result, ordinal and input; `facts()` is final after
exhaustion or terminal failure. `poll()` waits for one native polling interval
and returns NULL while pending, distinguished from EOF by `facts()`; this lets
R's main thread call `cancel()`. Use `close()` on early exit. Files and requests
wait for the first pull. R interrupts and native limits remain effective.

Complete failures retain the six ordinary condition kinds and expose typed
`complete`, including available native final facts and stopped position. Cache,
record, replay, attempts and changed-reading identity stay native. Printing a
carrier withholds its contents.

Complete carriers preserve native coordinates: ordinals and recognition starts
are zero-based, recognition ends are exclusive Unicode scalar offsets, and
physical lines are one-based inclusive. Native rank values remain one-based
ranks. Ordinary R rank/find `place` remains the original input index plus one;
ordinary recognition frame starts retain R's one-based inclusive convention.
Result and cache identity are computed before any compatibility conversion.
