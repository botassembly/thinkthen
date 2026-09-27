# The shared cases of conformance/cases.json through the R verbs. Each case
# runs in its own child on the 0092 case arm with a fresh cache. Every
# expected request digest names the canonical URL, so the child recomputes
# it for the URL the backend served. The runner reports each case as pass,
# fail, or not run with its reason, and the three counts sum to the file's
# count. Run through tests/with-backend.sh.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
home <- normalizePath(file.path(Sys.getenv("TT_TESTS"), "..", "..", ".."))
document <- jsonlite::fromJSON(file.path(home, "conformance", "cases.json"), simplifyVector = FALSE)
canonical <- "https://api.typesafe.ai/v1/systemone"
`%||%` <- function(one, two) if (is.null(one)) two else one

# The cases no R call can carry, each with the reason.
unreachable <- c(
  "20-usage-fault" = "an injection point inside the engine, which no R call reaches",
  "22-local-fault" = "an injection point inside the engine, which no R call reaches",
  "23-cancelled-fault" = "R raises its own interrupt; tests/interrupt.R proves the batch stop",
  "25-defect-fault" = "an injection point inside the engine, which no R call reaches",
  "30-local-question-file" = "the R verbs take no decide question file"
)

sha <- function(url, request) {
  file <- tempfile()
  writeBin(charToRaw(enc2utf8(paste0("systemone\n", url, "\n", request))), file)
  strsplit(system2("env", c(clean_env(), "sha256sum", shQuote(file)), stdout = TRUE), " ")[[1]][[1]]
}

# A value with names sorted and numbers as doubles, so two readings compare.
norm <- function(value) {
  if (!length(value)) return(NULL)
  if (is.list(value) && is.null(names(value)) && all(vapply(value, function(one) is.atomic(one) && length(one) == 1L, TRUE))) {
    value <- unlist(value)
  }
  if (is.list(value)) {
    if (!is.null(names(value))) value <- value[order(names(value))]
    return(lapply(value, norm))
  }
  if (is.numeric(value)) as.double(value) else value
}
same <- function(what, got, want) {
  if (!isTRUE(all.equal(norm(got), norm(want), tolerance = 1e-9))) {
    stop(sprintf("%s: got %s, want %s", what, jsonlite::toJSON(got, auto_unbox = TRUE, null = "null"),
                 jsonlite::toJSON(want, auto_unbox = TRUE, null = "null")), call. = FALSE)
  }
}
bare <- function(value) if (length(value) == 1L && is.na(value)) NULL else value

# One case in this child: the checks, then its send count.
run_case <- function(case, served) {
  exchanges <- case$exchanges
  renamed <- list()
  for (exchange in exchanges) renamed[[sha(canonical, exchange$request)]] <- sha(served, exchange$request)
  swap <- function(value) {
    if (is.list(value)) return(lapply(value, swap))
    if (is.character(value) && length(value) == 1L && !is.null(renamed[[value]])) renamed[[value]] else value
  }
  answers <- swap(case$expect$success$answers)
  evidence <- vapply(exchanges, function(one) one$evidence %||% "", character(1))
  meta <- function(row, want) {
    same("question_sha256", row$meta$question_sha256, want$details$question_sha256)
    same("model", row$meta$model, want$details$model)
    same("requests", row$meta$requests, want$details$requests)
    if (case$expect$success$kind != "single") return(invisible())  # run facts ride on single cases only
    field <- function(from, name) if (name %in% names(from)) from[[name]] else "absent"  # NULL is JSON null too
    for (name in c("usage", "requests_sent", "cached")) same(name, field(row$meta, name), field(want$details, name))
    same("confidence", field(row$answer, "confidence"), field(want$details$answer, "confidence"))
    same("url", row$meta$url, served)
  }
  detailed <- function(question) {
    for (want in answers) {
      row <- tt_details(question, evidence[[want$exchange + 1L]])
      same("answer", row$answer, want$details$answer)
      same("value", row$value, want$bare)
      meta(row, want)
    }
  }
  file <- tempfile(fileext = ".json")
  asked <- case$question %||% case$question_set
  writeLines(jsonlite::toJSON(asked, auto_unbox = TRUE, digits = NA), file)
  if (!is.null(case$expect$error)) return(refused(case))
  switch(case$verb,
    decide = , choose = , score = , tag = {
      question <- thinkthen:::.tt_built(asked)
      detailed(question)
      got <- switch(case$verb, decide = tt_decide(question, evidence), choose = tt_choose(question, evidence),
                    score = tt_score(question, evidence), tag = tt_tag(question, evidence))
      for (want in answers) same("bare", bare(got[[want$exchange + 1L]]), want$bare)
      counters <- case$expect$success$counters
      if (!is.null(counters)) {
        carried <<- tt_usage()$requests_sent
        folder <- tempfile("counters")
        dir.create(folder)
        tt_engine(cache = folder)
        before <- tt_usage()
        for (i in seq_len(counters$calls)) tt_decide(question, evidence)
        after <- tt_usage()
        same("requests", after$requests_sent - before$requests_sent, counters$requests)
        same("cache answers", after$cache_answers - before$cache_answers, counters$cache_answers)
      }
    },
    filter = {
      question <- thinkthen:::.tt_built(asked)
      kept <- unlist(case$expect$success$operation$indexes) + 1L
      same("kept", tt_filter(question, evidence), evidence[kept])
      if (length(answers)) detailed(question)
    },
    rank = {
      ranking <- case$expect$success$operation$ranking
      ranked <- tt_rank(asked$decide, evidence)
      same("places", ranked$place, vapply(ranking, function(at) at$index + 1L, integer(1)))
      same("probabilities", ranked$probability, vapply(ranking, function(at) at$probability, numeric(1)))
    },
    find = {
      operation <- case$expect$success$operation
      found <- tt_find(asked$find, unlist(asked$units), none = asked$none)
      picked <- Filter(function(row) identical(row$index, operation$selected), operation$probabilities)
      same("found", found, if (is.null(operation$selected)) list(place = NA_integer_, unit = NA_character_, probability = NA_real_)
           else list(place = operation$selected + 1L, unit = asked$units[[operation$selected + 1L]], probability = picked[[1]]$probability))
    },
    annotate = {
      records <- if (is.null(case$record)) evidence else as.character(jsonlite::toJSON(case$record, auto_unbox = TRUE))
      frame <- data.frame(input = records, stringsAsFactors = FALSE)
      got <- tt_annotate(file, frame, on = "input")
      for (want in answers) {
        cell <- got[[want$name]][[if (is.null(case$record)) want$exchange + 1L else 1L]]
        same(want$name, if (is.list(cell)) cell else bare(cell), want$bare)
      }
    },
    recognize = {
      found <- tt_recognize(case$text, paste0("@", file))[[1]]
      want <- answers[[1]]$bare
      same("texts", found$text, vapply(want$entities, `[[`, "", "text"))
      same("starts", found$start, vapply(want$entities, function(one) one$start + 1, 0))
      same("ends", found$end, vapply(want$entities, function(one) as.double(one$end), 0))
      same("lengths", found$length, vapply(want$entities, function(one) as.double(one$length), 0))
      same("kinds", found$kind, vapply(want$entities, `[[`, "", "kind"))
      same("strengths", found$strength, vapply(want$entities, function(one) as.double(one$strength), 0))
      relations <- attr(found, "relations")
      same("relations", nrow(relations) %||% 0L, length(want$relations))
      for (at in seq_along(want$relations)) {
        one <- want$relations[[at]]
        same("relation", unlist(relations[at, c("relation", "source", "target")], use.names = FALSE),
             c(one$relation, one$source$text, one$target$text))
        same("probability", relations$probability[[at]], one$probability)
      }
    },
    relate = {
      entities <- data.frame(name = vapply(case$entities, `[[`, "", "name"),
                             kind = vapply(case$entities, `[[`, "", "kind"), stringsAsFactors = FALSE)
      edges <- tt_relate(entities, relations = paste0("@", file))
      want <- answers[[1]]$bare
      key <- function(relation, source, target) paste(relation, source, target, sep = "|")
      got <- stats::setNames(edges$probability, key(edges$relation, edges$source, edges$target))
      expected <- vapply(want, function(one) one$probability, 0)
      names(expected) <- vapply(want, function(one) key(one$relation, one$source$name, one$target$name), "")
      same("edges", got[order(names(got))], expected[order(names(expected))])
    }
  )
}

