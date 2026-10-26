source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
stub <- tt_client(backend = "stub")

# Feature: filter originals retain ordinary frame and column attributes.
text <- structure(setNames(c("alpha", "bravo", "again"), c("a", "b", "c")), provenance = "keep")
rows <- tt_filter(text, "Ready?", client = stub)
stopifnot(nrow(rows) == length(text), identical(rows$item, unname(as.list(text))))
frame <- data.frame(id = c("same", "same", "same"), body = unname(text),
                    day = as.Date("2026-01-01") + seq_len(3L), row.names = c("one", "two", "three"))
attr(frame$body, "provenance") <- "column"
rows <- tt_filter(frame, "Ready?", evidence = "/body", client = stub)
stopifnot(nrow(rows) == 3L, identical(rows$position, c(1, 2, 3)))
for (i in seq_len(3L)) {
  stopifnot(identical(row.names(rows$item[[i]]), row.names(frame)[i]),
            inherits(rows$item[[i]]$day, "Date"),
            identical(attr(rows$item[[i]]$body, "provenance"), "column"))
}

# Feature: separately admitted integer IDs remain available as numeric evidence.
id <- tt_id_integer("18446744073709551615")
root_id <- tt_decide(list(list(id = id, body = "text")), "Ready?", evidence = "/id", client = stub)
stopifnot(root_id$status == "answered", identical(format(root_id$id[[1]]), format(id)),
          identical(format(tt_id_integer("001")), "1"))

# Feature: one-element domains keep array identity in typed selections and annotations.
definitions <- list(
  labels = tt_tag_question("Topics?", "only"),
  people = tt_recognize_question("person"),
  choice = tt_choose_question("Topic?", c("left", "right")),
  scale = tt_score_question("Urgent?", c("low", "high")),
  relation = tt_relate_question(vctrs::list_of(tt_rule("person", "person", "knows")))
)
functions <- c(labels = "tt_tag", people = "tt_recognize", choice = "tt_choose", scale = "tt_score", relation = "tt_relate")
annotation <- tt_annotate("Ada knows Bea", tt_annotation_questions(definitions), client = stub)
for (name in names(definitions)) {
  invoke <- get(functions[[name]])
  selected <- tt_question(tt_toolbox(definitions), name)
  named <- invoke("Ada knows Bea", selected, client = stub)
  single <- invoke("Ada knows Bea", tt_question(tt_toolbox(setNames(list(definitions[[name]]), name)), name), client = stub)
  stopifnot(nrow(named) == 1L, identical(named, single),
            identical(as.character(annotation$answer[[1]][[name]]$status), as.character(named$status[[1]])))
}
stopifnot(nrow(tt_decide("alpha", "questions.json#ready", client = stub)) == 1L)

# Feature: detaching retains native code while shared workers may own allocations.
dll_path <- getLoadedDLLs()[["thinkthen"]][["path"]]
detach("package:thinkthen", unload = TRUE)
stopifnot(!"thinkthen" %in% loadedNamespaces(), identical(getLoadedDLLs()[["thinkthen"]][["path"]], dll_path))
library(thinkthen)
stopifnot(tt_decide("alpha", "Ready?", client = tt_client(backend = "stub"))$status == "answered")
stopifnot(inherits(tryCatch(rawToChar(as.raw(c(97L, 0L, 98L))), error = identity), "error"))
cat("R filter attributes, domain arrays and native ownership passed\n")
