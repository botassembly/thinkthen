library(thinkthen)

kinds <- c("person", "organization", "place")
text <- paste0(
  "Maria Chen joined Northwind Freight, ",
  "a company in Chicago."
)
names <- tt_recognize(text, kinds)$value[[1]]
stopifnot(identical(
  names$text,
  c("Maria Chen", "Northwind Freight", "Chicago")
))
stopifnot(identical(names$kind, kinds))
