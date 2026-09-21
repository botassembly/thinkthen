# The R half of the surface. The Rust half converts arguments into one
# engine call and converts the answer back; this half carries the ruled
# shape: the tt_ prefix, NA as "not sure", a column in and a column out,
# and the six error kinds as R conditions with the retry signal.

# Raise one of the six kinds as an R condition, carrying retryable. The
# Rust half packs "kind", "retryable", and "message" around \u{1f} marks;
# anything else is a plain error and passes through unchanged.
tt_raise <- function(text) {
  parts <- strsplit(text, "", fixed = TRUE)[[1]]
  if (length(parts) != 3L) {
    stop(text, call. = FALSE)
  }
  kind <- parts[[1]]
  retryable <- identical(parts[[2]], "true")
  message <- parts[[3]]
  stop(structure(
    class = c(paste0("thinkthen_", kind), "thinkthen_error", "error", "condition"),
    list(message = message, kind = kind, retryable = retryable, call = NULL)
  ))
}

# Every call runs through here: an engine failure raises as its condition,
# and an interrupt's jump leaves the token registered so on.exit can stop
# the batch.
.tt_call <- function(expr) {
  on.exit(.tt_cleanup(), add = TRUE)
  tryCatch(expr, error = function(e) tt_raise(conditionMessage(e)))
}

# The cleanup an interrupt's jump reaches: stop the call in flight so no
# new request starts. Cheap when no call is active.
.tt_cleanup <- function() {
  invisible(tt_cancel_active())
}

# The question-file's own number text: 0.9, or 0.2:0.8 for a band.
.tt_number_text <- function(x) format(x, digits = 15L, trim = TRUE)

# One threshold as the file grammar writes it: NULL, a cut as a number,
# or a band as the lo:hi string.
.tt_threshold_value <- function(threshold) {
  if (is.null(threshold)) return(NULL)
  if (!is.numeric(threshold) || anyNA(threshold) || length(threshold) == 0L ||
      length(threshold) > 2L || any(threshold < 0) || any(threshold > 1)) {
    stop("threshold must be one or two numbers in [0, 1]", call. = FALSE)
  }
  if (length(threshold) == 1L) as.numeric(threshold) else
    paste(.tt_number_text(threshold), collapse = ":")
}

# The one grammar, as JSON, from the parts a caller gives.
.tt_body <- function(kind, text, members, threshold, model) {
  body <- list(text)
  names(body) <- kind
  if (!is.null(members)) {
    held <- list(as.character(members))
    names(held) <- switch(kind, choose = "options", score = "levels", "labels")
    body <- c(body, held)
  }
  if (!is.null(threshold)) {
    held <- list(.tt_threshold_value(threshold))
    names(held) <- "threshold"
    body <- c(body, held)
  }
  if (!is.null(model)) {
    held <- list(as.character(model)[[1L]])
    names(held) <- "model"
    body <- c(body, held)
  }
  jsonlite::toJSON(body, auto_unbox = TRUE)
}

# Build a question value from parts: exactly one kind, named members for
# choose, score, and tag, and a threshold a decide question may take.
tt_question <- function(decide = NULL, choose = NULL, options = NULL,
                        score = NULL, levels = NULL, tag = NULL, labels = NULL,
                        threshold = NULL, model = NULL) {
  kinds <- list(decide = decide, choose = choose, score = score, tag = tag)
  named <- vapply(kinds, function(x) is.null(x) || is.character(x) && length(x) == 1L,
                  logical(1))
  if (sum(!vapply(kinds, is.null, logical(1))) != 1L) {
    stop("a question names exactly one kind: decide, choose, score, or tag",
         call. = FALSE)
  }
  kind <- names(which(!vapply(kinds, is.null, logical(1))))[[1L]]
  text <- kinds[[kind]]
  if (is.null(text)) stop("the question text is missing", call. = FALSE)
  members <- switch(kind,
    decide = NULL,
    choose = options,
    score = levels,
    tag = labels,
    stop("unknown kind", call. = FALSE)
  )
  if (kind != "decide" && is.null(members)) {
    stop(paste0(kind, " needs its members: options, levels, or labels"),
         call. = FALSE)
  }
  if (!is.null(threshold) && kind != "decide") {
    stop("only a decide question takes a threshold", call. = FALSE)
  }
  structure(
    .tt_call(tt_question_grammared(.tt_body(kind, text, members, threshold, model))),
    class = c("thinkthen_question", "externalptr")
  )
}

