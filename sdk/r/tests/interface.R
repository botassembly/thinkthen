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
stopifnot(identical(plain(tt_decide(c(NA_character_, ""), "Ready?", client = stub, errors = "na")), c(NA, NA)))
stopifnot(identical(tt_decide(character(), "Ready?", client = stub), logical()))
stopifnot(identical(tt_rank(c("alpha", "bravo", "charlie"), "How much?", c("low", "medium", "high"), client = stub), c(3, 1, 2)))
# Feature: filter returns original kept records and retains failures in its condition.
input <- c("alpha", NA_character_, "bravo", "charlie")
judged <- tt_filter_rows(input, "Ready?", client = stub)
kept <- plain(tt_filter(input, "Ready?", client = stub, errors = "na"))
stopifnot(identical(kept, input[judged$position[which(judged$answer)]]))
failure <- condition(tt_filter(input, "Ready?", client = stub), "thinkthen_failed_rows", "record.missing")
stopifnot(identical(failure$kept, kept), nrow(failure$failed_rows) == 1L)
# Feature: annotation answers are typed columns, directly from a reusable toolbox.
tools <- tt_toolbox(list(ready = tt_decide_question("Ready?"), team = tt_choose_question("Team?", c("billing", "shipping"))))
answers <- tt_annotate(c("alpha", "bravo"), tools, client = stub)
stopifnot(inherits(answers, "tbl_df"), identical(names(answers), c("ready", "team")), is.logical(answers$ready), is.character(answers$team))
# Feature: user meanings enter the admitted question and affect its identity.
unmeant <- tt_decide_rows("alpha", "Ready?", client = stub)
meaning <- tt_decide_rows("alpha", "Ready?", yes_means = "Ready to ship", no_means = "Not ready", client = stub)
stopifnot(!identical(unmeant$source[[1]]$question_identity, meaning$source[[1]]$question_identity))
labels <- tt_choose_rows("alpha", "Team?", c("billing", "shipping"), descriptions = c(billing = "Payments"), client = stub)
stopifnot(nrow(labels) == 1L)
condition(tt_choose("alpha", "Team?", c("billing", "shipping"), descriptions = c(other = "Other"), client = stub), "thinkthen_usage_error")
# Feature: relation rules use ordinary R values.
rule <- tt_rule("person", "person", "knows")
stopifnot(identical(tt_relate("Alice knows Bob", rule, client = stub), tt_relate("Alice knows Bob", list(rule), client = stub)))
# Feature: errors = "na" keeps a failure apart from not sure.
band <- c(.000001, 1)
input <- c("alpha", NA, "")
stopifnot(identical(as.character(tt_decide_rows(input, "Ready?", threshold = band, client = stub)$status),
                    c("unsure", "failed", "failed")))
summary <- "ThinkThen returned failed records: record.missing (1), record.empty (1)."
result <- caught(tt_decide(input, "Ready?", threshold = band, client = stub, errors = "na"))
warning <- result$warnings[[1]]
stopifnot(length(result$warnings) == 1L, inherits(warning, "thinkthen_failed_rows_warning"),
          !inherits(warning, "thinkthen_failed_rows"), identical(warning$code, "record.missing"),
          identical(warning$codes, c("record.missing", "record.empty")),
          identical(conditionMessage(warning), paste(summary, "tt_failures() lists them.")),
          nrow(warning$rows) == 3L, nrow(warning$failed_rows) == 2L,
          identical(plain(result$value), c(NA, NA, NA)))
failures <- tt_failures(result$value)
stopifnot(identical(names(failures), c("position", "id", "question", "error_code", "error_message", "error_http_status")),
          inherits(failures, "tbl_df"), identical(failures$position, c(2, 3)), is.list(failures$id),
          identical(failures$question, c(NA_character_, NA_character_)),
          identical(as.character(failures$error_code), c("record.missing", "record.empty")),
          identical(levels(failures$error_code), levels(tt_decide_rows("alpha", "Ready?", client = stub)$error_code)),
          all(nzchar(failures$error_message)), identical(failures$error_http_status, c(NA_integer_, NA_integer_)),
          identical(tt_failures(tt_decide_rows(input, "Ready?", threshold = band, client = stub)), failures),
          nrow(tt_failures(result$value[2:3])) == 0L,
          any(grepl(".thinkthen_failures", capture.output(print(result$value)), fixed = TRUE)),
          length(caught(suppressWarnings(tt_decide(input, "Ready?", client = stub, errors = "na")))$warnings) == 0L)
