library(thinkthen)
stub <- tt_client(adapter = "stub")
cases <- jsonlite::fromJSON(commandArgs(TRUE)[[1]], simplifyVector = FALSE)

# Public result helpers distinguish result frames from ordinary table columns.
rows <- tt_decide(c("alpha", "evidence 5", "alpha"), "Ready?", client = stub)
stopifnot(identical(tt_answer(rows), c(FALSE, TRUE, FALSE)))
expect_r_error <- function(expression) {
  error <- tryCatch({ force(expression); NULL }, error = identity)
  stopifnot(inherits(error, "error"), !inherits(error, "thinkthen_error"))
}
ordinary <- data.frame(answer = c("alpha", "evidence 5", "alpha"))
expect_r_error(tt_answer(ordinary))
expect_r_error(tt_answer(tt_filter(ordinary, "Ready?", on = "/answer", client = stub)))

# JSON object field order and integral JSON numbers have no R shape promise.
canonical <- function(value) {
  if (!is.list(value)) return(value)
  if (length(names(value))) value <- value[order(names(value))]
  lapply(value, canonical)
}

# Shared stub construction cases pass through both table kinds without joins.
for (case in cases) {
  request <- case$request
  verb <- request[["function"]]
  frame <- data.frame(id = rep("same", length(request$records)),
                      index = rep(7L, length(request$records)),
                      body = unlist(request$records, use.names = FALSE),
                      row.names = paste("source", seq_along(request$records)))
  attr(frame, "origin") <- "retain"
  frame$day <- as.Date("2026-01-01") + seq_len(nrow(frame))
  for (input in list(frame, tibble::as_tibble(frame))) {
    question_name <- switch(verb, recognize = "kinds", relate = "rules", annotate = "toolbox", "question")
    question <- request$question$value
    if (verb == "recognize") question <- unlist(question, use.names = FALSE)
    options <- request$options
    for (key in intersect(c("options", "labels", "levels"), names(options))) {
      options[[key]] <- unlist(options[[key]], use.names = FALSE)
    }
    args <- c(list(x = input), setNames(list(question), question_name),
              options, list(on = "/body", client = stub))
    result <- do.call(get(paste0("tt_", verb)), args)
    if (verb == "filter") {
      selected <- which(vapply(case$expected, function(row) isTRUE(row$answer), logical(1)))
      stopifnot(identical(result$body, input$body[selected]),
                identical(class(result), class(input)), identical(result$day, input$day[selected]),
                identical(row.names(result), row.names(input[selected, , drop = FALSE])))
      expect_r_error(tt_bind(input, result, "out"))
      next
    }
    expected <- lapply(case$expected, `[[`, "answer")
    answers <- tt_answer(result)
    if (verb == "tag") expected <- lapply(expected, function(value) as.character(unlist(value, use.names = FALSE)))
    if (is.list(answers)) {
      comparison <- all.equal(canonical(answers), canonical(expected), check.attributes = FALSE)
      if (!isTRUE(comparison)) stop(paste(verb, paste(comparison, collapse = "; ")))
    } else stopifnot(isTRUE(all.equal(unname(answers), unlist(expected, use.names = FALSE), check.attributes = FALSE)))
    if (verb == "rank") {
      expect_r_error(tt_bind(input, result, "out"))
      next
    }
    bound <- tt_bind(input, result, "out")
    stopifnot(identical(class(bound), class(input)), identical(row.names(bound), row.names(input)),
              all(vapply(names(input), function(name) identical(bound[[name]], input[[name]]), logical(1))),
              identical(bound$out_answer, answers),
              identical(bound$out_status, result$status), identical(bound$out_error, result$error),
              identical(bound$out_p, result$p), identical(bound$out_detail, result$detail))
    for (attribute in setdiff(names(attributes(input)), "names")) {
      stopifnot(identical(attr(bound, attribute), attr(input, attribute)))
    }
  }
}

