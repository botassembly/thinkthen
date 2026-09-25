# The build configuration for the Rust half, following rextendr's shape.
# Both shapes build --locked --offline: a missing crate fails here, and
# `cargo fetch --locked` on a networked machine is the one step that fills
# the cache (R4-19).
#
# The tarball shape (tools/make-tarball.sh at the repository's
# libraries/r) vendors the thinkthen crate and every registry dependency
# under src/rust/vendor, with a config.toml that redirects crates-io there.
# CARGO_HOME points at the shipped rust/.cargo so cargo finds that config,
# and the document step is skipped because the wrappers ship in the
# tarball. The repository shape keeps the builder's own CARGO_HOME and
# regenerates the wrappers.
.tarball_shape <- dir.exists("src/rust/vendor/registry")

if (.tarball_shape) {
  .cargo_export <- "export CARGO_HOME=$(CURDIR)/rust/.cargo && "
  .doc <- ""
  if (!file.exists("R/extendr-wrappers.R")) {
    stop("the tarball shape must ship R/extendr-wrappers.R (make-tarball.sh verifies it)")
  }
  # R CMD build may strip dot-directories, so the vendored-sources config
  # is also written here, at install time. The tarball ships the same bytes.
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
  .cargo_export <- ""
  .doc <- paste(
    "&& cargo run --locked --offline --bin document",
    "--manifest-path=./rust/Cargo.toml --target-dir $(TARGET_DIR)"
  )
}

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
    LIBDIR = "release",
    CRAN_FLAGS = "--locked --offline",
    CARGO_EXPORT = .cargo_export,
    DOC = .doc,
    PROFILE = "--release",
    TARGET = "",
    CLEAN_TARGET = "$(TARGET_DIR)",
    PANIC_EXPORTS = ""
  )
)
