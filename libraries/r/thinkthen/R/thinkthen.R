# The R half of the surface. The Rust half converts arguments into one
# engine call and converts the answer back; this half carries the ruled
# shape: the tt_ prefix, NA as "not sure", a column in and a column out,
# and the six error kinds as R conditions with the retry signal.

# Every call runs through here: the engine's failure raises as its
# condition, a pending interrupt raises R's own interrupt condition, and
# the cleanup stops any call still in flight. The Rust half never lets R's
# interrupt jump cross its frames: it checks for a pending interrupt under
# a guard before entering and cancels the call's token, and reports the
# marker this function raises.
#
# The error hook ordering, fourth review: R consults options(error) at
# signal time, not at unwind time - a hook held aside for the crossing
# and restored by on.exit is restored too late, so a thinkthen error
# under a set hook skipped the hook, continued the script, and exited
# zero. The crossing therefore captures the mapped condition as data,
# restores the hook while the stack is quiet, and only then stops: the
# hook runs for a thinkthen error exactly as it runs for any other.
.tt_call <- function(expr) {
  on.exit(.tt_cleanup(), add = TRUE)
  # R's own interrupt signaling runs the user's options(error = ...) hook
  # while the guarded check below consumes the interrupt - the third
  # review's probe caught the hook firing on a plain Ctrl-C. The hook is
  # held aside for the crossing itself and restored below, before any
  # error is raised, so it fires for genuinely uncaught errors and never
  # for an interrupt.
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  if (tt_interrupt_pending()) .tt_interrupt()
  held <- tryCatch(list(value = expr), error = function(e) list(cond = .tt_error_condition(e)))
  # Restore the hook while nothing is raising: the stop below meets the
  # same options(error) a plain stop would.
  if (!is.null(had_hook)) options(error = had_hook)
  if (is.null(held$cond)) {
    if (tt_interrupt_pending()) .tt_interrupt()
    return(held$value)
  }
  stop(held$cond)
}

# The condition one engine error maps to, without stopping, returned as
# data so .tt_call can restore the error hook before the raise. An interrupt marker is delivered for real
# instead, from inside the handler where cleanup has already run.
.tt_error_condition <- function(e) {
  text <- conditionMessage(e)
  parts <- strsplit(text, "\u{1f}", fixed = TRUE)[[1]]
  if (length(parts) == 3L && identical(parts[[1]], "interrupt")) {
    .tt_interrupt()
  }
  .tt_condition(text)
}

# One of the six kinds as an R condition, carrying retryable. The Rust
# half packs "kind", "retryable", and "message" around \u{1f} marks and
# escapes any mark inside the message; anything else is a plain error.
.tt_condition <- function(text) {
  parts <- strsplit(text, "\u{1f}", fixed = TRUE)[[1]]
  if (length(parts) != 3L) {
    return(simpleError(text))
  }
  kind <- parts[[1]]
  retryable <- identical(parts[[2]], "true")
  message <- parts[[3]]
  structure(
    class = c(paste0("thinkthen_", kind), "thinkthen_error", "error", "condition"),
    list(message = message, kind = kind, retryable = retryable, call = NULL)
  )
}

# R's own interrupt, delivered for real. The guarded check that reported
# it consumed the pending signal, so the process sends itself the real
# SIGINT and waits: the jump lands inside R's own delivery machinery,
# caught by `tryCatch(interrupt = ...)` exactly as a genuine Ctrl-C is,
# and uncaught it halts without the error hook - the third review's
# probe. No Rust frame sits under that jump: the old fallback called the
# extendr-wrapped tt_raise_interrupt, whose R_CheckUserInterrupt longjmp
# crossed the wrapper's own Rust frame - the fourth review's finding -
# and it is deleted. If the real signal does not land within the sleep,
# control falls through to the synthetic condition below, with the error
# hook held aside so an interrupt never fires it; the call still stops
# rather than silently continuing (surfaces-review-5: an early return
# made this fallback unreachable).
.tt_interrupt <- function() {
  delivered <- FALSE
  if (.Platform$OS.type == "unix" && requireNamespace("tools", quietly = TRUE)) {
    try(delivered <- tools::pskill(Sys.getpid(), 2L), silent = TRUE)
    if (isTRUE(delivered)) {
      # The signal's flag is set; Sys.sleep is an R checkpoint, and the
      # real jump lands inside it if it lands at all - pure R frames.
      Sys.sleep(0.1)
    }
  }
  had_hook <- getOption("error")
  if (!is.null(had_hook)) {
    options(error = NULL)
    on.exit(options(error = had_hook), add = TRUE)
  }
  stop(structure(
    class = c("interrupt", "condition"),
    list(message = "", call = NULL)
  ))
}

