# The slide sample, exactly as drawn in
# the product deck's surfaces page.
# The harness defines the tickets frame before the drawn block and checks
# the answer after; the drawn block itself is untouched.

.libPaths(c("rlib", .libPaths()))

tickets <- data.frame(
  body = c(
    "I want a refund for order 9",
    "thanks, everything arrived fine",
    "maybe there is a problem with my bill"
  ),
  stringsAsFactors = FALSE
)

library(dplyr)
library(thinkthen)

teams <- c("billing", "shipping", "account")
levels <- c("Routine.", "Soon.", "Immediate.")

# a column goes in and a column comes out.
# NA is "not sure", and filter() drops those rows
tickets |>
  filter(tt_decide("Is this a complaint?", body)) |>
  mutate(
    team = tt_choose("Which team owns this?", body, teams),
    urgency = tt_score("How urgent is this?", body, levels)
  ) |>
  arrange(desc(urgency))