# Native column expressions evaluate the whole column, including duplicate rows.
text <- c("alpha", "evidence 5", "alpha")
frame <- data.frame(body = text, row.names = c("ten", "two", "again"))
frame$answer <- tt_answer(tt_decide(frame$body, "Ready?", client = stub))
tbl <- dplyr::mutate(tibble::as_tibble(frame),
                      answer = tt_answer(tt_decide(body, "Ready?", client = stub)))
DT <- data.table::as.data.table(frame)
DT[, answer := tt_answer(tt_decide(body, "Ready?", client = stub))]
stopifnot(identical(frame$answer, c(FALSE, TRUE, FALSE)),
          identical(tbl$answer, frame$answer), identical(DT$answer, frame$answer))

# Attachment preserves unusual names and attributes without recycling or repair.
frame <- data.frame(body = text, row.names = c("ten", "two", "again"))
frame[["another"]] <- factor(c("b", "a", "b"))
names(frame) <- c("odd name", "odd name")
attr(frame, "origin") <- "retain"
attr(frame[[1]], "source") <- "column"
bound <- tt_bind(frame, rows, "a b")
stopifnot(identical(names(bound)[1:2], names(frame)), identical(bound[[1]], frame[[1]]),
          identical(bound[[2]], frame[[2]]), identical(attr(bound, "origin"), "retain"),
          identical(bound[["a b_answer"]], tt_answer(rows)), identical(row.names(bound), row.names(frame)))
expect_r_error(tt_bind(bound, rows, "a b"))
for (prefix in list(NULL, NA_character_, "", c("a", "b"), 1)) expect_r_error(tt_bind(frame, rows, prefix))
expect_r_error(tt_bind(frame, rows))
expect_r_error(tt_bind(frame[1, , drop = FALSE], rows, "out"))
expect_r_error(tt_bind(frame, rows[1, , drop = FALSE], "out"))
expect_r_error(tt_bind(data.table::data.table(body = text), rows, "out"))
expect_r_error(tt_bind(dplyr::group_by(tibble::tibble(body = text), body), rows, "out"))

# Filter preserves each original column even when names repeat.
frame <- data.frame(body = text, category = factor(c("b", "a", "b")),
                    day = as.Date("2026-01-01") + seq_along(text))
names(frame)[2:3] <- c("duplicate", "duplicate")
attr(frame[[3]], "source") <- "retain"
selected <- tt_filter(frame, "Ready?", on = "/body", client = stub)
positions <- match(row.names(selected), row.names(frame))
stopifnot(identical(names(selected), names(frame)))
for (position in seq_along(frame)) {
  expected <- frame[[position]][positions]
  attr(expected, "source") <- attr(frame[[position]], "source")
  stopifnot(identical(selected[[position]], expected))
}

# Empty inputs keep their table kind and typed empty output columns.
for (input in list(data.frame(body = character()), tibble::tibble(body = character()))) {
  empty <- tt_decide(input$body, "Ready?", client = stub)
  bound <- tt_bind(input, empty, "out")
  stopifnot(nrow(bound) == 0L, identical(class(bound), class(input)),
            is.logical(bound$out_answer), is.double(bound$out_p), is.character(bound$out_status),
            is.list(bound$out_error), is.character(bound$out_detail))
}

# Collection helpers preserve the shared terminal failure.
input <- data.frame(body = c("alpha", "evidence 5", "alpha"))
for (run in list(tt_decide, tt_find)) {
  failure <- tryCatch(run(input, "Ready?", on = "/missing", client = stub), thinkthen_error = identity)
  stopifnot(inherits(failure, "thinkthen_usage_error"), identical(failure$code, "usage.missing"))
}
nothing <- tt_find(" \r\n\t\n", "Which line?", client = stub)
stopifnot(identical(nothing$status, "answered"), is.null(tt_answer(nothing)[[1]]))
# Retaining only answer columns must not hide final engine errors.
error <- tryCatch(tt_answer(tt_decide("alpha", "Ready?", threshold = "invalid", client = stub)),
                  thinkthen_error = identity)
stopifnot(inherits(error, "thinkthen_usage_error"), identical(error$code, "usage.threshold"))
cat("R data frame, tibble, mutate and data.table helpers passed\n")
