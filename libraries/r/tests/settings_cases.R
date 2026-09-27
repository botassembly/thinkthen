# The shared settings corpus through tt_engine's R argument names.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
corpus <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "conformance", "settings.json"), simplifyVector = FALSE)
check("settings schema", identical(corpus$schema, "thinkthen.settings-cases/1"))
literal <- function(value) as.character(jsonlite::toJSON(value, auto_unbox = TRUE))

for (case in corpus$cases) {
  folder <- tempfile(paste0("setting-", case$setting, "-"))
  dir.create(folder)
  profile <- tempfile(fileext = ".json")
  if (!is.null(case$profile)) writeLines(jsonlite::toJSON(case$profile, auto_unbox = TRUE), profile)
  before <- backend_count()
  for (step in case$steps) {
    settings <- step$settings
    settings <- lapply(settings, function(value) {
      if (identical(value, "$FOLDER")) folder else if (identical(value, "$PROFILE")) profile else value
    })
    body <- sprintf('settings <- jsonlite::fromJSON(%s, simplifyVector = FALSE)',
                    literal(literal(settings)))
    # R's JSON reader returns scalar numbers as doubles; tt_engine checks whole numbers.
    lines <- c(
      'library(jsonlite)', body,
      'tryCatch({',
      'do.call(tt_engine, settings)',
      if (identical(step$verb, "decide_many"))
        sprintf('value <- tt_decide(%s, c("refund one", "refund two", "refund three"))', literal(corpus$question))
      else if (!is.null(step$model))
        sprintf('value <- tt_details(%s, %s)', literal(corpus$question), literal(step$text))
      else sprintf('value <- tt_decide(%s, %s)', literal(corpus$question), literal(step$text)),
      'cat(jsonlite::toJSON(list(value = value), auto_unbox = TRUE), "\\n")',
      '}, thinkthen_error = function(e) cat(jsonlite::toJSON(list(error = e$kind), auto_unbox = TRUE), "\\n"))'
    )
    out <- child(lines, c(paste0("THINKTHEN_BASE_URL=", arm(case$arm)), paste0("THINKTHEN_CACHE=", folder)))
    check(paste(case$id, "child status"), out$status == 0L)
    got <- jsonlite::fromJSON(out$text, simplifyVector = FALSE)
    if (!is.null(step$error)) check(paste(case$id, "error"), identical(got$error, step$error))
    else if (!is.null(step$model)) {
      check(paste(case$id, "model"), identical(got$value$meta$model, step$model))
      check(paste(case$id, "value"), isTRUE(got$value$value))
    } else check(paste(case$id, "value"), isTRUE(got$value))
    check(paste(case$id, "count"), backend_count() - before == step$count)
  }
  if (!is.null(case$entries)) check(paste(case$id, "entries"),
    length(list.files(folder, pattern = "[.]json$")) == case$entries)
}
finish("settings-cases", backend_count())
