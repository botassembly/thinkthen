# Backend selection crosses R only as a name. All sends stay on loopback.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
rows <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "../../../conformance/binding-backends.json"))$backends
rows <- rows[rows$name == Sys.getenv("TT_NAMED_BACKEND"), ]
stopifnot(nrow(rows) == 1L)
slots <- c("generic_systemone", "generic_decisions", "generic_custom", "capture_systemone",
           "capture_decisions", "capture_custom", "other", "non_post")
snapshot <- function(command) {
  output <- Sys.getenv("TT_BACKEND_OUT")
  before <- length(grep("^\\{", readLines(output)))
  backend_say(command)
  until <- Sys.time() + 30
  repeat {
    lines <- grep("^\\{", readLines(output), value = TRUE)
    if (length(lines) > before) return(jsonlite::fromJSON(tail(lines, 1L)))
    if (Sys.time() >= until) stop("backend did not report the requested counts")
    Sys.sleep(0.02)
  }
}
expected <- setNames(rep(0L, length(slots)), slots)
for (i in seq_len(nrow(rows))) {
  row <- rows[i, ]
  out <- child(c(
    sprintf('tt_engine(backend = "%s", base_url = "%s", cache = FALSE)', row$name, arm("arm/full/capture/v1")),
    sprintf('Sys.setenv(%s = "fake-named-r-later")', row$key),
    'q <- tt_question(decide = "Does it need attention?", true = list(what = "yes", examples = list("refund")))',
    'stopifnot(isTRUE(tt_decide(q, "refund")$value))',
    sprintf('stopifnot(is.null(tt_engine(backend = "%s", base_url = "%s", cache = FALSE)))', row$name, arm("arm/full/capture/v1")),
    'message <- tryCatch(tt_engine(backend = "nowhere"), thinkthen_usage = function(e) conditionMessage(e))',
    sprintf('stopifnot(grepl(\'backend = "%s"\', message, fixed = TRUE), grepl("start a new R session", message, fixed = TRUE))', row$name),
    'cat("named backend accepted")'
  ), env = paste0(row$key, "=fake-named-r-captured"))
  check(paste(row$name, "answers and retains session settings"), out$status == 0L && grepl("named backend accepted", out$text, fixed = TRUE))
  check(paste(row$name, "keeps markers out of output"), !grepl("fake-named-r", out$text, fixed = TRUE))
  expected[[row$path]] <- expected[[row$path]] + 1L
  paths <- snapshot("paths")
  check(paste(row$name, "uses its posting path"), identical(unlist(paths[slots]), expected) && identical(paths$overflow, FALSE))
  check(paste(row$name, "sends once"), backend_count() == i)
  bearers <- snapshot("bearers")
  check(paste(row$name, "captures its named key"), bearers$markers$captured == i && bearers$markers$later == 0L && bearers$absent == 0L && bearers$unknown == 0L && !bearers$overflow)
  body <- jsonlite::fromJSON(tail(capture(), 1L)[[1]], simplifyVector = FALSE)
  check(paste(row$name, "uses its model"), identical(body$model, row$model))
  criteria <- body$questions$q1$criteria
  wanted <- if (row$form == "text") "yes" else list(what = "yes", examples = list("refund"))
  check(paste(row$name, "retains its question form"), identical(criteria$true, wanted) &&
    if (row$form == "both") is.list(criteria$false) && length(criteria$false) == 0L else is.null(criteria$false))
}
before <- backend_count()
out <- child(c(
  'for (value in list(1, TRUE, list(), character(), NA_character_, "", "nowhere")) {',
  '  kind <- tryCatch({tt_engine(backend = value); "none"}, thinkthen_usage = function(e) e$kind)',
  '  stopifnot(identical(kind, "usage"))',
  '}',
  'stopifnot(length(formals(thinkthen:::tt_engine_set)) == 14L)',
  'stopifnot(getDLLRegisteredRoutines("thinkthen")$.Call$wrap__tt_engine_set$numParameters == 14L)',
  'partial <- tryCatch(tt_engine(back = "typesafe"), thinkthen_usage = function(e) e$kind)',
  'positional <- tryCatch(do.call(tt_engine, rep(list(NULL), 14L)), thinkthen_usage = function(e) e$kind)',
  'stopifnot(identical(partial, "usage"), identical(positional, "usage"))',
  'cat("all refused")'
))
check("invalid names and host types refuse", out$status == 0L && grepl("all refused", out$text, fixed = TRUE))
check("refusal sends nothing", backend_count() == before)
finish("named_backends", 1L)
