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
    options <- .tt_described(options)
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
      if (inherits(question, "tt_toolbox")) {
        entries <- names(.tt_snapshot_value(question))
        question <- tt_annotation_questions(setNames(lapply(entries, function(name) tt_question(question, name)), entries))
      }
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

#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_decide_question <- function(question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("decide", question, .tt_options(threshold = threshold, `true` = yes_means, `false` = no_means, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_choose_question <- function(question, choices, ..., descriptions = NULL, threshold = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(choices)) choices <- NULL
  .tt_admit_question("choose", question, .tt_options(choices = choices, descriptions = descriptions, threshold = threshold, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_tag_question <- function(question, labels, ..., descriptions = NULL, threshold = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(labels)) labels <- NULL
  .tt_admit_question("tag", question, .tt_options(labels = labels, descriptions = descriptions, threshold = threshold, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_score_question <- function(question, levels, ..., descriptions = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(levels)) levels <- NULL
  .tt_admit_question("score", question, .tt_options(levels = levels, descriptions = descriptions, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_filter_question <- function(question, ..., threshold = NULL, yes_means = NULL, no_means = NULL, field = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("filter", question, .tt_options(threshold = threshold, `true` = yes_means, `false` = no_means, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_rank_question <- function(question, levels, ..., descriptions = NULL, top = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(levels)) levels <- NULL
  .tt_admit_question("rank", question, .tt_options(levels = levels, descriptions = descriptions, top = top, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_find_question <- function(question, ..., threshold = NULL, field = NULL) {
  dots <- .tt_dots(...)
  .tt_admit_question("find", question, .tt_options(threshold = threshold, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_recognize_question <- function(kinds, ..., threshold = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(kinds)) kinds <- NULL
  .tt_admit_question("recognize", kinds, .tt_options(threshold = threshold, field = field, .dots = dots))
}
#' Admit an immutable question and its domain through the shared engine.
#'
#' @rdname tt_decide_question
#' @export
tt_relate_question <- function(rules, ..., threshold = NULL, field = NULL) {
  dots <- .tt_dots(...)
  if (missing(rules)) rules <- NULL
  .tt_admit_question("relate", rules, .tt_options(threshold = threshold, field = field, .dots = dots))
}

#' Admit one typed relation rule for a single rule or list of rules.
#'
#' @rdname tt_rule
#' @export
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
  if (inherits(rules, "tt_rule")) rules <- list(rules)
  if (is.list(rules)) lapply(rules, function(rule) {
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

#' Collect named admitted questions in an immutable toolbox.
#'
#' @rdname tt_toolbox
#' @export
tt_toolbox <- function(questions) {
  values <- .tt_question_values(questions)
  json <- .tt_native("r_admit", .tt_context_json(values, "r_question_conversion_error"), 2L)$value
  .tt_snapshot(json, "tt_toolbox")
}

#' Read the canonical saved toolbox JSON representation.
#'
#' @rdname tt_toolbox
#' @export
tt_toolbox_from_json <- function(json) {
  json <- .tt_native("r_admit", json, 3L)$value
  .tt_snapshot(json, "tt_toolbox")
}

#' Write canonical saved toolbox JSON from an immutable toolbox.
#'
#' @rdname tt_toolbox
#' @export
tt_toolbox_to_json <- function(toolbox) {
  if (!inherits(toolbox, "tt_toolbox")) .tt_native("r_question_conversion_error")
  .tt_native("r_admit", toolbox$json, 4L)$value
}

#' Select an admitted question by name from an immutable toolbox.
#'
#' @rdname tt_toolbox
#' @export
tt_question <- function(toolbox, name) {
  force(toolbox); force(name)
  .tt_conversion("r_question_conversion_error", {
    if (!inherits(toolbox, "tt_toolbox") || !.tt_json_finite(name)) .tt_native("r_question_conversion_error")
    json <- .tt_native("r_question", toolbox$json, name)$value
    verb <- jsonlite::fromJSON(json, simplifyVector = FALSE)[["function"]]
    .tt_snapshot(json, c(paste0("tt_", verb, "_question"), "tt_question"))
  })
}

#' Admit named child questions, excluding rank and nested annotation.
#'
#' @rdname tt_toolbox
#' @export
tt_annotation_questions <- function(questions) {
  values <- .tt_question_values(questions)
  definition <- list(`function` = "annotate", questions = values)
  json <- .tt_native("r_admit", .tt_context_json(list(question = definition), "r_question_conversion_error"), 0L)$value
  .tt_snapshot(.tt_json(jsonlite::fromJSON(json, simplifyVector = FALSE)$questions), "tt_annotation_questions")
}

.tt_described <- function(options) {
  descriptions <- options$descriptions
  fields <- intersect(c("choices", "labels", "levels"), names(options))
  if (is.null(descriptions) || !length(fields)) return(options)
  if (!is.character(descriptions) || is.null(names(descriptions)) || anyNA(names(descriptions)) ||
      anyDuplicated(names(descriptions))) {
    .tt_r_error("`descriptions` must be a named character vector.", "argument")
  }
  field <- fields[[1]]
  domain <- options[[field]]
  options[[field]] <- lapply(domain, function(name) {
    if (name %in% names(descriptions)) list(name = name, description = unname(descriptions[[name]])) else name
  })
  unmatched <- descriptions[!names(descriptions) %in% domain]
  options$descriptions <- if (length(unmatched)) as.list(unmatched) else NULL
  options
}

#' Read a toolbox from a JSON file.
#'
#' @rdname tt_toolbox
#' @export
tt_toolbox_read <- function(path) tt_toolbox_from_json(paste(readLines(path, warn = FALSE), collapse = "\n"))
#' Write a toolbox to a JSON file and return the toolbox invisibly.
#'
#' @rdname tt_toolbox
#' @export
tt_toolbox_write <- function(toolbox, path) {
  writeLines(tt_toolbox_to_json(toolbox), path, useBytes = TRUE)
  invisible(toolbox)
}
