# Ordinary columns enter named calls; the caller extracts typed row values.
tickets <- data.frame(body = c("I want a refund for order 9", "thanks, everything arrived fine", "maybe there is a problem with my bill"), stringsAsFactors = FALSE)
library(dplyr)
library(thinkthen)
teams <- c("billing", "shipping", "account")
levels <- c("Routine.", "Soon.", "Immediate.")
complaints <- function(tickets, options = list()) {
  tickets |>
  filter(vapply(tt_decide("Is this a complaint?", body, options = options)$value, isTRUE, TRUE)) |>
  mutate(
    team = tt_choose(list(choose = "Which team owns this?", options = teams), body)$value,
    urgency = tt_score(list(score = "How urgent is this?", levels = levels), body)$value
  ) |>
  arrange(desc(urgency))
}
complaints(tickets)
