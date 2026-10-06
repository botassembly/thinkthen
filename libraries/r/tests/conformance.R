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
all_ids <- vapply(document$cases, function(case) case$id, character(1))
if (length(all_ids) != document$case_count || anyDuplicated(all_ids)) stop("shared case IDs or count disagree", call. = FALSE)
selector <- Sys.getenv("THINKTHEN_CONFORMANCE_IDS", unset = NA_character_)
selected_ids <- all_ids
if (!is.na(selector)) {
  if (!startsWith(selector, "/")) stop("THINKTHEN_CONFORMANCE_IDS takes an absolute path", call. = FALSE)
  chosen <- trimws(readLines(selector, warn = FALSE))
  chosen <- chosen[nzchar(chosen) & !startsWith(chosen, "#")]
  if (!length(chosen)) stop("the selected case list is empty", call. = FALSE)
  if (anyDuplicated(chosen)) stop("duplicate selected case ID", call. = FALSE)
  absent <- setdiff(chosen, all_ids)
  if (length(absent)) stop(paste("selected case is absent from the shared corpus:", absent[[1]]), call. = FALSE)
  selected_ids <- chosen
}

# The cases no R call can carry, each with the reason.
unreachable <- c(
  "20-usage-fault" = "an injection point inside the engine, which no R call reaches",
  "22-local-fault" = "an injection point inside the engine, which no R call reaches",
  "23-cancelled-fault" = "R raises its own interrupt; tests/interrupt.R proves the batch stop",
  "25-defect-fault" = "an injection point inside the engine, which no R call reaches",
  "18-annotate-two-groups" = "one recorded request per group; ADR 0111 section 5 packs a record's groups into one"
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
  legacy_batch_one <- case$id %in% c("13-filter-records", "14-filter-none",
    "15-rank-records", "16-rank-stable-tie", "27-decide-many",
    "28-decide-many-repeated-texts", "34-annotate-repeated-texts",
    "35-annotate-score-repeated-texts")
  tt_engine(batch = if (legacy_batch_one) 1L else "max")
  exchanges <- case$exchanges
  renamed <- list()
  # Every row lists question keys by ADR 0111, so each digest becomes the
  # keys of the request it named.
  for (exchange in exchanges) {
    renamed[[sha(canonical, exchange$request)]] <- as.list(question_keys(served, exchange$request))
  }
  swap <- function(value) {
    if (is.list(value)) {
      swapped <- lapply(value, swap)
      # A list of digests becomes the flat list of their keys.
      if (is.null(names(value)) && length(value) && all(vapply(value, is.character, TRUE))) {
        return(do.call(c, lapply(swapped, as.list)))
      }
      return(swapped)
    }
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
      row <- tt_details(question, evidence[[want$exchange + 1L]])$value
      same("answer", row$answer, want$details$answer)
      same("value", row$value, want$bare)
      meta(row, want)
    }
  }
  file <- tempfile(fileext = ".json")
  asked <- case$question %||% case$question_set
  writeLines(jsonlite::toJSON(asked, auto_unbox = TRUE, digits = NA), file)
  if (!is.null(case$expect$error)) return(refused(case, file))
  switch(case$verb,
    decide = , choose = , score = , tag = {
      question <- thinkthen:::.tt_built(asked)
      detailed(question)
      got <- switch(case$verb, decide = tt_decide(question, evidence)$value, choose = tt_choose(question, evidence)$value,
                    score = tt_score(question, evidence)$value, tag = tt_tag(question, evidence)$value)
      for (want in answers) same("bare", bare(got[[want$exchange + 1L]]), want$bare)
      counters <- case$expect$success$counters
      if (!is.null(counters)) {
        # The value checks warmed this session's cache. Count in a fresh child;
        # tt_engine intentionally refuses replacing a session's chosen settings.
        held <- child(c(
          sprintf("tt_engine(batch = %s)", if (legacy_batch_one) "1L" else '"max"'),
          sprintf("question <- tt_question(file = %s)", encodeString(file, quote = '"')),
          sprintf("for (i in seq_len(%dL)) invisible(tt_decide(question, %s))",
                  counters$calls, paste(deparse(evidence), collapse = "\n")),
          'cat("COUNTERS ", jsonlite::toJSON(tt_usage(), auto_unbox = TRUE), "\\n", sep = "")'
        ))
        lines <- grep("^COUNTERS ", strsplit(held$text, "\n")[[1L]], value = TRUE)
        check("the counter child completed", held$status == 0L && length(lines) == 1L)
        after <- jsonlite::fromJSON(sub("^COUNTERS ", "", lines[[1L]]))
        carried <<- after$requests_sent
        same("requests", after$requests_sent, counters$requests)
        same("cache answers", after$cache_answers, counters$cache_answers)
      }
    },
    filter = {
      question <- thinkthen:::.tt_built(asked)
      kept <- unlist(case$expect$success$operation$indexes) + 1L
      same("kept", tt_filter(question, evidence)$value, evidence[kept])
      if (length(answers)) detailed(question)
    },
    rank = {
      ranking <- case$expect$success$operation$ranking
      call <- tt_rank(asked$decide, evidence)
      ranked <- call$value
      same("ranked records", ranked$record, evidence[vapply(ranking, function(at) at$index + 1L, integer(1))])
      same("original observation indexes", sort(unique(vapply(call$details, `[[`, 0, "index"))), seq_along(evidence) - 1L)
      same("places", ranked$place, vapply(ranking, function(at) at$index + 1L, integer(1)))
      same("probabilities", ranked$probability, vapply(ranking, function(at) at$probability, numeric(1)))
    },
    find = {
      operation <- case$expect$success$operation
      call <- tt_find(asked$find, unlist(asked$units), none = asked$none)
      found <- call$value
      if (case$id == "18-find-second") {
        same("saved canonical selected index", operation$selected, 1L)
        same("R selected original position", found$place, 2L)
        same("canonical selected candidate", call$details[[1]]$answer, "u002")
      }
      picked <- Filter(function(row) identical(row$index, operation$selected), operation$probabilities)
      same("found", found, if (is.null(operation$selected)) list(place = NA_integer_, unit = NA_character_, probability = NA_real_)
           else list(place = operation$selected + 1L, unit = asked$units[[operation$selected + 1L]], probability = picked[[1]]$probability))
    },
    annotate = {
      records <- if (is.null(case$record)) evidence else as.character(jsonlite::toJSON(case$record, auto_unbox = TRUE))
      frame <- data.frame(input = records, stringsAsFactors = FALSE)
      got <- tt_annotate(file, frame, on = "input")$value
      for (want in answers) {
        cell <- got[[want$name]][[if (is.null(case$record)) want$exchange + 1L else 1L]]
        same(want$name, if (is.list(cell)) cell else bare(cell), want$bare)
      }
    },
    recognize = {
      found <- tt_recognize(case$text, paste0("@", file))$value[[1]]
      want <- answers[[1]]$bare
      same("texts", found$text, vapply(want$entities, `[[`, "", "text"))
      same("starts", found$start, vapply(want$entities, function(one) one$start + 1, 0))
      same("ends", found$end, vapply(want$entities, function(one) as.double(one$end), 0))
      same("lengths", found$length, vapply(want$entities, function(one) as.double(one$length), 0))
      same("kinds", found$kind, vapply(want$entities, `[[`, "", "kind"))
      same("strengths", found$strength, vapply(want$entities, function(one) as.double(one$strength), 0))
      if (case$id == "41-offsets-past-an-accent-and-an-emoji") {
        path <- tempfile(fileext = ".txt")
        writeBin(charToRaw(enc2utf8(case$text)), path)
        before <- tt_usage()$requests_sent
        located <- tt_files(case$question, path, unit = "file")
        sends <- tt_usage()$requests_sent - before
        same("located recognition reuses native cached answers", sends, 0L)
        spans <- located$value[[1]]$value$entities
        same("located native starts", lapply(spans, `[[`, "start"), lapply(want$entities, `[[`, "start"))
        same("located native ends", lapply(spans, `[[`, "end"), lapply(want$entities, `[[`, "end"))
        for (span in spans) {
          same("physical span lines", c(span$first_line, span$last_line), c(1L, 1L))
          same("native scalar slice", substr(case$text, span$start + 1L, span$end), span$text)
        }
        same("multibyte R span", c(found$start, found$end, found$length), c(11, 20, 10))
        same("multibyte native span", c(spans[[1]]$start, spans[[1]]$end), c(10, 20))
        same("prefix UTF-8 bytes differ from scalar offset", nchar(substr(case$text, 1, 10), type = "bytes"), 14L)
        unlink(path)
      }
      relations <- attr(found, "relations")
      same("relations", nrow(relations) %||% 0L, length(want$relations))
      for (at in seq_along(want$relations)) {
        one <- want$relations[[at]]
        same("relation", unlist(relations[at, c("relation", "source", "target")], use.names = FALSE),
             c(one$relation, one$source$text, one$target$text))
        same("probability", relations$probability[[at]], one$probability)
        same("either", relations$either[[at]], isTRUE(one$either))
      }
    },
    relate = {
      entities <- data.frame(name = vapply(case$entities, `[[`, "", "name"),
                             kind = vapply(case$entities, `[[`, "", "kind"), stringsAsFactors = FALSE)
      edges <- tt_relate(entities, relations = paste0("@", file))$value
      want <- answers[[1]]$bare
      key <- function(relation, source, target, either) paste(relation, source, target, either, sep = "|")
      got <- stats::setNames(edges$probability, key(edges$relation, edges$source, edges$target, edges$either))
      expected <- vapply(want, function(one) one$probability, 0)
      names(expected) <- vapply(want, function(one) key(one$relation, one$source$name, one$target$name, isTRUE(one$either)), "")
      same("edges", got[order(names(got))], expected[order(names(expected))])
    }
  )
}

