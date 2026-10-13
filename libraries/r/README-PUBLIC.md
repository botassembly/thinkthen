# ThinkThen for R

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```r
install.packages("thinkthen", repos = c("https://botassembly.r-universe.dev", "https://cloud.r-project.org"))
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```r
library(thinkthen)
refunds <- c("Please refund my order. It arrived broken.",
             "Thanks for the quick help yesterday!", "I want to send this back.")
messages <- c("Please refund the extra fee on my invoice.",
              "My parcel went to the wrong address.", "I cannot reset my password.")
teams <- list(billing = "Invoices, fees, and refunds.",
              shipping = "Parcels and delivery.", account = "Logins and passwords.")
for (text in refunds) {
  refund <- tt_decide("Does the customer ask for a refund?", text)$answer
  print(refund)
}
for (text in messages) {
  team <- tt_choose(list(choose = "Which team owns this?", options = teams), text)$answer
  print(team)
}
```

```text
[1] TRUE
[1] FALSE
[1] TRUE
[1] "billing"
[1] "shipping"
[1] "account"
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
