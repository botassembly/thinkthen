# The build configuration for the Rust half, following rextendr's shape.
# NOT_CRAN is always set here: this package builds from the repository's own
# contract and stand-in crates by path, never from a vendor tarball.

env_not_cran <- Sys.getenv("NOT_CRAN")

.cran_flags <- ""
.profile <- "--release"
.clean_targets <- "$(TARGET_DIR)"

.LIBDIR <- "release"

configure_file <- function(in_file, out_file, values) {
  lines <- readLines(in_file)
  for (key in names(values)) {
    lines <- gsub(paste0("@", key, "@"), values[[key]], lines, fixed = TRUE)
  }
  writeLines(lines, out_file)
}

configure_file(
  "src/Makevars.in",
  "src/Makevars",
  list(
    LIBDIR = .LIBDIR,
    CRAN_FLAGS = .cran_flags,
    PROFILE = .profile,
    TARGET = "",
    CLEAN_TARGET = .clean_targets,
    PANIC_EXPORTS = ""
  )
)
