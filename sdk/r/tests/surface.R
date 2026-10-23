library(thinkthen)
stub <- tt_client(adapter = "stub")
cases <- jsonlite::fromJSON(commandArgs(TRUE)[[1]], simplifyVector = FALSE)
filter_case <- Filter(function(case) case$request[["function"]] == "filter", cases)[[1]]
filter_text <- unlist(filter_case$request$records, use.names = FALSE)
positions <- which(vapply(filter_case$expected, function(row) isTRUE(row$answer), logical(1)))
expect_code <- function(expression, code, family) {
  error <- tryCatch({ force(expression); NULL }, thinkthen_error = identity)
  if (!inherits(error, paste0("thinkthen_", family, "_error")) || !identical(error$code, code)) {
    stop(sprintf("expected %s/%s; got %s/%s", family, code, paste(class(error), collapse = ","), error$code))
  }
}
expect_code(tt_decide("alpha", "Ready?", threshold = "wrong", client = stub), "usage.threshold", "usage")
expect_code(tt_decide("alpha", tt_reference("ready"), client = stub), "usage.reference", "usage")
expect_code(tt_decide("alpha", tt_question(list(questions = list(a = list(decide = "Ready?")), unexpected = TRUE)), client = stub), "usage.question", "usage")

# Missing structured fields report the shared terminal error at every input length.
for (input in list(list(list(body = "alpha")), list(list(body = "alpha"), list(body = "beta")))) {
  expect_code(tt_decide(input, "Ready?", on = "/missing", client = stub), "usage.missing", "usage")
}
empty <- tt_decide(character(), "Ready?", client = stub)
stopifnot(is.logical(empty$answer), is.double(empty$p), is.list(empty$id), is.character(empty$detail))

text <- structure(setNames(filter_text, c("a", "b", "again")), class = "character", provenance = "keep")
selected_text <- tt_filter(text, "Ready?", client = stub)
stopifnot(identical(as.character(selected_text), unname(filter_text[positions])),
          identical(names(selected_text), names(text)[positions]),
          identical(attr(selected_text, "provenance"), "keep"),
          identical(attr(selected_text, "class"), attr(text, "class")))
frame <- data.frame(id = c("same", "same", "same"), body = filter_text, day = as.Date(c("2026-01-01", "2026-01-02", "2026-01-03")), row.names = c("one", "two", "three"))
attr(frame, "provenance") <- "frame"
attr(frame$body, "provenance") <- "column"
selected <- tt_filter(frame, "Ready?", on = "/body", client = stub)
stopifnot(identical(rownames(selected), c("one", "three")), nrow(selected) == 2L,
          identical(attr(selected, "provenance"), "frame"),
          identical(attr(selected$body, "provenance"), "column"),
          inherits(selected$day, "Date"))
collection <- structure(setNames(as.list(filter_text), c("a", "b", "again")), provenance = "list")
stopifnot(identical(tt_filter(collection, "Ready?", client = stub), structure(collection[positions], provenance = "list")))
none <- tt_filter(frame, "Ready?", on = "/body", threshold = 1, client = stub)
stopifnot(nrow(none) == 0L, identical(names(none), names(frame)), inherits(none$day, "Date"), identical(attr(none$body, "provenance"), "column"))
stopifnot(is.null(tt_filter(NULL, "Ready?", client = stub)))

# Structured IDs and arrays retain shape, nulls retain their position.
records <- list(list(id = list("a"), body = "alpha", array = list("one")), list(id = structure(list(), names = character()), body = "beta"))
rows <- tt_decide(records, "Ready?", on = "/body", client = stub)
stopifnot(identical(rows$id[[1]], list("a")), identical(rows$id[[2]], structure(list(), names = character())), nrow(rows) == 2L)
missing <- tt_decide(c(NA_character_, "alpha"), "Ready?", client = stub)
stopifnot(nrow(missing) == 2L, missing$id[[2]] == 2, missing$status[[2]] == "answered")

# One-label collections stay JSON arrays; inline filenames remain literal text.
stopifnot(nrow(tt_tag("alpha", "Topics?", labels = "only", client = stub)) == 1L,
          nrow(tt_recognize("Ada", kinds = "person", client = stub)) == 1L,
          nrow(tt_decide("alpha", "questions.json#ready", client = stub)) == 1L)
