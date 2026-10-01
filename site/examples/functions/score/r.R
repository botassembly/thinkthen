library(thinkthen)

question <- "How urgent is this?"
levels <- c("Routine.", "Soon.", "Immediate.")
outage <- paste0(
  "Our checkout page is down and customers cannot pay.",
  "\n"
)
urgency <- tt_score(question, outage, levels)$value
stopifnot(identical(urgency, 2.0))
