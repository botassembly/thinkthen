source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))

# Feature: exact integer IDs use the approved decimal-backed record scalar.
expected <- c("-9223372036854775808", "9007199254740993",
              "9223372036854775807", "18446744073709551615")
ids <- tt_id_integer(expected)
stopifnot(inherits(ids, "tt_id_integer"), inherits(ids, "vctrs_rcrd"),
          !is.character(ids), identical(format(ids), expected),
          inherits(ids[[1]], "tt_id_integer"), length(tt_id_integer(character())) == 0L,
          is.na(tt_id_integer(NA_character_)))
client <- tt_client(backend = "stub")
for (i in seq_along(expected)) {
  out <- tt_decide(list(list(id = ids[[i]], body = "text")), "Ready?", client = client)
  stopifnot(nrow(out) == 1L, out$status != "failed",
            inherits(out$id[[1]], "tt_id_integer"),
            identical(format(out$id[[1]]), expected[[i]]))
}

# Both direct record-vector columns and mixed list columns retain ID identity.
frame <- tibble::tibble(id = ids, body = rep("text", length(ids)))
rows <- tt_decide(frame, "Ready?", client = client)
stopifnot(nrow(rows) == length(ids),
          identical(vapply(rows$id, format, character(1)), expected))
mixed <- tibble::tibble(id = list(ids[[4]], expected[[4]], 1L, 2),
                        body = rep("text", 4L))
rows <- tt_decide(mixed, "Ready?", client = client)
stopifnot(inherits(rows$id[[1]], "tt_id_integer"),
          identical(format(rows$id[[1]]), expected[[4]]),
          is.character(rows$id[[2]]), identical(rows$id[[2]], expected[[4]]),
          inherits(rows$id[[3]], "tt_id_integer"), identical(format(rows$id[[3]]), "1"),
          inherits(rows$id[[4]], "tt_id_integer"), identical(format(rows$id[[4]]), "2"))

# Safe ordinary doubles remain numeric IDs; the next boundary requires a wrapper.
safe <- c(-(2^53 - 1), 2^53 - 1)
rows <- tt_decide(lapply(safe, function(id) list(id = id, body = "text")),
                  "Ready?", client = client)
stopifnot(all(rows$status != "failed"),
          all(vapply(rows$id, inherits, logical(1), "tt_id_integer")),
          identical(vapply(rows$id, format, character(1)),
                    c("-9007199254740991", "9007199254740991")))

# Slicing, concatenation and row binding preserve exact values and missingness.
stopifnot(identical(format(c(ids[1:2], ids[3:4])), expected),
          identical(format(vctrs::vec_c(ids[1:2], ids[3:4])), expected))
rebound <- vctrs::vec_rbind(frame[1:2, ], frame[3:4, ])
stopifnot(inherits(rebound$id, "tt_id_integer"), identical(format(rebound$id), expected))
missing <- vctrs::vec_init(ids, 2L)
stopifnot(inherits(missing, "tt_id_integer"), identical(is.na(missing), c(TRUE, TRUE)))
for (convert in list(as.character, as.integer, as.double)) {
  error <- tryCatch(convert(ids), error = identity)
  stopifnot(inherits(error, "error"))
}
stopifnot(inherits(tryCatch(ids + 1, error = identity), "error"))
for (bad in list(1, "1e3", "1.5", " 1", "-9223372036854775809", "18446744073709551616")) {
  condition(tt_id_integer(bad), "thinkthen_usage_error")
}
# An absent member is valid, and remains distinct from a present NULL member.
absent <- tt_decide(list(list(body = "text")), "Ready?", client = client)
stopifnot(absent$status != "failed", is.null(absent$id[[1]]))
tt_close(client)

# Feature: host conversion failures keep rows and make no backend request.
# The parent fixture runner must supply a counted loopback URL, serve no replies,
# and assert its final request count is zero. Missing wiring is a hard failure.
url <- Sys.getenv("THINKTHEN_TEST_URL")
stopifnot(nzchar(url))
for (backend in c("systemone", "openai")) {
  client <- tt_client(backend = backend, url = url, api_key = "synthetic-local-key",
                       retry = tt_retry(max_retries = 0L))
  bad_ids <- list(2^53, -(2^53), Inf, NaN, 1.5, TRUE, NULL,
                  tt_id_integer(NA_character_), list(1L))
  records <- lapply(bad_ids, function(id) list(id = id, body = "text"))
  rows <- tt_decide(records, "Ready?", client = client, .progress = FALSE)
  stopifnot(nrow(rows) == length(records),
            identical(rows$position, as.double(seq_along(records))),
            all(rows$status == "failed"),
            all(as.character(rows$error_code) == "record.invalid"))
  tt_close(client)
}
cat("R exact integer IDs and host conversion failures passed\n")
