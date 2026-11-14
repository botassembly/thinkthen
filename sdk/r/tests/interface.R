source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
stub <- tt_client(backend = "stub")
# Feature: ordinary calls produce answers directly; rows retain provenance.
is_ready <- tt_decide(c("alpha", "bravo"), "Ready?", client = stub)
stopifnot(is.logical(is_ready), length(is_ready) == 2L)
rows <- tt_decide(details = TRUE, c("alpha", "bravo"), "Ready?", client = stub)
stopifnot(identical(is_ready, tt_answer(rows)))
for (old in c("tt_client_from_env", paste0("tt_", c("decide", "choose", "tag", "score", "filter", "rank", "find", "annotate", "recognize", "relate"), "_vec"))) {
  stopifnot(!old %in% getNamespaceExports("thinkthen"))
}
# Feature: missing and blank records raise with all paid-for outcomes.
failed <- condition(tt_decide(c("alpha", NA, ""), "Ready?", client = stub),
                    "thinkthen_failed", "record.missing")
stopifnot(nrow(failed$outcomes) == 3L, nrow(failed$rows) == 2L)
stopifnot(identical(plain(tt_decide(c(NA_character_, ""), "Ready?", client = stub, errors = "na")), c(NA, NA)))
stopifnot(identical(tt_decide(character(), "Ready?", client = stub), logical()))
stopifnot(identical(tt_rank(c("alpha", "bravo", "charlie"), "How much?", c("low", "medium", "high"), client = stub), c(3, 1, 2)))
# Feature: filter returns original kept records and retains failures in its condition.
input <- c("alpha", NA_character_, "bravo", "charlie")
failure <- condition(tt_filter(input,"Ready?",client=stub),"thinkthen_failed","record.missing")
judged <- failure$outcomes
kept <- failure$kept
stopifnot(identical(kept,input[judged$position[which(judged$answer)]]),nrow(failure$rows)==1L)
# Feature: annotation answers are typed columns, directly from a reusable toolbox.
tools <- tt_toolbox(list(ready = tt_decide_question("Ready?"), team = tt_choose_question("Team?", c("billing", "shipping"))))
answers <- tt_annotate(c("alpha", "bravo"), tools, client = stub)
stopifnot(inherits(answers, "tbl_df"), identical(names(answers), c("ready", "team")), is.logical(answers$ready), is.factor(answers$team))
# Feature: user meanings enter the admitted question and affect its identity.
unmeant <- tt_decide(details = TRUE, "alpha", "Ready?", client = stub)
meaning <- tt_decide(details = TRUE, "alpha", "Ready?", yes_means = "Ready to ship", no_means = "Not ready", client = stub)
stopifnot(!identical(unmeant$source[[1]]$question_identity, meaning$source[[1]]$question_identity))
labels <- tt_choose(details = TRUE, "alpha", "Team?", c("billing", "shipping"), descriptions = c(billing = "Payments"), client = stub)
stopifnot(nrow(labels) == 1L)
condition(tt_choose("alpha", "Team?", c("billing", "shipping"), descriptions = c(other = "Other"), client = stub), "thinkthen_usage_error")
# Feature: relation rules use ordinary R values.
rule <- tt_rule("person", "person", "knows")
stopifnot(identical(tt_relate("Alice knows Bob", rule, client = stub), tt_relate("Alice knows Bob", list(rule), client = stub)))
# Feature: errors = "na" keeps a failure apart from not sure.
band <- c(.000001, 1)
input <- c("alpha", NA, "")
stopifnot(identical(as.character(tt_decide(details = TRUE, input, "Ready?", threshold = band, client = stub)$status),
                    c("unsure", "failed", "failed")))
summary <- paste0("failed: ", c("record.missing", "record.empty"), ": ",
  tt_decide(details=TRUE, input, "Ready?", client=stub)$error_message[2:3], collapse="\n")
result <- caught(tt_decide(input, "Ready?", threshold=band, client=stub, errors="na"))
warning <- result$warnings[[1]]
stopifnot(length(result$warnings)==1L, inherits(warning,"thinkthen_record_warning"),
 !inherits(warning,"thinkthen_failed"), warning$code=="record.missing",
 identical(conditionMessage(warning),summary),nrow(warning$rows)==2L,nrow(warning$outcomes)==3L,
 identical(plain(result$value), c(NA,NA,NA)))
failures <- tt_failures(result$value)
stopifnot(inherits(failures,"tbl_df"),identical(failures$position,c(2,3)),is.list(failures$id),
 is.character(failures$error_code),identical(failures$question,c(NA_character_,NA_character_)),
 identical(failures$error_code,c("record.missing","record.empty")),
 identical(tt_failures(tt_decide(details=TRUE,input,"Ready?",threshold=band,client=stub)),failures),
 nrow(tt_failures(result$value[2:3]))==0L,
 any(grepl(".thinkthen_failures",capture.output(print(result$value)),fixed=TRUE)))
