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

# Two shapes, decided by whether the registry vendor tree is present.
#
# The tarball shape (make-tarball.sh): every registry dependency is
# vendored under src/rust/vendor/registry with a config.toml redirecting
# crates-io there, so the install needs no network and no populated
# CARGO_HOME - the fourth review's fresh-checkout probe failed on an
# empty, gitignored src/.cargo. CARGO_HOME points at the shipped
# rust/.cargo so cargo finds that config, the build runs --offline, and
# the document step is skipped: the wrappers ship in the tarball, so the
# install never builds the debug binary that step pulled in.
#
# The repository shape: no vendored registry, so CARGO_HOME stays the
# developer's own (the empty-dir override is gone), the build runs
# --locked with the network available for a first fetch, and the
# document step regenerates the wrappers as before.
.tarball_shape <- dir.exists("src/rust/vendor/registry")

if (.tarball_shape) {
  .cran_flags <- paste("--locked --offline", .synthetic_flags)
  .cargo_export <- "export CARGO_HOME=$(CURDIR)/rust/.cargo && "
  .doc <- ""
  if (!file.exists("R/extendr-wrappers.R")) {
    stop("the tarball shape must ship R/extendr-wrappers.R (make-tarball.sh verifies it)")
  }
  # R CMD build may strip dot-directories, so the vendored-sources config
  # is also written here, at install time, where Makevars' CARGO_HOME
  # finds it. Idempotent: the tarball ships the same bytes.
  dir.create("src/rust/.cargo", showWarnings = FALSE, recursive = TRUE)
  writeLines(
    c(
      "[source.crates-io]",
      "replace-with = \"vendored-sources\"",
      "",
      "[source.vendored-sources]",
      "directory = \"vendor/registry\""
    ),
    "src/rust/.cargo/config.toml"
  )
} else {
  .cran_flags <- paste("--locked", .synthetic_flags)
  .cargo_export <- ""
  .doc <- paste(
    "&& cargo run --locked --bin document",
    "--manifest-path=./rust/Cargo.toml --target-dir $(TARGET_DIR)"
  )
}

.profile <- "--release"
# The tarball shape removes its build folder after the install, as
# rextendr does, so an installed tarball leaves nothing behind. The
# repository shape keeps src/rust/target, which is gitignored and which
# the workspace lint already builds into: the gate installs twice (the
# fixture build, then the production restore), and each install compiled
# every dependency again (sdlc/records/surfaces-notes/NOTES-speed.md).
.clean_targets <- if (.tarball_shape) "$(TARGET_DIR)" else ""

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
    CARGO_EXPORT = .cargo_export,
    DOC = .doc,
    PROFILE = .profile,
    TARGET = "",
    CLEAN_TARGET = .clean_targets,
    PANIC_EXPORTS = ""
  )
)
