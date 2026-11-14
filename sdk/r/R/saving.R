# Convert R's native vector records back to the shared saved representation.
.tt_wire_value <- function(value) {
  if (inherits(value, "vctrs_rcrd")) {
    records <- lapply(seq_along(value), function(i) {
      fields <- setNames(lapply(vctrs::fields(value), function(name) {
        field <- vctrs::field(value, name)
        field <- if (is.list(field) && !inherits(field, "vctrs_rcrd")) field[[i]] else field[i]
        converted <- .tt_wire_value(field)
        if (inherits(field, "vctrs_rcrd")) converted[[1]] else converted
      }), vctrs::fields(value))
      if ("start" %in% names(fields) && !is.null(fields$start)) fields$start <- fields$start - 1
      fields
    })
    return(records)
  }
  if (is.factor(value)) return(as.character(value))
  if (is.list(value)) return(lapply(unclass(value), .tt_wire_value))
  if (!is.null(names(value))) return(as.list(value))
  value
}

.tt_wire_answer <- function(value, verb, i) {
  if (verb == "find") {
    span <- value[i]
    if (identical(span$text, "")) return(NULL)
    return(.tt_wire_value(span)[[1]])
  }
  if (verb == "choose") return(as.character(value[i]))
  if (verb == "tag") return(as.list(as.character(value[[i]])))
  if (verb %in% c("recognize", "relate")) return(.tt_wire_value(value[[i]]))
  value[[i]]
}

.tt_wire_outcome <- function(rows, i, verb, name = NULL) {
  prefix <- if (is.null(name)) "" else paste0(name, "_")
  field <- function(key) rows[[paste0(prefix, key)]][i]
  status <- as.character(field("status"))
  outcome <- list(`function` = verb, status = status)
  if (status == "answered") {
    answer <- if (is.null(name)) rows$answer else rows[[name]]
    outcome["answer"] <- list(.tt_wire_answer(answer, verb, i))
  }
  if (!is.na(field("probability"))) outcome$probability <- field("probability")
  if (status == "failed") {
    outcome$error <- list(code = field("error_code"), message = field("error_message"))
    if (!is.na(field("error_http_status"))) outcome$error$http_status <- field("error_http_status")
  }
  detail <- field("detail")[[1]]
  if (!is.null(detail)) outcome$detail <- .tt_wire_value(detail)
  outcome
}

#' Save details as canonical shared JSON rows, with character offsets from zero.
#'
#' @param details A details table returned by a ThinkThen function.
#' @param path A JSON Lines file to write.
#' @export
tt_details_write <- function(details, path) {
  verb <- .tt_result(details)
  lines <- vapply(seq_len(nrow(details)), function(i) {
    if (verb == "annotate") {
      row <- list(`function` = verb, status = as.character(details$status[i]))
      if (row$status == "answered") {
        children <- attr(details, ".thinkthen_children", exact = TRUE)
        row$answer <- setNames(lapply(names(children), function(name) {
          .tt_wire_outcome(details, i, children[[name]][["function"]], name)
        }), names(children))
      } else {
        row$error <- list(code = details$error_code[i], message = details$error_message[i])
        if (!is.na(details$error_http_status[i])) row$error$http_status <- details$error_http_status[i]
      }
    } else row <- .tt_wire_outcome(details, i, verb)
    row$position <- details$position[i]
    row$source <- .tt_wire_value(details$source[[i]])
    id <- details$id[[i]]
    admitted <- if (is.null(id)) NULL else .tt_prepare_id(id)
    if (!is.null(id) && is.null(admitted)) .tt_r_error("The details contain an invalid identifier.", "id")
    .tt_native("r_saved_row", .tt_json(row), admitted)$value
  }, character(1))
  writeLines(lines, path, useBytes = TRUE)
  invisible(details)
}
