library(thinkthen)

entities <- data.frame(
  name = c(
    "Paul McCartney",
    "Ringo Starr",
    "Yesterday",
    "Octopus's Garden"
  ),
  kind = c("singer", "singer", "song", "song")
)
sings <- list(
  name = "sings",
  source = "singer",
  target = "song"
)
who_sings <- tt_relate(
  list(version = 1, relate = list(relations = list(sings))),
  entities
)$results[[1]]$value
singers <- vapply(who_sings, \(edge) edge$source$name, "")
songs <- vapply(who_sings, \(edge) edge$target$name, "")
stopifnot(
  identical(singers, c("Paul McCartney", "Ringo Starr")),
  identical(songs, c("Yesterday", "Octopus's Garden"))
)
