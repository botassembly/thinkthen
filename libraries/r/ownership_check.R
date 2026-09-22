# Punch-list item 3 for R: canonical results and ownership. The values a
# call returns are the host's own: mutating them must not change the
# engine's next answer, they stay readable through a gc() pass and later
# calls, repeated names carry distinct offsets, and relation endpoints
# are the entities' own ids. Offline: the stand-in answers from the
# recordings, no network, no key.
.libPaths(c("rlib", .libPaths()))
library(thinkthen)

passed <- 0
fail <- function(what) stop(paste("failed:", what), call. = FALSE)
check <- function(what, held) {
  if (!isTRUE(held)) fail(what)
  passed <<- passed + 1
}

twice <- "Chicago sent a delegation in March, and Chicago hosted the reply in June."

first <- tt_recognize(twice, "place")
check("repeated names give two entities", nrow(first[[1]]) == 2L)
check("the two names are both Chicago", identical(first[[1]]$text, c("Chicago", "Chicago")))
check("the two points differ", first[[1]]$start[[1]] != first[[1]]$start[[2]])
check("both offsets slice their name",
      identical(mapply(substr, twice, first[[1]]$start, first[[1]]$end, USE.NAMES = FALSE),
                first[[1]]$text))

# Mutate every layer the caller was handed, then ask again: the engine's
# next answer must be untouched.
first[[1]]$text[[1]] <- "Oslo"
first[[1]]$kind[[1]] <- "organization"
first[[1]]$start[[1]] <- 999L
first[[1]] <- NULL
invisible(gc())
second <- tt_recognize(twice, "place")
check("mutation of the first result changes nothing",
      identical(second[[1]]$text, c("Chicago", "Chicago")))
check("the kinds survive the mutation", identical(second[[1]]$kind, c("place", "place")))
check("the offsets survive the mutation",
      identical(mapply(substr, twice, second[[1]]$start, second[[1]]$end, USE.NAMES = FALSE),
                second[[1]]$text))

# Lifetime: values stay readable through gc() and later calls.
kept <- tt_recognize("Le café \U0001F600 Maria Chen arrived.", "person")
invisible(gc())
invisible(tt_usage())
check("the name is readable after gc", identical(kept[[1]]$text, "Maria Chen"))
check("the slice holds after gc",
      identical(substr("Le café \U0001F600 Maria Chen arrived.",
                       kept[[1]]$start, kept[[1]]$end), "Maria Chen"))

# Relation endpoints are the entities' ids.
sentence <- "Maria Chen joined Northwind Freight in Chicago last spring."
with_rel <- tt_recognize(sentence, c("person", "organization", "place"),
                         relations = "works_for=person:organization")
rel <- attr(with_rel[[1]], "relations")
check("the endpoint ids are the entities' own",
      identical(as.integer(rel$source), 1L) && identical(as.integer(rel$target), 2L))
check("the endpoints name the right rows",
      identical(with_rel[[1]]$text[[rel$source]], "Maria Chen") &&
      identical(with_rel[[1]]$text[[rel$target]], "Northwind Freight"))

cat(sprintf("ownership: %d checks, 0 failures\n", passed))
