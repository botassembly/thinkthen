library(dplyr)
library(thinkthen)

question <- "Does the customer ask for a refund?"
messages <- tibble(
  body = c(
    "Please refund my order. It arrived broken.",
    "I want to send this back."
  )
)

answered <- messages |>
  mutate(
    is_refund = tt_decide(
      question,
      body,
      threshold = "0.2:0.8"
    )$value
  )
print(answered)

refunds <- answered |> filter(is_refund)
stopifnot(identical(refunds$body, messages$body[1]))
