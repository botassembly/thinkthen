# Exact root IDs cross the native boundary separately from generic JSON values.
.tt_id_value <- function(decimal, kind) {
  vctrs::new_rcrd(list(decimal = decimal, kind = kind), class = "tt_id_integer")
}

tt_id_integer <- function(x) {
  if (!is.character(x)) {
    # Shared admission supplies the code and message for an invalid constructor.
    .tt_native("r_id_integer", .tt_context_json(list(x), "r_question_conversion_error"))
  }
  admitted <- lapply(x, function(value) {
    if (is.na(value)) list(decimal = NA_character_, kind = NA_character_) else {
      .tt_native("r_id_integer", value)
    }
  })
  .tt_id_value(vapply(admitted, `[[`, character(1), "decimal"),
               vapply(admitted, `[[`, character(1), "kind"))
}

format.tt_id_integer <- function(x, ...) vctrs::field(x, "decimal")
vec_ptype_abbr.tt_id_integer <- function(x, ...) "tt_id"
vec_ptype2.tt_id_integer.tt_id_integer <- function(x, y, ...) .tt_id_value(character(), character())
vec_cast.tt_id_integer.tt_id_integer <- function(x, to, ...) x

# Admit IDs separately so exact integer IDs never pass through generic JSON.
.tt_prepare_id <- function(id) {
  if (is.factor(id)) id <- as.character(id)
  tryCatch({
    if (inherits(id, "tt_id_integer") && length(id) == 1L && !is.na(id)) {
      .tt_native("r_id_integer", format(id))$value
    } else if (is.character(id) && length(id) == 1L && !is.na(id)) {
      .Call("wrap__r_id_string", enc2utf8(id), PACKAGE = "thinkthen")
    } else if ((is.integer(id) || is.double(id)) && length(id) == 1L &&
               !is.object(id) && is.finite(id) && id == trunc(id) && abs(id) <= 2^53 - 1) {
      .tt_native("r_id_integer", sprintf("%.0f", id))$value
    } else NULL
  }, error = function(error) NULL)
}

# Skip quoted delimiters and count nesting before splitting array-level commas.
# Byte offsets avoid rescanning UTF-8 prefixes for every punctuation mark or row.
.tt_frame_json <- function(x) {
  json <- enc2utf8(as.character(jsonlite::toJSON(x, dataframe = "rows", auto_unbox = TRUE,
    null = "null", na = "null", digits = I(17), force = FALSE)))
  if (nrow(x) == 0L) return(character())
  Encoding(json) <- "bytes"
  punctuation <- gregexpr(r"("[^"\\]*(?:\\.[^"\\]*)*"(*SKIP)(*F)|[][{},])",
                         json, perl = TRUE, useBytes = TRUE)[[1]]
  tokens <- substring(json, punctuation, punctuation)
  depth <- cumsum((tokens %in% c("[", "{")) - (tokens %in% c("]", "}")))
  commas <- punctuation[tokens == "," & depth == 1L]
  rows <- substring(json, c(2L, commas + 1L), c(commas - 1L, nchar(json, type = "bytes") - 1L))
  Encoding(rows) <- "UTF-8"
  rows
}

.tt_prepare_frame <- function(x) {
  has_id <- "id" %in% names(x)
  values <- if (has_id) x[, names(x) != "id", drop = FALSE] else x
  encoded <- tryCatch(.tt_frame_json(values), error = function(error) {
    # A bulk conversion failure must not invalidate serializable neighbors.
    lapply(seq_len(nrow(values)), function(position) {
      tryCatch(.tt_frame_json(values[position, , drop = FALSE])[[1]],
               error = function(error) NULL)
    })
  })
  lapply(seq_len(nrow(x)), function(position) {
    admitted <- if (has_id) .tt_prepare_id(x$id[[position]]) else NULL
    if (is.null(encoded[[position]]) || (has_id && is.null(admitted))) {
      .tt_native("r_prepare_record", "", 3L, NULL)$value
    } else .tt_native("r_prepare_record", encoded[[position]], 1L, admitted)$value
  })
}

.tt_prepare <- function(x, position) {
  if (is.character(x) && !is.na(x[[position]])) {
    return(.tt_native("r_prepare_record", enc2utf8(x[[position]]), 0L, NULL)$value)
  }
  value <- x[[position]]
  has_id <- is.list(value) && "id" %in% names(value)
  admitted <- NULL
  if (has_id) {
    admitted <- .tt_prepare_id(value$id)
    if (is.null(admitted)) return(.tt_native("r_prepare_record", "", 3L, NULL)$value)
    value <- value[names(value) != "id"]
  }
  encoded <- tryCatch(.tt_json(value), error = function(error) NULL)
  if (is.null(encoded)) .tt_native("r_prepare_record", "", 3L, NULL)$value else {
    .tt_native("r_prepare_record", encoded, 1L, admitted)$value
  }
}
