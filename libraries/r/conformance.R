# The R surface's slice of the conformance file, run offline against the
# null backend (or the wire when ENGINE_BASE_URL names one). Prints one
# line a case: ok, skip with a reason, diverge with its record, or FAIL.
`%||%` <- function(one, two) if (is.null(one)) two else one
# A nonzero exit follows any FAIL. Mirrors the Rust surface's example.

.libPaths(c("rlib", .libPaths()))
library(thinkthen)

# Case 74 pins the stand-in's one synthesized partial failure (0054), a
# compile-time build door since review 1: this run needs a package
# installed with THINKTHEN_R_SYNTHETIC_PARTIAL=1 (check.sh installs one
# and restores the production shape afterwards).

file <- jsonlite::fromJSON("../../conformance/conformance.json", simplifyVector = FALSE)
cases <- file$cases
skips <- file$skips %||% list()
wire <- Sys.getenv("ENGINE_BASE_URL") != "" || Sys.getenv("THINKTHEN_BASE_URL") != ""

# The shared skip table's entry for this case on this surface, or NULL.
# First match wins; entries naming a surface apply only there.
central_skip <- function(surface, case, wire) {
  kind <- case$expect$error$kind
  for (entry in skips) {
    if (!is.null(entry$surfaces) && !(surface %in% unlist(entry$surfaces))) next
    if (identical(entry$unless, "wire") && wire) next
    when <- entry$when
    if (!is.null(when$id) && !identical(case$id, when$id)) next
    if (!is.null(when$verb)) {
      listed <- unlist(when$verb)
      if (!(case$verb %in% listed)) next
    }
    if (!is.null(when$kind) && !identical(kind, when$kind)) next
    if (!is.null(when$form) && !identical(case$form, when$form)) next
    if (!is.null(when$none) && !identical(isTRUE(case$none), when$none)) next
    if (!is.null(when$error) && !identical(!is.null(case$expect$error), when$error)) next
    if (identical(when$record, "null")) {
      has_null <- any(vapply(case$records %||% list(), is.null, logical(1)))
      if (!has_null) next
    }
    return(list(disposition = entry$as %||% "skip", why = entry$why))
  }
  NULL
}

passed <- 0
failures <- 0

say <- function(line) cat(line, "\n", sep = "")

built <- function(case) {
  structure(
    thinkthen:::.tt_call(thinkthen:::tt_question_grammared(jsonlite::toJSON(case$question, auto_unbox = TRUE))),
    class = c("thinkthen_question", "externalptr")
  )
}

answer_of <- function(held) {
  if (is.null(held)) NA else held
}

same_number <- function(one, two) isTRUE(all.equal(one, two, tolerance = 1e-9))

