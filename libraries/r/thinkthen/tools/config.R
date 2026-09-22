# The build configuration for the Rust half, following rextendr's shape.
# NOT_CRAN is always set here: this package builds from the repository's own
# contract and stand-in crates by path, never from a vendor tarball.

env_not_cran <- Sys.getenv("NOT_CRAN")

# The stand-in's compile-time fixture door (the synthesized partial failure
# conformance case 74 replays). A gate build asks for it with
# THINKTHEN_R_SYNTHETIC_PARTIAL=1 on the R CMD INSTALL line; the default is
# off, so a shipped package carries no fixture code and no environment
# variable can arm one at run time.
.synthetic_flags <- if (identical(Sys.getenv("THINKTHEN_R_SYNTHETIC_PARTIAL"), "1")) {
  "--features synthetic-partial"
} else {
  ""
}

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
    CRAN_FLAGS = paste(.cran_flags, .synthetic_flags),
    PROFILE = .profile,
    TARGET = "",
    CLEAN_TARGET = .clean_targets,
    PANIC_EXPORTS = ""
  )
)
