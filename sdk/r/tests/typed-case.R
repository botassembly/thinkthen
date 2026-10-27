source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
case <- read_case(commandArgs(TRUE)[[1]])
# Python identifies numeric tokens before jsonlite reads them as decimal text.
# The supplied fixture contains ordinary public R input values, not a backend.
numeric_ids <- jsonlite::fromJSON(Sys.getenv("THINKTHEN_TEST_NUMERIC_IDS", "{}"),
                                  simplifyVector = FALSE)
if (length(numeric_ids)) {
  id_values <- dget(Sys.getenv("THINKTHEN_TEST_ID_VALUES"))
  for (index in names(numeric_ids)) {
    value <- id_values[[numeric_ids[[index]]]]
    stopifnot(!is.null(value), is.object(value))
    case$request$records[[as.integer(index)]]$id <- value
  }
}
settings <- case$client
if (is.null(settings)) settings <- list()
settings$url <- Sys.getenv("THINKTHEN_TEST_URL")
settings$api_key <- "synthetic-loopback-key"

# Compare typed R values to shared JSON expectations without a second decoder.
field <- function(value, name) {
  if (inherits(value, "vctrs_rcrd")) vctrs::field(value, name) else value[[name]]
}
expect_value <- function(actual, expected) {
  if (is.null(expected)) {
    stopifnot(is.null(actual) || (length(actual) == 1L && is.na(actual)))
  } else if (is.list(expected)) {
    if (!is.null(names(expected))) {
      for (name in names(expected)) expect_value(field(actual, name), expected[[name]])
    } else {
      stopifnot(length(actual) == length(expected))
      for (i in seq_along(expected)) {
        item <- if (inherits(actual, "vctrs_rcrd")) actual[i] else actual[[i]]
        expect_value(item, expected[[i]])
      }
    }
  } else {
    if (is.factor(actual)) actual <- as.character(actual)
    stopifnot(isTRUE(all.equal(actual, expected, check.attributes = FALSE, tolerance = 0)))
  }
}

result <- tryCatch({
  client <- do.call(tt_client, settings)
  rows <- invoke(case$request$records, case$request$question, client)
  verb <- case$request$question[["function"]]
  check_rows(rows, verb, length(case$expected))
  for (i in seq_len(nrow(rows))) {
    expected <- case$expected[[i]]
    stopifnot(rows$position[[i]] == expected$position,
              as.character(rows$status[[i]]) == expected$status)
    if (is.character(expected$id) && expected$position %in% as.integer(names(numeric_ids))) {
      stopifnot(is.object(rows$id[[i]]), !is.character(rows$id[[i]]),
                identical(format(rows$id[[i]]), expected$id))
    } else expect_value(rows$id[[i]], expected$id)
    expect_value(rows$source[[i]], expected$source)
    if (expected$status == "failed") {
      expect_value(as.character(rows$error_code[[i]]), expected$error$code)
      expect_value(rows$error_http_status[[i]], expected$error$http_status)
      stopifnot(nzchar(rows$error_message[[i]]), is.na(rows$probability[[i]]))
    } else {
      if (expected$status == "answered") {
        answer <- if (inherits(rows$answer, "vctrs_rcrd")) rows$answer[i] else rows$answer[[i]]
        expect_value(answer, expected$answer)
      } else {
        absent <- if (is.list(rows$answer) && !inherits(rows$answer, "vctrs_rcrd")) {
          is.null(rows$answer[[i]])
        } else is.na(rows$answer[i])
        stopifnot(absent)
      }
      expect_value(rows$probability[[i]], expected$probability)
      stopifnot(is.na(rows$error_code[[i]]))
      expect_value(rows$detail[[i]], expected$detail)
      if (!is.null(expected$detail)) {
        stopifnot(!identical(class(rows$detail[[i]]), "list"),
                  !is.character(rows$detail[[i]]))
      }
    }
    if (verb == "find" && expected$status == "answered" && !is.null(expected$answer)) {
      span <- rows$answer[i]
      stopifnot(inherits(span, "tt_span"),
                identical(substr(case$request$records[[i]], span$start, span$end), span$text))
    }
    if (verb == "filter") {
      stopifnot(identical(rows$item[[i]], case$request$records[[i]]),
                identical(rows$kept[[i]], if (expected$status == "failed") NA else isTRUE(expected$answer)))
    }
  }
  # Prototype signatures expose backend-dependent shape drift to the driver.
  signature <- list(rows = class(rows), answer = class(rows$answer),
                    status = levels(rows$status), codes = levels(rows$error_code),
                    detail = lapply(rows$detail, class))
  tt_close(client)
  list(signature = signature)
}, thinkthen_error = function(e) {
  stopifnot(is.list(case$expected), !is.null(case$expected$error))
  expect_value(as.character(e$code), case$expected$error)
  list(error = as.character(e$code))
})
writeLines(jsonlite::toJSON(result, auto_unbox = TRUE, null = "null", digits = NA), useBytes = TRUE)