# A question as the verbs take it: text or a built question. A threshold at
# the call site applies to the text form; a built question stands as built.
.tt_settled <- function(question, threshold, kind_needed = NULL, members = NULL,
                        member_name = NULL) {
  if (inherits(question, "thinkthen_question")) {
    return(question)
  }
  text <- as.character(question)[[1L]]
  if (!is.null(kind_needed) && !is.null(members)) {
    parts <- list(text)
    names(parts) <- kind_needed
    held <- list(as.character(members))
    names(held) <- member_name
    body <- c(parts, held)
    if (kind_needed == "choose" && !is.null(threshold)) {
      cut <- list(.tt_threshold_value(threshold))
      names(cut) <- "threshold"
      body <- c(body, cut)
    }
  } else {
    body <- list(text)
    names(body) <- "decide"
    if (!is.null(threshold)) {
      held <- list(.tt_threshold_value(threshold))
      names(held) <- "threshold"
      body <- c(body, held)
    }
  }
  .tt_call(tt_question_grammared(jsonlite::toJSON(body, auto_unbox = TRUE)))
}

# decide: one evidence answers TRUE, FALSE, or NA; a column crosses once.
.tt_ints <- function(held) {
  if (is.null(held)) return(NA_integer_)
  if (!is.list(held)) held <- list(held)
  vapply(held, function(one) {
    if (is.null(one) || length(one) == 0L) NA_integer_ else as.integer(one)
  }, integer(1))
}

tt_decide <- function(question, evidence, threshold = NULL) {
  question <- .tt_settled(question, threshold)
  evidence <- as.character(evidence)
  code <- rep(NA_integer_, length(evidence))
  live <- !is.na(evidence)
  if (any(live)) {
    if (sum(live) == 1L) {
      code[live] <- .tt_ints(.tt_call(tt_decide_one(question, evidence[live]))$ans)
    } else {
      code[live] <- .tt_ints(.tt_call(tt_decide_column(question, evidence[live]))$ans)
    }
  }
  ans <- rep(NA, length(code))
  ans[!is.na(code) & code == 1L] <- TRUE
  ans[!is.na(code) & code == 0L] <- FALSE
  ans
}

# choose: one evidence answers the winning option, or NA when unsure. The
# contract carries one choose a request, so a column asks one request a
# row; the winning digest is the engine's, never the host's.
tt_choose <- function(question, evidence, options = NULL, threshold = NULL) {
  question <- .tt_settled(question, threshold, "choose", options, "options")
  unname(vapply(as.character(evidence), function(one) {
    if (is.na(one)) return(NA_character_)
    pick <- .tt_call(tt_choose_one(question, one))
    if (is.null(pick)) NA_character_ else as.character(pick)
  }, character(1)))
}

# score: one evidence answers the weighted position on the levels, with the
# nearest level's name in tt_details. A column asks one request a row.
tt_score <- function(question, evidence, levels = NULL) {
  question <- .tt_settled(question, NULL, "score", levels, "levels")
  unname(vapply(as.character(evidence), function(one) {
    if (is.na(one)) return(NA_real_)
    .tt_call(tt_score_one(question, one))$value
  }, numeric(1)))
}

# tag: one evidence answers the labels that held, in the question's order.
tt_tag <- function(question, evidence, labels = NULL) {
  question <- .tt_settled(question, NULL, "tag", labels, "labels")
  lapply(as.character(evidence), function(one) {
    if (is.na(one)) return(character())
    .tt_call(tt_tag_one(question, one))
  })
}

