source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
case <- read_case(commandArgs(TRUE)[[1]])
request <- case$request
settings <- case$client
if (is.null(settings)) settings <- list()
settings$url <- Sys.getenv("THINKTHEN_TEST_URL")
if (!nzchar(settings$url)) settings$url <- NULL
records <- request$records
for (index in names(case$integer_ids)) records[[as.integer(index)]]$id <- tt_id_integer(case$integer_ids[[index]])
for (index in unlist(case$invalid_integer_ids, use.names = FALSE)) records[[index]]$id <- 2^53

# Translate public typed R projections for comparison to the shared cases.
record_fields <- function(x, i = 1L) {
  fields <- setNames(lapply(vctrs::fields(x), function(field) {
    value <- vctrs::field(x, field)
    normal(if (is.list(value) && !inherits(value, "vctrs_rcrd")) value[[i]] else value[i])
  }), vctrs::fields(x))
  if (inherits(x, "tt_relation") && identical(fields$either, FALSE)) fields$either <- NULL
  fields
}
normal <- function(x) {
  if (inherits(x, "tt_id_integer")) return(list(integer = format(x)))
  if (inherits(x, "tt_piece") || inherits(x, "tt_name_detail") || inherits(x, "tt_name") || inherits(x, "tt_relation")) {
    return(lapply(seq_along(x), function(i) record_fields(x, i)))
  }
  if (inherits(x, "vctrs_rcrd")) return(record_fields(x))
  if (is.factor(x)) return(as.character(x))
  if (is.list(x)) return(lapply(unclass(x), normal))
  if (is.atomic(x) && !is.null(names(x))) return(as.list(x))
  x
}
answer_value <- function(x, verb, i = 1L) {
  if (verb == "find") {
    if (is.na(x[i])) NULL else record_fields(x, i)
  } else if (verb %in% c("recognize", "relate")) {
    value <- x[[i]]
    if (is.null(value)) NULL else lapply(seq_along(value), function(j) record_fields(value, j))
  } else if (verb == "tag") as.list(x[[i]]) else if (verb == "annotate") {
    lapply(x[[i]], function(entry) {
      child <- entry[["function"]]
      result <- list(`function` = child, status = as.character(entry$status))
      if (entry$status == "answered") result["answer"] <- list(answer_value(entry$answer, child))
      if (!is.null(entry$probability)) result$probability <- entry$probability
      if (!is.null(entry$error)) result$error <- normal(entry$error)
      if (!is.null(entry$detail)) result$detail <- normal(entry$detail)
      result
    })
  } else x[[i]]
}
result <- tryCatch({
  client <- do.call(tt_client, settings)
  output <- invoke(records, request$question, client, request$name)
  if (identical(case$name, "annotate-columns")) {
    input <- as.data.frame(do.call(rbind, lapply(records, as.data.frame)), stringsAsFactors = FALSE)
    attached <- tt_bind(input, output)
    stopifnot(nrow(attached) == 4L, identical(attached[names(input)], input))
    for (name in names(request$question$questions)) {
      for (i in seq_len(nrow(attached))) {
        child <- case$expected[[i]]$answer[[name]]
        stopifnot(identical(attached[[name]][[i]], child$answer),
                  as.character(attached[[paste0(name, "_status")]][[i]]) == child$status)
        probability <- attached[[paste0(name, "_probability")]][[i]]
        stopifnot(if (is.null(child$probability)) is.na(probability) else identical(probability, child$probability))
      }
    }
  }
  verb <- request$question[["function"]]
  check_rows(output, verb, length(case$expected))
  rows <- lapply(seq_len(nrow(output)), function(i) {
    row <- list(`function` = verb, position = output$position[[i]],
                status = as.character(output$status[[i]]), source = normal(output$source[[i]]))
    if (!is.null(output$id[[i]])) row$id <- normal(output$id[[i]])
    if (row$status == "failed") {
      row$error <- list(code = as.character(output$error_code[[i]]), message = output$error_message[[i]])
      if (!is.na(output$error_http_status[[i]])) row$error$http_status <- output$error_http_status[[i]]
    } else {
      if (row$status == "answered") row["answer"] <- list(answer_value(output$answer, verb, i))
      if (!is.na(output$probability[[i]])) row$probability <- output$probability[[i]]
      if (!is.null(output$detail[[i]])) row$detail <- normal(output$detail[[i]])
    }
    if (verb == "filter") {
      stopifnot(identical(output$item[[i]], records[[output$position[[i]]]]),
                identical(output$kept[[i]], if (row$status == "failed") NA else isTRUE(row$answer)))
    }
    if (verb == "find" && !is.null(row$answer)) {
      stopifnot(identical(substr(records[[output$position[[i]]]], row$answer$start, row$answer$end), row$answer$text))
    }
    row
  })
  list(rows = rows)
}, thinkthen_error = function(error) list(error = error$code, family = class(error)[[1]]))
writeLines(jsonlite::toJSON(result, auto_unbox = TRUE, null = "null", na = "null", digits = I(17)), useBytes = TRUE)
