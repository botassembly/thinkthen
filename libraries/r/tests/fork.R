# A fork after the first call answers in the child, and the parent's
# counters do not move (0096, Q15). mcparallel forks a process whose
# parent already holds the default engine.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
check("the parent's first call answers", isTRUE(tt_decide("Q?", "before the fork")$value))
before <- tt_usage()
job <- parallel::mcparallel(c(tt_decide("Q?", "in the child")$value, tt_usage()$requests_sent))
got <- parallel::mccollect(job, timeout = 60)[[1]]
# The child counts from zero after the fork, so it reports its one send.
check("the forked child answers and counts its own send", identical(got, c(1L, 1L)))
check("the parent's counters do not move", identical(tt_usage(), before))
finish("fork", 2L)