# An error case: the kind the R call raises.
refused <- function(case) {
  question <- function() thinkthen:::.tt_built(case$question)
  kind <- switch(case$id,
    "21-backend-fault" = kind_of(tt_decide(question(), "any text")),
    "24-deadline-fault" = kind_of(tt_decide(question(), "any text", deadline = 0)),
    "29-usage-json-text" = kind_of(tt_decide(question(), case$evidence)),
    "31-usage-rank-blank-question" = kind_of(tt_rank(case$question$decide, strsplit(case$evidence, "\n")[[1]])),
    stop("no R call for this error case", call. = FALSE))
  same("kind", kind, case$expect$error$kind)
}

one <- Sys.getenv("TT_CASE")
carried <- 0
if (nzchar(one)) {
  case <- Filter(function(held) identical(held$id, one), document$cases)[[1]]
  said <- tryCatch({ run_case(case, paste0(Sys.getenv("THINKTHEN_BASE_URL"), "/systemone")); "pass" },
                   error = function(e) paste("fail:", conditionMessage(e)))
  cat("RESULT", said, "\n")
  cat("SENT", carried + tt_usage()$requests_sent, "\n")
  quit(save = "no")
}

counts <- c(pass = 0L, fail = 0L, "not run" = 0L)
sent <- 0L
for (case in document$cases) {
  id <- case$id
  if (!is.na(unreachable[id])) {
    cat("not run ", id, ": ", unreachable[[id]], "\n", sep = "")
    counts[["not run"]] <- counts[["not run"]] + 1L
    next
  }
  path <- if (identical(id, "21-backend-fault")) "arm/refuse/v1" else paste0("case/", id, "/v1")
  held <- child(sprintf('source("%s")', file.path(Sys.getenv("TT_TESTS"), "conformance.R")),
                env = c(paste0("TT_CASE=", id), paste0("THINKTHEN_BASE_URL=", arm(path))))$text
  said <- sub("^.*RESULT (.*?) *\n.*$", "\\1", held, perl = TRUE)
  if (!grepl("RESULT", held, fixed = TRUE)) said <- paste("fail:", held)
  sent <- sent + as.integer(sub("^.*SENT ([0-9]+).*$", "\\1", held))[!is.na(held) & grepl("SENT", held)]
  verdict <- if (identical(said, "pass")) "pass" else "fail"
  counts[[verdict]] <- counts[[verdict]] + 1L
  cat(verdict, " ", id, if (verdict == "fail") paste0(": ", sub("^fail: ", "", said)), "\n", sep = "")
}
cat(sprintf("conformance: %d pass, %d fail, %d not run, of %d cases\n",
            counts[["pass"]], counts[["fail"]], counts[["not run"]], document$case_count))
check("the three counts sum to the file's count", sum(counts) == document$case_count && length(document$cases) == document$case_count)
check("every run case passes", counts[["fail"]] == 0L)
finish("conformance", sent)
