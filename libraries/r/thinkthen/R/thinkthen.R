# The R half of the surface. The Rust half runs one engine call and converts
# the answer; this half carries the ruled shape: the tt_ prefix, NA as "not
# sure", a column in and a column out, and the six error kinds as R
# conditions with the retry signal.

# Every call runs through here (ADR 0042). The guarded check runs before the
# crossing is forced and after it returns, and the Rust half checks before
# its worker starts and at each 100 ms tick. A user's options(error = ...)
# hook is held aside while the checks run and restored before anything is
# raised, because R consults the hook at signal time (R7-13).
.tt_call <- function(expr) {
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  held <- if (tt_interrupt_pending()) {
    list(interrupt = TRUE)
  } else {
    tryCatch(list(value = expr), error = function(e) .tt_error_condition(e))
  }
  if (is.null(held$cond) && is.null(held$interrupt) &&
      is.list(held$value) && isTRUE(held$value$tt_envelope)) {
    envelope <- held$value
    if (!is.null(envelope$error)) {
      held <- .tt_error_condition(simpleError(envelope$error))
      if (!is.null(held$cond)) {
        held$cond$facts <- envelope$facts
        held$cond$details <- envelope$details
      }
    } else {
      held$value <- structure(list(value = envelope$value, probability = envelope$probability,
                                   facts = envelope$facts,
                                   details = envelope$details), class = "thinkthen_call")
    }
  }
  if (is.null(held$cond) && is.null(held$interrupt) && tt_interrupt_pending()) {
    held <- list(interrupt = TRUE)
  }
  if (!is.null(had_hook)) options(error = had_hook)
  if (!is.null(held$interrupt)) .tt_interrupt()
  if (!is.null(held$cond)) stop(held$cond)
  held$value
}

# Claim before evaluating any asking argument or the first guarded interrupt
# check. An early exit settles only a still-claimed receipt; a running worker
# owns final settlement after its already-sent work ends.
.tt_asking <- function(completion, expr) {
  if (!is.null(completion)) {
    tryCatch(tt_completion_claim(completion),
             error = function(e) stop(.tt_condition(conditionMessage(e))))
  }
  early <- "interrupt"
  on.exit(if (!is.null(completion))
    try(tt_completion_settle_early(completion, early), silent = TRUE), add = TRUE)
  tryCatch({
    value <- expr
    early <- "success"
    value
  }, error = function(e) {
    early <<- if (inherits(e, "thinkthen_error")) e$kind else "local"
    stop(e)
  })
}

.tt_result <- function(value, native) {
  structure(list(value = value, probability = native$probability,
                 facts = native$facts, details = native$details),
            class = "thinkthen_call")
}

# R catches names its formal arguments do not use. Every caught name is a
# usage error, including the old deadline spelling and core-only settings.
.tt_unknown <- function(...) {
  names <- names(list(...))
  if ("deadline" %in% names) .tt_usage("deadline was renamed deadline_ms, in milliseconds")
  .tt_usage(paste0("unknown keyword: ", if (length(names)) names[[1L]] else "unnamed argument"))
}

.tt_settings <- function(kind, fields) {
  if (!length(fields)) return(invisible(NULL))
  .tt_call(tt_settings_check(.tt_json(fields), kind))
}

.tt_call_settings <- function(kind, batch, context, deadline_ms) {
  plain <- function(value) if (is.numeric(value) && identical(class(value), "AsIs"))
    as.numeric(value) else value
  fields <- list()
  if (!is.null(batch)) fields$batch <- plain(batch)
  if (!is.null(context)) fields$context <- context
  if (!is.null(deadline_ms)) fields$deadline_ms <- plain(deadline_ms)
  .tt_settings(kind, fields)
}

tt_completion <- function() tt_completion_new()
tt_completion_read <- function(handle) .tt_call(tt_completion_read_native(handle))

# One engine error as data: the interrupt marker, or its condition.
.tt_error_condition <- function(e) {
  if (inherits(e, "thinkthen_error")) return(list(cond = e))
  text <- conditionMessage(e)
  parts <- strsplit(text, "\u{1f}", fixed = TRUE)[[1]]
  if (length(parts) == 3L && identical(parts[[1]], "interrupt")) {
    return(list(interrupt = TRUE))
  }
  list(cond = .tt_condition(text))
}

