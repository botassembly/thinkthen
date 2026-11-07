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
