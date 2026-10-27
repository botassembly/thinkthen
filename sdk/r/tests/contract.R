# Feature: R enum values and numeric discriminators match the engine registry.
root <- dirname(dirname(dirname(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))))))
registry <- jsonlite::fromJSON(file.path(root, "schemas/contract.json"), simplifyVector = FALSE)
domains <- c(functions = "function", statuses = "status", backends = "backend",
             id_kinds = "id", record_kinds = "record", offsets = "offset", codes = "code")
for (name in names(domains)) {
  key <- domains[[name]]
  entries <- registry[[name]]
  actual <- get(paste0(".tt_", key, "_names"), envir = asNamespace("thinkthen"), inherits = FALSE)
  expected <- setNames(vapply(entries, `[[`, character(1), "name"),
                       vapply(entries, function(x) as.character(x$value), character(1)))
  stopifnot(identical(actual[order(names(actual))], expected[order(names(expected))]))
}
cat("R enums match the engine registry\n")