# One of the six kinds as an R condition carrying retryable. The Rust half
# packs kind, retryable, and message around \u{1f} marks.
.tt_condition <- function(text) {
  parts <- strsplit(text, "\u{1f}", fixed = TRUE)[[1]]
  if (length(parts) != 3L) return(simpleError(text))
  structure(
    class = c(paste0("thinkthen_", parts[[1]]), "thinkthen_error", "error", "condition"),
    list(message = parts[[3]], kind = parts[[1]], retryable = identical(parts[[2]], "true"),
         call = NULL)
  )
}

# A caller's own mistake, refused before any request with the usage kind.
.tt_usage <- function(message) {
  stop(.tt_condition(paste("usage", "false", message, sep = "\u{1f}")))
}

.tt_local <- function(message) {
  stop(.tt_condition(paste("local", "false", message, sep = "\u{1f}")))
}

# R's own interrupt, delivered for real: the guarded check consumed the
# pending signal, so the process sends itself SIGINT and sleeps, and the
# jump lands in R's own machinery with no Rust frame under it. If the
# signal does not land, the synthetic condition below still stops the call.
.tt_interrupt <- function() {
  delivered <- FALSE
  if (.Platform$OS.type == "unix") {
    try(delivered <- tools::pskill(Sys.getpid(), 2L), silent = TRUE)
    if (isTRUE(delivered)) Sys.sleep(0.1)
  }
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  stop(structure(class = c("interrupt", "condition"), list(message = "", call = NULL)))
}

# The one JSON writer: every string reaches jsonlite as UTF-8 under any
# locale. Native bytes that are valid UTF-8 are UTF-8, latin1 converts, and
# native bytes that are not valid UTF-8 are refused by name (R5-10).
.tt_json <- function(value) {
  utf8 <- function(text) {
    if (!is.character(text) || !length(text)) return(text)
    native <- Encoding(text) == "unknown" & !is.na(text)
    if (any(native & !validUTF8(text))) {
      .tt_usage("a text carries native-marked bytes that are not valid UTF-8 under this locale; convert it with enc2utf8() or iconv() first")
    }
    Encoding(text)[native] <- "UTF-8"
    enc2utf8(text)
  }
  held <- rapply(list(value), utf8, classes = "ANY", how = "replace")[[1L]]
  tryCatch(as.character(jsonlite::toJSON(held, auto_unbox = TRUE, digits = NA,
                                       na = "null", null = "null")),
           error = function(e) .tt_usage("the question cannot be written as JSON"))
}

.tt_json_value <- function(value, what, top = FALSE) {
  if (is.null(value)) {
    if (top) .tt_usage(paste0(what, " must be text, an object, or an array"))
    return(invisible(NULL))
  }
  if (is.list(value) && !is.data.frame(value)) {
    names <- names(value)
    if (!is.null(names) && (anyNA(names) || any(!nzchar(names)) || anyDuplicated(names))) {
      .tt_usage(paste0(what, " has an empty or repeated object key"))
    }
    for (one in value) .tt_json_value(one, what)
    return(invisible(NULL))
  }
  if (top && !is.character(value)) .tt_usage(paste0(what, " must be text, an object, or an array"))
  if (!(is.character(value) || is.logical(value) || is.numeric(value)) ||
      (is.object(value) && !inherits(value, "AsIs")) || anyNA(value) ||
      (is.numeric(value) && any(!is.finite(value)))) {
    .tt_usage(paste0(what, " has an unsupported or missing JSON value"))
  }
  invisible(NULL)
}

.tt_description <- function(value, what) {
  if (!is.null(value) && !is.character(value) && !is.list(value)) {
    .tt_usage(paste0(what, " must be text, an object, an array, or NULL"))
  }
  .tt_json_value(value, what)
  invisible(NULL)
}

