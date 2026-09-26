library(thinkthen)

text <- paste0(
  "Maria Chen joined Northwind Freight, ",
  "a company in Chicago."
)
kinds <- c("person", "organization", "place")
rules <- c(
  "works_for=person:organization",
  "based_in=organization:place"
)
found <- tt_recognize(text, kinds, relations = rules)[[1]]
stopifnot(identical(
  found$name,
  c("Maria Chen", "Northwind Freight", "Chicago")
))
stopifnot(identical(found$kind, kinds))
links <- attr(found, "relations")
stopifnot(identical(
  links$relation,
  c("works_for", "based_in")
))
stopifnot(identical(
  links$source,
  c("Maria Chen", "Northwind Freight")
))
stopifnot(identical(
  links$target,
  c("Northwind Freight", "Chicago")
))