# The cleanup an interrupt's jump reaches: stop the call in flight so no
# new request starts. Cheap when no call is active.
.tt_cleanup <- function() {
  invisible(tt_cancel_active())
}

# The one JSON writer for the grammar's text: every string reaches
# jsonlite as UTF-8 under any locale (surfaces-review-5: under LC_ALL=C a
# native-marked question text went out as "caf<c3><a9>"). The rule is the
# Rust half's own: native bytes that are valid UTF-8 are UTF-8, latin1
# converts, and native bytes that are not valid UTF-8 refuse by name.
.tt_json <- function(value) {
  utf8 <- function(text) {
    if (!is.character(text) || !length(text)) return(text)
    native <- Encoding(text) == "unknown" & !is.na(text)
    if (any(native & !validUTF8(text))) {
      stop(.tt_condition(paste("usage", "false",
        "a text carries native-marked bytes that are not valid UTF-8 under this locale; convert it with enc2utf8() or iconv() first",
        sep = "\u{1f}")))
    }
    Encoding(text)[native] <- "UTF-8"
    enc2utf8(text)
  }
  held <- rapply(list(value), utf8, classes = "ANY", how = "replace")[[1L]]
  jsonlite::toJSON(held, auto_unbox = TRUE)
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
  .tt_json(body)
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
  .tt_call(tt_question_grammared(.tt_json(body)))
}

# decide: one evidence answers TRUE, FALSE, or NA; a column crosses once.
.tt_ints <- function(held) {
  if (is.null(held)) return(NA_integer_)
  if (!is.list(held)) held <- list(held)
  vapply(held, function(one) {
    if (is.null(one) || length(one) == 0L) NA_integer_ else as.integer(one)
  }, integer(1))
}

tt_decide <- function(question, evidence, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold)
  evidence <- as.character(evidence)
  code <- rep(NA_integer_, length(evidence))
  live <- !is.na(evidence)
  if (any(live)) {
    if (sum(live) == 1L) {
      code[live] <- .tt_ints(.tt_call(tt_decide_one(question, evidence[live], deadline))$ans)
    } else {
      code[live] <- .tt_ints(.tt_call(tt_decide_column(question, evidence[live], deadline))$ans)
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
tt_choose <- function(question, evidence, options = NULL, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold, "choose", options, "options")
  unname(vapply(as.character(evidence), function(one) {
    if (is.na(one)) return(NA_character_)
    pick <- .tt_call(tt_choose_one(question, one, deadline))
    if (is.null(pick)) NA_character_ else as.character(pick)
  }, character(1)))
}

# score: one evidence answers the weighted position on the levels, with the
# nearest level's name in tt_details. A column asks one request a row.
tt_score <- function(question, evidence, levels = NULL, deadline = NULL) {
  question <- .tt_settled(question, NULL, "score", levels, "levels")
  unname(vapply(as.character(evidence), function(one) {
    if (is.na(one)) return(NA_real_)
    .tt_call(tt_score_one(question, one, deadline))$value
  }, numeric(1)))
}

# tag: one evidence answers the labels that held, in the question's order.
tt_tag <- function(question, evidence, labels = NULL, deadline = NULL) {
  question <- .tt_settled(question, NULL, "tag", labels, "labels")
  lapply(as.character(evidence), function(one) {
    if (is.na(one)) return(character())
    .tt_call(tt_tag_one(question, one, deadline))
  })
}

# filter: keep the records whose evidence reached the mark. Cut questions
# alone; the engine refuses a band with the usage kind.
tt_filter <- function(question, records, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold)
  records <- as.character(records)
  if (anyNA(records)) {
    stop("filter takes no NA records; tt_decide answers NA for those rows",
         call. = FALSE)
  }
  records[.tt_call(tt_filter_places(question, records, deadline))]
}