.tt_batch_valid <- function(value) {
  is.null(value) || (is.character(value) && length(value) == 1L &&
                      !is.na(value) && identical(value, "max")) ||
    (is.numeric(value) && is.null(attr(value, "class")) && length(value) == 1L &&
       !is.na(value) && is.finite(value) && value >= 1 &&
       value <= 9007199254740991 && value == floor(value))
}

.tt_controls <- function(batch, context, context_ok = TRUE, batch_ok = TRUE) {
  if (!batch_ok && !is.null(batch)) .tt_usage("this call does not take batch")
  if (!context_ok && !is.null(context)) .tt_usage("this call does not take context")
  if (!.tt_batch_valid(batch)) .tt_usage("batch is max or one positive whole number")
  if (!is.null(context) && (!is.character(context) || length(context) != 1L ||
                            is.na(context) || !validUTF8(enc2utf8(context)) ||
                            !nzchar(trimws(context)))) {
    .tt_usage("context must be nonblank UTF-8 text")
  }
  if (!is.null(context)) .tt_json_value(context, "context")
}

.tt_members <- function(value, what) {
  if (is.character(value) && is.null(names(value))) {
    if (anyNA(value)) .tt_usage(paste0(what, " has NA names"))
    return(I(value))
  }
  if (is.character(value) && !is.null(names(value))) value <- as.list(value)
  if (!is.list(value) || is.null(names(value)) ||
      anyNA(names(value)) || any(!nzchar(names(value))) || anyDuplicated(names(value))) {
    .tt_usage(paste0(what, " must be an ordered character vector or named description map"))
  }
  for (one in value) .tt_description(one, what)
  value
}

# One threshold as the file grammar writes it: a cut, or a band as "lo:hi".
.tt_threshold_value <- function(threshold) {
  if (is.character(threshold) && length(threshold) == 1L && !is.na(threshold)) {
    return(threshold[[1L]])
  }
  if (!is.numeric(threshold) || anyNA(threshold) || !length(threshold) ||
      length(threshold) > 2L || any(threshold < 0) || any(threshold > 1)) {
    .tt_usage("threshold must be one or two numbers in [0, 1]")
  }
  if (length(threshold) == 1L) as.numeric(threshold) else
    paste(format(threshold, digits = 15L, trim = TRUE), collapse = ":")
}

# The question file's object from its parts. Members stay a JSON array even
# when there is one (R6-10).
.tt_body <- function(kind, text, members = NULL, threshold = NULL, model = NULL,
                     sides = list()) {
  .tt_json_value(text, "question text", top = TRUE)
  body <- stats::setNames(list(text), kind)
  if (!is.null(members)) {
    body[[switch(kind, choose = "options", score = "levels", "labels")]] <-
      .tt_members(members, "the members")
  }
  for (side in names(sides)) {
    .tt_description(sides[[side]], side)
    body[side] <- list(sides[[side]])
  }
  if (!is.null(threshold)) body$threshold <- .tt_threshold_value(threshold)
  if (!is.null(model)) {
    if (!is.character(model) || length(model) != 1L || is.na(model)) .tt_usage("model is one string")
    body$model <- model
  }
  body
}

# A question value: its file JSON, checked once, and its parts for print.
.tt_built <- function(body) {
  json <- .tt_json(body)
  kind <- .tt_call(tt_question_check(json))
  structure(list(json = json, kind = kind, text = body[[1L]],
                 members = body$options %||% body$levels %||% body$labels),
            class = "thinkthen_question")
}

