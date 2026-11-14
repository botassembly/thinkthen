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
result <- tryCatch({
  client <- do.call(tt_client, settings)
  verb <- request$question[["function"]]
  output <- tryCatch(invoke(records, request$question, client, request$name),
                     thinkthen_failed = identity)
  if (verb == "filter" && !inherits(output, "thinkthen_failed")) {
    list(kept = normal(output))
  } else {
    if (inherits(output, "thinkthen_failed")) output <- output$outcomes
    check_rows(output, verb, length(case$expected))
    path <- tempfile()
    tt_details_write(output, path)
    rows <- lapply(readLines(path, encoding="UTF-8"), jsonlite::fromJSON, simplifyVector = FALSE, bigint_as_char = TRUE)
    for (i in seq_along(rows)) {
      if (!is.null(output$id[[i]])) rows[[i]]$id <- normal(output$id[[i]])
    }
    list(rows = rows)
  }
}, thinkthen_error = function(error) c(list(error = error$code, family = class(error)[[1]]),
                                       if (!is.null(error$http_status)) list(http_status = error$http_status)))
# JSON has one number type. Preserve R's numeric storage before serialization
# so the external comparator can reject integer probability/score projections.
numeric_types <- function(value, path = list()) {
  output <- list()
  if (is.list(value)) {
    keys <- if (is.null(names(value))) as.list(seq_along(value) - 1L) else as.list(names(value))
    for (i in seq_along(value)) output <- c(output, numeric_types(value[[i]], c(path, list(keys[[i]]))))
    return(unname(output))
  }
  if (!is.numeric(value)) return(output)
  if (length(value) != 1L) {
    for (i in seq_along(value)) output <- c(output, numeric_types(value[[i]], c(path, list(i - 1L))))
    return(unname(output))
  }
  list(list(path = path, type = typeof(value)))
}
if (exists("client", inherits = FALSE) && !is.null(case$requests)) {
  stopifnot(tt_usage(client)$attempts == case$requests)
}
result$numeric_types <- numeric_types(result)
writeLines(jsonlite::toJSON(result, auto_unbox = TRUE, null = "null", na = "null", digits = I(17)), useBytes = TRUE)
