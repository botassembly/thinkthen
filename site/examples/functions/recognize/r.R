library(thinkthen)

kinds <- list(
  person = NULL,
  product = NULL,
  place = NULL
)
text <- paste0(
  "Sam Patel says his Kindle arrived broken ",
  "in Denver."
)
facts <- tt_recognize(
  list(version = 1, recognize = list(kinds = kinds)),
  text
)$value[[1]]
entity_names <- vapply(facts$entities, `[[`, "", "text")
found_kinds <- vapply(facts$entities, `[[`, "", "kind")
stopifnot(
  identical(
    entity_names,
    c("Sam Patel", "Kindle", "Denver")
  ),
  identical(found_kinds, names(kinds))
)
