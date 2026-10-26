# Locked canonical snapshots contain no caller-owned mutable R values.
.tt_snapshot <- function(json, classes) {
  value <- new.env(parent = emptyenv())
  value$json <- json
  class(value) <- classes
  lockEnvironment(value, bindings = TRUE)
  value
}
.tt_snapshot_value <- function(value) jsonlite::fromJSON(value$json, simplifyVector = FALSE)
print.tt_question <- function(x, ...) { cat("<", class(x)[[1]], ">\n", sep = ""); invisible(x) }

.tt_definition <- function(verb, question, options) {
  force(question); force(options)
  if (inherits(question, "tt_question")) {
    if (!inherits(question, paste0("tt_", verb, "_question"))) {
      .tt_native("r_question_conversion_error")
    }
    definition <- .tt_snapshot_value(question)
    if (length(options)) {
      .tt_native("r_admit",
        .tt_context_json(list(question = c(definition, options)), "r_question_conversion_error"), 0L)
      .tt_native("r_question_conversion_error")
    }
    return(definition)
  }
  .tt_conversion("r_question_conversion_error", {
    for (field in intersect(c("choices", "labels", "levels", "kinds"), names(options))) {
      options[[field]] <- I(options[[field]])
    }
    if (is.numeric(options$threshold) && length(options$threshold) == 2L) {
      options$threshold <- list(low = options$threshold[[1]], high = options$threshold[[2]])
    }
    field <- switch(verb, recognize = "kinds", relate = "rules", annotate = "questions", "question")
    if (verb == "recognize") question <- I(question)
    if (verb == "relate" && !is.null(question)) {
      question <- .tt_rule_values(question)
    }
    if (verb == "annotate") {
      if (!inherits(question, "tt_annotation_questions")) .tt_native("r_question_conversion_error")
      question <- .tt_snapshot_value(question)
    }
    c(list(`function` = verb), setNames(list(question), field), options)
  })
}

.tt_admit_question <- function(verb, question, options) {
  definition <- .tt_definition(verb, question, options)
  json <- .tt_native("r_admit",
    .tt_context_json(list(question = definition), "r_question_conversion_error"), 0L)$value
  .tt_snapshot(json, c(paste0("tt_", verb, "_question"), "tt_question"))
}

tt_decide_question <- function(question, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("decide", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots))
}
tt_choose_question <- function(question, choices, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(choices)) choices <- NULL
  .tt_admit_question("choose", question, .tt_options(choices = choices, threshold = threshold, evidence = evidence, .dots = dots))
}
tt_tag_question <- function(question, labels, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(labels)) labels <- NULL
  .tt_admit_question("tag", question, .tt_options(labels = labels, threshold = threshold, evidence = evidence, .dots = dots))
}
tt_score_question <- function(question, levels, ..., evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(levels)) levels <- NULL
  .tt_admit_question("score", question, .tt_options(levels = levels, evidence = evidence, .dots = dots))
}
tt_filter_question <- function(question, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("filter", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots))
}
tt_rank_question <- function(question, levels, ..., top = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(levels)) levels <- NULL
  .tt_admit_question("rank", question, .tt_options(levels = levels, top = top, evidence = evidence, .dots = dots))
}
tt_find_question <- function(question, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("find", question, .tt_options(threshold = threshold, evidence = evidence, .dots = dots))
}
tt_recognize_question <- function(kinds, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(kinds)) kinds <- NULL
  .tt_admit_question("recognize", kinds, .tt_options(threshold = threshold, evidence = evidence, .dots = dots))
}
tt_relate_question <- function(rules, ..., threshold = NULL, evidence = NULL) {
  dots <- .tt_dots(...)
  if (missing(rules)) rules <- NULL
  .tt_admit_question("relate", rules, .tt_options(threshold = threshold, evidence = evidence, .dots = dots))
}

tt_rule <- function(source, target, relation, ..., reads = NULL, either = FALSE) {
  dots <- .tt_dots(...)
  rule <- .tt_options(source = source, target = target, relation = relation, reads = reads, either = either, .dots = dots)
  definition <- list(`function` = "relate", rules = list(rule))
  json <- .tt_native("r_admit", .tt_context_json(list(question = definition), "r_question_conversion_error"), 0L)$value
  admitted <- jsonlite::fromJSON(json, simplifyVector = FALSE)$rules[[1]]
  fields <- lapply(admitted, function(value) if (is.null(value)) NA_character_ else value)
  vctrs::new_rcrd(fields, class = "tt_rule")
}
format.tt_rule <- function(x, ...) vctrs::field(x, "relation")
vec_ptype_abbr.tt_rule <- function(x, ...) "tt_rule"
vec_ptype2.tt_rule.tt_rule <- function(x, y, ...) vctrs::vec_slice(x, integer())
vec_cast.tt_rule.tt_rule <- function(x, to, ...) x

.tt_rule_values <- function(rules) {
  if (inherits(rules, "vctrs_list_of")) lapply(rules, function(rule) {
    if (!inherits(rule, "tt_rule") || length(rule) != 1L) .tt_native("r_question_conversion_error")
    setNames(lapply(vctrs::fields(rule), function(field) vctrs::field(rule, field)), vctrs::fields(rule))
  }) else .tt_native("r_question_conversion_error")
}

.tt_question_values <- function(questions) {
  force(questions)
  .tt_conversion("r_question_conversion_error", {
    if (!is.list(questions)) .tt_native("r_question_conversion_error")
    lapply(questions, function(question) {
      if (!inherits(question, "tt_question")) .tt_native("r_question_conversion_error")
      .tt_snapshot_value(question)
    })
  })
}

tt_toolbox <- function(questions) {
  values <- .tt_question_values(questions)
  json <- .tt_native("r_admit", .tt_context_json(values, "r_question_conversion_error"), 2L)$value
  .tt_snapshot(json, "tt_toolbox")
}

tt_toolbox_from_json <- function(json) {
  json <- .tt_native("r_admit", json, 3L)$value
  .tt_snapshot(json, "tt_toolbox")
}

tt_toolbox_to_json <- function(toolbox) {
  if (!inherits(toolbox, "tt_toolbox")) .tt_native("r_question_conversion_error")
  .tt_native("r_admit", toolbox$json, 4L)$value
}

tt_question <- function(toolbox, name) {
  force(toolbox); force(name)
  .tt_conversion("r_question_conversion_error", {
    if (!inherits(toolbox, "tt_toolbox") || !.tt_json_finite(name)) .tt_native("r_question_conversion_error")
    json <- .tt_native("r_question", toolbox$json, name)$value
    verb <- jsonlite::fromJSON(json, simplifyVector = FALSE)[["function"]]
    .tt_snapshot(json, c(paste0("tt_", verb, "_question"), "tt_question"))
  })
}

tt_annotation_questions <- function(questions) {
  values <- .tt_question_values(questions)
  definition <- list(`function` = "annotate", questions = values)
  json <- .tt_native("r_admit", .tt_context_json(list(question = definition), "r_question_conversion_error"), 0L)$value
  .tt_snapshot(.tt_json(jsonlite::fromJSON(json, simplifyVector = FALSE)$questions), "tt_annotation_questions")
}
