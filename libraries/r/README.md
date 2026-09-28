# The R surface

The package `thinkthen` binds R to the public `thinkthen` Rust API (ticket 0108). Its asking verbs carry the `tt_` prefix: `tt_decide`, `tt_choose`, `tt_score`, `tt_tag`, `tt_filter`, `tt_rank`, `tt_find`, `tt_annotate`, `tt_recognize`, `tt_relate`, and `tt_details`. Each returns a `thinkthen_call` list with `$value`, `$facts`, and `$details`. The original column, frame, or scalar answer is in `$value`; `NA` still means not sure. The six error kinds arrive as R conditions, such as `thinkthen_usage`, and each carries `retryable`. `tt_usage`, `tt_question`, and `tt_engine` are not asking verbs.

```r
library(dplyr)
library(thinkthen)
tickets |>
  filter(tt_decide("Is this a complaint?", body)$value) |>
  mutate(team = tt_choose("Which team owns this?", body, c("billing", "shipping", "account"))$value)
```

- `tt_choose`, `tt_score`, and `tt_tag` over a column use the loaded question's dynamic-label many-record path. They retain label order and saved profiles. A structured question text uses one record per request; plain text can pack compatible records.
- `tt_question(file = path)` keeps a saved calibration `profile` on a single question. A built profiled question is refused by `tt_rank` and `tt_find` before a request; pass plain text to those verbs. The constructor also accepts structured question text and description values. On a decide question, `true = NULL` writes an explicit JSON null while an omitted `false` writes no key.
- `tt_recognize(...)$value` holds one data frame per text, with the columns `text`, `start`, `end`, `length`, `kind`, and `strength`. The `start` and `end` columns count characters from one, so `substr(text, start, end)` gives the name, and `length` counts its characters. With no kinds, every name has the kind `ENTITY`. Relations ride in the `relations` attribute.
- `tt_relate` takes a data frame with `name` and `kind` columns, such as `tidyr::unnest()` of `tt_recognize(...)$value`. A frame with `text` and no `name` column is named by its `text`. Its `$value` holds edges whose first two columns `igraph::graph_from_data_frame` reads.
- `tt_engine()` sets the base URL, model, throttle, request cap, request-byte ceiling, cache, timeout, retries, profile, record, strict replay, and batch setting once per R session, over the environment's own settings. `batch = "max"` packs compatible records; a positive whole number caps records per request. Eligible record calls accept `batch =` and literal nonblank `context =`. `tt_annotate` accepts `batch =` only. The key comes only from `THINKTHEN_API_KEY`.
- The throttle caps the requests in flight in one loaded copy of the engine. A process that loads two copies, such as this package and a database extension, can run up to twice the throttle (ADR 0047 item 5).
- Ctrl-C stops a call at the next 100 ms tick and raises R's own `interrupt` condition (ADR 0042). Requests already sent finish on the backend. Supply `completion = tt_completion()` to an asking call when its final post-interrupt account matters; `tt_completion_read(handle)` reports unused, claimed, running, then terminal. The same handle belongs to one call.

The answer cache is on by default. Each entry holds the complete request and reply, the judged text included, in plain text, with no expiry. Whoever can write the selected cache or recording folder controls the answers read from it; keep that folder private to people whose answers you trust. `cache prune` is the only thing that removes entries. Turn it off with `tt_engine(cache = FALSE)`.

## Run facts

Every asking call's `$facts` gives final call-scoped records, requests sent, cache answers, elapsed seconds, and provider tokens or model when reported. `$details` holds owned, ordered question observations with zero-based original input indexes and per-row request shares. An accounted error carries the facts and details on its named R condition. A refusal before Rust accounting has no facts. An all-`NA` permitted column has a measured zero-work account.

`tt_details(question, text)$value` is the command's `--details` line for one text, schema `thinkthen.result/1`. Its `meta` holds the model, reported usage, request digests, cache flag, and backend URL. A field the backend did not report is absent. No call reports cost or provider server time yet.

`tt_usage()` returns this engine's running totals of requests sent, retries, cache answers and tokens.

## Building and checking

```sh
R CMD INSTALL -l rlib thinkthen    # builds the crate with cargo --locked --offline
./check.sh                         # the whole check, offline
```

`check.sh` needs R 4.2 or later, jsonlite 2.0.0, dplyr, tidyr, igraph, and the cargo cache. It reports "not run" and exits 77 when one is missing. `tools/setup.sh` fetches the pinned R archives on a networked machine. Each R test file runs against its own loopback backend through `tests/with-backend.sh`, and no test can reach a paid backend.

`tools/make-tarball.sh` runs inside a `git archive` tree and builds a source tarball with the `thinkthen` crate and every registry crate vendored. It installs with an empty cargo home and no network.

Nothing is published: no CRAN and no R-universe. `NOTES.md` holds the rulings and the measured behavior.
