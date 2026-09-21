# The recognize and relate acceptance: the R sections of
# recognize-surfaces.md run as written, the offsets proven in R's own
# string indexing (substr counts code points from one), and the
# 255-record refusal. Offline: the stand-in answers from the recordings,
# no network, no key.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)
library(dplyr)
library(tidyr)

passed <- 0
fail <- function(what) stop(paste("failed:", what), call. = FALSE)
check <- function(what, held) {
  if (!isTRUE(held)) fail(what)
  passed <<- passed + 1
}

sentence <- "Maria Chen joined Northwind Freight in Chicago last spring."

# The acceptance call, as written: a list column of data frames, and
# tidyr::unnest() makes one row per name.
tickets <- data.frame(body = c(sentence), stringsAsFactors = FALSE)
names_rows <- tickets |>
  mutate(names = tt_recognize(body, c("person", "organization"))) |>
  tidyr::unnest(names)
check("one row per name", nrow(names_rows) == 2L)
check("the names, in text order",
      identical(names_rows$text, c("Maria Chen", "Northwind Freight")))
check("the kinds are the user's own words",
      identical(names_rows$kind, c("person", "organization")))
check("the offsets slice in R's own indexing",
      identical(substr(sentence, names_rows$start, names_rows$end), names_rows$text))
check("strength rides with each name",
      all(names_rows$strength > 0 & names_rows$strength <= 1))
check("the list column holds data frames",
      inherits(tt_recognize(sentence, c("person", "organization"))[[1]], "data.frame"))

# Relations turn on with a rule and ride in the frame's attribute.
with_rel <- tt_recognize(sentence, c("person", "organization", "place"),
                         relations = "works_for=person:organization")
rel <- attr(with_rel[[1]], "relations")
check("the relation attribute is a data frame", inherits(rel, "data.frame"))
check("works_for, 1 to 2", identical(rel$name, "works_for") &&
  identical(as.integer(rel$source), 1L) && identical(as.integer(rel$target), 2L))
check("a relation's number is its probability",
      isTRUE(all.equal(rel$probability, 1.0)))

# The any-kind end is the one-character string "*".
any_kind <- tt_recognize("The road from Hull to Leeds was closed.",
                         "place", relations = "located_in=*:place")
check("any to place answers", identical(any_kind[[1]]$text, c("Hull", "Leeds")))
check("and finds no located_in pair", is.null(attr(any_kind[[1]], "relations")))

# The offset case: an accented letter and an emoji before the name, in
# R's own string indexing.
emoji <- "Le café 😀 Maria Chen arrived."
found <- tt_recognize(emoji, "person")[[1]]
check("the emoji case finds the name", identical(found$text, "Maria Chen"))
check("the emoji case slices by code point",
      identical(substr(emoji, found$start, found$end), "Maria Chen"))
check("the emoji case offsets are R's own",
      identical(as.integer(found$start), 11L) && identical(as.integer(found$end), 20L))

# A recorded text with no names comes back as an empty frame, not an error.
none <- tt_recognize("Athletics is her favorite subject at school.", "person")[[1]]
check("a text with no names is an empty frame", nrow(none) == 0L)
check("the empty frame keeps its columns",
      identical(names(none), c("text", "kind", "start", "end", "strength")))

# NA evidence: an empty frame in place, the column keeps its length.
mixed <- tt_recognize(c(NA_character_, sentence), "person")
check("NA evidence keeps the column length", length(mixed) == 2L)
check("NA evidence gives an empty frame", nrow(mixed[[1]]) == 0L)

# The relate acceptance call, as written: a data frame of edges.
alerts <- data.frame(body = c(
  "Checkout returns 500 at the payment step.",
  "Card charges are failing for every customer.",
  "The nightly export ran two hours late.",
  "The payments database ran out of disk space."
), stringsAsFactors = FALSE)
edges <- tt_relate(alerts$body, relations = "caused_by", either = "same_as")
check("edges is a data frame", inherits(edges, "data.frame"))
check("the four-column shape", identical(names(edges),
      c("name", "source", "target", "probability")))
check("four edges", nrow(edges) == 4L)
check("the first edge", identical(edges$name[[1]], "caused_by") &&
  identical(as.integer(edges$source[[1]]), 1L) &&
  identical(as.integer(edges$target[[1]]), 2L) &&
  isTRUE(all.equal(edges$probability[[1]], 0.59)))

# The bar drops the 0.59 edge, mirroring the deck's Python comment.
strict <- tt_relate(alerts$body, relations = "caused_by", threshold = 0.9)
check("the bar keeps the 0.94 edge", identical(as.integer(strict$source[[1]]), 1L) &&
  identical(as.integer(strict$target[[1]]), 4L))

# More than 255 records refuses with the usage kind, before anything else.
refused <- tryCatch(tt_relate(rep("one alert", 256), relations = "caused_by"),
                    thinkthen_error = function(e) e)
check("256 records is a usage error", identical(refused$kind, "usage"))
check("the refusal names the limit and the count",
      grepl("255", refused$message) && grepl("256", refused$message))

# NA records are the host's own refusal, named.
na_relate <- tryCatch(tt_relate(c("a", NA), relations = "caused_by"),
                      error = function(e) e)
check("relate refuses NA records", inherits(na_relate, "error"))

# The reachable kinds: usage for an unrecorded text, an unrecorded rule,
# and a named end outside the asked kinds.
unrecorded <- tryCatch(tt_recognize("This sentence was never recorded.", "person"),
                       thinkthen_error = function(e) e)
check("an unrecorded text is usage", identical(unrecorded$kind, "usage"))
covered <- tryCatch(tt_recognize(sentence, c("person", "place"),
                                 relations = "located_in=*:place"),
                    thinkthen_error = function(e) e)
check("an unrecorded rule is usage", identical(covered$kind, "usage"))
check("the refusal names the covered rules", grepl("works_for", covered$message))
outside <- tryCatch(tt_recognize(sentence, "person",
                                 relations = "works_for=person:company"),
                    thinkthen_error = function(e) e)
check("a named end outside the kinds is usage", identical(outside$kind, "usage"))

cat("recognize acceptance:", passed, "checks passed\n")