# rank: the ruled pair (settled 2026-09-21) as a data frame — the record's
# place in the input, the record, and the probability the backend gave it —
# most likely yes first, ties keeping input order. The question names no
# threshold of its own; the engine refuses one that does.
tt_rank <- function(question, records, top = NULL, deadline = NULL) {
  question <- .tt_settled(question, NULL)
  records <- as.character(records)
  if (anyNA(records)) {
    stop("rank takes no NA records", call. = FALSE)
  }
  ranked <- .tt_call(tt_rank_all(question, records, deadline))
  held <- data.frame(
    place = ranked$place,
    record = records[ranked$place],
    probability = ranked$prob,
    stringsAsFactors = FALSE
  )
  if (!is.null(top)) utils::head(held, top) else held
}

# find: the ruled pair (settled 2026-09-21) — the winning unit's place, the
# unit itself, and its probability. `place` and `unit` are NA when the
# question's none arm won. Two to 255 units.
tt_find <- function(question, units, deadline = NULL) {
  question <- .tt_settled(question, NULL)
  units <- as.character(units)
  found <- .tt_call(tt_find_one(question, units, deadline))
  place <- if (is.null(found$place)) NA_integer_ else as.integer(found$place)
  list(
    place = place,
    unit = if (is.na(place)) NA_character_ else units[[place]],
    probability = found$prob
  )
}

# annotate: a question set file over a data frame, one request a row, a new
# column a question. Each column's type comes from its question's kind,
# never from the first answer's shape: decision columns read TRUE/FALSE/NA,
# choose columns character with NA, score columns numeric, tag columns hold
# lists of their labels. A question that failed for any record widens its
# column to a list whose cells are the bare answer or the ruled marker
# `list(failed = ...)`, so a failure never reads as NA (0054).
.tt_marker <- function(field) {
  is.list(field) && !is.null(field$failed)
}

# One answer cell as its kind's bare shape: a decision TRUE/FALSE/NA, a
# choice its own string or NA, a score its number, a tag its labels.
.tt_bare <- function(field, kind) {
  switch(kind,
    decide = if (length(field) == 0L) NA else field == 1L,
    choose = if (length(field) == 0L) NA_character_ else as.character(field)[[1L]],
    score = if (is.null(field)) NA_real_ else field$value,
    tag = as.character(field),
    stop("the question set carries a kind this surface does not know", call. = FALSE)
  )
}

# One answer column, typed by its question's kind.
.tt_answer_column <- function(fields, kind) {
  switch(kind,
    decide = vapply(fields, .tt_bare, logical(1), kind = "decide"),
    choose = vapply(fields, .tt_bare, character(1), kind = "choose"),
    score = vapply(fields, .tt_bare, numeric(1), kind = "score"),
    tag = lapply(fields, .tt_bare, kind = "tag"),
    stop("the question set carries a kind this surface does not know", call. = FALSE)
  )
}

tt_annotate <- function(file, data, on, deadline = NULL) {
  column <- as.character(data[[on]])
  kinds <- .tt_call(tt_annotate_kinds(as.character(file)))
  # The input's columns are preserved, so a question that would land on
  # one of their names refuses before any request is paid: the third
  # review's probe showed the on column itself overwritten otherwise.
  clashes <- intersect(kinds$names, names(data))
  if (length(clashes) > 0L) {
    stop(sprintf(
      paste0("annotate cannot add a question named '%s': the input already ",
              "has a column by that name; rename one"),
      clashes[[1L]]), call. = FALSE)
  }
  rows <- .tt_call(tt_annotate_file(as.character(file), column, deadline))
  if (!identical(length(rows), length(column))) {
    stop("annotate returned a row a record or nothing", call. = FALSE)
  }
  held <- as.list(kinds$kinds)
  names(held) <- kinds$names
  column_names <- if (length(rows)) names(rows[[1]]) else kinds$names
  added <- list()
  for (name in column_names) {
    kind <- held[[name]]
    if (is.null(kind)) {
      stop("annotate returned a column the question set does not name", call. = FALSE)
    }
    fields <- lapply(rows, function(row) row[[name]])
    if (any(vapply(fields, .tt_marker, logical(1)))) {
      # The widened column: good answers keep their kind's bare shape, the
      # failed cells carry the marker list.
      added[[name]] <- lapply(fields, function(field) {
        if (.tt_marker(field)) return(field)
        .tt_bare(field, kind)
      })
      next
    }
    added[[name]] <- .tt_answer_column(fields, kind)
  }
  base <- as.data.frame(data, stringsAsFactors = FALSE)
  for (name in names(added)) base[[name]] <- added[[name]]
  base
}

