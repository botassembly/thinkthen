library(thinkthen)

kinds <- list(
  person = NULL,
  organization = NULL,
  place = NULL
)
text <- paste0(
  "Maria Chen joined Northwind Freight, ",
  "a company in Chicago."
)
facts <- tt_recognize(
  list(version = 1, recognize = list(kinds = kinds)),
  text
)$results[[1]]$value
entity_names <- vapply(facts$entities, `[[`, "", "text")
found_kinds <- vapply(facts$entities, `[[`, "", "kind")
stopifnot(
  identical(
    entity_names,
    c("Maria Chen", "Northwind Freight", "Chicago")
  ),
  identical(found_kinds, names(kinds))
)
