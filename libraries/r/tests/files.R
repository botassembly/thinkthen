# Explicit source questions use the same native reader for every verb.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/v1"), cache = FALSE)
documents <- file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "specification", "fixtures", "files", "documents")
questions <- list(list(decide = "Q?"), list(choose = "Q?", options = c("policy", "contract")),
  list(tag = "Q?", labels = c("refund", "support")), list(score = "Q?", levels = c("low", "high")),
  list(filter = "Q?"), list(rank = "Q?"), list(find = "Q?"),
  list(annotate = list(version = 1, questions = list(urgent = list(decide = "Q?")))),
  list(version = 1, recognize = list(kinds = list(person = NULL))),
  list(version = 1, relate = list(relations = list(list(name = "supports", source = "*", target = "*")))))
for (i in seq_along(questions)) {
  called <- tt_files(questions[[i]], documents, unit = "file")
  check("source call sent", called$facts$requests_sent > 0)
  rows <- if (i == 10) unlist(lapply(called$value$edges, function(e) list(e$source, e$target)), recursive = FALSE) else if (i == 7) list(called$value) else called$value
  check("source rows present", length(rows) > 0)
  for (row in rows) check("physical file range and original newlines", row$first_line == 1 && row$last_line == 4 && grepl("\n", row$record, fixed = TRUE))
}
# Skipped blank lines do not renumber physical locations; repeated text remains separate.
path <- tempfile(fileext = ".txt")
writeLines(c("", "Le café 😀 Maria Chen arrived.", "", "Le café 😀 Maria Chen arrived."), path, useBytes = TRUE)
ranked <- tt_files(list(rank = "Q?"), path)$value
check("located rank indexes count records rather than physical lines",
  identical(vapply(ranked, `[[`, 0L, "index"), 0:1) &&
  identical(vapply(ranked, `[[`, 0L, "first_line"), c(2L, 4L)))
recognized <- tt_files(questions[[9]], path)$value
check("both duplicate source occurrences survive recognition", length(recognized) == 2L)
for (i in seq_along(recognized)) {
  row <- recognized[[i]]
  span <- row$value$entities[[1]]
  check("located multibyte spans keep native scalar offsets and physical lines",
    identical(c(span$start, span$end, span$length), c(21L, 29L, 8L)) &&
    identical(c(span$first_line, span$last_line), rep(c(2L, 4L)[i], 2)) &&
    identical(substr(row$record, span$start + 1L, span$end), span$text))
}
unlink(path)
before <- backend_count()
check("invalid reader refused", identical(kind_of(tt_files(list(decide = "Q?"), "missing", window = 2)), "usage"))
check("invalid reader sends nothing", backend_count() == before)
cat(sprintf("expect count %d\n", backend_count()))
cat(sprintf("%d checks passed\n", passed))