failed <- condition(tt_decide(input, "Ready?", threshold = band, client = stub), "thinkthen_failed_rows", "record.missing")
stopifnot(identical(failed$codes, c("record.missing", "record.empty")), identical(conditionMessage(failed), summary),
          identical(class(failed)[1:2], c("thinkthen_failed_rows", "thinkthen_error")))
# Feature: filter keeps its failures beside the kept records.
input <- c("alpha", NA_character_, "bravo", "charlie")
result <- caught(tt_filter(input, "Ready?", client = stub, errors = "na"))
stopifnot(length(result$warnings) == 1L, identical(result$warnings[[1]]$kept, plain(result$value)),
          identical(tt_failures(result$value)$position, 2))
# Feature: a call with no failure stays a plain value with no warning.
clean <- caught(tt_decide(c("alpha", "bravo"), "Ready?", client = stub, errors = "na"))
stopifnot(identical(clean$value, tt_answer(tt_decide_rows(c("alpha", "bravo"), "Ready?", client = stub))),
          is.null(attributes(clean$value)), length(clean$warnings) == 0L)
empty <- tt_failures(clean$value)
stopifnot(nrow(empty) == 0L, identical(vctrs::vec_ptype(empty), vctrs::vec_ptype(failures)))
for (value in list(c(1, 2), NULL, data.frame(a = 1), tt_decide_rows("alpha", "Ready?", client = stub),
                   tt_annotate_rows("alpha", tools, client = stub))) {
  stopifnot(identical(tt_failures(value), empty))
}
# Feature: every plain verb keeps its failures on its own answer type.
levels <- c("low", "medium", "high")
input <- c("Ada knows Bea", NA)
verbs <- list(
  function(x, q, ...) tt_choose(x, q, c("left", "right"), ...),
  function(x, q, ...) tt_tag(x, q, c("left", "right"), ...),
  function(x, q, ...) tt_score(x, q, levels, ...),
  function(x, q, ...) tt_rank(x, q, levels, ...),
  tt_find, tt_keep,
  function(x, q, ...) tt_recognize(x, "person", ...),
  function(x, q, ...) tt_relate(x, rule, ...))
for (verb in verbs) {
  result <- caught(verb(input, "Ready?", client = stub, errors = "na"))
  stopifnot(length(result$warnings) == 1L, identical(tt_failures(result$value)$position, 2))
  capture.output(print(result$value))
}
# Feature: positions count the input after a rank reorders its rows.
ranked <- tt_rank_rows(c("", "alpha", "bravo", NA), "How much?", levels, client = stub)
stopifnot(!identical(ranked$position, c(1, 2, 3, 4)), identical(tt_failures(ranked)$position, c(1, 4)))
# Feature: annotation failures name their question and count records, not entries.
records <- list(list(body = "text"), NULL, list(body = ""))
absent <- tt_annotation_questions(list(body = tt_decide_question("Ready?", field = "/body"),
  gone = tt_decide_question("Ready?", field = "/gone"), lost = tt_decide_question("Ready?", field = "/lost")))
result <- caught(tt_annotate(records, absent, client = stub, errors = "na"))
failures <- tt_failures(result$value)
stopifnot(inherits(result$value, "tbl_df"), length(result$warnings) == 1L,
          identical(result$warnings[[1]]$codes, c("record.missing", "record.empty")),
          identical(failures$position, c(1, 1, 2, 3, 3, 3)),
          identical(failures$question, c("gone", "lost", NA, "body", "gone", "lost")),
          identical(as.character(failures$error_code)[[4]], "record.empty"),
          identical(conditionMessage(condition(tt_annotate(records, absent, client = stub), "thinkthen_failed_rows", "record.missing")),
                    "ThinkThen returned failed records: record.missing (3), record.empty (1)."))
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
  columns <- plain(tt_annotate(input, tools, client = stub, errors = "na"))
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
