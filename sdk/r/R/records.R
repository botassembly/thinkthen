# Every record is a host projection of an already admitted shared value.
.tt_record <- function(fields, class) vctrs::new_rcrd(fields, class = class)
.tt_span <- function(rows = list(), class = "tt_span") {
  fields <- list(text = .tt_scalar(rows, "text", NA_character_),
                 start = .tt_scalar(rows, "start", NA_real_, as.double),
                 end = .tt_scalar(rows, "end", NA_real_, as.double))
  if (class %in% c("tt_name", "tt_endpoint")) fields$kind <- .tt_scalar(rows, "kind", NA_character_)
  if (class == "tt_name") {
    fields$probability <- .tt_scalar(rows, "probability", NA_real_, as.double)
  }
  .tt_record(fields, class)
}
`$.tt_span` <- function(x, name) vctrs::field(x, name)
`$.tt_name` <- function(x, name) vctrs::field(x, name)
format.tt_span <- function(x, ...) vctrs::field(x, "text")
format.tt_name <- function(x, ...) vctrs::field(x, "text")
vec_ptype_abbr.tt_span <- function(x, ...) "tt_span"
vec_ptype_abbr.tt_name <- function(x, ...) "tt_name"
vec_ptype2.tt_span.tt_span <- function(x, y, ...) .tt_span()
vec_cast.tt_span.tt_span <- function(x, to, ...) x
vec_ptype2.tt_name.tt_name <- function(x, y, ...) .tt_span(class = "tt_name")
vec_cast.tt_name.tt_name <- function(x, to, ...) x

.tt_relations <- function(rows = list()) {
  .tt_record(list(source = .tt_span(lapply(rows, `[[`, "source"), "tt_endpoint"),
                  target = .tt_span(lapply(rows, `[[`, "target"), "tt_endpoint"),
                  relation = .tt_scalar(rows, "relation", NA_character_),
                  probability = .tt_scalar(rows, "probability", NA_real_, as.double),
                  either = .tt_scalar(rows, "either", FALSE)), "tt_relation")
}
`$.tt_relation` <- function(x, name) vctrs::field(x, name)
format.tt_relation <- function(x, ...) vctrs::field(x, "relation")
vec_ptype_abbr.tt_relation <- function(x, ...) "tt_relation"
vec_ptype2.tt_relation.tt_relation <- function(x, y, ...) .tt_relations()
vec_cast.tt_relation.tt_relation <- function(x, to, ...) x

.tt_scalar <- function(rows, key, missing, convert = identity) {
  vapply(rows, function(row) {
    value <- row[[key]]
    if (is.null(value)) missing else convert(value)
  }, missing)
}

.tt_answer_values <- function(rows, verb) {
  switch(verb,
    decide = .tt_scalar(rows, "answer", NA), filter = .tt_scalar(rows, "answer", NA),
    choose = .tt_scalar(rows, "answer", NA_character_),
    score = .tt_scalar(rows, "answer", NA_real_, as.double),
    rank = .tt_scalar(rows, "answer", NA_real_, as.double),
    tag = vctrs::new_list_of(lapply(rows, function(row) {
      if (is.null(row$answer)) NULL else as.character(unlist(row$answer, use.names = FALSE))
    }), ptype = character()),
    find = .tt_span(lapply(rows, `[[`, "answer")),
    recognize = vctrs::new_list_of(lapply(rows, function(row) {
      if (is.null(row$answer)) NULL else .tt_span(row$answer, "tt_name")
    }), ptype = .tt_span(class = "tt_name")),
    relate = vctrs::new_list_of(lapply(rows, function(row) {
      if (is.null(row$answer)) NULL else .tt_relations(row$answer)
    }), ptype = .tt_relations()),
    annotate = lapply(rows, function(row) {
      if (is.null(row$answer)) return(NULL)
      lapply(row$answer, function(outcome) {
        verb <- outcome[["function"]]
        outcome$answer <- .tt_answer_values(list(outcome), verb)
        outcome$detail <- .tt_detail_value(outcome$detail, verb)
        status <- outcome$status
        if (!status %in% .tt_status_names) .tt_check_abi_failure()
        outcome$status <- factor(status, levels = unname(.tt_status_names))
        if (!is.null(outcome$error)) {
          if (!outcome$error$code %in% .tt_code_names) .tt_check_abi_failure()
          outcome$error$code <- factor(outcome$error$code, levels = unname(.tt_code_names))
          if (!is.null(outcome$error$http_status)) outcome$error$http_status <- as.integer(outcome$error$http_status)
          class(outcome$error) <- "tt_error"
        }
        structure(outcome, class = c(paste0("tt_", verb, "_", status, "_outcome"), "tt_outcome"))
      })
    }))
}

