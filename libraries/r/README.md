# The R surface

The package `thinkthen` binds R to the public `thinkthen` Rust API (ticket 0108). Its verbs carry the `tt_` prefix: `tt_decide`, `tt_choose`, `tt_score`, `tt_tag`, `tt_filter`, `tt_rank`, `tt_find`, `tt_annotate`, `tt_recognize`, `tt_relate`, `tt_details`, `tt_usage`, `tt_question`, and `tt_engine`. A column goes in and a column of the same length comes out. `NA` means not sure. The six error kinds arrive as R conditions, such as `thinkthen_usage`, and each carries `retryable`.

```r
library(dplyr)
library(thinkthen)
tickets |>
  filter(tt_decide("Is this a complaint?", body)) |>
  mutate(team = tt_choose("Which team owns this?", body, c("billing", "shipping", "account")))
```

- `tt_choose`, `tt_score`, and `tt_tag` over a column send one annotate call, so their requests run in parallel.
- `tt_recognize` returns one data frame per text, with the columns `text`, `start`, `end`, `length`, `kind`, and `strength`. The `start` and `end` columns count characters from one, so `substr(text, start, end)` gives the name, and `length` counts its characters. With no kinds, every name has the kind `ENTITY`. Relations ride in the `relations` attribute.
- `tt_relate` takes a data frame with `name` and `kind` columns, such as `tidyr::unnest()` of `tt_recognize`. A frame with `text` and no `name` column is named by its `text`. It returns edges whose first two columns `igraph::graph_from_data_frame` reads.
- `tt_engine()` sets the base URL, model, throttle, request cap, and cache once per R session, over the environment's own settings. The key comes only from `THINKTHEN_API_KEY`.
- The throttle caps the requests in flight in one loaded copy of the engine. A process that loads two copies, such as this package and a database extension, can run up to twice the throttle (ADR 0047 item 5).
- Ctrl-C stops a call at the next 100 ms tick and raises R's own `interrupt` condition (ADR 0042). Requests already sent finish on the backend.

## Run facts

`tt_details(question, text)` returns the command's `--details` line for one text as a list, schema `thinkthen.result/1`. The backend's reply supplies `meta.model`, `meta.usage` with its input and output tokens, and every probability, with `answer.confidence` when the backend sends one. The engine counts `meta.requests_sent` and sets `meta.cached` when a cache or recording answered. `meta.requests` holds the recording digest of each request, and `meta.url` names the address that answered. A field the backend did not report is absent. No call reports cost or time yet.

`tt_usage()` returns this engine's running totals of requests sent, cache answers and tokens.

## Building and checking

```sh
R CMD INSTALL -l rlib thinkthen    # builds the crate with cargo --locked --offline
./check.sh                         # the whole check, offline
```

`check.sh` needs R 4.2 or later, jsonlite 2.0.0, dplyr, tidyr, igraph, and the cargo cache. It reports "not run" and exits 77 when one is missing. `tools/setup.sh` fetches the pinned R archives on a networked machine. Each R test file runs against its own loopback backend through `tests/with-backend.sh`, and no test can reach a paid backend.

`tools/make-tarball.sh` runs inside a `git archive` tree and builds a source tarball with the `thinkthen` crate and every registry crate vendored. It installs with an empty cargo home and no network.

Nothing is published: no CRAN and no R-universe. `NOTES.md` holds the rulings and the measured behavior.
