.tt_result <- function(rows) {
  producer <- attr(rows, ".thinkthen_function", exact = TRUE)
  if (!inherits(rows, "tt_rows") || !is.character(producer) || length(producer) != 1L || is.na(producer)) {
    .tt_r_error("Expected ThinkThen result rows.", "rows")
  }
  producer
}

tt_answer <- function(rows) {
  producer <- .tt_result(rows)
  failures <- which(rows$status == "failed")
  child_failures <- if (producer == "annotate") unlist(lapply(rows$answer, function(entries) {
    names(entries)[vapply(entries, function(entry) isTRUE(entry$status == "failed"), logical(1))]
  }), use.names = FALSE) else character()
  if (length(failures) || length(child_failures)) {
    cli::cli_warn("ThinkThen returned failed rows or annotation entries.",
      class = "thinkthen_failed_rows", call = NULL, positions = rows$position[failures],
      codes = as.character(rows$error_code[failures]), entries = child_failures)
  }
  rows$answer
}

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

tt_decide_vec <- function(...) tt_answer(tt_decide(...))
tt_choose_vec <- function(...) tt_answer(tt_choose(...))
tt_tag_vec <- function(...) tt_answer(tt_tag(...))
tt_score_vec <- function(...) tt_answer(tt_score(...))
tt_filter_vec <- function(...) tt_answer(tt_filter(...))
tt_keep <- tt_filter_vec
tt_rank_vec <- function(...) tt_answer(tt_rank(...))
tt_find_vec <- function(...) tt_answer(tt_find(...))
tt_annotate_vec <- function(...) tt_answer(tt_annotate(...))
tt_recognize_vec <- function(...) tt_answer(tt_recognize(...))
tt_relate_vec <- function(...) tt_answer(tt_relate(...))
