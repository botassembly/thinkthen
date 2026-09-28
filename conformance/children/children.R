# A test child's whole environment, built from nothing (ticket 0127).
# `clean_env` returns the words `env` runs a program under: `-i`, then PATH,
# each kept name the parent has, and each value set here, every one quoted.
# A secret-shaped or THINKTHEN_ name is never kept. A call site runs
# system2("env", c(clean_env(keep, values), shQuote(program), shQuote(arguments))).
clean_env <- function(keep = character(), values = character()) {
  secret <- grepl("^THINKTHEN_|KEY|TOKEN|SECRET|PASSWORD|CREDENTIAL|AUTH", keep, ignore.case = TRUE)
  if (any(secret)) {
    stop(sprintf("a test child may not keep %s from the parent: set a THINKTHEN_ value or a fake key explicitly",
                 keep[secret][[1]]), call. = FALSE)
  }
  kept <- keep[nzchar(Sys.getenv(keep, unset = ""))]
  pairs <- c(PATH = Sys.getenv("PATH"), vapply(kept, Sys.getenv, ""), values)
  c("-i", shQuote(paste0(names(pairs), "=", pairs)))
}
