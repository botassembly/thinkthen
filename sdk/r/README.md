# ThinkThen for R

Install `jsonlite`, then run `R CMD INSTALL /path/to/thinkthen_0.2.0.tar.gz` with the supplied package. Source packages require R development headers, a C compiler, make and Rust. Binary packages include the native engine.

The example uses the stub backend and needs no key or service.

```r
library(thinkthen)

messages <- data.frame(
  id = c("a", "b"),
  body = c("Please refund my order.", "Where is my delivery?")
)

refunds <- tt_decide(
  messages,
  "Does the customer ask for money back?",
  evidence = "/body",
  client = tt_client(backend = "stub"),
  threshold = c(0.2, 0.8)
)
refunds[c("status", "answer", "probability")]
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/r/API.md).
