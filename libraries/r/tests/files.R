# Explicit source questions use the same native reader for every verb.
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
tt_engine(base_url = arm("arm/full/v1"), cache = FALSE)
documents <- file.path(Sys.getenv("TT_TESTS"), "..", "..", "..", "specification", "fixtures", "files", "documents")
questions <- list(list(decide = "Q?"), list(choose = "Q?", options = c("policy", "contract")),
  list(tag = "Q?", labels = c("refund", "support")), list(score = "Q?", levels = c("low", "high")),
  list(decide = "Q?"), list(decide = "Q?"), list(find = "Q?"),
  list(version = 1, questions = list(urgent = list(decide = "Q?"))),
  list(version = 1, recognize = list(kinds = list(person = NULL))),
  list(version = 1, relate = list(relations = list(list(name = "supports", source = "*", target = "*")))))
verbs <- c("decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate")
for (i in seq_along(questions)) {
  arrivals <- sent_by(called <- get(paste0("tt_", verbs[i]))(questions[[i]], tt_files(documents, unit = "file")))
  check(paste(verbs[i], "returns a typed native call with observed sends"),
    inherits(called, "thinkthen_Call") && inherits(called$facts, "thinkthen_Facts") && called$facts$requests_sent == arrivals && arrivals > 0)
  rows <- called$results
  check(paste(verbs[i], "retains typed answers"), length(rows) > 0 &&
    all(vapply(rows, function(row) inherits(row, "thinkthen_complete") && inherits(row$answer_id, "thinkthen_AnswerId"), TRUE)))
  if (i == 7) rows <- Filter(function(row) !inherits(row$source, "thinkthen_absent"), rows[[1]]$candidates)
  if (i == 10) rows <- lapply(rows[[1]]$input_sources, function(at) list(source = at$source, input = rows[[1]]$input[[at$index + 1]]))
  check(paste(verbs[i], "retains both physical documents"), length(rows) == 2L)
  for (row in rows) check("physical file range and original newlines",
    row$source$first_line == 1 && row$source$last_line == 4 &&
    basename(row$source$file) %in% c("01-policy.txt", "02-contract.txt") &&
    identical(row$input, readChar(row$source$file, file.info(row$source$file)$size, useBytes = TRUE)))
}
# Skipped blank lines do not renumber physical locations; repeated text remains separate.
path <- tempfile(fileext = ".txt")
writeLines(c("", "Le café 😀 Maria Chen arrived.", "", "Le café 😀 Maria Chen arrived."), path, useBytes = TRUE)
ranked <- tt_rank("Q?", tt_files(path))$results
check("located rank indexes count records rather than physical lines",
  identical(vapply(ranked, `[[`, 0, "index"), c(0, 1)) &&
  identical(vapply(ranked, function(row) row$source$first_line, 0), c(2, 4)))
recognized <- tt_recognize(questions[[9]], tt_files(path))$results
check("both duplicate source occurrences survive recognition", length(recognized) == 2L)
for (i in seq_along(recognized)) {
  row <- recognized[[i]]
  span <- row$value$entities[[1]]
  check("located multibyte spans keep native scalar offsets and physical lines",
    identical(c(span$start, span$end, span$length), c(21, 29, 8)) &&
    identical(c(span$first_line, span$last_line), rep(c(2, 4)[i], 2)) &&
    identical(substr(row$input, span$start + 1L, span$end), span$text))
}
unlink(path)
before <- backend_count()
check("invalid reader refused", identical(kind_of(tt_decide("Q?", tt_files("missing", window = 2))), "usage"))
check("invalid reader sends nothing", backend_count() == before)
cat(sprintf("expect count %d\n", backend_count()))
cat(sprintf("%d checks passed\n", passed))
