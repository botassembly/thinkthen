# Ian's item 6 for R: a fork after the first call answers in the child.
#
# mclapply's children are forked after the parent's first call, so each
# worker is the hostile shape: a fork whose parent holds engine state. A
# hang fails through the outer timeout in check.sh, which runs this under
# `timeout 60`.
#
# Run with: ENGINE_NULL=1 Rscript fork_check.R

.libPaths(c("rlib", .libPaths()))
library(thinkthen)
stopifnot(Sys.getenv("ENGINE_NULL") == "1")

question <- tt_question(decide = "Does the customer ask for a refund?")
first <- tt_decide(question, "I want a refund for order 9")
stopifnot(isTRUE(first))

answers <- parallel::mclapply(
  1:2,
  function(i) tt_decide(question, "I want a refund for order 9"),
  mc.cores = 2
)
stopifnot(all(vapply(answers, isTRUE, logical(1))))

cat("fork after the first call answers in the children\n")