tt_question <- function(decide = NULL, choose = NULL, options = NULL,
                        score = NULL, levels = NULL, tag = NULL, labels = NULL,
                        threshold = NULL, model = NULL, file = NULL,
                        true = NULL, false = NULL) {
  sides <- list()
  if (!missing(true)) sides["true"] <- list(true)
  if (!missing(false)) sides["false"] <- list(false)
  if (!is.null(file)) {
    if (any(!vapply(list(decide, choose, options, score, levels, tag, labels,
                         threshold, model), is.null, logical(1))) || length(sides)) {
      .tt_usage("a question file stands alone; it takes no question keywords")
    }
    if (!is.character(file) || length(file) != 1L || is.na(file) || !nzchar(file) ||
        Encoding(file) == "bytes" || !validUTF8(file)) {
      .tt_usage("a question file is one path")
    }
    json <- tryCatch(suppressWarnings(readChar(file, file.info(file)$size, useBytes = TRUE)),
                     error = function(e) .tt_local("the question file could not be read"))
    kind <- tryCatch(.tt_call(tt_question_check(json)),
                     thinkthen_error = function(e) .tt_local(conditionMessage(e)))
    body <- tryCatch(jsonlite::fromJSON(json, simplifyVector = FALSE),
                     error = function(e) .tt_local("the question file could not be read"))
    return(structure(list(json = json, kind = kind, text = body[[kind]],
                          members = body$options %||% body$levels %||% body$labels),
                     class = "thinkthen_question"))
  }
  kinds <- list(decide = decide, choose = choose, score = score, tag = tag)
  given <- !vapply(kinds, is.null, logical(1))
  if (sum(given) != 1L) {
    .tt_usage("a question names exactly one kind: decide, choose, score, or tag")
  }
  kind <- names(kinds)[given]
  if (length(sides) && kind != "decide") .tt_usage("only a decide question takes true or false meanings")
  members <- switch(kind, decide = NULL, choose = options, score = levels, tag = labels)
  if (kind != "decide" && is.null(members)) {
    .tt_usage(paste0(kind, " needs its members: options, levels, or labels"))
  }
  if (!is.null(threshold) && kind == "score") {
    .tt_usage("score takes no threshold")
  }
  .tt_built(.tt_body(kind, kinds[[kind]], members, threshold, model, sides))
}

# A question as the verbs take it: a built question stands as built, and
# text with the call's members and threshold becomes one.
.tt_settled <- function(question, kind = "decide", members = NULL, threshold = NULL,
                        sides = list()) {
  fields <- sides
  if (!is.null(threshold)) fields$threshold <- .tt_threshold_value(threshold)
  if (!is.null(members)) fields[[switch(kind, choose = "options", score = "levels", tag = "labels")]] <-
    .tt_members(members, "the members")
  .tt_settings(kind, fields)
  if (inherits(question, "thinkthen_question")) {
    if (!length(fields)) return(question)
    body <- jsonlite::fromJSON(question$json, simplifyVector = FALSE)
    if (length(intersect(names(body), names(fields))))
      .tt_usage(paste0("settings repeats `", intersect(names(body), names(fields))[[1L]], "` from the question or named arguments"))
    return(.tt_built(c(body, fields)))
  }
  .tt_built(.tt_body(kind, question, members, threshold, sides = sides))
}

# The question text rank and find read; a built question gives its own.
.tt_text <- function(question, verb) {
  if (inherits(question, "thinkthen_question")) {
    spec <- jsonlite::fromJSON(question$json, simplifyVector = FALSE)
    if (is.null(spec$decide)) .tt_usage(paste0(verb, " takes a decide question"))
    extra <- setdiff(names(spec), "decide")
    if (length(extra)) .tt_usage(paste0(verb, " takes a decide question with no ", extra[[1L]]))
    return(spec$decide)
  }
  text <- tryCatch(as.character(question), error = function(e) .tt_usage(paste0(verb, " takes one question text")))
  if (length(text) != 1L || is.na(text)) .tt_usage(paste0(verb, " takes one question text"))
  text[[1L]]
}

# A judge captures a checked question and call controls. The same bound value
# drives immediate calls and applications, so a question is checked once.
.tt_bind <- function(kind, question, members, threshold, sides, batch, context) {
  .tt_controls(batch, context)
  .tt_call_settings(kind, batch, context, NULL)
  list(kind = kind, question = .tt_settled(question, kind, members, threshold, sides),
       batch = batch, context = context)
}