questions <- list(questions = list(ready = list(decide = "Ready?")))
stopifnot(identical(tt_decide("alpha", tt_question(questions, "ready"), client = stub)$answer,
                    tt_decide("alpha", "Ready?", client = stub)$answer))

# Toolbox arrays use the same R vectors as direct calls, including singletons.
# Named questions deliberately include a list-field name to catch broad recursion.
definitions <- list(
  labels = list(tag = "Topics?", labels = "only"),
  people = list(recognize = "person"),
  choice = list(choose = "Topic?", options = c("left", "right")),
  scale = list(score = "Urgent?", levels = c("low", "high")),
  relation = list(relate = list(list(name = "knows", source = "person", target = "person")))
)
box <- list(questions = definitions)
direct <- list(
  labels = tt_tag("Ada knows Bea", "Topics?", labels = "only", client = stub),
  people = tt_recognize("Ada knows Bea", kinds = "person", client = stub),
  choice = tt_choose("Ada knows Bea", "Topic?", options = c("left", "right"), client = stub),
  scale = tt_score("Ada knows Bea", "Urgent?", levels = c("low", "high"), client = stub),
  relation = tt_relate("Ada knows Bea", definitions$relation$relate, client = stub)
)
functions <- c(labels = "tt_tag", people = "tt_recognize", choice = "tt_choose", scale = "tt_score", relation = "tt_relate")
annotation <- tt_annotate("Ada knows Bea", box, client = stub)
for (name in names(definitions)) {
  invoke <- get(functions[[name]])
  named <- invoke("Ada knows Bea", tt_question(box, name), client = stub)
  single <- invoke("Ada knows Bea", tt_question(definitions[[name]]), client = stub)
  stopifnot(nrow(named) == 1L, identical(named$answer, direct[[name]]$answer),
            identical(named$detail, direct[[name]]$detail), identical(single, named),
            identical(annotation$answer[[1]][[name]]$status, named$status[[1]]))
}
# A one-element scale/choice reaches the engine's domain validation, like arrays.
for (field in c("options", "levels")) {
  definition <- if (field == "options") list(choose = "Topic?") else list(score = "Urgent?")
  definition[[field]] <- "only"
  as_list <- definition
  as_list[[field]] <- list("only")
  evaluate <- function(value) tryCatch(tt_annotate("alpha", value, client = stub), thinkthen_error = identity)
  vector_result <- evaluate(definition)
  list_result <- evaluate(as_list)
  stopifnot(identical(class(vector_result), class(list_result)))
  if (inherits(vector_result, "thinkthen_error")) {
    stopifnot(identical(vector_result$code, list_result$code), identical(conditionMessage(vector_result), conditionMessage(list_result)))
  } else stopifnot(identical(vector_result, list_result))
}

raw <- tt_decide("alpha", "Ready?", client = stub)
saved <- raw$detail
stopifnot(is.character(saved), is.list(tt_detail(raw)), identical(saved, raw$detail), !is.null(tt_detail(raw)[[1]]$source))
copy <- raw
copy$answer <- !copy$answer
stopifnot(!identical(raw$answer, copy$answer))
cat("R collection, shape, ownership and condition checks passed\n")

# Namespace detachment must not unload code which shared workers may still use.
dll_path <- getLoadedDLLs()[["thinkthen"]][["path"]]
detach("package:thinkthen", unload = TRUE)
stopifnot(!"thinkthen" %in% loadedNamespaces(),
          identical(getLoadedDLLs()[["thinkthen"]][["path"]], dll_path))
library(thinkthen)
stopifnot(tt_decide("alpha", "Ready?", client = tt_client(adapter = "stub"))$status == "answered")

# R's public string constructor rejects embedded NUL before a library can see it.
nul_string <- tryCatch(rawToChar(as.raw(c(97L, 0L, 98L))), error = identity)
stopifnot(inherits(nul_string, "error"))

stopifnot(inherits(tryCatch(tt_client(backend = "stub"), error = identity), "error"))
