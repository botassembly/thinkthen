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
# The published shape (ticket 0128): R-universe builds this folder alone, with
# no vendored crates and no repository around it. The engine then comes from
# crates.io at this package's exact version, and cargo may fetch.
.published_shape <- !.tarball_shape && !file.exists("../../../crates/thinkthen/Cargo.toml")

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
} else if (.published_shape) {
  .cargo_export <- ""
  .doc <- ""
  .version <- read.dcf("DESCRIPTION", fields = "Version")[[1]]
  .manifest <- readLines("src/rust/Cargo.toml")
  .engine <- grepl("^thinkthen = \\{ path = ", .manifest)
  if (sum(.engine) != 1) stop("src/rust/Cargo.toml must name the thinkthen path dependency once")
  .manifest[.engine] <- sprintf('thinkthen = { version = "=%s", default-features = false, features = ["bundled-sqlite"] }', .version)
  writeLines(.manifest, "src/rust/Cargo.toml")
  # The lock gains the registry entry and keeps every other pin.
  if (system2("cargo", c("update", "--package", "thinkthen", "--manifest-path", "src/rust/Cargo.toml")) != 0) {
    stop("cargo could not resolve thinkthen ", .version, " from crates.io")
  }
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

# Ticket 0304 slice 3b: on Linux the package keeps the archive's bundled
# SQLite and Rust names local, so they never bind to another package's
# SQLite. R finds the routines through their registration, not by name.
.exclude_libs <- if (Sys.info()[["sysname"]] == "Linux") "-Wl,--exclude-libs,ALL" else ""

configure_file(
  "src/Makevars.in",
  "src/Makevars",
  list(
    LIBDIR = "release",
    CRAN_FLAGS = if (.published_shape) "--locked" else "--locked --offline",
    CARGO_EXPORT = .cargo_export,
    DOC = .doc,
    PROFILE = "--release",
    TARGET = "",
    CLEAN_TARGET = "$(TARGET_DIR)",
    PANIC_EXPORTS = "",
    EXCLUDE_LIBS = .exclude_libs
  )
)
