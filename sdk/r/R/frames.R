.tt_result <- function(rows) {
  producer <- attr(rows, ".thinkthen_function", exact = TRUE)
  if (!is.data.frame(rows) || !is.character(producer) || length(producer) != 1L ||
      is.na(producer) || producer == "filter") {
    stop("rows must be a complete ThinkThen result frame", call. = FALSE)
  }
  producer
}

tt_answer <- function(rows) {
  .tt_result(rows)
  answer <- rows$answer
  if (is.list(answer)) {
    absent <- vapply(answer, is.null, logical(1)) & rows$status != "answered"
    answer[absent] <- rep(list(NA), sum(absent))
  }
  answer
}

tt_bind <- function(data, rows, prefix) {
  producer <- .tt_result(rows)
  if (producer == "rank") stop("rank results are in rank order", call. = FALSE)
  if (!identical(class(data), "data.frame") &&
      !identical(class(data), c("tbl_df", "tbl", "data.frame"))) {
    stop("data must be a base data frame or an ungrouped tibble", call. = FALSE)
  }
  if (nrow(data) != nrow(rows)) stop("data and rows must have the same row count", call. = FALSE)
  if (missing(prefix) || !is.character(prefix) || length(prefix) != 1L ||
      is.na(prefix) || !nzchar(prefix)) {
    stop("prefix must be one nonempty string", call. = FALSE)
  }
  columns <- list(answer = tt_answer(rows), p = rows$p, status = rows$status,
                  error = rows$error, detail = rows$detail)
  names(columns) <- paste0(prefix, "_", names(columns))
  if (any(names(columns) %in% names(data))) stop("result columns already exist in data", call. = FALSE)
  attrs <- attributes(data)
  attrs$names <- c(names(data), names(columns))
  result <- c(unclass(data), columns)
  attributes(result) <- attrs
  result
}
