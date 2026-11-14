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
    request <- .tt_native("r_prepare_request",
      .tt_context_json(.tt_options(question = definition, name = name), "r_question_conversion_error"),
      names(.tt_metadata(list())), .tt_entry_suffixes)
    definition <- jsonlite::fromJSON(request$definition, simplifyVector = FALSE)
    request <- request$value
    size <- if (is.data.frame(x)) nrow(x) else length(x)
    if (size == 0L) {
      result <- .tt_frame(list(), function_name, definition)
      if (function_name == "rank") attr(result, ".thinkthen_input_size") <- 0L
      result$item <- .tt_subset(x, integer())
      if (function_name == "filter") attr(result, ".thinkthen_kept") <- logical()
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
    result <- .tt_frame(rows[seq_len(count)], function_name, definition)
    if (function_name == "rank") attr(result, ".thinkthen_input_size") <- size
    result$item <- .tt_subset(x, result$position)
    if (function_name == "filter") attr(result, ".thinkthen_kept") <- kept[seq_len(count)]
    result
  }, interrupt = function(interrupt) .tt_native("r_cancel_release", call))
}

.tt_domain <- function(value, supplied, question) {
  typed <- inherits(question, "tt_question")
  if (supplied && typed) .tt_native("r_question_conversion_error")
  if (supplied) value else NULL
}

# Details expose states; a plain column reports missing answers only after all rows finish.
.tt_call <- function(x, verb, question, options, client, name, .progress, details, errors) {
  errors <- .tt_errors(errors)
  if (!is.logical(details) || length(details) != 1L || is.na(details)) {
    .tt_r_error("`details` must be TRUE or FALSE.", "argument")
  }
  rows <- .tt_run(x, verb, question, options, client, name, .progress)
  if (details) rows else tt_answer(rows, errors = errors)
}

# Append host syntax after the shared engine's threshold refusal.
.tt_filter_context <- function(code) {
  tryCatch(force(code), thinkthen_usage_error = function(failure) {
    if (identical(failure$code, "usage.threshold")) {
      failure$message <- paste0(conditionMessage(failure),
        " For not-sure results, use tt_decide(..., details = TRUE).")
    }
    stop(failure)
  })
}

#' Return logical answers, or a details table carrying every state.
#' @rdname tt_decide
#' @export
tt_decide <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL,
                      details = FALSE, errors = "error") {
  .tt_call(x, "decide", question, .tt_options(threshold = threshold, yes_means = yes_means,
    no_means = no_means, field = field, .dots = .tt_dots(...)), client, name, .progress, details, errors)
}

#' Return a factor whose levels are the question's choices.
#' @rdname tt_decide
#' @export
tt_choose <- function(x, question, choices, ..., descriptions = NULL, threshold = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL,
                      details = FALSE, errors = "error") {
  domain <- .tt_domain(choices, !missing(choices), question)
  .tt_call(x, "choose", question, .tt_options(choices = domain, descriptions = descriptions,
    threshold = threshold, field = field, .dots = .tt_dots(...)),
    client, name, .progress, details, errors)
}

#' Return one factor of matching labels per record.
#' @rdname tt_decide
#' @export
tt_tag <- function(x, question, labels, ..., descriptions = NULL, threshold = NULL,
                   field = NULL, client = NULL, .progress = TRUE, name = NULL,
                   details = FALSE, errors = "error") {
  domain <- .tt_domain(labels, !missing(labels), question)
  .tt_call(x, "tag", question, .tt_options(labels = domain, descriptions = descriptions,
    threshold = threshold, field = field, .dots = .tt_dots(...)),
    client, name, .progress, details, errors)
}

#' Return positions along the supplied scale.
#' @rdname tt_decide
#' @export
tt_score <- function(x, question, levels, ..., descriptions = NULL, field = NULL,
                     client = NULL, .progress = TRUE, name = NULL,
                     details = FALSE, errors = "error") {
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_call(x, "score", question, .tt_options(levels = domain, descriptions = descriptions,
    field = field, .dots = .tt_dots(...)), client, name, .progress, details, errors)
}

#' Return kept original records, reporting failures after every record is judged.
#' @rdname tt_decide
#' @export
tt_filter <- function(x, question, ..., threshold = NULL, yes_means = NULL, no_means = NULL,
                      field = NULL, client = NULL, .progress = TRUE, name = NULL) {
  rows <- .tt_filter_context(.tt_run(x, "filter", question,
    .tt_options(threshold = threshold, yes_means = yes_means, no_means = no_means,
      field = field, .dots = .tt_dots(...)), client, name, .progress))
  kept <- .tt_subset(x, rows$position[attr(rows, ".thinkthen_kept", exact = TRUE)])
  attr(rows, ".thinkthen_kept") <- NULL
  .tt_report(rows, "error", kept, kept = kept)
}

#' Return input-aligned ranks, or rank-ordered details with the caller's records.
#' @rdname tt_decide
#' @export
tt_rank <- function(x, question, levels, ..., descriptions = NULL, top = NULL,
                    field = NULL, client = NULL, .progress = TRUE, name = NULL,
                    details = FALSE, errors = "error") {
  domain <- .tt_domain(levels, !missing(levels), question)
  .tt_call(x, "rank", question, .tt_options(levels = domain, descriptions = descriptions,
    top = top, field = field, .dots = .tt_dots(...)), client, name, .progress, details, errors)
}

#' Return spans, including an empty answered span when no line answers.
#' @rdname tt_decide
#' @export
tt_find <- function(x, question, ..., threshold = NULL, field = NULL, client = NULL,
                    .progress = TRUE, name = NULL, details = FALSE, errors = "error") {
  .tt_call(x, "find", question, .tt_options(threshold = threshold, field = field,
    .dots = .tt_dots(...)), client, name, .progress, details, errors)
}

#' Return one typed answer column per named question.
#' @rdname tt_decide
#' @export
tt_annotate <- function(x, questions, ..., field = NULL, client = NULL, .progress = TRUE,
                        name = NULL, details = FALSE, errors = "error") {
  .tt_call(x, "annotate", questions, .tt_options(field = field, .dots = .tt_dots(...)),
    client, name, .progress, details, errors)
}

#' Return one set of names per record.
#' @rdname tt_decide
#' @export
tt_recognize <- function(x, kinds, ..., threshold = NULL, field = NULL, client = NULL,
                         .progress = TRUE, name = NULL, details = FALSE, errors = "error") {
  if (missing(kinds)) kinds <- NULL
  .tt_call(x, "recognize", kinds, .tt_options(threshold = threshold, field = field,
    .dots = .tt_dots(...)), client, name, .progress, details, errors)
}

#' Return one set of relations per record.
#' @rdname tt_decide
#' @export
tt_relate <- function(x, rules, ..., threshold = NULL, field = NULL, client = NULL,
                      .progress = TRUE, name = NULL, details = FALSE, errors = "error") {
  if (missing(rules)) rules <- NULL
  .tt_call(x, "relate", rules, .tt_options(threshold = threshold, field = field,
    .dots = .tt_dots(...)), client, name, .progress, details, errors)
}
