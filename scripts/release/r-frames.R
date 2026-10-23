inventory <- jsonlite::fromJSON(file.path(root, "prerequisites/r-frames/inventory.json"), simplifyVector = FALSE)
stopifnot(identical(inventory$r, as.character(getRversion())))
for (name in inventory$install_order) {
  install_binary(file.path(root, "prerequisites/r-frames", name))
}
for (name in names(inventory$packages)) {
  stopifnot(identical(normalizePath(find.package(name)), normalizePath(file.path(lib, name))))
}
status <- system2(file.path(R.home("bin"), "Rscript"), c("--vanilla",
  shQuote(file.path(root, "checks/r-frame-cases.R")), shQuote(file.path(root, "checks/stub.json"))))
stopifnot(status == 0L)
cat("candidate R: matching binary table packages and frame helpers pass\n")
