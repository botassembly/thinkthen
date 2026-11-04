source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
stub <- tt_client(backend = "stub")
# Feature: ordinary calls produce answers directly; rows retain provenance.
is_ready <- tt_decide(c("alpha", "bravo"), "Ready?", client = stub)
stopifnot(is.logical(is_ready), length(is_ready) == 2L)
rows <- tt_decide_rows(c("alpha", "bravo"), "Ready?", client = stub)
stopifnot(identical(is_ready, tt_answer(rows)))
for (old in c("tt_client_from_env", paste0("tt_", c("decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"), "_vec"))) {
  stopifnot(!old %in% getNamespaceExports("thinkthen"))
}
# Feature: missing and blank records raise with all paid-for outcomes.
failed <- condition(tt_decide(c("alpha", NA, ""), "Ready?", client = stub),
                    "thinkthen_failed_rows", "record.missing")
stopifnot(nrow(failed$rows) == 3L, nrow(failed$failed_rows) == 2L)
stopifnot(identical(tt_decide(c(NA_character_, ""), "Ready?", client = stub, errors = "na"), c(NA, NA)))
stopifnot(identical(tt_decide(character(), "Ready?", client = stub), logical()))
stopifnot(identical(tt_rank(c("alpha", "bravo", "charlie"), "How much?", c("low", "medium", "high"), client = stub), c(3, 1, 2)))
# Feature: filter returns original kept records and retains failures in its condition.
input <- c("alpha", NA_character_, "bravo", "charlie")
judged <- tt_filter_rows(input, "Ready?", client = stub)
kept <- tt_filter(input, "Ready?", client = stub, errors = "na")
stopifnot(identical(kept, input[judged$position[which(judged$answer)]]))
failure <- condition(tt_filter(input, "Ready?", client = stub), "thinkthen_failed_rows", "record.missing")
stopifnot(identical(failure$kept, kept), nrow(failure$failed_rows) == 1L)
# Feature: annotation answers are typed columns, directly from a reusable toolbox.
tools <- tt_toolbox(list(ready = tt_decide_question("Ready?"), team = tt_choose_question("Team?", c("billing", "shipping"))))
answers <- tt_annotate(c("alpha", "bravo"), tools, client = stub)
stopifnot(inherits(answers, "tbl_df"), identical(names(answers), c("ready", "team")), is.logical(answers$ready), is.character(answers$team))
# Feature: user meanings enter the admitted question and affect its identity.
plain <- tt_decide_rows("alpha", "Ready?", client = stub)
meaning <- tt_decide_rows("alpha", "Ready?", yes_means = "Ready to ship", no_means = "Not ready", client = stub)
stopifnot(!identical(plain$source[[1]]$question_identity, meaning$source[[1]]$question_identity))
labels <- tt_choose_rows("alpha", "Team?", c("billing", "shipping"), descriptions = c(billing = "Payments"), client = stub)
stopifnot(nrow(labels) == 1L)
condition(tt_choose("alpha", "Team?", c("billing", "shipping"), descriptions = c(other = "Other"), client = stub), "thinkthen_usage_error")
# Feature: relation rules use ordinary R values.
rule <- tt_rule("person", "person", "knows")
stopifnot(identical(tt_relate("Alice knows Bob", rule, client = stub), tt_relate("Alice knows Bob", list(rule), client = stub)))
# Feature: an unset backend argument reads the project environment.
withr::with_envvar(c(THINKTHEN_BACKEND = "stub"), {
  env_client <- tt_client()
  stopifnot(as.character(tt_decide_rows("alpha", "Ready?", client = env_client)$source[[1]]$backend) == "stub")
  tt_close(env_client)
})
# Feature: file helpers preserve canonical saved questions.
path <- tempfile(fileext = ".json")
tt_toolbox_write(tools, path)
stopifnot(identical(tt_toolbox_to_json(tt_toolbox_read(path)), tt_toolbox_to_json(tools)))
unlink(path)
# Feature: not-sure is a missing answer and every answer function has a real signature.
uncertain <- tt_decide(c("alpha", "bravo"), "Ready?", threshold = c(.000001, 1), client = stub)
stopifnot(all(is.na(uncertain)))
condition(tt_decide("alpha", "Ready?", client = stub, errors = "ignore"), "thinkthen_usage_error", "usage.argument")
for (verb in c("decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate")) {
  fn <- get(paste0("tt_", verb))
  stopifnot("x" %in% names(formals(fn)), "errors" %in% names(formals(fn)))
}
# Feature: annotation columns retain their declared types with no successful rows.
for (input in list(character(), c(NA_character_, ""))) {
  columns <- tt_annotate(input, tools, client = stub, errors = "na")
  stopifnot(identical(names(columns), c("ready", "team")),
            nrow(columns) == length(input), is.logical(columns$ready),
            is.character(columns$team), all(is.na(columns$ready)), all(is.na(columns$team)))
}
# Feature: environment defaults remain raw and an explicit backend wins.
condition(tt_client(backend = NA_character_), "thinkthen_usage_error", "usage.settings")
withr::with_envvar(c(THINKTHEN_BACKEND = ""), {
  condition(tt_client(), "thinkthen_usage_error")
  explicit <- tt_client(backend = "stub")
  stopifnot(as.character(tt_decide_rows("alpha", "Ready?", client = explicit)$source[[1]]$backend) == "stub")
  tt_close(explicit)
})
tt_close(stub)
cat("R answer-first public interface passed\n")
