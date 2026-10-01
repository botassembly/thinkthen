library(dplyr)
library(thinkthen)

is_refund <- tt_decide(
  "Does the customer ask for a refund?",
  threshold = "0.2:0.8"
)
messages <- tibble(
  shop = c("north", "south"),
  body = c(
    "Please refund my order. It arrived broken.",
    "I want to send this back."
  )
)

answered <- messages |>
  group_by(shop) |>
  mutate(is_refund = is_refund(body)$value) |>
  ungroup()
print(select(answered, shop, is_refund))
