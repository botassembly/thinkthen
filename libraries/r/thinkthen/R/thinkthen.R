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
  if (is.null(held$cond) && is.null(held$interrupt) && tt_interrupt_pending()) {
    held <- list(interrupt = TRUE)
  }
  if (!is.null(had_hook)) options(error = had_hook)
  if (!is.null(held$interrupt)) .tt_interrupt()
  if (!is.null(held$cond)) stop(held$cond)
  held$value
}

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
  as.character(jsonlite::toJSON(held, auto_unbox = TRUE, digits = NA, na = "null"))
}

# One threshold as the file grammar writes it: a cut, or a band as "lo:hi".
.tt_threshold_value <- function(threshold) {
  if (!is.numeric(threshold) || anyNA(threshold) || !length(threshold) ||
      length(threshold) > 2L || any(threshold < 0) || any(threshold > 1)) {
    .tt_usage("threshold must be one or two numbers in [0, 1]")
  }
  if (length(threshold) == 1L) as.numeric(threshold) else
    paste(format(threshold, digits = 15L, trim = TRUE), collapse = ":")
}

# The question file's object from its parts. Members stay a JSON array even
# when there is one (R6-10).
.tt_body <- function(kind, text, members = NULL, threshold = NULL, model = NULL) {
  body <- stats::setNames(list(text), kind)
  if (!is.null(members)) {
    body[[switch(kind, choose = "options", score = "levels", "labels")]] <- I(as.character(members))
  }
  if (!is.null(threshold)) body$threshold <- .tt_threshold_value(threshold)
  if (!is.null(model)) body$model <- as.character(model)[[1L]]
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
                        threshold = NULL, model = NULL) {
  kinds <- list(decide = decide, choose = choose, score = score, tag = tag)
  given <- !vapply(kinds, is.null, logical(1))
  if (sum(given) != 1L) {
    .tt_usage("a question names exactly one kind: decide, choose, score, or tag")
  }
  kind <- names(kinds)[given]
  members <- switch(kind, decide = NULL, choose = options, score = levels, tag = labels)
  if (kind != "decide" && is.null(members)) {
    .tt_usage(paste0(kind, " needs its members: options, levels, or labels"))
  }
  if (!is.null(threshold) && kind != "decide") {
    .tt_usage("only a decide question takes a threshold")
  }
  .tt_built(.tt_body(kind, as.character(kinds[[kind]])[[1L]], members, threshold, model))
}

# A question as the verbs take it: a built question stands as built, and
# text with the call's members and threshold becomes one.
.tt_settled <- function(question, kind = "decide", members = NULL, threshold = NULL) {
  if (inherits(question, "thinkthen_question")) return(question)
  .tt_built(.tt_body(kind, as.character(question)[[1L]], members, threshold))
}

# The question text rank and find read; a built question gives its own.
.tt_text <- function(question) {
  if (inherits(question, "thinkthen_question")) question$text else as.character(question)[[1L]]
}

tt_decide <- function(question, evidence, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold = threshold)
  evidence <- as.character(evidence)
  code <- rep(NA_integer_, length(evidence))
  live <- !is.na(evidence)
  if (any(live)) {
    code[live] <- .tt_call(tt_decide_column(question$json, evidence[live], deadline))$answer
  }
  ifelse(code < 0L, NA, code == 1L)
}

# choose, score, and tag over a column: one annotate of a one-question set
# (ticket 0095). NA evidence answers NA, or no labels, with no request.
.tt_column <- function(question, evidence, deadline, empty) {
  evidence <- as.character(evidence)
  cells <- rep(list(empty), length(evidence))
  live <- !is.na(evidence)
  if (any(live)) cells[live] <- .tt_call(tt_column(question$json, evidence[live], deadline))
  cells
}

tt_choose <- function(question, evidence, options = NULL, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, "choose", options, threshold)
  cells <- .tt_column(question, evidence, deadline, NULL)
  vapply(cells, function(one) if (is.null(one)) NA_character_ else one, character(1))
}

tt_score <- function(question, evidence, levels = NULL, deadline = NULL) {
  question <- .tt_settled(question, "score", levels)
  vapply(.tt_column(question, evidence, deadline, NA_real_), as.numeric, numeric(1))
}

tt_tag <- function(question, evidence, labels = NULL, deadline = NULL) {
  question <- .tt_settled(question, "tag", labels)
  lapply(.tt_column(question, evidence, deadline, character()), as.character)
}

tt_filter <- function(question, records, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold = threshold)
  records <- as.character(records)
  if (anyNA(records)) {
    .tt_usage("filter takes no NA records; tt_decide answers NA for those rows")
  }
  records[.tt_call(tt_filter_places(question$json, records, deadline))]
}

tt_rank <- function(question, records, top = NULL, deadline = NULL) {
  records <- as.character(records)
  if (anyNA(records)) .tt_usage("rank takes no NA records")
  ranked <- .tt_call(tt_rank_all(.tt_text(question), records, deadline))
  held <- data.frame(place = ranked$place, record = records[ranked$place],
                     probability = ranked$probability, stringsAsFactors = FALSE)
  if (!is.null(top)) utils::head(held, top) else held
}

