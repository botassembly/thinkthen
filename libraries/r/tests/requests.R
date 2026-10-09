source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(batch = 1L)
exports <- getNamespaceExports("thinkthen")
check("one public named call family", !any(grepl("^tt_(decide|choose|tag|score|filter|rank|find|annotate|recognize|relate)_(complete|batch)$", exports)) && !"tt_details" %in% exports)
check("explicit question printing withholds wording", identical(capture.output(print(tt_question("private wording"))), "<question: content withheld>"))
arrivals <- backend_count()
for (call in list(
  function() tt_decide("Q?", "x", list(top = 1)),
  function() tt_choose(list(choose = "Q?", options = c("a", "b")), "x", list(context = NULL)),
  function() tt_decide("Q?", tt_input("records", items = list(), paths = list("missing"))),
  function() tt_decide(list(decide = "Q?", item_schema = list(type = "string")), list(n = 1))
)) check("canonical admission refuses invalid values", identical(kind_of(call()), "usage"))
check("invalid request sends nothing", backend_count() == arrivals)
empty <- tt_decide("No records?", NULL)
check("ordinary NULL supplies no records and sends nothing", length(empty$results) == 0L &&
  empty$facts$requests_sent == 0 && backend_count() == arrivals)
null_plan <- tt_plan("Explicit null?", tt_input("json", value = NULL))
check("explicit JSON null remains one original", null_plan$records == 1 && backend_count() == arrivals)
plan <- tt_plan("Plan question?", c("first", "second"))
check("native preview sends nothing", plan$records == 2 && backend_count() == arrivals)
check("plan prints safely", identical(capture.output(print(plan)), "<complete carrier: content withheld>"))

questions <- list(
  decide = "Decide Q?",
  choose = list(choose = "Choose Q?", options = c("first", "second")),
  tag = list(tag = "Tag Q?", labels = c("first", "second")),
  score = list(score = "Score Q?", levels = c("low", "high")),
  filter = "Filter Q?", rank = "Rank Q?", find = "Find Q?",
  annotate = list(version = 1L, questions = list(yes = list(decide = "Annotation Q?"))),
  recognize = list(version = 1L, recognize = list(kinds = list(person = NULL, organization = NULL))),
  relate = list(version = 1L, relate = list(relations = list(list(name = "knows", source = "person", target = "person"))))
)
for (verb in names(questions)) {
  input <- switch(verb, relate = list(list(name = "Maria Chen", kind = "person"), list(name = "Alex", kind = "person")),
    find = c("first candidate", "second candidate"), annotate = data.frame(body = "annotation text"),
    "Maria Chen joined Northwind Freight.")
  cat("named call:", verb, "\n")
  call <- get(paste0("tt_", verb))(questions[[verb]], input)
  check(paste(verb, "uses generated typed results"), inherits(call, "thinkthen_Call") &&
    length(call$results) > 0L && inherits(call$results[[1L]], "thinkthen_complete") &&
    inherits(call$facts$call_id, "thinkthen_CallId"))
  result <- call$results[[1L]]
  retained <- result$value
  gc()
  check(paste(verb, "retains independent field access"), identical(result[["value"]], retained) &&
    identical(capture.output(print(result)), "<complete carrier: content withheld>"))
}
call <- tt_decide(list(decide = "Duplicates?", false = NULL), c("same", "other", "same"))
check("duplicate records retain native positions", identical(vapply(call$results, `[[`, 0, "index"), c(0, 1, 2)))
check("absent and null remain distinct", inherits(call$results[[1]]$source, "thinkthen_absent") && is.null(call$results[[1]]$question$false))
for (records in list(c(first = "one", second = "two"),
                     list(first = list(body = "one", count = 1L), second = list(body = "two", count = 2L)))) {
  call <- tt_filter("Named records?", records)
  check("outer names do not replace record positions or inner fields", identical(vapply(call$results, `[[`, 0, "index"), c(0, 1)) &&
    isTRUE(all.equal(lapply(call$results, function(row) row$input), unname(as.list(records)))))
}
file <- tempfile("native-source-")
writeLines(c("file first", "file second"), file)
call <- tt_decide("Files?", tt_files(file))
check("native reader retains physical source", length(call$results) == 2L && call$results[[1L]]$source$first_line == 1)
unlink(file)
batch <- tt_batch("decide", "Batch Q?", c("one", "two"))
rows <- list(batch$next_result(), batch$next_result())
check("session yields native owned results", all(vapply(rows, inherits, TRUE, "thinkthen_DecideResult")))
check("session keeps actual terminal facts", is.null(batch$next_result()) && inherits(batch$facts(), "thinkthen_Facts"))
batch$close()
deadline <- tryCatch(tt_decide("Deadline?", "not sent", deadline_ms = 0), error = identity)
check("deadline retains a typed safe failure", inherits(deadline, "thinkthen_error") && deadline$kind == "deadline" &&
  inherits(deadline$complete, "thinkthen_CallError") &&
  identical(capture.output(print(deadline$complete)), "<complete carrier: content withheld>"))
failed_batch <- tt_batch("decide", "Session deadline?", "not sent", list(deadline_ms = 0))
failure <- tryCatch(failed_batch$next_result(), error = identity)
check("native session failures retain their condition", inherits(failure, "thinkthen_error") && failure$kind == "deadline" && inherits(failure$complete, "thinkthen_CallError"))
failed_batch$close()
finish("canonical requests", 21L)