for (case in cases) {
  id <- case$id
  expect <- case$expect
  central <- central_skip("r", case, wire)
  if (!is.null(central)) {
    say(paste0(central$disposition, "  ", id, ": ", central$why))
    next
  }
  outcome <- tryCatch({
    kind <- expect$error$kind
    if (!is.null(kind)) {
      if (kind == "deadline") {
        held <- tryCatch(
          tt_decide(built(case), case$evidence, deadline = 0),
          thinkthen_error = function(e) e$kind
        )
        if (identical(held, kind)) paste0("ok       ", id) else
          paste0("FAIL     ", id, ": expected the ", kind, " kind, got ", held)
      } else {
        run <- function() {
          question <- built(case)
          switch(case$verb,
            decide = tt_decide(question, case$evidence),
            filter = tt_filter(question, unlist(case$records)),
            choose = tt_choose(question, case$evidence),
            rank = tt_rank(question, unlist(case$records)),
            NULL
          )
        }
        held <- tryCatch(run(), thinkthen_error = function(e) e$kind)
        if (identical(held, kind)) paste0("ok       ", id) else
          paste0("FAIL     ", id, ": expected the ", kind, " kind, got ", held)
      }
    } else {
      question <- if (case$verb %in% c("find", "recognize", "relate")) NULL else built(case)
      switch(case$verb,
        decide = {
          held <- tt_details(question, case$evidence)
          wanted <- answer_of(expect$answer)
          if (identical(held$answer, wanted) && same_number(held$probability, expect$details$probability)) {
            paste0("ok       ", id)
          } else {
            paste0("FAIL     ", id, ": expected ", wanted, " at ", expect$details$probability,
                   ", got ", held$answer, " at ", held$probability)
          }
        },
        filter = {
          records <- unlist(case$records)
          kept <- tt_filter(question, records)
          wanted <- if (length(expect$indexes)) records[unlist(expect$indexes) + 1] else character()
          rows <- if (is.null(expect$rows)) TRUE else {
            # The ruled record row (go-ahead item 4): this host's own
            # list pair, `input` and `value`, in input order.
            want <- lapply(expect$rows, function(row) list(input = row$input, value = row$value))
            got <- lapply(kept, function(record) list(input = record, value = TRUE))
            identical(got, want)
          }
          if (identical(kept, wanted) && rows) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", length(wanted), " kept, got ", length(kept))
        },
        choose = {
          picked <- tt_choose(question, case$evidence)
          wanted <- answer_of(expect$answer)
          same <- (is.na(picked) && is.na(wanted)) || identical(picked, wanted)
          if (same) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", wanted, ", got ", picked)
        },
        score = {
          scored <- tt_score(question, case$evidence)
          if (same_number(scored, expect$answer)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", expect$answer, ", got ", scored)
        },
        tag = {
          held <- tt_tag(question, case$evidence)[[1]]
          wanted <- unlist(expect$answer)
          # An empty answer is a zero-length character here, not NULL.
          if (is.null(wanted)) wanted <- character()
          if (is.null(held)) held <- character()
          if (identical(held, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", length(wanted), " labels, got ", length(held))
        },
        annotate = {
          set_file <- tempfile(fileext = ".json")
          wrapped <- list(version = 1, questions = case$set)
          writeLines(jsonlite::toJSON(wrapped, auto_unbox = TRUE), set_file)
          held_records <- if (is.null(case$records)) case$evidence else unlist(case$records)
          frame <- data.frame(body = held_records, stringsAsFactors = FALSE)
          held <- tt_annotate(set_file, frame, on = "body")
          if (!is.null(expect$rows)) {
            # The multi-record form: one answer a record, in input order,
            # each field the bare answer (a score is its position).
            wanted <- expect$rows
            good <- nrow(held) == length(wanted)
            if (good) {
              for (at in seq_along(wanted)) {
                row <- wanted[[at]]
                for (name in names(row$value)) {
                  value <- row$value[[name]]
                  field <- held[[name]][[at]]
                  good <- good && if (is.numeric(value)) same_number(field, value) else identical(field, value)
                  if (!good) break
                }
                if (!good) break
              }
            }
            if (good) paste0("ok       ", id) else
              paste0("FAIL     ", id, ": the per-record answers diverged")
          } else {
            wanted <- expect$answers
            good <- all(vapply(names(wanted), function(name) {
              one <- held[[name]][[1]]
              if (!is.null(wanted[[name]]$failed)) {
                # The ruled marker (0054), in this host's own spelling.
                return(identical(one, list(failed = wanted[[name]]$failed)))
              }
              two <- answer_of(wanted[[name]]$answer)
              (is.na(one) && is.na(two)) || isTRUE(identical(one, two)) || same_number(one, two)
            }, logical(1)))
            if (good && !is.null(expect$failed_questions)) {
              counted <- sum(vapply(held, function(column) {
                is.list(column) && any(vapply(column, function(cell) {
                  is.list(cell) && !is.null(cell$failed)
                }, logical(1)))
              }, logical(1)))
              good <- isTRUE(counted == expect$failed_questions)
            }
            if (good) paste0("ok       ", id) else
              paste0("FAIL     ", id, ": the assembled answers diverged")
          }
        },
        details = {
          held <- tt_details(question, case$evidence)
          # The audit's identity fields and the two 0053/0054 additions;
          # the recorded probability is not compared because the null
          # backend's own rule cannot reproduce case 73's recorded number.
          good <- identical(held$model, expect$details$model) &&
            identical(held$digest, expect$details$question_sha256)
          if (good && !is.null(expect$details$requests)) {
            good <- identical(as.character(held$requests), as.character(unlist(expect$details$requests)))
          }
          if (good && !is.null(expect$details$failed_questions)) {
            good <- identical(as.integer(held$failed_questions), as.integer(expect$details$failed_questions))
          }
          if (good) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the details diverged")
        },
        decide_many = {
          records <- unlist(case$records)
          judgments <- thinkthen:::.tt_call(thinkthen:::tt_decide_column(question, records, NULL))
          answers <- vapply(judgments$ans, function(one) {
            if (is.null(one)) NA else one == 1
          }, logical(1))
          wanted <- vapply(expect$answers, function(one) {
            if (is.null(one)) NA else one
          }, logical(1))
          rows <- if (is.null(expect$rows)) TRUE else {
            want <- lapply(expect$rows, function(row) list(input = row$input, value = row$value))
            got <- Map(function(record, value) list(input = record, value = value),
                       records, as.list(answers))
            identical(unname(got), want)
          }
          if (identical(answers, wanted) && rows) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the column's answers diverged")
        },
        rank = {
          records <- unlist(case$records)
          held <- tt_rank(question, records)
          wanted <- records[unlist(expect$ranking) + 1]
          if (identical(held$record, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the ranked order diverged")
        },
        find = {
          units <- unlist(case$records)
          held <- tt_find(case$question, units)
          wanted <- units[[expect$answer + 1]]
          if (identical(held$unit, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", wanted, ", got ", held$unit)
        },
        recognize = {
          spec <- jsonlite::toJSON(case$question, auto_unbox = TRUE)
          ask <- thinkthen:::.tt_call(thinkthen:::tt_recognize_grammared(spec))
          found <- thinkthen:::.tt_call(thinkthen:::tt_recognize_column(ask, case$text, NULL))[[1]]
          ents <- expect$entities
          good <- identical(length(found$text), length(ents))
          if (good && length(ents)) {
            for (j in seq_along(ents)) {
              e <- ents[[j]]
              good <- good &&
                identical(found$text[[j]], e$text) &&
                identical(found$kind[[j]], e$kind) &&
                identical(as.integer(found$start[[j]]), as.integer(e$start) + 1L) &&
                identical(as.integer(found$end[[j]]), as.integer(e$end)) &&
                same_number(found$strength[[j]], e$strength) &&
                identical(substr(case$text, as.integer(e$start) + 1L, as.integer(e$end)), e$text)
              if (!good) break
            }
          }
          rels <- expect$relations
          if (good) {
            held_rels <- found$relations
            if (length(rels)) {
              good <- !is.null(held_rels) && identical(length(held_rels$name), length(rels))
              if (good) {
                for (j in seq_along(rels)) {
                  e <- rels[[j]]
                  good <- good &&
                    identical(held_rels$name[[j]], e$name) &&
                    identical(as.integer(held_rels$source[[j]]), as.integer(e$source)) &&
                    identical(as.integer(held_rels$target[[j]]), as.integer(e$target)) &&
                    same_number(held_rels$probability[[j]], e$probability)
                  if (!good) break
                }
              }
            } else {
              good <- is.null(held_rels) || length(held_rels$name) == 0L
            }
          }
          if (good) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the recognized shape diverged")
        },
        relate = {
          spec <- jsonlite::toJSON(case$question, auto_unbox = TRUE)
          ask <- thinkthen:::.tt_call(thinkthen:::tt_relate_grammared(spec))
          edges <- thinkthen:::.tt_call(thinkthen:::tt_relate_records(ask, unlist(case$records), NULL))
          want <- expect$edges
          good <- identical(length(edges$name), length(want))
          if (good && length(want)) {
            for (j in seq_along(want)) {
              e <- want[[j]]
              good <- good &&
                identical(edges$name[[j]], e$name) &&
                identical(as.integer(edges$source[[j]]), as.integer(e$source)) &&
                identical(as.integer(edges$target[[j]]), as.integer(e$target)) &&
                same_number(edges$probability[[j]], e$probability)
              if (!good) break
            }
          }
          if (good) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the edges diverged")
        },
        paste0("skip     ", id, ": the ", case$verb, " case is not one this runner expresses")
      )
    }
  }, error = function(e) paste0("FAIL     ", id, ": ", conditionMessage(e)))

  if (grepl("^ok", outcome)) passed <- passed + 1
  if (grepl("^FAIL", outcome)) failures <- failures + 1
  say(outcome)
}

if (failures > 0) {
  stop(paste(failures, "conformance case(s) failed"), call. = FALSE)
}
say(paste("conformance slice green for the R surface"))