.tt_named_double <- function(value) {
  result <- as.double(unlist(value, use.names = FALSE))
  names(result) <- names(value)
  result
}
.tt_detail_value <- function(detail, verb) {
  if (is.null(detail)) return(NULL)
  if (verb %in% c("choose", "tag")) detail$labels <- .tt_named_double(detail$labels)
  if (verb %in% c("score", "rank")) detail$levels <- as.double(unlist(detail$levels, use.names = FALSE))
  if (verb == "relate") {
    detail$names <- .tt_span(detail$names, "tt_name")
    detail$pairs <- .tt_relations(detail$pairs)
  }
  if (verb == "recognize") {
    detail$pieces <- .tt_record(list(
      text = .tt_scalar(detail$pieces, "text", NA_character_),
      start = .tt_scalar(detail$pieces, "start", NA_real_, as.double),
      end = .tt_scalar(detail$pieces, "end", NA_real_, as.double),
      tags = vctrs::new_list_of(lapply(detail$pieces, function(piece) .tt_named_double(piece$tags)), ptype = double())
    ), "tt_piece")
    detail$names <- .tt_record(list(
      start = .tt_scalar(detail$names, "start", NA_real_, as.double),
      end = .tt_scalar(detail$names, "end", NA_real_, as.double),
      kinds = vctrs::new_list_of(lapply(detail$names, function(name) .tt_named_double(name$kinds)), ptype = double())
    ), "tt_name_detail")
  }
  structure(detail, class = paste0("tt_", verb, "_detail"))
}

.tt_frame <- function(rows, verb) {
  statuses <- unname(.tt_status_names)
  if (any(!vapply(rows, function(row) row$status %in% statuses, logical(1)))) .tt_check_abi_failure()
  codes <- vapply(rows, function(row) if (is.null(row$error)) NA_character_ else row$error$code, character(1))
  if (any(!is.na(codes) & !codes %in% .tt_code_names)) .tt_check_abi_failure()
  result <- tibble::new_tibble(list(
    position = .tt_scalar(rows, "position", NA_real_, as.double),
    id = lapply(rows, `[[`, "id"),
    status = factor(.tt_scalar(rows, "status", NA_character_), levels = statuses),
    answer = .tt_answer_values(rows, verb),
    probability = .tt_scalar(rows, "probability", NA_real_, as.double),
    source = lapply(rows, function(row) {
      source <- row$source
      if (!source$backend %in% .tt_backend_names) .tt_check_abi_failure()
      source$backend <- factor(source$backend, levels = unname(.tt_backend_names))
      structure(source, class = "tt_source")
    }),
    error_code = factor(codes, levels = unname(.tt_code_names)),
    error_message = vapply(rows, function(row) if (is.null(row$error)) NA_character_ else row$error$message, character(1)),
    error_http_status = vapply(rows, function(row) if (is.null(row$error$http_status)) NA_integer_ else as.integer(row$error$http_status), integer(1)),
    detail = lapply(rows, function(row) .tt_detail_value(row$detail, verb))
  ), nrow = length(rows), class = c(paste0("tt_", verb, "_rows"), "tt_rows"))
  attr(result, ".thinkthen_function") <- verb
  result
}

.tt_check_abi_failure <- function() {
  rlang::abort(.tt_abi_message, class = c("thinkthen_usage_error", "thinkthen_error"),
               code = .tt_abi_code, call = NULL, trace = rlang::trace_back(bottom = globalenv()))
}

vec_ptype2.tt_rows.tt_rows <- function(x, y, ...) {
  if (!identical(class(x), class(y))) .tt_r_error("Result row types must match.", "rows")
  out <- vctrs::vec_ptype2(tibble::as_tibble(x), tibble::as_tibble(y))
  class(out) <- class(x)
  attr(out, ".thinkthen_function") <- attr(x, ".thinkthen_function")
  out
}
vec_cast.tt_rows.tt_rows <- function(x, to, ...) {
  if (!identical(class(x), class(to))) .tt_r_error("Result row types must match.", "rows")
  out <- vctrs::vec_cast(tibble::as_tibble(x), tibble::as_tibble(to))
  class(out) <- class(to)
  attr(out, ".thinkthen_function") <- attr(to, ".thinkthen_function")
  out
}

`$.tt_endpoint` <- function(x, name) vctrs::field(x, name)
format.tt_endpoint <- function(x, ...) vctrs::field(x, "text")
vec_ptype_abbr.tt_endpoint <- function(x, ...) "tt_endpoint"
vec_ptype2.tt_endpoint.tt_endpoint <- function(x, y, ...) .tt_span(class = "tt_endpoint")
vec_cast.tt_endpoint.tt_endpoint <- function(x, to, ...) x
format.tt_piece <- function(x, ...) vctrs::field(x, "text")
vec_ptype_abbr.tt_piece <- function(x, ...) "tt_piece"
vec_ptype2.tt_piece.tt_piece <- function(x, y, ...) vctrs::vec_slice(x, integer())
vec_cast.tt_piece.tt_piece <- function(x, to, ...) x
format.tt_name_detail <- function(x, ...) as.character(vctrs::field(x, "start"))
vec_ptype_abbr.tt_name_detail <- function(x, ...) "tt_name_detail"
vec_ptype2.tt_name_detail.tt_name_detail <- function(x, y, ...) vctrs::vec_slice(x, integer())
vec_cast.tt_name_detail.tt_name_detail <- function(x, to, ...) x