failed <- condition(tt_decide(input,"Ready?",threshold=band,client=stub),"thinkthen_failed","record.missing")
stopifnot(identical(conditionMessage(failed),summary),inherits(failed,"thinkthen_no_answer"))
# Feature: filter keeps paid-for outcomes beside kept originals after failure.
input <- c("alpha", NA_character_, "bravo", "charlie")
failed <- condition(tt_filter(input,"Ready?",client=stub),"thinkthen_failed")
stopifnot(identical(failed$rows$position,2),identical(failed$kept,input[failed$outcomes$position[which(failed$outcomes$answer)]]))
# Feature: a call with no failure stays a plain value with no warning.
clean <- caught(tt_decide(c("alpha", "bravo"), "Ready?", client = stub, errors = "na"))
stopifnot(identical(clean$value, tt_answer(tt_decide(details = TRUE, c("alpha", "bravo"), "Ready?", client = stub))),
          is.null(attributes(clean$value)), length(clean$warnings) == 0L)
empty <- tt_failures(clean$value)
stopifnot(nrow(empty) == 0L, identical(vctrs::vec_ptype(empty[names(empty)!="item"]), vctrs::vec_ptype(failures[names(failures)!="item"])))
for (value in list(c(1, 2), NULL, data.frame(a = 1), tt_decide(details = TRUE, "alpha", "Ready?", client = stub),
                   tt_annotate(details = TRUE, "alpha", tools, client = stub))) {
  stopifnot(nrow(tt_failures(value))==0L, identical(names(tt_failures(value)),names(empty)))
}
# Feature: every plain verb keeps its failures on its own answer type.
levels <- c("low", "medium", "high")
input <- c("Ada knows Bea", NA)
verbs <- list(
  function(x, q, ...) tt_choose(x, q, c("left", "right"), ...),
  function(x, q, ...) tt_tag(x, q, c("left", "right"), ...),
  function(x, q, ...) tt_score(x, q, levels, ...),
  function(x, q, ...) tt_rank(x, q, levels, ...),
  tt_find, tt_decide,
  function(x, q, ...) tt_recognize(x, "person", ...),
  function(x, q, ...) tt_relate(x, rule, ...))
for (verb in verbs) {
  result <- caught(verb(input, "Ready?", client = stub, errors = "na"))
  stopifnot(length(result$warnings) == 1L, identical(tt_failures(result$value)$position, 2))
  capture.output(print(result$value))
}
# Feature: positions count the input after a rank reorders its rows.
ranked <- tt_rank(details = TRUE, c("", "alpha", "bravo", NA), "How much?", levels, client = stub)
stopifnot(!identical(ranked$position, c(1, 2, 3, 4)), identical(tt_failures(ranked)$position, c(1, 4)))
# Feature: annotation failures name their question and count records, not entries.
records <- list(list(body = "text"), NULL, list(body = ""))
absent <- tt_annotation_questions(list(body = tt_decide_question("Ready?", field = "/body"),
  gone = tt_decide_question("Ready?", field = "/gone"), lost = tt_decide_question("Ready?", field = "/lost")))
result <- caught(tt_annotate(records, absent, client = stub, errors = "na"))
failures <- tt_failures(result$value)
stopifnot(inherits(result$value,"tbl_df"),length(result$warnings)==1L,
 identical(failures$position,c(1,2,3)),identical(failures$question,c("gone",NA,"body")),
 identical(failures$error_code,c("record.missing","record.missing","record.empty")))
failed <- condition(tt_annotate(records,absent,client=stub),"thinkthen_failed","record.missing")
stopifnot(length(strsplit(conditionMessage(failed),"\n",fixed=TRUE)[[1]])==3L)
# Feature: an unset backend argument reads the project environment.
withr::with_envvar(c(THINKTHEN_BACKEND = "stub"), {
  env_client <- tt_client()
  stopifnot(as.character(tt_decide(details = TRUE, "alpha", "Ready?", client = env_client)$source[[1]]$backend) == "stub")
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
  stopifnot("x" %in% names(formals(fn)), if(verb=="filter") !any(c("errors","details") %in% names(formals(fn))) else all(c("errors","details") %in% names(formals(fn))))
}
# Feature: annotation columns retain their declared types with no successful rows.
for (input in list(character(), c(NA_character_, ""))) {
  columns <- plain(tt_annotate(input, tools, client = stub, errors = "na"))
  stopifnot(identical(names(columns), c("ready", "team")),
            nrow(columns) == length(input), is.logical(columns$ready),
            is.factor(columns$team), all(is.na(columns$ready)), all(is.na(columns$team)))
}
# Feature: environment defaults remain raw and an explicit backend wins.
condition(tt_client(backend = NA_character_), "thinkthen_usage_error", "usage.settings")
withr::with_envvar(c(THINKTHEN_BACKEND = ""), {
  condition(tt_client(), "thinkthen_usage_error")
  explicit <- tt_client(backend = "stub")
  stopifnot(as.character(tt_decide(details = TRUE, "alpha", "Ready?", client = explicit)$source[[1]]$backend) == "stub")
  tt_close(explicit)
})
tt_close(stub)
cat("R answer-first public interface passed\n")

# Feature: usage copies survive closing without keeping the client engine alive.
condition(tt_client(backend = "stub", max_requests = -1), "thinkthen_usage_error")
usage_client <- tt_client(backend = "stub", max_requests = 0)
usage_before <- tt_usage(usage_client)
stopifnot(usage_before$attempts == 0, usage_before$input_tokens$reported == 0)
tt_close(usage_client)
stopifnot(identical(tt_usage(usage_client), usage_before))
usage_before$attempts <- 9
stopifnot(tt_usage(usage_client)$attempts == 0)
