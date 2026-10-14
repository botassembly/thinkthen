library(thinkthen)

entities <- data.frame(
  name = c(
    "Apple",
    "Amazon",
    "iPhone",
    "Kindle"
  ),
  kind = c("company", "company", "product", "product")
)
makes <- list(
  name = "makes",
  source = "company",
  target = "product"
)
who_makes <- tt_relate(
  list(version = 1, relate = list(relations = list(makes))),
  entities
)$value
companies <- vapply(who_makes, \(edge) edge$source$name, "")
products <- vapply(who_makes, \(edge) edge$target$name, "")
stopifnot(
  identical(companies, c("Apple", "Amazon")),
  identical(products, c("iPhone", "Kindle"))
)
