# Read typed answers from the installed 0.2 family.
library(thinkthen)
result <- function(call, probability = NULL) {
  stopifnot(inherits(call, "thinkthen_Call"),
            all(c("value", "probability", "results", "facts") %in% names(call)),
            !"answer" %in% names(call), is.list(call$results),
            inherits(call$facts, "thinkthen_Facts"),
            identical(call$probability, probability))
  call
}
stopifnot(identical(result(tt_decide("A complaint?", "Refund please"), 0.9)$value, TRUE))
stopifnot(identical(result(tt_choose(list(choose = "Which team?", options = c("billing", "shipping")), "Refund please"), 0.9)$value, "billing"))
stopifnot(identical(result(tt_tag(list(tag = "Which labels?", labels = c("refund", "shipping")), "Refund shipping"))$value[[1]], c("refund", "shipping")))
stopifnot(result(tt_score(list(score = "How urgent?", levels = c("low", "soon", "high")), "Later"))$value == 0.15)
stopifnot(result(tt_filter("A complaint?", c("first", "second")), list(0.9, 0.9))$value[[1]] == "first")
stopifnot(result(tt_rank("A complaint?", c("first", "second")), list(0.9, 0.9))$value[[1]] == 0)
stopifnot(result(tt_find("Which line?", c("first", "second")), 0.9)$value == "first")
stopifnot(isTRUE(result(tt_annotate(list(version = 1L, questions = list(approved = list(decide = "Approved?"))), "Yes"))$value[[1]]$approved))
recognition <- tt_recognize_question(kinds = list(person = NULL, organization = NULL))
recognized <- tt_recognize(recognition, "Maria Chen joined Northwind Freight.")
stopifnot(result(recognized, lapply(recognized$value$entities, function(entity) {
  kept <- Filter(function(proposal) isTRUE(proposal$kept) &&
    identical(proposal$selected$start, entity$start) && identical(proposal$selected$end, entity$end) &&
    identical(proposal$kind, entity$kind), recognized$results[[1L]]$answer$proposals)
  kept[[1L]]$span_probability
}))$value$entities[[1]]$text == "Maria Chen")
relation <- tt_relate_question(relations = list(list(name = "knows", source = "person", target = "person")))
related <- tt_relate(relation, list(list(name = "Ada", kind = "person"), list(name = "Bo", kind = "person")))
edges <- result(related, lapply(related$value, `[[`, "probability"))$value
stopifnot(edges[[1]]$source$name == "Ada", edges[[1]]$target$name == "Bo")
cat("ten typed answers\n")
