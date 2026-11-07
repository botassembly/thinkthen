.tt_result <- function(rows) {
  producer <- attr(rows, ".thinkthen_function", exact = TRUE)
  if (!inherits(rows, "tt_rows") || !is.character(producer) || length(producer) != 1L || is.na(producer)) {
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
  if (inherits(x, "tt_rows")) return(.tt_failure_table(x))
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
    columns <- list()
    for (name in names(attr(rows, ".thinkthen_children", exact = TRUE))) {
      verb <- attr(rows, ".thinkthen_children", exact = TRUE)[[name]]
      children <- lapply(rows$answer[order], `[[`, name)
      values <- lapply(children, function(child) {
        if (is.null(child)) .tt_answer_values(list(list()), verb) else child$answer
      })
      answer <- do.call(vctrs::vec_c, c(values, list(.ptype = .tt_answer_values(list(), verb))))
      status <- factor(vapply(seq_along(children), function(i) {
        if (is.null(children[[i]])) as.character(rows$status[order[i]]) else as.character(children[[i]]$status)
      }, character(1)), levels = unname(.tt_status_names))
      columns <- c(columns, setNames(list(answer, status,
        .tt_scalar(children, "probability", NA_real_, as.double), lapply(children, `[[`, "detail")),
        c(name, paste0(name, c("_status", "_probability", "_detail")))))
    }
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

.tt_child_failures <- function(rows) {
  if (!inherits(rows, "tt_annotate_rows")) return(list())
  lapply(rows$answer, function(entries) {
    Filter(function(entry) isTRUE(entry$status == "failed"), entries)
  })
}

.tt_failure_table <- function(rows) {
  failed <- which(rows$status == "failed")
  children <- .tt_child_failures(rows)
  entries <- unlist(children, recursive = FALSE)
  child_errors <- unname(lapply(entries, `[[`, "error"))
  owners <- c(failed, rep(seq_along(children), lengths(children)))
  table <- tibble::tibble(
    position = rows$position[owners],
    id = rows$id[owners],
    question = c(rep(NA_character_, length(failed)), as.character(names(entries))),
    error_code = factor(c(as.character(rows$error_code[failed]),
      .tt_scalar(child_errors, "code", NA_character_, as.character)), levels = unname(.tt_code_names)),
    error_message = c(rows$error_message[failed], .tt_scalar(child_errors, "message", NA_character_)),
    error_http_status = c(rows$error_http_status[failed], .tt_scalar(child_errors, "http_status", NA_integer_))
  )
  table[order(table$position), ]
}

# Raise for errors = "error"; otherwise warn once and keep the failures beside the value.
.tt_report <- function(rows, errors, value, kept = NULL) {
  failures <- .tt_failure_table(rows)
  if (!nrow(failures)) return(value)
  codes <- unique(as.character(failures$error_code))
  counts <- vapply(codes, function(code) {
    length(unique(failures$position[failures$error_code == code]))
  }, integer(1))
  message <- paste0("ThinkThen returned failed records: ", paste0(codes, " (", counts, ")", collapse = ", "), ".")
  fields <- list(code = codes[[1]], codes = codes, rows = rows,
    failed_rows = rows[which(rows$status == "failed"), ], kept = kept,
    entries = as.list(unlist(.tt_child_failures(rows), recursive = FALSE)))
  if (errors == "error") {
    rlang::abort(message, class = c("thinkthen_failed_rows", "thinkthen_error"), !!!fields,
      call = rlang::caller_env())
  }
  rlang::warn(paste(message, "tt_failures() lists them."), class = "thinkthen_failed_rows_warning", !!!fields)
  attr(value, ".thinkthen_failures") <- failures
  value
}

.tt_annotation_answers <- function(rows) {
  children <- attr(rows, ".thinkthen_children", exact = TRUE)
  columns <- lapply(names(children), function(name) {
    verb <- children[[name]]
    values <- lapply(rows$answer, function(entries) {
      child <- entries[[name]]
      if (is.null(child)) .tt_answer_values(list(list()), verb) else child$answer
    })
    do.call(vctrs::vec_c, c(values, list(.ptype = .tt_answer_values(list(), verb))))
  })
  names(columns) <- names(children)
  tibble::new_tibble(columns, nrow = nrow(rows))
}
