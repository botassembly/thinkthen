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
before <- backend_count()
check("invalid reader refused", identical(kind_of(tt_files(list(decide = "Q?"), "missing", window = 2)), "usage"))
check("invalid reader sends nothing", backend_count() == before)
cat(sprintf("expect count %d\n", backend_count()))
cat(sprintf("%d checks passed\n", passed))
