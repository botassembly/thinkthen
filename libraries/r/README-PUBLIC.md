# ThinkThen for R

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```r
install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

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
stopifnot(length(is_refund$probability) == 3, length(is_refund$results) == 3)
routing <- list(choose = "Which team owns this?", options = teams)
owners <- tt_choose(routing, messages)
stopifnot(identical(owners$value, c("billing", "shipping", "account")))
stopifnot(length(owners$probability) == 3, length(owners$results) == 3)
```

Read `$value`, `$probability`, `$results` and `$facts` on the same call. A failed call raises an R condition.

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
