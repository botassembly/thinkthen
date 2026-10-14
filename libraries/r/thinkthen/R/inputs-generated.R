# Generated from the canonical request schema; do not edit.
tt_options <- function(attempts = NULL, batch = NULL, context = NULL, context_field = NULL, deadline_ms = NULL, examples = NULL, examples_field = NULL, field = NULL, files_only = NULL, max_requests_total = NULL, mode = NULL, model = NULL, none = NULL, options_field = NULL, relation_threshold = NULL, seed_spans = NULL, seed_spans_field = NULL, snippet_pieces = NULL, stage_context = NULL, threshold = NULL, top = NULL) {
  values <- list(attempts = attempts, batch = batch, context = context, context_field = context_field, deadline_ms = deadline_ms, examples = examples, examples_field = examples_field, field = field, files_only = files_only, max_requests_total = max_requests_total, mode = mode, model = model, none = none, options_field = options_field, relation_threshold = relation_threshold, seed_spans = seed_spans, seed_spans_field = seed_spans_field, snippet_pieces = snippet_pieces, stage_context = stage_context, threshold = threshold, top = top)
  values[c(!missing(attempts), !missing(batch), !missing(context), !missing(context_field), !missing(deadline_ms), !missing(examples), !missing(examples_field), !missing(field), !missing(files_only), !missing(max_requests_total), !missing(mode), !missing(model), !missing(none), !missing(options_field), !missing(relation_threshold), !missing(seed_spans), !missing(seed_spans_field), !missing(snippet_pieces), !missing(stage_context), !missing(threshold), !missing(top))]
}
.tt_engine_settings <- function(backend = NULL, base_url = NULL, batch = NULL, cache = NULL, max_estimated_input_tokens_total = NULL, max_request_bytes = NULL, max_requests = NULL, max_requests_total = NULL, max_retries = NULL, model = NULL, profile = NULL, proxy = NULL, record = NULL, refresh_cache = NULL, replay = NULL, throttle = NULL, timeout = NULL, usd_per_million_input = NULL, usd_per_million_output = NULL) {
  values <- list(backend = backend, base_url = base_url, batch = batch, cache = cache, max_estimated_input_tokens_total = max_estimated_input_tokens_total, max_request_bytes = max_request_bytes, max_requests = max_requests, max_requests_total = max_requests_total, max_retries = max_retries, model = model, profile = profile, proxy = proxy, record = record, refresh_cache = refresh_cache, replay = replay, throttle = throttle, timeout = timeout, usd_per_million_input = usd_per_million_input, usd_per_million_output = usd_per_million_output)
  values[!vapply(values, is.null, TRUE)]
}
tt_decide <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("decide", question, input, options, deadline_ms, completion)
tt_choose <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("choose", question, input, options, deadline_ms, completion)
tt_tag <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("tag", question, input, options, deadline_ms, completion)
tt_score <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("score", question, input, options, deadline_ms, completion)
tt_filter <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("filter", question, input, options, deadline_ms, completion)
tt_rank <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("rank", question, input, options, deadline_ms, completion)
tt_find <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("find", question, input, options, deadline_ms, completion)
tt_annotate <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("annotate", question, input, options, deadline_ms, completion)
tt_recognize <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("recognize", question, input, options, deadline_ms, completion)
tt_relate <- function(question, input, options = tt_options(), deadline_ms = NULL, completion = NULL) .tt_named_call("relate", question, input, options, deadline_ms, completion)
tt_engine <- function(base_url = NULL, model = NULL, throttle = NULL, max_requests = NULL, max_requests_total = NULL, max_request_bytes = NULL, cache = NULL, timeout = NULL, max_retries = NULL, record = NULL, replay = NULL, profile = NULL, batch = NULL, ..., backend = NULL, max_estimated_input_tokens_total = NULL, proxy = NULL, refresh_cache = FALSE, usd_per_million_input = NULL, usd_per_million_output = NULL) {
  if (length(list(...))) .tt_unknown(...)
  settings <- .tt_engine_settings(base_url = base_url, model = model, throttle = throttle, max_requests = max_requests, max_requests_total = max_requests_total, max_request_bytes = max_request_bytes, cache = cache, timeout = timeout, max_retries = max_retries, record = record, replay = replay, profile = profile, batch = batch, backend = backend, max_estimated_input_tokens_total = max_estimated_input_tokens_total, proxy = proxy, refresh_cache = refresh_cache, usd_per_million_input = usd_per_million_input, usd_per_million_output = usd_per_million_output)
  .tt_call(tt_engine_set(if (length(settings)) .tt_json(settings) else "{}"))
  invisible(NULL)
}
tt_recognize_question <- function(entity_definition = NULL, instructions = NULL, kinds = NULL, mode = NULL, relations = NULL, snippet_pieces = NULL, stage_context = NULL) {
  values <- list(entity_definition = entity_definition, instructions = instructions, kinds = kinds, mode = mode, relations = relations, snippet_pieces = snippet_pieces, stage_context = stage_context)
  tt_question(list(version = 1L, recognize = values[!vapply(values, is.null, TRUE)]))
}
tt_relate_question <- function(fields = NULL, relations = NULL) {
  values <- list(fields = fields, relations = relations)
  tt_question(list(version = 1L, relate = values[!vapply(values, is.null, TRUE)]))
}
