library(thinkthen)

names <- data.frame(
  name = c(
    "Paul McCartney",
    "Ringo Starr",
    "Yesterday",
    "Octopus's Garden"
  ),
  kind = c("singer", "singer", "song", "song")
)
who_sings <- tt_relate(
  names,
  relations = "sings=singer:song"
)$value
stopifnot(identical(
  who_sings$source,
  c("Paul McCartney", "Ringo Starr")
))
stopifnot(identical(
  who_sings$target,
  c("Yesterday", "Octopus's Garden")
))
