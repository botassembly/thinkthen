.tt_state <- new.env(parent = emptyenv())

.tt_native <- function(name, ...) {
  result <- .Call(paste0("wrap__", name), ..., PACKAGE = "thinkthen")
  if (!is.null(result$error)) {
    error <- result$error
    stop(structure(
      list(message = error$message, call = NULL, code = error$code),
      class = c(paste0("thinkthen_", error$family, "_error"),
                "thinkthen_error", "error", "condition")
    ))
  }
  result
}

.tt_dots <- function(...) {
  if (length(list(...))) stop("unused arguments", call. = FALSE)
}

.tt_json <- function(value) {
  as.character(jsonlite::toJSON(value, auto_unbox = TRUE, null = "null",
                               na = "null", digits = NA, force = FALSE))
}

.tt_options <- function(...) {
  values <- list(...)
  values[!vapply(values, is.null, logical(1))]
}

tt_client <- function(adapter = NULL, ..., mode = NULL, url = NULL,
                      key_source = NULL, model = NULL, batch = NULL) {
  .tt_dots(...)
  settings <- .tt_options(adapter = adapter, mode = mode, url = url,
                          key_source = key_source, model = model, batch = batch)
  if (!length(settings)) settings <- structure(list(), names = character())
  .tt_native("r_client", .tt_json(settings))$value
}

# R vectors in domain fields represent JSON arrays even at length one.
.tt_toolbox <- function(toolbox) {
  definition <- function(value) {
    if (!is.list(value)) return(value)
    for (field in intersect(c("options", "labels", "levels", "recognize", "relate"), names(value))) {
      if (is.atomic(value[[field]]) && !is.null(value[[field]])) value[[field]] <- I(value[[field]])
    }
    value
  }
  if (!is.list(toolbox)) return(toolbox)
  if (is.list(toolbox$questions)) {
    toolbox$questions <- lapply(toolbox$questions, definition)
    toolbox
  } else definition(toolbox)
}

tt_question <- function(toolbox, name = NULL) {
  structure(.tt_options(type = "toolbox", value = .tt_toolbox(toolbox), name = name),
            class = "thinkthen_question")
}

tt_reference <- function(name) {
  structure(list(type = "reference", name = name), class = "thinkthen_question")
}

tt_detail <- function(rows) {
  lapply(rows$detail, function(value) {
    if (is.na(value)) NULL else jsonlite::fromJSON(value, simplifyVector = FALSE)
  })
}

.tt_restore <- function(original, selected) {
  attrs <- attributes(original)
  for (name in intersect(names(attrs), c("names", "row.names", "dim", "dimnames"))) {
    attrs[name] <- list(attr(selected, name))
  }
  attributes(selected) <- attrs
  selected
}

.tt_subset <- function(x, positions) {
  if (is.data.frame(x)) {
    selected <- x[positions, , drop = FALSE]
    for (column in seq_along(selected)) {
      selected[[column]] <- .tt_restore(x[[column]], selected[[column]])
    }
  } else {
    selected <- x[positions]
  }
  selected <- .tt_restore(x, selected)
  attr(selected, ".thinkthen_function") <- NULL
  selected
}

.tt_frame <- function(rows, detail, function_name) {
  scalar <- function(key, missing, convert = identity) {
    vapply(rows, function(row) {
      value <- row[[key]]
      if (is.null(value)) missing else convert(value)
    }, missing)
  }
  answers <- switch(function_name,
    decide = scalar("answer", NA),
    choose = scalar("answer", NA_character_),
    score = scalar("answer", NA_real_, as.double),
    rank = scalar("answer", NA_integer_, as.integer),
    tag = lapply(rows, function(row) {
      if (is.null(row$answer)) NULL else as.character(unlist(row$answer, use.names = FALSE))
    }),
    lapply(rows, `[[`, "answer")
  )
  structure(list(id = lapply(rows, `[[`, "id"),
                 status = scalar("status", NA_character_),
                 answer = answers,
                 p = scalar("p", NA_real_, as.double),
                 error = lapply(rows, `[[`, "error"),
                 detail = detail),
            class = "data.frame", row.names = .set_row_names(length(rows)),
            .thinkthen_function = function_name)
}

