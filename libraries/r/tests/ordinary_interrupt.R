source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
script <- tempfile(fileext = ".R")
output <- tempfile()
cache <- tempfile("cache-")
dir.create(cache)
writeLines(c('library(thinkthen)', 'tt_engine(cache = FALSE, max_retries = 0L)',
  'receipt <- tt_completion()',
  'tryCatch(tt_decide("Held?", "one", completion = receipt), interrupt = function(e) { cat("CAUGHT", tt_completion_read(receipt)$state, "\\n"); flush(stdout()) })',
  'deadline <- Sys.time() + 10',
  'repeat { value <- tt_completion_read(receipt); if (value$state == "terminal") break; if (Sys.time() > deadline) stop("receipt did not settle"); Sys.sleep(0.01) }',
  'stopifnot(value$kind == "cancelled", value$facts$requests_sent == 1)',
  'cat("SETTLED\\n")'), script)
held <- paste0("THINKTHEN_BASE_URL=", arm("arm/held/v1"))
pid <- background_child(paste("env", paste(clean_env(inherited, child_values(cache, held)), collapse = " "),
                              paste(rscript_args(script), collapse = " ")), output)
tryCatch({
stopifnot(backend_wait(1L) == 1L)
tools::pskill(pid, 2L)
deadline <- Sys.time() + 10
lines <- function() if (file.exists(output)) readLines(output, warn = FALSE) else character()
while (!any(grepl("CAUGHT", lines(), fixed = TRUE)) && Sys.time() < deadline) Sys.sleep(0.01)
check("ordinary native wait delivers R interrupt before provider release", any(grepl("CAUGHT running", lines(), fixed = TRUE)))
backend_say("round")
while (tools::pskill(pid, 0L) && Sys.time() < deadline) Sys.sleep(0.01)
check("receipt settles after caller interrupt and provider release", any(grepl("SETTLED", lines(), fixed = TRUE)) && !tools::pskill(pid, 0L))
finish("ordinary interrupt", 1L)
}, finally = { if (tools::pskill(pid, 0L)) tools::pskill(pid, 9L); backend_say("release") })