# filter: keep the records whose evidence reached the mark. Cut questions
# alone; the engine refuses a band with the usage kind.
tt_filter <- function(question, records, threshold = NULL) {
  question <- .tt_settled(question, threshold)
  records <- as.character(records)
  if (anyNA(records)) {
    stop("filter takes no NA records; tt_decide answers NA for those rows",
         call. = FALSE)
  }
  records[.tt_call(tt_filter_places(question, records))]
}

# rank: the records most likely yes first, ties keeping input order. The
# question names no threshold of its own; the engine refuses one that does.
tt_rank <- function(question, records, top = NULL) {
  question <- .tt_settled(question, NULL)
  records <- as.character(records)
  if (anyNA(records)) {
    stop("rank takes no NA records", call. = FALSE)
  }
  ranked <- .tt_call(tt_rank_all(question, records))
  kept <- records[ranked$place]
  if (!is.null(top)) utils::head(kept, top) else kept
}

# find: the unit that best answers the question, relative to its peers, or
# NA when the question's none arm won. Two to 255 units.
tt_find <- function(question, units) {
  question <- .tt_settled(question, NULL)
  units <- as.character(units)
  found <- .tt_call(tt_find_one(question, units))
  if (is.null(found$place)) NA_character_ else units[[found$place]]
}

# annotate: a question set file over a data frame, one request a row, a new
# column a question. Decision columns read TRUE/FALSE/NA; choose columns
# character with NA; score columns numeric; tag columns hold lists.
tt_annotate <- function(file, data, on) {
  column <- as.character(data[[on]])
  rows <- .tt_call(tt_annotate_file(as.character(file), column))
  if (!identical(length(rows), length(column))) {
    stop("annotate returned a row a record or nothing", call. = FALSE)
  }
  added <- list()
  for (name in names(rows[[1]])) {
    fields <- lapply(rows, function(row) row[[name]])
    shape <- if (is.list(fields[[1]])) {
      "score"
    } else if (is.character(fields[[1]]) && length(fields[[1]]) > 1L) {
      "tags"
    } else if (is.character(fields[[1]])) {
      "choose"
    } else {
      "decide"
    }
    added[[name]] <- switch(shape,
      score = vapply(fields, function(field) field$value, numeric(1)),
      tags = fields,
      choose = vapply(fields, function(field) {
        if (length(field) == 0L) NA_character_ else field[[1L]]
      }, character(1)),
      decide = {
        held <- vapply(fields, function(field) {
          if (length(field) == 0L) NA_integer_ else as.integer(field)
        }, integer(1))
        out <- rep(NA, length(held))
        out[!is.na(held) & held == 1L] <- TRUE
        out[!is.na(held) & held == 0L] <- FALSE
        out
      }
    )
  }
  base <- as.data.frame(data, stringsAsFactors = FALSE)
  for (name in names(added)) base[[name]] <- added[[name]]
  base
}

# The audit view of one judgment: probability, answer, model, digest, and
# the sends that produced it, so the caller sees what the bill sees.
tt_details <- function(question, evidence, threshold = NULL) {
  question <- .tt_settled(question, threshold)
  one <- as.character(evidence)[[1L]]
  held <- .tt_call(tt_details_one(question, one))
  held$answer <- if (is.null(held$answer) || length(held$answer) == 0L) NA else held$answer == 1
  held
}

# Return the text unchanged, for testing.
tt_probe <- function(body) {
  tt_probe_native(as.character(body)[[1L]])
}

# The process counters: requests count sends, so a retried send shows
# twice; cache_answers and tokens report what the engine served.
tt_usage <- function() {
  tt_usage_counters()
}

# Zero the counters.
tt_reset_usage <- function() {
  invisible(tt_reset_usage_counters())
}

print.thinkthen_question <- function(x, ...) {
  parts <- tt_question_parts(x)
  members <- if (length(parts$members)) paste0(" over ", length(parts$members), " members") else ""
  cat("<thinkthen ", parts$kind, ": ", parts$text, members, ">\n", sep = "")
  invisible(x)
}
