.tt_run <- function(x, function_name, question, options, client, name, .progress) {
  call <- NULL
  progress <- NULL
  on.exit({
    if (!is.null(call)) .Call("wrap__r_close", call, PACKAGE = "thinkthen")
    if (!is.null(progress)) cli::cli_progress_done(id = progress)
  })
  tryCatch({
    force(question); force(options); force(name); force(.progress)
    definition <- .tt_definition(function_name, question, options)
    request <- .tt_context_json(.tt_options(question = definition, name = name), "r_question_conversion_error")
    if (is.null(client)) {
      if (is.null(.tt_state$client)) .tt_state$client <- tt_client_from_env()
      client <- .tt_state$client
    }
    size <- if (is.data.frame(x)) nrow(x) else length(x)
    if (size > 2^53 - 1) .tt_native("r_question_conversion_error")
    if (isTRUE(getOption("thinkthen.verbose"))) message("ThinkThen ", function_name, " started")
    if (isTRUE(.progress) && interactive()) progress <- cli::cli_progress_bar(total = size)
    prepared <- lapply(seq_len(size), function(position) .tt_prepare(x, position))
    call <- .tt_native("r_open_finite", client, request, prepared)$value
    rows <- list()
    repeat {
      event <- .tt_native("r_poll", call)
      if (event$event == "row") {
        row <- jsonlite::fromJSON(event$json, simplifyVector = FALSE, bigint_as_char = TRUE)
        row["id"] <- list(switch(event$id$kind, absent = NULL, string = event$id$text,
                         signed = .tt_id_value(event$id$decimal, "signed"),
                         unsigned = .tt_id_value(event$id$decimal, "unsigned")))
        rows[[length(rows) + 1L]] <- row
        if (isTRUE(getOption("thinkthen.verbose")) && row$status == "failed") {
          message("ThinkThen failed at position ", row$position)
        }
        if (!is.null(progress)) cli::cli_progress_update(id = progress, inc = 1L)
      } else if (event$event == "finished") break
    }
    result <- .tt_frame(rows, function_name)
    if (function_name == "annotate") {
      attr(result, ".thinkthen_children") <- lapply(definition$questions, `[[`, "function")
    }
    if (function_name == "filter") {
      result$item <- lapply(result$position, function(position) {
        if (is.data.frame(x)) .tt_subset(x, position) else x[[position]]
      })
      result$kept <- !is.na(result$answer) & result$answer
    }
    result
  }, interrupt = function(interrupt) .tt_native("r_cancel_release", call))
}

.tt_domain <- function(value, supplied, question) {
  typed <- inherits(question, "tt_question")
  if (supplied && typed) .tt_native("r_question_conversion_error")
  if (supplied) value else NULL
}

tt_decide <- function(x, question, ..., threshold = NULL, evidence = NULL, client = NULL,
                      .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "decide", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_choose <- function(x, question, choices, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(choices, !missing(choices), question)
  .tt_run(x, "choose", question, .tt_options(choices = domain, threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_tag <- function(x, question, labels, ..., threshold = NULL, evidence = NULL,
                   client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(labels, !missing(labels), question)
  .tt_run(x, "tag", question, .tt_options(labels = domain, threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_score <- function(x, question, levels, ..., evidence = NULL,
                     client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_run(x, "score", question, .tt_options(levels = domain, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_filter <- function(x, question, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "filter", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_rank <- function(x, question, levels, ..., top = NULL, evidence = NULL,
                    client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_run(x, "rank", question, .tt_options(levels = domain, top = top, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_find <- function(x, question, ..., threshold = NULL, evidence = NULL,
                    client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "find", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_annotate <- function(x, toolbox, ..., evidence = NULL,
                        client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "annotate", toolbox, .tt_options(evidence = evidence, .dots = dots), client, name, .progress)
}
tt_recognize <- function(x, kinds, ..., threshold = NULL, evidence = NULL,
                         client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  if (missing(kinds)) kinds <- NULL
  .tt_run(x, "recognize", kinds, .tt_options(threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
tt_relate <- function(x, rules, ..., threshold = NULL, evidence = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  if (missing(rules)) rules <- NULL
  .tt_run(x, "relate", rules, .tt_options(threshold = threshold, evidence = evidence, .dots = dots), client, name, .progress)
}
