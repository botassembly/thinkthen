# ThinkThen for R

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```r
install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```r
library(thinkthen)
question <- "Does the customer ask for a refund?"
refunds <- c("Please refund my order. It arrived broken.",
             "Thanks for the quick help yesterday!", "I want to send this back.")
messages <- c("Please refund the extra fee on my invoice.",
              "My parcel went to the wrong address.", "I cannot reset my password.")
teams <- list(billing = "Invoices, fees, and refunds.", shipping = "Parcels and delivery.",
              account = "Logins and passwords.")
is_refund <- tt_decide(question, refunds)
stopifnot(identical(is_refund$value, c(TRUE, FALSE, TRUE)))
stopifnot(identical(is_refund$probability, c(0.99, 0.01, 0.54)), length(is_refund$results) == 3)
routing <- list(choose = "Which team owns this?", options = teams)
owners <- tt_choose(routing, messages)
stopifnot(identical(owners$value, c("billing", "shipping", "account")))
stopifnot(identical(owners$probability, c(1, 1, 1)), length(owners$results) == 3)
```

Read `$value`, `$probability`, `$results` and `$facts` on the same call. A failed call raises an R condition.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/r/) for results, failures, record inputs and offline replay.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
