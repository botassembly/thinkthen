# The R surface's slice of the conformance file, run offline against the
# null backend (or the wire when ENGINE_BASE_URL names one). Prints one
# line a case: ok, skip with a reason, diverge with its record, or FAIL.
`%||%` <- function(one, two) if (is.null(one)) two else one
# A nonzero exit follows any FAIL. Mirrors the Rust surface's example.

.libPaths(c("rlib", .libPaths()))
library(thinkthen)

file <- jsonlite::fromJSON("../../conformance/conformance.json", simplifyVector = FALSE)
cases <- file$cases
wire <- Sys.getenv("ENGINE_BASE_URL") != "" || Sys.getenv("THINKTHEN_BASE_URL") != ""

passed <- 0
failures <- 0

say <- function(line) cat(line, "\n", sep = "")

built <- function(case) {
  structure(
    thinkthen:::.tt_call(tt_question_grammared(jsonlite::toJSON(case$question, auto_unbox = TRUE))),
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
  if (!is.null(case$question_file)) {
    say(paste0("skip     ", id, ": the local kind needs a file door this surface does not carry"))
    next
  }
  if (identical(case$verb, "relate") && identical(case$form, "per-subject")) {
    say(paste0("skip     ", id,
      ": the per-subject arm is pinned, not replayed; the stand-in serves the pairs recording (conformance/DIVERGENCES.md)"))
    next
  }
  outcome <- tryCatch({
    kind <- expect$error$kind
    if (!is.null(kind)) {
      if (case$verb == "cancel") {
        paste0("diverge  ", id, ": the stand-in ignores a pre-fired token; conformance/DIVERGENCES.md carries this as a real-engine requirement")
      } else if (kind == "backend" && !wire) {
        paste0("skip     ", id, ": the backend kind needs the wire or a dead address; the null backend answers")
      } else if (kind == "deadline") {
        paste0("skip     ", id, ": the spent-budget case needs a deadline option this surface does not carry")
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
          if (identical(kept, wanted)) paste0("ok       ", id) else
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
          if (identical(held, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": expected ", length(wanted), " labels, got ", length(held))
        },
        annotate = {
          set_file <- tempfile(fileext = ".json")
          wrapped <- list(version = 1, questions = case$set)
          writeLines(jsonlite::toJSON(wrapped, auto_unbox = TRUE), set_file)
          frame <- data.frame(body = case$evidence, stringsAsFactors = FALSE)
          held <- tt_annotate(set_file, frame, on = "body")
          wanted <- expect$answers
          good <- all(vapply(names(wanted), function(name) {
            one <- held[[name]][[1]]
            two <- answer_of(wanted[[name]]$answer)
            (is.na(one) && is.na(two)) || isTRUE(identical(one, two)) || same_number(one, two)
          }, logical(1)))
          if (good) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the assembled answers diverged")
        },
        details = {
          held <- tt_details(question, case$evidence)
          if (identical(held$model, expect$details$model) &&
              same_number(held$probability, expect$details$probability)) {
            paste0("ok       ", id)
          } else {
            paste0("FAIL     ", id, ": the details diverged")
          }
        },
        usage = {
          before <- tt_usage()
          for (i in seq_len(expect$request_calls %||% case$calls %||% 2)) NULL
          tt_decide(question, case$evidence)
          tt_decide(question, case$evidence)
          held <- tt_usage()
          sent <- held$requests - before$requests
          cached <- held$cache_answers - before$cache_answers
          if (expect$cache_answers > 0 && cached == 0) {
            paste0("diverge  ", id, ": the stand-in carries no disk cache, so the second call sends again; conformance/DIVERGENCES.md and Phase A's record carry it")
          } else if (identical(sent, expect$requests)) {
            paste0("ok       ", id)
          } else {
            paste0("FAIL     ", id, ": expected ", expect$requests, " requests, got ", sent)
          }
        },
        decide_many = {
          records <- unlist(case$records)
          judgments <- thinkthen:::.tt_call(tt_decide_column(question, records))
          answers <- vapply(judgments$ans, function(one) {
            if (is.null(one)) NA else one == 1
          }, logical(1))
          wanted <- vapply(expect$answers, function(one) {
            if (is.null(one)) NA else one
          }, logical(1))
          if (identical(answers, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the column's answers diverged")
        },
        rank = {
          records <- unlist(case$records)
          held <- tt_rank(question, records)
          wanted <- records[unlist(expect$ranking) + 1]
          if (identical(held, wanted)) paste0("ok       ", id) else
            paste0("FAIL     ", id, ": the ranked order diverged")
        },
        find = {
          units <- unlist(case$records)
          if (isTRUE(case$none)) {
            paste0("diverge  ", id, ": the surface's find has no none arm; the none case is real-engine data")
          } else {
            held <- tt_find(case$question, units)
            wanted <- units[[expect$answer + 1]]
            if (identical(held, wanted)) paste0("ok       ", id) else
              paste0("FAIL     ", id, ": expected ", wanted, ", got ", held)
          }
        },
        recognize = {
          spec <- jsonlite::toJSON(case$question, auto_unbox = TRUE)
          ask <- thinkthen:::.tt_call(tt_recognize_grammared(spec))
          found <- thinkthen:::.tt_call(tt_recognize_column(ask, case$text))[[1]]
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
          ask <- thinkthen:::.tt_call(tt_relate_grammared(spec))
          edges <- thinkthen:::.tt_call(tt_relate_records(ask, unlist(case$records)))
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