.tt_execute <- function(bound, input, deadline_ms, completion) {
  .tt_call_settings(bound$kind, NULL, NULL, deadline_ms)
  question <- bound$question
  batch <- bound$batch
  context <- bound$context
  if (bound$kind == "decide") {
    input <- as.character(input)
    code <- rep(NA_integer_, length(input))
    probability <- rep(NA_real_, length(input))
    live <- !is.na(input)
    native <- .tt_call(tt_decide_column(question$json, input[live],
                                        as.integer(which(live) - 1L), deadline_ms,
                                        batch, context, completion))
    code[live] <- native$value$answer
    probability[live] <- native$value$probability
    answer <- .tt_result(ifelse(code < 0L, NA, code == 1L), native)
    answer$probability <- probability
    return(answer)
  }
  empty <- switch(bound$kind, choose = NULL, score = NA_real_, tag = character())
  held <- .tt_column(question, input, deadline_ms, empty, batch, context, completion)
  value <- switch(bound$kind,
    choose = vapply(held$value, function(one) if (is.null(one)) NA_character_ else one, character(1)),
    score = vapply(held$value, as.numeric, numeric(1)),
    tag = lapply(held$value, as.character))
  .tt_result(value, held)
}

# Omitted input constructs a function. Only deadline and completion belong to
# its application; old deadline remains caught by ... with the exact rename.
.tt_judge <- function(bound) {
  judge <- function(input, ..., deadline_ms = NULL, completion = NULL) {
    .tt_asking(completion, {
      if (length(list(...))) .tt_unknown(...)
      if (missing(input)) .tt_usage("a judge needs input")
      .tt_execute(bound, input, deadline_ms, completion)
    })
  }
  attr(judge, "thinkthen_bound") <- bound
  class(judge) <- c("thinkthen_judge", "function")
  judge
}

.tt_verb <- function(kind, question, input, members, threshold, sides, context, batch,
                     deadline_ms, completion, omitted) {
  if (omitted) {
    if (!is.null(deadline_ms) || !is.null(completion))
      .tt_usage("deadline_ms and completion belong to judge application")
    return(.tt_judge(.tt_bind(kind, question, members, threshold, sides, batch, context)))
  }
  bound <- .tt_bind(kind, question, members, threshold, sides, batch, context)
  .tt_execute(bound, input, deadline_ms, completion)
}

tt_decide <- function(question, input, threshold = NULL, true = NULL, false = NULL,
                      context = NULL, batch = NULL, ..., deadline_ms = NULL,
                      completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    sides <- list()
    if (!missing(true)) sides["true"] <- list(true)
    if (!missing(false)) sides["false"] <- list(false)
    .tt_verb("decide", question, input, NULL, threshold, sides, context, batch,
             deadline_ms, completion, missing(input))
  })
}

# The one-question column path preserves original positions and uses one
# deadline for every internal request.
.tt_column <- function(question, input, deadline_ms, empty, batch, context, completion) {
  input <- as.character(input)
  cells <- rep(list(empty), length(input))
  live <- !is.na(input)
  native <- .tt_call(tt_column(question$json, input[live],
                                as.integer(which(live) - 1L), deadline_ms,
                                batch, context, completion))
  cells[live] <- native$value$value
  answer <- .tt_result(cells, native)
  if (question$kind == "choose") {
    probability <- rep(NA_real_, length(input))
    probability[live] <- native$value$probability
    answer$probability <- probability
  }
  answer
}

tt_choose <- function(question, input, options = NULL, threshold = NULL,
                      true = NULL, false = NULL, context = NULL, batch = NULL,
                      ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    if (!missing(true)) .tt_usage("the settings key `true` does not belong to this verb")
    if (!missing(false)) .tt_usage("the settings key `false` does not belong to this verb")
    .tt_verb("choose", question, input, options, threshold, list(), context, batch,
             deadline_ms, completion, missing(input))
  })
}

tt_score <- function(question, input, levels = NULL, threshold = NULL,
                     true = NULL, false = NULL, context = NULL, batch = NULL,
                     ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    if (!missing(true)) .tt_usage("the settings key `true` does not belong to this verb")
    if (!missing(false)) .tt_usage("the settings key `false` does not belong to this verb")
    .tt_verb("score", question, input, levels, threshold, list(), context, batch,
             deadline_ms, completion, missing(input))
  })
}

tt_tag <- function(question, input, labels = NULL, threshold = NULL,
                   true = NULL, false = NULL, context = NULL, batch = NULL,
                   ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    if (!missing(true)) .tt_usage("the settings key `true` does not belong to this verb")
    if (!missing(false)) .tt_usage("the settings key `false` does not belong to this verb")
    .tt_verb("tag", question, input, labels, threshold, list(), context, batch,
             deadline_ms, completion, missing(input))
  })
}

