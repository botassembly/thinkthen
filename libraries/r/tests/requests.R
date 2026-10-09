source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(batch = 1L)
exports <- getNamespaceExports("thinkthen")
check("one public named call family", !any(grepl("_(complete|batch)$", exports)) && !"tt_details" %in% exports)
check("explicit question printing withholds wording", identical(capture.output(print(tt_question("private wording"))), "<question: content withheld>"))
arrivals <- backend_count()
for (call in list(
  function() tt_decide("Q?", "x", list(top = 1)),
  function() tt_choose(list(choose = "Q?", options = c("a", "b")), "x", list(context = NULL)),
  function() tt_decide("Q?", tt_input("records", items = list(), paths = list("missing"))),
  function() tt_decide(list(decide = "Q?", item_schema = list(type = "string")), list(n = 1))
)) check("canonical admission refuses invalid values", identical(kind_of(call()), "usage"))
check("invalid request sends nothing", backend_count() == arrivals)
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
  recognize = list(version = 1L, recognize = list(kinds = c("person", "organization"))),
  relate = list(version = 1L, relate = list(relations = list(list(name = "knows", source = "person", target = "person"))))
)
for (verb in names(questions)) {
  input <- switch(verb, relate = list(list(name = "Maria Chen", kind = "person"), list(name = "Alex", kind = "person")),
    find = c("first candidate", "second candidate"), annotate = data.frame(body = "annotation text"),
    "Maria Chen joined Northwind Freight.")
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
call <- tt_decide("Duplicates?", c("same", "other", "same"))
check("duplicate records retain native positions", identical(vapply(call$results, `[[`, 0, "index"), c(0, 1, 2)))
check("absent and null remain distinct", inherits(call$results[[1]]$source, "thinkthen_absent") && is.null(call$results[[1]]$question$true))
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
finish("canonical requests", backend_count())
