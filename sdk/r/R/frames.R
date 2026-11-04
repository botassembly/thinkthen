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
  .tt_failures(rows, errors)
  if (producer == "annotate") return(.tt_annotation_answers(rows))
  if (producer == "rank") {
    size <- attr(rows, ".thinkthen_input_size", exact = TRUE)
    if (is.null(size)) size <- max(c(0, rows$position))
    return(rows$answer[match(seq_len(size), rows$position)])
  }
  rows$answer
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

.tt_failures <- function(rows, errors, kept = NULL) {
  if (errors == "na") return(invisible(NULL))
  failed <- which(rows$status == "failed")
  children <- if (.tt_result(rows) == "annotate") unlist(lapply(rows$answer, function(entries) {
    Filter(function(entry) isTRUE(entry$status == "failed"), entries)
  }), recursive = FALSE) else list()
  if (!length(failed) && !length(children)) return(invisible(NULL))
  first <- if (length(failed)) as.character(rows$error_code[failed[[1]]]) else as.character(children[[1]]$error$code)
  rlang::abort("ThinkThen returned failed records.", class = c("thinkthen_failed_rows", "thinkthen_error"),
    code = first, rows = rows, failed_rows = rows[failed, ], kept = kept,
    entries = children, call = rlang::caller_env())
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