# Preview the bound question over the same live texts execution uses. Missing
# input is a caller mistake; an explicit NULL is a valid empty column.
tt_plan <- function(judge, input, batch = NULL, context = NULL, ...) {
  if (length(list(...))) .tt_unknown(...)
  bound <- attr(judge, "thinkthen_bound", exact = TRUE)
  if (!inherits(judge, "thinkthen_judge") || !is.list(bound))
    .tt_usage("plan takes a ThinkThen judge")
  if (missing(input)) .tt_usage("plan needs input")
  batch <- batch %||% bound$batch
  context <- context %||% bound$context
  .tt_controls(batch, context)
  .tt_call_settings(bound$kind, batch, context, NULL)
  texts <- as.character(input)
  .tt_call(tt_plan_column(bound$question$json, texts[!is.na(texts)], batch, context))
}

tt_filter <- function(question, records, threshold = NULL,
                      batch = NULL, context = NULL, ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    .tt_controls(batch, context)
    question <- .tt_settled(question, threshold = threshold)
    texts <- as.character(records)
    if (anyNA(texts)) .tt_usage("filter takes no NA records; tt_decide answers NA for those rows")
    native <- .tt_call(tt_filter_places(question$json, texts, deadline_ms, batch, context, completion))
    .tt_result(records[native$value], native)
  })
}

tt_rank <- function(question, records, top = NULL,
                    batch = NULL, context = NULL, ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    .tt_controls(batch, context)
    texts <- as.character(records)
    if (anyNA(texts)) .tt_usage("rank takes no NA records")
    native <- .tt_call(tt_rank_all(.tt_text(question, "rank"), texts, deadline_ms,
                                    batch, context, completion))
    ranked <- native$value
    held <- data.frame(place = ranked$place, record = records[ranked$place],
                       probability = ranked$probability, stringsAsFactors = FALSE)
    .tt_result(if (!is.null(top)) utils::head(held, top) else held, native)
  })
}

tt_find <- function(question, units, none = FALSE, ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    texts <- as.character(units)
    if (!is.logical(none) || length(none) != 1L || is.na(none)) .tt_usage("none is TRUE or FALSE")
    native <- .tt_call(tt_find_one(.tt_text(question, "find"), texts, none, deadline_ms, completion))
    found <- native$value
    place <- if (is.null(found$place)) NA_integer_ else as.integer(found$place)
    .tt_result(list(place = place, unit = if (is.na(place)) NA_character_ else units[[place]],
                    probability = if (is.null(found$probability)) NA_real_ else found$probability), native)
  })
}

# One answer cell as its kind's bare shape.
.tt_bare <- function(cell, kind) {
  switch(kind,
    decide = if (cell < 0L) NA else cell == 1L,
    choose = if (is.null(cell)) NA_character_ else cell,
    score = cell,
    tag = as.character(cell)
  )
}

# annotate: a question set file over a data frame, a new column a question.
# Each column's type comes from its question's kind. A question that failed
# for any record widens its column to a list whose failed cells hold the
# ruled marker list(failed = list(kind, cause)), so a failure never reads
# as NA (0054).
tt_annotate <- function(file, data, on, ..., deadline_ms = NULL, batch = NULL, completion = NULL) {
  .tt_asking(completion, {
  if (length(list(...))) .tt_unknown(...)
  .tt_controls(batch, NULL)
  if (!is.data.frame(data) || !is.character(on) || length(on) != 1L ||
      is.na(on) || !on %in% names(data)) .tt_usage("annotate needs an input frame and one existing on column")
  if (!is.character(file) || length(file) != 1L || is.na(file) ||
      Encoding(file) == "bytes" || !validUTF8(file)) .tt_usage("annotate needs one question set path")
  column <- as.character(data[[on]])
  native <- .tt_call(tt_annotate_file(as.character(file), column, names(data), deadline_ms,
                                       batch, completion))
  held <- native$value
  base <- as.data.frame(data, stringsAsFactors = FALSE)
  for (at in seq_along(held$names)) {
    kind <- held$kinds[[at]]
    cells <- held$columns[[at]]
    failed <- vapply(cells, function(cell) is.list(cell) && !is.null(cell$failed), logical(1))
    bare <- lapply(seq_along(cells), function(i) if (failed[[i]]) cells[[i]] else .tt_bare(cells[[i]], kind))
    base[[held$names[[at]]]] <- if (any(failed) || kind == "tag") bare else
      unlist(bare, use.names = FALSE) %||% switch(kind, decide = logical(), choose = character(), numeric())
  }
  .tt_result(base, native)
  })
}

