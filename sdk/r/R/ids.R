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

.tt_prepare <- function(x, position) {
  if (is.character(x) && !is.na(x[[position]])) {
    return(.tt_native("r_prepare_record", enc2utf8(x[[position]]), 0L, NULL)$value)
  }
  is_frame <- is.data.frame(x)
  value <- if (is_frame) x[position, , drop = FALSE] else x[[position]]
  has_id <- is.list(value) && "id" %in% names(value)
  admitted <- NULL
  if (has_id) {
    id <- if (is_frame) value$id[[1]] else value$id
    if (is.factor(id)) id <- as.character(id)
    admitted <- tryCatch({
      if (inherits(id, "tt_id_integer") && length(id) == 1L && !is.na(id)) {
        .tt_native("r_id_integer", format(id))$value
      } else if (is.character(id) && length(id) == 1L && !is.na(id)) {
        .Call("wrap__r_id_string", enc2utf8(id), PACKAGE = "thinkthen")
      } else if ((is.integer(id) || is.double(id)) && length(id) == 1L &&
                 !is.object(id) && is.finite(id) && id == trunc(id) && abs(id) <= 2^53 - 1) {
        .tt_native("r_id_integer", sprintf("%.0f", id))$value
      } else NULL
    }, error = function(error) NULL)
    if (is.null(admitted)) return(.tt_native("r_prepare_record", "", 3L, NULL)$value)
    value <- if (is_frame) value[, names(value) != "id", drop = FALSE] else value[names(value) != "id"]
  }
  encoded <- tryCatch({
    if (is_frame) {
      json <- .tt_json(value)
      substr(json, 2L, nchar(json) - 1L)
    } else .tt_json(value)
  }, error = function(error) NULL)
  if (is.null(encoded)) .tt_native("r_prepare_record", "", 3L, NULL)$value else {
    .tt_native("r_prepare_record", encoded, 1L, admitted)$value
  }
}