# An error case: the kind the R call raises.
refused <- function(case, file) {
  question <- function() thinkthen:::.tt_built(case$question)
  kind <- switch(case$id,
    "21-backend-fault" = kind_of(tt_decide(question(), "any text")$value),
    "24-deadline-fault" = kind_of(tt_decide(question(), "any text", deadline_ms = 0)$value),
    "29-usage-json-text" = kind_of(tt_decide(question(), case$evidence)$value),
    "30-local-question-file" = {
      before <- tt_usage()$requests_sent
      error <- tryCatch(tt_question(file = file), thinkthen_error = function(e) e)
      check("a bad named question is a non-retryable local error",
            inherits(error, "thinkthen_local") && isFALSE(error$retryable))
      same("question-file sends", tt_usage()$requests_sent - before, 0L)
      error$kind
    },
    "31-usage-rank-blank-question" = kind_of(tt_rank(case$question$decide, strsplit(case$evidence, "\n")[[1]])$value),
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
  if (!(id %in% selected_ids)) next
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
cat(sprintf("conformance: total=%d selected=%d pass=%d fail=%d not_run=%d unselected=%d\n",
            document$case_count, length(selected_ids), counts[["pass"]], counts[["fail"]],
            counts[["not run"]], document$case_count - length(selected_ids)))
check("the three counts sum to the selected cases", sum(counts) == length(selected_ids))
check("every run case passes", counts[["fail"]] == 0L)
finish("conformance", sent)
