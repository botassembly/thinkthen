library(thinkthen)

question <- "How urgent is this?"
levels <- c("Routine.", "Soon.", "Immediate.")
outage <- paste0(
  "Our checkout page is down and customers cannot pay.",
  "\n"
)
urgency <- tt_score(
  list(score = question, levels = levels),
  outage
)$answer
stopifnot(identical(urgency, 2))