# One rule string as the file grammar's entry: "name" means any kind to
# any kind, "name=from:to" names the ends, and "*" is the any kind.
.tt_rule_entry <- function(one, either) {
  one <- as.character(one)[[1L]]
  name <- one
  source <- "*"
  target <- "*"
  if (grepl("=", one, fixed = TRUE)) {
    halves <- strsplit(one, "=", fixed = TRUE)[[1L]]
    if (length(halves) != 2L || !nzchar(halves[[1L]])) {
      stop("a relation rule reads NAME or NAME=FROM:TO", call. = FALSE)
    }
    name <- halves[[1L]]
    ends <- strsplit(halves[[2L]], ":", fixed = TRUE)[[1L]]
    if (length(ends) != 2L || !nzchar(ends[[1L]]) || !nzchar(ends[[2L]])) {
      stop("a relation rule's ends read FROM:TO; a missing end is a usage error",
           call. = FALSE)
    }
    source <- ends[[1L]]
    target <- ends[[2L]]
  }
  entry <- list(name = name, source = source, target = target)
  if (either) entry$either <- TRUE
  entry
}

# The rule entries from the vectors a caller gives.
.tt_rules <- function(relations, either = NULL) {
  entries <- list()
  if (length(relations)) {
    entries <- c(entries, lapply(as.character(relations), .tt_rule_entry, either = FALSE))
  }
  if (length(either)) {
    entries <- c(entries, lapply(as.character(either), .tt_rule_entry, either = TRUE))
  }
  entries
}

# Is this one string a question file rather than a single kind or rule?
# The command's own "@file.json" spelling, or a path that exists.
.tt_is_file <- function(one) {
  length(one) == 1L && (startsWith(one, "@") ||
    (grepl("\\.json$", one) && file.exists(one)))
}

# The named section of a question file, as the file grammar reads it.
.tt_section <- function(path_like, section_name) {
  path <- sub("^@", "", path_like)
  if (!file.exists(path)) {
    stop(paste0("no question file at ", path), call. = FALSE)
  }
  parsed <- jsonlite::fromJSON(path, simplifyVector = FALSE)
  section <- parsed[[section_name]]
  if (is.null(section)) {
    stop(paste0("the question file carries no ", section_name, " section"), call. = FALSE)
  }
  section
}

# The recognize section's JSON from the parts a caller gives. A question
# file named where the kinds go carries the whole spec; explicit parts
# override the file's.
.tt_recognize_spec <- function(kinds, relations, threshold, relation_threshold) {
  if (.tt_is_file(kinds)) {
    spec <- .tt_section(kinds, "recognize")
  } else {
    spec <- list(kinds = I(as.character(kinds)))
  }
  rules <- .tt_rules(relations)
  if (length(rules)) spec$relations <- rules
  if (!is.null(threshold)) spec$threshold <- as.numeric(threshold)[[1L]]
  if (!is.null(relation_threshold)) {
    spec$relation_threshold <- as.numeric(relation_threshold)[[1L]]
  }
  .tt_json(spec)
}

