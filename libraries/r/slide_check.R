# The slide sample's check. The drawn block lives in slide.R untouched;
# this harness runs it and reads the frame it returns, then proves the
# comment beside it: NA is "not sure", and filter() drops those rows.
.libPaths(c("rlib", .libPaths()))

drawn <- source("slide.R")$value

stopifnot(nrow(drawn) == 2)
stopifnot(identical(drawn$body, c("I want a refund for order 9",
                                  "maybe there is a problem with my bill")))
stopifnot(isTRUE(all.equal(drawn$urgency, c(1.7, 1.05))))
# On the null backend the three teams tie exactly, and an exact tie is
# unsure: the choose column answers NA, the host's own empty value.
stopifnot(all(is.na(drawn$team)))

# The comment's second half: a band makes "not sure" a real answer, and
# filter() drops those rows. The same drawn verb carries the threshold.
tickets <- data.frame(
  body = c(
    "I want a refund for order 9",
    "thanks, everything arrived fine",
    "maybe there is a problem with my bill"
  ),
  stringsAsFactors = FALSE
)
banded <- tickets |>
  filter(tt_decide("Is this a complaint?", body, threshold = c(0.2, 0.8)))
stopifnot(nrow(banded) == 1, identical(banded$body, "I want a refund for order 9"))

cat("slide sample green: 2 rows kept, NA dropped by filter, choose NA on a tie\n")