`%||%` <- function(one, two) if (is.null(one)) two else one

# One rule string as the file grammar's entry: "name" is any kind to any
# kind, and "name=from:to" names the ends, where "*" is any kind.
.tt_rule <- function(one, either) {
  ends <- c("*", "*")
  name <- one
  if (grepl("=", one, fixed = TRUE)) {
    halves <- strsplit(one, "=", fixed = TRUE)[[1L]]
    ends <- strsplit(halves[2L], ":", fixed = TRUE)[[1L]]
    name <- halves[[1L]]
    if (length(halves) != 2L || !nzchar(name) || length(ends) != 2L || !all(nzchar(ends))) {
      .tt_usage("a relation rule reads NAME or NAME=FROM:TO")
    }
  }
  rule <- list(name = name, source = ends[[1L]], target = ends[[2L]])
  if (either) rule$either <- TRUE
  rule
}

.tt_rules <- function(relations, either = NULL) {
  c(lapply(as.character(relations), .tt_rule, either = FALSE),
    lapply(as.character(either), .tt_rule, either = TRUE))
}

# A question file named where the kinds or rules go: "@path", or a .json path
# that exists.
.tt_path <- function(one) {
  if (length(one) != 1L || is.na(one)) return(NULL)
  if (startsWith(one, "@")) return(substring(one, 2L))
  if (grepl("\\.json$", one) && file.exists(one)) one else NULL
}

# A spec file's JSON from its section and its cuts.
.tt_spec <- function(verb, section, threshold, relation_threshold = NULL) {
  spec <- stats::setNames(list(1L, section), c("version", verb))
  if (!is.null(threshold)) spec$threshold <- as.numeric(threshold)[[1L]]
  if (!is.null(relation_threshold)) spec$relation_threshold <- as.numeric(relation_threshold)[[1L]]
  .tt_json(spec)
}

.tt_frame <- function(columns) as.data.frame(columns, stringsAsFactors = FALSE)

# recognize: every name in each text with its kind. Each record is a data
# frame of names (text, start, end, length, kind, strength), and the
# relations the rules turn on ride in its "relations" attribute.
# substr(text, start, end) is the name, and length is its character count.
# With no kinds, every name has the kind ENTITY.
tt_recognize <- function(input, kinds = NULL,
                         relations = NULL, threshold = NULL,
                         relation_threshold = NULL, ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
  if (length(list(...))) .tt_unknown(...)
  path <- .tt_path(kinds)
  spec <- path %||% {
    section <- list(kinds = stats::setNames(as.list(rep(NA, length(kinds))), as.character(kinds)))
    if (length(relations)) section$relations <- .tt_rules(relations)
    .tt_spec("recognize", section, threshold, relation_threshold)
  }
  input <- as.character(input)
  empty <- .tt_frame(list(text = character(), start = numeric(), end = numeric(),
                          length = numeric(), kind = character(), strength = numeric()))
  held <- rep(list(empty), length(input))
  live <- which(!is.na(input))
  native <- .tt_call(tt_recognize_column(spec, !is.null(path), input[live],
                                         as.integer(live - 1L), deadline_ms, completion))
  if (length(live)) {
    found <- native$value
    for (i in seq_along(live)) {
      frame <- .tt_frame(found[[i]][c("text", "start", "end", "length", "kind", "strength")])
      links <- .tt_frame(found[[i]]$relations)
      if (nrow(links)) attr(frame, "relations") <- links
      held[[live[[i]]]] <- frame
    }
  }
  .tt_result(held, native)
  })
}