# The relate section's JSON from the parts a caller gives.
.tt_relate_spec <- function(relations, either, kind_field, threshold) {
  if (.tt_is_file(relations)) {
    spec <- .tt_section(relations, "relate")
  } else {
    spec <- list()
    rules <- .tt_rules(relations)
    if (length(rules)) spec$relations <- rules
  }
  extra <- .tt_rules(NULL, either)
  if (length(extra)) {
    held <- if (is.null(spec$relations)) list() else spec$relations
    spec$relations <- c(held, extra)
  }
  if (!is.null(kind_field)) spec$kind_field <- as.character(kind_field)[[1L]]
  if (!is.null(threshold)) spec$threshold <- as.numeric(threshold)[[1L]]
  .tt_json(spec)
}

# recognize: every name in each text with its kind, and the relations the
# rules turn on. A column crosses once; each record comes back as a data
# frame of names (text, kind, start, end, strength) with the relations in
# the frame's "relations" attribute, so tidyr::unnest() makes one row per
# name. Offsets are R's own: substr(text, start, end) is the name.
# "Strength" is the tool's own computed number: the least of the word
# probabilities times the mean of the kind probabilities.
tt_recognize <- function(evidence, kinds = c("person", "organization", "place"),
                         relations = NULL, threshold = NULL,
                         relation_threshold = NULL, deadline = NULL) {
  spec <- .tt_recognize_spec(kinds, relations, threshold, relation_threshold)
  ask <- .tt_call(tt_recognize_grammared(spec))
  evidence <- as.character(evidence)
  held <- vector("list", length(evidence))
  live <- which(!is.na(evidence))
  if (length(live)) {
    found <- .tt_call(tt_recognize_column(ask, evidence[live], deadline))
    for (i in seq_along(live)) {
      one <- found[[i]]
      frame <- as.data.frame(one[c("text", "kind", "start", "end", "strength")],
                             stringsAsFactors = FALSE)
      if (!is.null(one$relations)) {
        attr(frame, "relations") <- as.data.frame(one$relations, stringsAsFactors = FALSE)
      }
      held[[live[[i]]]] <- frame
    }
  }
  for (i in which(is.na(evidence))) {
    held[[i]] <- as.data.frame(
      list(text = character(), kind = character(), start = integer(),
           end = integer(), strength = numeric()),
      stringsAsFactors = FALSE)
  }
  held
}

# relate: the edges between records, as a data frame ready for
# igraph::graph_from_data_frame. Every record crosses at once; more than
# 255 records is a usage error before anything else.
tt_relate <- function(records, relations = NULL, either = NULL,
                      kind_field = NULL, threshold = NULL, deadline = NULL) {
  if (!length(relations) && !length(either) && !.tt_is_file(relations)) {
    stop("relate needs at least one relation rule", call. = FALSE)
  }
  spec <- .tt_relate_spec(relations, either, kind_field, threshold)
  ask <- .tt_call(tt_relate_grammared(spec))
  records <- as.character(records)
  if (anyNA(records)) {
    stop("relate takes no NA records; tt_recognize answers NA for those rows",
         call. = FALSE)
  }
  held <- .tt_call(tt_relate_records(ask, records, deadline))
  as.data.frame(held, stringsAsFactors = FALSE)
}

# The audit view of one judgment: probability, answer, model, digest, the
# sends that produced it, the logical requests' digests, the
# failed-question count, and the nearest level's name on a score question
# (NULL on every other verb).
tt_details <- function(question, evidence, threshold = NULL, deadline = NULL) {
  question <- .tt_settled(question, threshold)
  one <- as.character(evidence)[[1L]]
  held <- .tt_call(tt_details_one(question, one, deadline))
  held$answer <- if (is.null(held$answer) || length(held$answer) == 0L) NA else held$answer == 1
  held
}

# The process counters: requests count sends, so a retried send shows
# twice; cache_answers and tokens report what the engine served.
tt_usage <- function() {
  tt_usage_counters()
}

print.thinkthen_question <- function(x, ...) {
  parts <- tt_question_parts(x)
  members <- if (length(parts$members)) paste0(" over ", length(parts$members), " members") else ""
  cat("<thinkthen ", parts$kind, ": ", parts$text, members, ">\n", sep = "")
  invisible(x)
}
