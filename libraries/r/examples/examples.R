# Every R function's example, run as one test. examples.json is keyed by
# function: the call, the answer the 0092 generic arm gives, and any
# question files the call names. The site's R tabs draw from this file,
# so an example nobody runs cannot reach the site. Each example runs in its
# own child with its own cache. Run through tests/with-backend.sh.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
examples <- jsonlite::fromJSON(file.path(Sys.getenv("TT_TESTS"), "..", "examples", "examples.json"), simplifyVector = FALSE)$examples
for (name in names(examples)) {
  example <- examples[[name]]
  folder <- tempfile("example-")
  dir.create(folder)
  for (file in names(example$files)) writeLines(example$files[[file]], file.path(folder, file))
  got <- child(c(sprintf("setwd('%s')", folder), example$r))$text
  check(paste0(name, ": want ", example$expected, ", got ", got), identical(trimws(got), example$expected))
}
finish("examples", 12L)
