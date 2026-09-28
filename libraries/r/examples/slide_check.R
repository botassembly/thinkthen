# The slide sample's check. The drawn block lives in slide.R.
# This harness runs it on the generic arm and reads the frame it returns,
# then proves the comment beside it: NA is "not sure", and filter() drops
# those rows. Run through tests/with-backend.sh.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
drawn <- source(file.path(Sys.getenv("TT_TESTS"), "..", "examples", "slide.R"))$value
check("every ticket holds", identical(drawn$body, tickets$body))
check("the team and urgency columns", identical(drawn$team, rep("billing", 3L)) &&
      isTRUE(all.equal(drawn$urgency, rep(0.15, 3L))))
# A band the generic 0.9 falls inside makes every row "not sure", and
# filter() drops them all.
banded <- tickets |> filter(tt_decide("Is this a complaint?", body, threshold = c(0.2, 0.95))$value)
check("filter drops the not-sure rows", identical(nrow(banded), 0L))
finish("slide", 3L)
