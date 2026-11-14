.tt_result <- function(rows) {
  producer <- attr(rows, ".thinkthen_function", exact = TRUE)
  if (!inherits(rows, "tt_details") || !is.character(producer) || length(producer) != 1L || is.na(producer)) {
    .tt_r_error("Expected ThinkThen result rows.", "rows")
  }
  producer
}

#' Project typed answers from rows, keeping rank answers aligned to input positions.
#'
#' @rdname tt_answer
#' @export
tt_answer <- function(rows, errors = "error") {
  producer <- .tt_result(rows)
  errors <- .tt_errors(errors)
  answers <- if (producer == "annotate") {
    .tt_annotation_answers(rows)
  } else if (producer == "rank") {
    size <- attr(rows, ".thinkthen_input_size", exact = TRUE)
    if (is.null(size)) size <- max(c(0, rows$position))
    rows$answer[match(seq_len(size), rows$position)]
  } else {
    rows$answer
  }
  .tt_report(rows, errors, answers)
}

#' List the failures of an errors = "na" result or of any result rows.
#'
#' @rdname tt_failures
#' @export
tt_failures <- function(x) {
  if (inherits(x, "tt_details")) return(.tt_failure_table(x))
  failures <- attr(x, ".thinkthen_failures", exact = TRUE)
  if (is.null(failures)) .tt_failure_table(.tt_frame(list(), "decide")) else failures
}

#' Attach outcome columns by position to a copy of the original data frame.
#'
#' @rdname tt_answer
#' @export
tt_bind <- function(data, rows, prefix) {
  producer <- .tt_result(rows)
  if (!is.data.frame(data) || (producer != "annotate" &&
      (missing(prefix) || !is.character(prefix) || length(prefix) != 1L || is.na(prefix) || !nzchar(prefix))) ||
      nrow(data) != nrow(rows) ||
      !setequal(rows$position, seq_len(nrow(data))) || anyDuplicated(rows$position)) {
    .tt_r_error("Data, rows and prefix must describe matching frame rows and a nonempty prefix.", "bind")
  }
  allowed <- any(vapply(list("data.frame", c("tbl_df", "tbl", "data.frame"),
    c("grouped_df", "tbl_df", "tbl", "data.frame"), c("data.table", "data.frame")),
    function(classes) identical(class(data), classes), logical(1)))
  if (!allowed) .tt_r_error("Unsupported data frame class.", "bind")
  order <- match(seq_len(nrow(data)), rows$position)
  if (producer == "annotate") {
    names <- names(attr(rows, ".thinkthen_children", exact = TRUE))
    selected <- unlist(lapply(names, function(name) c(name, paste0(name, .tt_entry_suffixes))), use.names = FALSE)
    columns <- lapply(rows[selected], function(value) value[order])
  } else {
    columns <- as.list(tibble::as_tibble(rows[order, ]))
    columns <- columns[setdiff(names(columns), c("position", "id", "item", "kept"))]
    names(columns) <- paste0(prefix, "_", names(columns))
  }
  if (anyDuplicated(names(columns)) || any(names(columns) %in% names(data))) .tt_r_error("Result column names must be unique and absent from data.", "bind")
  if (inherits(data, "data.table")) {
    result <- data.table::copy(data)
    for (name in names(columns)) data.table::set(result, j = name, value = columns[[name]])
    return(result)
  }
  attrs <- attributes(data)
  attrs$names <- c(names(data), names(columns))
  result <- c(unclass(data), columns)
  attributes(result) <- attrs
  if (inherits(data, "grouped_df")) result <- dplyr::dplyr_reconstruct(result, data)
  result
}

.tt_errors <- function(errors) {
  if (!is.character(errors) || length(errors) != 1L || is.na(errors) || !errors %in% c("error", "na")) {
    .tt_r_error('`errors` must be "error" or "na".', "argument")
  }
  errors
}

# The admitted question map is already ordered by engine UTF-8 name order.
.tt_missing_table <- function(rows, status) {
  verb <- .tt_result(rows)
  children <- attr(rows, ".thinkthen_children", exact = TRUE)
  size <- nrow(rows)
  count <- 0L
  owners <- integer(size)
  questions <- character(size)
  functions <- character(size)
  probabilities <- double(size)
  codes <- character(size)
  messages <- character(size)
  http <- integer(size)
  for (i in order(rows$position)) {
    prefix <- ""
    child <- NA_character_
    function_name <- verb
    if (!isTRUE(rows$status[i] == status)) {
      if (verb != "annotate" || !isTRUE(rows$status[i] == "answered")) next
      candidates <- names(children)[vapply(names(children), function(name) {
        isTRUE(rows[[paste0(name, "_status")]][i] == status)
      }, logical(1))]
      if (!length(candidates)) next
      child <- candidates[[1]]
      prefix <- paste0(child, "_")
      function_name <- children[[child]][["function"]]
    }
    field <- function(name) rows[[paste0(prefix, name)]][i]
    count <- count + 1L
    owners[count] <- i
    questions[count] <- child
    functions[count] <- function_name
    probabilities[count] <- field("probability")
    codes[count] <- field("error_code")
    messages[count] <- field("error_message")
    http[count] <- field("error_http_status")
  }
  selected <- seq_len(count)
  owners <- owners[selected]
  result <- tibble::tibble(
    position = rows$position[owners], id = rows$id[owners],
    `function` = functions[selected], question = questions[selected],
    status = factor(rep(status, length(owners)), levels = unname(.tt_status_names)),
    probability = probabilities[selected], source = rows$source[owners],
    error_code = codes[selected], error_message = messages[selected], error_http_status = http[selected])
  result$item <- .tt_subset(rows$item, owners)
  result
}

.tt_failure_table <- function(rows) .tt_missing_table(rows, "failed")

# Report once after the collection; unsure is a quiet condition, never an error.
.tt_report <- function(rows, errors, value, kept = NULL) {
  failures <- .tt_failure_table(rows)
  if (nrow(failures)) {
    message <- paste0("failed: ", failures$error_code, ": ",
      gsub("[\r\n\t]", " ", failures$error_message))
    message <- paste(message, collapse = "\n")
    fields <- list(code = failures$error_code[[1]],
      http_status = if (is.na(failures$error_http_status[[1]])) NULL else failures$error_http_status[[1]],
      rows = failures, outcomes = rows, kept = kept)
    if (errors == "error") {
      rlang::abort(message, class = c("thinkthen_failed", "thinkthen_no_answer", "thinkthen_error"),
        !!!fields, call = rlang::caller_env())
    }
    rlang::warn(message, class = "thinkthen_record_warning", !!!fields)
    attr(value, ".thinkthen_failures") <- failures
  }
  unsure <- .tt_missing_table(rows, "unsure")
  if (nrow(unsure)) {
    signalCondition(structure(list(message = "unsure", call = NULL, rows = unsure, outcomes = rows),
      class = c("thinkthen_unsure", "thinkthen_no_answer", "condition")))
  }
  value
}

.tt_annotation_answers <- function(rows) {
  children <- names(attr(rows, ".thinkthen_children", exact = TRUE))
  tibble::new_tibble(setNames(lapply(children, function(name) rows[[name]]), children), nrow = nrow(rows))
}
