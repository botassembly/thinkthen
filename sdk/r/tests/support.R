# Public R surface helpers. No namespace internals or simulated engine results.
library(thinkthen)

condition <- function(expr, class, code = NULL) {
  value <- tryCatch({ force(expr); NULL }, error = identity)
  stopifnot(inherits(value, class), inherits(value, "thinkthen_error"))
  if (identical(class, "thinkthen_failed")) stopifnot(inherits(value, "thinkthen_no_answer"), !inherits(value, "thinkthen_record_error"))
  if (!is.null(code)) stopifnot(identical(as.character(value$code), code))
  invisible(value)
}

warned <- function(expr) {
  error <- tryCatch(force(expr), thinkthen_failed = identity)
  stopifnot(inherits(error, "thinkthen_failed"))
  plain(tt_answer(error$outcomes, errors = "na"))
}

# Remove what errors = "na" adds, leaving the value a caller compares.
plain <- function(expr) {
  value <- suppressWarnings(expr, classes = "thinkthen_record_warning")
  attr(value, ".thinkthen_failures") <- NULL
  value
}

# Return a value with every warning its evaluation signalled.
caught <- function(expr) {
  warnings <- list()
  value <- withCallingHandlers(expr, warning = function(warning) {
    warnings[[length(warnings) + 1L]] <<- warning
    invokeRestart("muffleWarning")
  })
  list(value = value, warnings = warnings)
}

check_rows <- function(rows, verb, n) {
  stopifnot(inherits(rows, "tt_details"), inherits(rows, "tbl_df"), nrow(rows) == n,
            is.double(rows$position), is.factor(rows$status),
            identical(levels(rows$status), c("answered", "unsure", "failed")),
            is.double(rows$probability), is.character(rows$error_code),
            is.character(rows$error_message), is.integer(rows$error_http_status),
            is.list(rows$detail), is.list(rows$source), is.list(rows$id),
            !any(c("p", "adapter", "raw", "response") %in% names(rows)))
  valid_answer <- switch(verb,
    decide = is.logical(rows$answer), filter = is.logical(rows$answer),
    choose = is.factor(rows$answer), score = is.double(rows$answer),
    rank = is.double(rows$answer), tag = inherits(rows$answer, "vctrs_list_of"),
    find = inherits(rows$answer, "tt_span"),
    recognize = is.list(rows$answer), relate = is.list(rows$answer),
    annotate = all(names(attr(rows,".thinkthen_children")) %in% names(rows)))
  stopifnot(valid_answer)
  if (verb %in% c("recognize", "relate")) {
    type <- if (verb == "recognize") "tt_name" else "tt_relation"
    for (answer in rows$answer) if (!is.null(answer)) stopifnot(inherits(answer, type))
  }
  for (src in rows$source) stopifnot(inherits(src, "tt_source"))
}

# Construct the admitted R value corresponding to one shared flat question.
question <- function(q) {
  verb <- q[["function"]]
  q[["function"]] <- NULL
  if (!is.null(q$threshold) && is.list(q$threshold)) {
    q$threshold <- c(q$threshold$low, q$threshold$high)
  }
  for (key in intersect(c("choices", "labels", "levels", "kinds"), names(q))) {
    q[[key]] <- as.character(unlist(q[[key]], use.names = FALSE))
  }
  if (verb == "annotate") {
    return(tt_annotation_questions(lapply(q$questions, question)))
  }
  if (verb == "relate") {
    q$rules <- do.call(vctrs::list_of, lapply(q$rules, function(r) do.call(tt_rule, r)))
  }
  do.call(get(paste0("tt_", verb, "_question")), q)
}

invoke <- function(records, q, client, name = NULL, vector = FALSE) {
  arg <- switch(q[["function"]], recognize = "kinds", relate = "rules",
                annotate = "questions", "question")
  options <- list(client = client, .progress = FALSE, name = name)
  if (!vector && q[["function"]] != "filter") options$details <- TRUE
  if (q[["function"]] == "annotate") options$field <- q$field
  do.call(get(paste0("tt_", q[["function"]], "")),
          c(list(x = records), setNames(list(question(q)), arg), options))
}

# Preserve decimal tokens until the specialist-approved R ID fixture supplies
# their lossless host values. Ordinary strings remain ordinary strings.
read_case <- function(path) {
  jsonlite::fromJSON(path, simplifyVector = FALSE, bigint_as_char = TRUE)
}
