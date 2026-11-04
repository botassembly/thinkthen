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
    size <- if (is.data.frame(x)) nrow(x) else length(x)
    if (size == 0L) {
      .tt_native("r_admit", request, 0L)
      result <- .tt_frame(list(), function_name)
      if (function_name == "rank") attr(result, ".thinkthen_input_size") <- 0L
      if (function_name == "annotate") attr(result, ".thinkthen_children") <- lapply(definition$questions, `[[`, "function")
      if (function_name %in% c("filter", "rank")) result$item <- list()
      if (function_name == "filter") result$kept <- logical()
      return(result)
    }
    if (is.null(client)) {
      if (is.null(.tt_state$client)) .tt_state$client <- tt_client()
      client <- .tt_state$client
    }
    if (!inherits(client, "tt_client")) .tt_r_error("`client` must be a tt_client.", "argument")
    if (size > 2^53 - 1) .tt_native("r_question_conversion_error")
    if (isTRUE(getOption("thinkthen.verbose"))) message("ThinkThen ", function_name, " started")
    if (isTRUE(.progress) && interactive()) progress <- cli::cli_progress_bar(total = size)
    prepared <- if (is.data.frame(x)) .tt_prepare_frame(x) else {
      lapply(seq_len(size), function(position) .tt_prepare(x, position))
    }
    call <- .tt_native("r_open_finite", client, request, prepared)$value
    rows <- vector("list", size)
    kept <- logical(size)
    count <- 0L
    repeat {
      event <- .tt_native("r_poll", call)
      # Service pending interrupts after Rust has returned, before the next poll.
      Sys.sleep(0)
      if (event$event == "row") {
        row <- jsonlite::fromJSON(event$json, simplifyVector = FALSE, bigint_as_char = TRUE)
        row["id"] <- list(switch(event$id$kind, absent = NULL, string = event$id$text,
                         signed = .tt_id_value(event$id$decimal, "signed"),
                         unsigned = .tt_id_value(event$id$decimal, "unsigned")))
        count <- count + 1L
        rows[[count]] <- row
        if (function_name == "filter") kept[[count]] <- event$kept
        if (isTRUE(getOption("thinkthen.verbose")) && row$status == "failed") {
          message("ThinkThen failed at position ", row$position)
        }
        if (!is.null(progress)) cli::cli_progress_update(id = progress, inc = 1L)
      } else if (event$event == "finished") break
    }
    result <- .tt_frame(rows[seq_len(count)], function_name)
    if (function_name == "rank") attr(result, ".thinkthen_input_size") <- size
    if (function_name == "annotate") {
      attr(result, ".thinkthen_children") <- lapply(definition$questions, `[[`, "function")
    }
    if (function_name %in% c("filter", "rank")) {
      result$item <- lapply(result$position, function(position) {
        if (is.data.frame(x)) .tt_subset(x, position) else x[[position]]
      })
      if (function_name == "filter") result$kept <- kept[seq_len(count)]
    }
    result
  }, interrupt = function(interrupt) .tt_native("r_cancel_release", call))
}

.tt_domain <- function(value, supplied, question) {
  typed <- inherits(question, "tt_question")
  if (supplied && typed) .tt_native("r_question_conversion_error")
  if (supplied) value else NULL
}

#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_decide_rows <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "decide", question, .tt_options(threshold = threshold, `true` = yes_means, `false` = no_means, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_choose_rows <- function(x, question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(choices, !missing(choices), question)
  .tt_run(x, "choose", question, .tt_options(choices = domain, descriptions = descriptions, threshold = threshold, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_tag_rows <- function(x, question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL,
                   client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(labels, !missing(labels), question)
  .tt_run(x, "tag", question, .tt_options(labels = domain, descriptions = descriptions, threshold = threshold, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_score_rows <- function(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_run(x, "score", question, .tt_options(levels = domain, descriptions = descriptions, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_filter_rows <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "filter", question, .tt_options(threshold = threshold, `true` = yes_means, `false` = no_means, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_rank_rows <- function(x, question, levels, ..., descriptions = NULL, top = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_run(x, "rank", question, .tt_options(levels = domain, descriptions = descriptions, top = top, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_find_rows <- function(x, question, ..., threshold = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "find", question, .tt_options(threshold = threshold, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_annotate_rows <- function(x, toolbox, ..., field = NULL,
                        client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  .tt_run(x, "annotate", toolbox, .tt_options(field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_recognize_rows <- function(x, kinds, ..., threshold = NULL, field = NULL,
                         client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  if (missing(kinds)) kinds <- NULL
  .tt_run(x, "recognize", kinds, .tt_options(threshold = threshold, field = field, .dots = dots), client, name, .progress)
}
#' Return full outcome rows, including failures, probability, source and detail.
#'
#' @rdname tt_decide
#' @export
tt_relate_rows <- function(x, rules, ..., threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL) {
  dots <- .tt_dots(...)
  if (missing(rules)) rules <- NULL
  .tt_run(x, "relate", rules, .tt_options(threshold = threshold, field = field, .dots = dots), client, name, .progress)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_decide <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_decide_rows(x = x, question = question, ..., threshold = threshold, yes_means = yes_means, no_means = no_means, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_choose <- function(x, question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_choose_rows(x = x, question = question, choices = choices, ..., descriptions = descriptions, threshold = threshold, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_tag <- function(x, question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL,
                   client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_tag_rows(x = x, question = question, labels = labels, ..., descriptions = descriptions, threshold = threshold, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_score <- function(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_score_rows(x = x, question = question, levels = levels, ..., descriptions = descriptions, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return kept original records; a failure condition retains kept records and every failed row.
#'
#' @rdname tt_decide
#' @export
tt_filter <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_filter_rows(x = x, question = question, ..., threshold = threshold, yes_means = yes_means, no_means = no_means, field = field, client = client, .progress = .progress, name = name)
  kept <- .tt_subset(x, rows$position[which(rows$answer)])
  .tt_failures(rows, errors, kept = kept)
  kept
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_rank <- function(x, question, levels, ..., descriptions = NULL, top = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_rank_rows(x = x, question = question, levels = levels, ..., descriptions = descriptions, top = top, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_find <- function(x, question, ..., threshold = NULL, field = NULL,
                    client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_find_rows(x = x, question = question, ..., threshold = threshold, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return one typed answer column per question from a toolbox or annotation collection.
#'
#' @rdname tt_decide
#' @export
tt_annotate <- function(x, toolbox, ..., field = NULL,
                        client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_annotate_rows(x = x, toolbox = toolbox, ..., field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_recognize <- function(x, kinds, ..., threshold = NULL, field = NULL,
                         client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_recognize_rows(x = x, kinds = kinds, ..., threshold = threshold, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return typed answers in input order; not-sure answers are missing values.
#'
#' @rdname tt_decide
#' @export
tt_relate <- function(x, rules, ..., threshold = NULL, field = NULL,
                      client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  rows <- tt_relate_rows(x = x, rules = rules, ..., threshold = threshold, field = field, client = client, .progress = .progress, name = name)
  tt_answer(rows, errors = errors)
}

#' Return a logical filter mask; failures raise unless errors is "na".
#'
#' @rdname tt_decide
#' @export
tt_keep <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL, client = NULL, .progress = TRUE, name = NULL, errors = "error") {
  errors <- .tt_errors(errors)
  tt_answer(tt_filter_rows(x, question, ..., threshold = threshold, yes_means = yes_means, no_means = no_means, field = field, client = client, .progress = .progress, name = name), errors = errors)
}
