library(thinkthen)

question <- "How urgent is this?"
levels <- c("Routine.", "Soon.", "Immediate.")
texts <- c(
  "Please update my mailing address when you can.",
  "Can you send the signed contract by Friday?",
  "Nobody can log in to the site right now."
)
urgency <- tt_score(question, texts, levels)
stopifnot(identical(urgency, c(0.06, 0.99, 2.0)))
