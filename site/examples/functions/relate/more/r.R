library(thinkthen)

rules <- c(
  "Book economy class for every flight under six hours.",
  "Submit receipts within 30 days of the trip.",
  "Hotel stays are capped at 200 dollars a night.",
  "Employees may book business class on any flight.",
  "Rental cars need a manager's approval.",
  paste0(
    "Receipts may be submitted at any time, ",
    "with no deadline."
  ),
  "Meals are reimbursed up to 60 dollars a day.",
  "Use the company travel portal for all bookings."
)
entities <- data.frame(name = rules, kind = "rule")
contradictions <- tt_relate(
  entities,
  either = "contradicts",
  threshold = 0.5
)$value
stopifnot(identical(contradictions$source, rules[c(1, 2)]))
stopifnot(identical(contradictions$target, rules[c(4, 6)]))
stopifnot(identical(
  contradictions$probability,
  c(0.83, 0.97)
))
