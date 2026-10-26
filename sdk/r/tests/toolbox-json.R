source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
# Feature: saved entries restore typed R questions and preserve execution.
root <- commandArgs(TRUE)[[1]]
json <- paste(readLines(file.path(root, "questions.json")), collapse = "\n")
box <- tt_toolbox_from_json(json)
stopifnot(inherits(tt_question(box, "ready"), "tt_filter_question"),
          inherits(tt_question(box, "topic"), "tt_choose_question"),
          inherits(tt_question(box, "names"), "tt_relate_question"))
saved <- tt_toolbox_to_json(box)
canonical <- function(value) {
  if (!is.list(value)) return(value)
  if (!is.null(names(value))) value <- value[order(names(value))]
  lapply(value, canonical)
}
stopifnot(identical(canonical(jsonlite::fromJSON(saved, simplifyVector = FALSE)),
                    canonical(jsonlite::fromJSON(json, simplifyVector = FALSE))))
loaded <- tt_toolbox_from_json(saved)
stub <- tt_client(backend = "stub")
inline <- tt_choose_question("Which topic?", c("billing", "shipping"), threshold = .7, evidence = "/body")
record <- list(list(body = "billing inquiry"))
stopifnot(identical(tt_choose(record, tt_question(loaded, "topic"), client = stub),
                    tt_choose(record, inline, client = stub)))
constructed <- tt_toolbox(list(choice = inline))
stopifnot(identical(tt_choose(record, tt_question(tt_toolbox_from_json(tt_toolbox_to_json(constructed)), "choice"), client = stub),
                    tt_choose(record, inline, client = stub)))
for (bad in jsonlite::fromJSON(file.path(root, "malformed.json"))) {
  condition(tt_toolbox_from_json(bad), "thinkthen_usage_error", "usage.question")
}
tt_close(stub)
cat("R saved toolbox translation passed\n")
