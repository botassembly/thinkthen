# Install only built candidate packages in an empty R library.
args <- commandArgs(TRUE)
root <- args[[1]]
expected <- trimws(readLines(file.path(root, "r/r-version")))
stopifnot(identical(as.character(getRversion()), expected))
lib <- Sys.getenv("R_LIBS_USER")
dir.create(lib, recursive = TRUE, showWarnings = FALSE)
.libPaths(c(lib, .Library))
install_binary <- function(path) {
  # A built-only archive cannot silently fall back to compiling source.
  files <- utils::untar(path, list = TRUE)
  description <- files[grepl("^[^/]+/DESCRIPTION$", files)]
  stopifnot(length(description) == 1L)
  metadata <- tempfile()
  dir.create(metadata)
  utils::untar(path, files = description, exdir = metadata)
  fields <- read.dcf(file.path(metadata, description))
  stopifnot("Built" %in% colnames(fields))
  unlink(metadata, recursive = TRUE)
  status <- system2(file.path(R.home("bin"), "R"), c("CMD", "INSTALL", paste0("--library=", shQuote(lib)), shQuote(path)))
  stopifnot(status == 0L)
}
dependency_dir <- file.path(root, "prerequisites/r-frames")
for (name in readLines(file.path(dependency_dir, "install-order.txt"))) {
  install_binary(file.path(dependency_dir, name))
}
archives <- list.files(file.path(root, "r"), pattern = "^thinkthen_.*\\.(tgz|tar.gz)$", full.names = TRUE)
stopifnot(length(archives) == 1L)
install_binary(archives[[1]])
stopifnot(identical(normalizePath(find.package("thinkthen")), normalizePath(file.path(lib, "thinkthen"))))
status <- system2(file.path(R.home("bin"), "Rscript"), c("--vanilla",
  shQuote(file.path(root, "checks/r-cases.R")), shQuote(file.path(root, "checks/stub.json"))))
stopifnot(status == 0L)
cat("candidate R: ten shared stub cases pass from binary packages\n")
source(file.path(root, "checks/r-frames.R"))