.tt_run <- function(x, function_name, question, options, client) {
  if (is.null(client)) {
    if (is.null(.tt_state$client)) .tt_state$client <- tt_client()
    client <- .tt_state$client
  }
  if (!inherits(question, "thinkthen_question")) {
    if (function_name == "annotate") question <- .tt_toolbox(question)
    question <- list(type = "inline", value = question)
  } else {
    question <- unclass(question)
  }
  if (!length(options)) options <- structure(list(), names = character())
  request <- .tt_json(list(`function` = function_name, question = question, options = options))
  call <- .tt_native("r_open", client, request)$value
  on.exit(.Call("wrap__r_close", call, PACKAGE = "thinkthen"))
  size <- if (is.data.frame(x)) nrow(x) else length(x)
  position <- 0
  rows <- list()
  detail <- character()
  kept <- numeric()
  repeat {
    event <- .tt_native("r_poll", call)
    if (event$event == "input") {
      if (position == size) {
        .Call("wrap__r_end", call, PACKAGE = "thinkthen")
      } else {
        position <- position + 1L
        if (is.character(x) && !is.na(x[[position]])) {
          .tt_native("r_text", call, enc2utf8(x[[position]]))
        } else {
          value <- if (is.data.frame(x)) {
            encoded <- .tt_json(x[position, , drop = FALSE])
            substr(encoded, 2L, nchar(encoded) - 1L)
          } else .tt_json(x[[position]])
          .tt_native("r_json", call, value)
        }
      }
    } else if (event$event == "row") {
      if (function_name == "filter") {
        if (event$kept) kept <- c(kept, event$position)
      } else {
        rows[[length(rows) + 1L]] <- jsonlite::fromJSON(event$json, simplifyVector = FALSE)
        detail <- c(detail, if (is.null(event$detail)) NA_character_ else event$detail)
      }
    } else if (event$event == "finished") break
  }
  if (function_name == "filter") .tt_subset(x, kept) else .tt_frame(rows, detail, function_name)
}

tt_decide <- function(x, question, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "decide", question, .tt_options(threshold = threshold, on = on), client)
}

tt_choose <- function(x, question, options = NULL, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "choose", question, .tt_options(options = if (!is.null(options)) I(options), threshold = threshold, on = on), client)
}

tt_tag <- function(x, question, labels = NULL, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "tag", question, .tt_options(labels = if (!is.null(labels)) I(labels), threshold = threshold, on = on), client)
}

tt_score <- function(x, question, levels = NULL, ..., on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "score", question, .tt_options(levels = if (!is.null(levels)) I(levels), on = on), client)
}

tt_filter <- function(x, question, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "filter", question, .tt_options(threshold = threshold, on = on), client)
}

tt_rank <- function(x, question, ..., levels = NULL, top = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "rank", question, .tt_options(levels = if (!is.null(levels)) I(levels), top = top, on = on), client)
}

tt_find <- function(x, question, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "find", question, .tt_options(threshold = threshold, on = on), client)
}

tt_annotate <- function(x, toolbox, ..., on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "annotate", toolbox, .tt_options(on = on), client)
}

tt_recognize <- function(x, kinds = character(), ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "recognize", if (inherits(kinds, "thinkthen_question") || is.null(kinds)) kinds else I(kinds),
          .tt_options(threshold = threshold, on = on), client)
}

tt_relate <- function(x, rules, ..., threshold = NULL, on = NULL, client = NULL) {
  .tt_dots(...)
  .tt_run(x, "relate", rules, .tt_options(threshold = threshold, on = on), client)
}