# relate: the edges among entities given as a data frame with name and kind
# columns, such as tidyr::unnest() of tt_recognize. A frame with text and no
# name column is named by its text. The first two columns are the endpoints,
# so igraph::graph_from_data_frame reads it unchanged.
tt_relate <- function(entities, relations = NULL, either = NULL, threshold = NULL,
                      ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
  if (length(list(...))) .tt_unknown(...)
  path <- .tt_path(relations)
  if (is.null(path) && !length(relations) && !length(either)) {
    .tt_usage("relate needs at least one relation rule")
  }
  if (!is.data.frame(entities) || !"kind" %in% names(entities) ||
      !any(c("name", "text") %in% names(entities))) {
    .tt_usage("relate takes a data frame with name (or text) and kind columns")
  }
  spec <- path %||% .tt_spec("relate", list(relations = .tt_rules(relations, either)), threshold)
  named <- as.character(if ("name" %in% names(entities)) entities$name else entities$text)
  kinds <- as.character(entities$kind)
  if (anyNA(named) || anyNA(kinds)) .tt_usage("relate takes no NA name or kind")
  native <- .tt_call(tt_relate_frame(spec, !is.null(path), named, kinds, deadline_ms, completion))
  .tt_result(.tt_frame(native$value), native)
  })
}

# The audit view of one judgment: the command's --details document.
tt_details <- function(question, input, threshold = NULL,
                       ..., deadline_ms = NULL, completion = NULL) {
  .tt_asking(completion, {
    if (length(list(...))) .tt_unknown(...)
    question <- .tt_settled(question, threshold = threshold)
    if (!is.character(input) || length(input) != 1L || is.na(input)) {
      .tt_usage("input is one nonmissing string")
    }
    native <- .tt_call(tt_details_one(question$json, input, deadline_ms, completion))
    .tt_result(jsonlite::fromJSON(native$value, simplifyVector = FALSE), native)
  })
}

# The counters of the engine in use, as doubles.
tt_usage <- function() .tt_call(tt_usage_counters())

# The engine settings (ADR 0017 section 5). NULL keeps what the environment
# gives. The key stays on THINKTHEN_API_KEY alone.
tt_engine <- function(base_url = NULL, model = NULL, throttle = NULL, max_requests = NULL,
                      max_requests_total = NULL,
                      max_request_bytes = NULL,
                      cache = NULL, timeout = NULL, max_retries = NULL,
                      record = NULL, replay = NULL, profile = NULL, batch = NULL, ...) {
  if (length(list(...))) .tt_unknown(...)
  string <- function(x) is.null(x) || (is.character(x) && length(x) == 1L && !is.na(x) && nzchar(x))
  whole <- function(x) is.null(x) || (is.numeric(x) && is.null(attr(x, "class")) &&
    length(x) == 1L && !is.na(x) && x == round(x))
  checks <- list(base_url = string(base_url), model = string(model),
                 throttle = whole(throttle), max_requests = whole(max_requests),
                 max_requests_total = whole(max_requests_total),
                 max_request_bytes = whole(max_request_bytes),
                 cache = identical(cache, FALSE) || string(cache),
                 timeout = whole(timeout), max_retries = whole(max_retries),
                 record = string(record), replay = string(replay), profile = string(profile),
                 batch = .tt_batch_valid(batch))
  refused <- names(checks)[!unlist(checks)]
  if (length(refused)) {
    .tt_usage(paste0(refused[[1L]], " is one string, one whole number, or FALSE for cache"))
  }
  .tt_call(tt_engine_set(base_url, model, throttle, max_requests, max_requests_total,
                         max_request_bytes, cache,
                         timeout, max_retries, record, replay, profile, batch))
  invisible(NULL)
}

print.thinkthen_question <- function(x, ...) {
  members <- if (length(x$members)) paste0(" over ", length(x$members), " members") else ""
  shown <- if (is.character(x$text) && length(x$text) == 1L) x$text else "<structured text>"
  cat("<thinkthen ", x$kind, ": ", shown, members, ">\n", sep = "")
  invisible(x)
}
