# The R surface

Lands in Phase B. The acceptance sample, drawn in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md`, runs
as drawn against the stand-in when this folder holds the package:

```r
library(thinkthen)

refund <- tt_question(decide = "...", threshold = c(0.2, 0.8))
tickets |>
    filter(tt_decide(refund, body)) |>
    mutate(team = tt_choose("Which team owns this?", body,
        options = c("billing", "shipping", "account")))
tickets |> tt_annotate("form.json", on = body)
```

`NA` is "not sure". A column is one call, inside `filter()` and `mutate()`.
The verbs carry the `tt_` prefix. Cancel lands here in the brief's item 7,
and packaging goes through R-universe first.
