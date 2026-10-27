# ThinkThen for R

Install the imported R packages listed in DESCRIPTION, then run `R CMD INSTALL /path/to/thinkthen_0.2.0.tar.gz` with the supplied package. Source packages require R >= 4.2.0, R development headers, a C compiler, GNU make and Rust >= 1.95.0. Source installation supports Unix platforms. Windows uses the supplied binary package. Binary packages include the native engine.

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