tt_find <- function(question, units, none = FALSE, deadline = NULL) {
  units <- as.character(units)
  if (!is.logical(none) || length(none) != 1L || is.na(none)) .tt_usage("none is TRUE or FALSE")
  found <- .tt_call(tt_find_one(.tt_text(question), units, none, deadline))
  place <- if (is.null(found$place)) NA_integer_ else as.integer(found$place)
  list(place = place, unit = if (is.na(place)) NA_character_ else units[[place]],
       probability = if (is.null(found$probability)) NA_real_ else found$probability)
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
tt_annotate <- function(file, data, on, deadline = NULL) {
  column <- as.character(data[[on]])
  held <- .tt_call(tt_annotate_file(as.character(file), column, names(data), deadline))
  base <- as.data.frame(data, stringsAsFactors = FALSE)
  for (at in seq_along(held$names)) {
    kind <- held$kinds[[at]]
    cells <- held$columns[[at]]
    failed <- vapply(cells, function(cell) is.list(cell) && !is.null(cell$failed), logical(1))
    bare <- lapply(seq_along(cells), function(i) if (failed[[i]]) cells[[i]] else .tt_bare(cells[[i]], kind))
    base[[held$names[[at]]]] <- if (any(failed) || kind == "tag") bare else
      unlist(bare, use.names = FALSE) %||% switch(kind, decide = logical(), choose = character(), numeric())
  }
  base
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
# frame of names (name, kind, start, end, strength), and the relations the
# rules turn on ride in its "relations" attribute. substr(text, start, end)
# is the name.
tt_recognize <- function(evidence, kinds = c("person", "organization", "place"),
                         relations = NULL, threshold = NULL,
                         relation_threshold = NULL, deadline = NULL) {
  path <- .tt_path(kinds)
  spec <- path %||% {
    section <- list(kinds = stats::setNames(as.list(rep(NA, length(kinds))), as.character(kinds)))
    if (length(relations)) section$relations <- .tt_rules(relations)
    .tt_spec("recognize", section, threshold, relation_threshold)
  }
  evidence <- as.character(evidence)
  empty <- .tt_frame(list(name = character(), kind = character(), start = numeric(),
                          end = numeric(), strength = numeric()))
  held <- rep(list(empty), length(evidence))
  live <- which(!is.na(evidence))
  if (length(live)) {
    found <- .tt_call(tt_recognize_column(spec, !is.null(path), evidence[live], deadline))
    for (i in seq_along(live)) {
      frame <- .tt_frame(found[[i]][c("name", "kind", "start", "end", "strength")])
      links <- .tt_frame(found[[i]]$relations)
      if (nrow(links)) attr(frame, "relations") <- links
      held[[live[[i]]]] <- frame
    }
  }
  held
}

# relate: the edges among entities given as a data frame with name and kind
# columns, such as tidyr::unnest() of tt_recognize. The first two columns
# are the endpoints, so igraph::graph_from_data_frame reads it unchanged.
tt_relate <- function(entities, relations = NULL, either = NULL, threshold = NULL,
                      deadline = NULL) {
  path <- .tt_path(relations)
  if (is.null(path) && !length(relations) && !length(either)) {
    .tt_usage("relate needs at least one relation rule")
  }
  if (!is.data.frame(entities) || !all(c("name", "kind") %in% names(entities))) {
    .tt_usage("relate takes a data frame with name and kind columns")
  }
  spec <- path %||% .tt_spec("relate", list(relations = .tt_rules(relations, either)), threshold)
  named <- as.character(entities$name)
  kinds <- as.character(entities$kind)
  if (anyNA(named) || anyNA(kinds)) .tt_usage("relate takes no NA name or kind")
  .tt_frame(.tt_call(tt_relate_frame(spec, !is.null(path), named, kinds, deadline)))
}

# The audit view of one judgment: the command's --details document.
tt_details <- function(question, evidence, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold = threshold)
  json <- .tt_call(tt_details_one(question$json, as.character(evidence)[[1L]], deadline))
  jsonlite::fromJSON(json, simplifyVector = FALSE)
}

# The counters of the engine in use, as doubles.
tt_usage <- function() .tt_call(tt_usage_counters())

# The engine settings (ADR 0017 section 5). NULL keeps what the environment
# gives. The key stays on THINKTHEN_API_KEY alone.
tt_engine <- function(base_url = NULL, model = NULL, throttle = NULL, max_requests = NULL,
                      cache = NULL, cache_bytes = NULL) {
  string <- function(x) is.null(x) || (is.character(x) && length(x) == 1L && !is.na(x) && nzchar(x))
  whole <- function(x) is.null(x) || (is.numeric(x) && is.null(attr(x, "class")) &&
    length(x) == 1L && !is.na(x) && x == round(x))
  checks <- list(base_url = string(base_url), model = string(model),
                 throttle = whole(throttle), max_requests = whole(max_requests),
                 cache = identical(cache, FALSE) || string(cache),
                 cache_bytes = whole(cache_bytes))
  refused <- names(checks)[!unlist(checks)]
  if (length(refused)) {
    .tt_usage(paste0(refused[[1L]], " is one string, one whole number, or FALSE for cache"))
  }
  .tt_call(tt_engine_set(base_url, model, throttle, max_requests, cache, cache_bytes))
  invisible(NULL)
}

print.thinkthen_question <- function(x, ...) {
  members <- if (length(x$members)) paste0(" over ", length(x$members), " members") else ""
  cat("<thinkthen ", x$kind, ": ", x$text, members, ">\n", sep = "")
  invisible(x)
}
