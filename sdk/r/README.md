# R SDK

The R SDK asks questions about text from R. Its first function, `tt_decide()`, answers a yes-or-no question with yes (`TRUE`), no (`FALSE`) or not sure (`NA`).

Install it with R 4.2.0 or later: `install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))`. It carries its own engine and reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```r
library(thinkthen)

is_refund <- tt_decide("Please refund my order.",
                       question = "Does the customer request a refund?")
print(is_refund)
```

Output:

```text
[1] TRUE
```

`TRUE` means yes: the message asks for a refund. `tt_decide()` returns one answer per text, so `if`, `stopifnot()` and `mutate()` use it as it is.

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. Not sure is `NA`, and `if (NA)` stops with an error, so not sure never passes for no. A text ThinkThen could not judge, such as a blank one, raises a `thinkthen_failed_rows` error that carries its `code`. Send both to a person.

```r
library(thinkthen)

question <- "Does the customer request a refund?"
is_refund <- tt_decide("I was charged twice for one invoice.",
                       question = question,
                       threshold = c(0.2, 0.8))
print(is_refund)

failure <- tryCatch(tt_decide("", question = question),
                    thinkthen_failed_rows = identity)
print(failure$code)
```

Output:

```text
[1] NA
[1] "record.empty"
```

Pass `errors = "na"` to keep going past a failure. The answers stay a plain vector with `NA` for both, the call warns with every failed code, and `tt_failures()` lists each failure, so you can tell it from not sure:

```r
library(thinkthen)

texts <- c("Please refund my order.", "")
is_refund <- tt_decide(texts, question = "Does the customer request a refund?",
                       errors = "na")
tt_failures(is_refund)[c("position", "error_code")]
```

## Next

The [R SDK page](https://thinkthen.dev/sdks/r/) shows all ten functions. The [R guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/r.md) and the [API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/r/API.md) cover every option.

## Usage

Copy cumulative usage from an explicit client:

```r
library(thinkthen)
client <- tt_client()
tt_decide("A message", "Ready?", client = client)
usage <- tt_usage(client)
tt_close(client)
usage$attempts
usage$input_tokens$reported
```

Snapshots are independent values. Token totals count only provider-reported usage; finished attempts with missing usage are counted separately. Active calls and attempts can still add spending. Use a separate client when you need isolated totals. No prices or per-row token allocation are inferred.

`tt_close` preserves a copied snapshot without waiting for independent calls. Large JSON integers above exact R numeric range are retained as character values.

## Request allowance

```r
library(thinkthen)
client <- tt_client(max_requests = 100)
tt_decide("A message", "Ready?", client = client)
tt_close(client)
```

The optional allowance belongs to this explicit client and counts actual remote send attempts, including retries. Unset is unlimited; zero forbids remote sends. Stub calls use no remote allowance. Exhaustion ends the call with `local.request-limit`; it does not promise a final bill or refund in-flight requests. This setting does not inherit the command's environment allowance.

`max_requests` accepts whole numeric values within R's exact integer range; unsupported precision is rejected before sends.

## Local response storage

Explicitly select `cache`, `record`, or `replay` storage; leaving it unset retains no text. For an existing recording:

Create `tt_response_store("./answers", "replay", "example", "model-v1")` and pass it to `tt_client(response_store = store)`. Read `tt_usage(client)$replay_hits` for the copied hit count. This assumes that `./answers` already contains a matching recording.


Store paths must name private local directories. This opt-in feature retains input and output text without encryption. Namespace and model generation are explicit nonsecret labels; change the generation when the provider's model binding changes. Replay opens existing recorded answers without credentials or network fallback, and a missing answer fails with `local.replay-miss`. Cache/replay hit counters count materialized asks, not output rows, and do not add historical token usage to current spending. Optional limits use the engine defaults: 64 MiB logical payload, 4096 eligible asks, 86400 seconds cache age, and 5000 ms busy timeout. Recordings do not expire or get evicted to continue a recording. Filesystem allocation may exceed the logical payload limit. No environment variable enables retention.
