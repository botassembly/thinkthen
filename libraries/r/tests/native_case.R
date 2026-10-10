library(thinkthen)
incoming <- file("stdin")
document <- jsonlite::fromJSON(readLines(incoming,n=1L,warn=FALSE),simplifyVector=FALSE)
prefix <- list()
plain <- get(".tt_complete_plain",asNamespace("thinkthen"))
tryCatch({
  do.call(tt_engine, document$settings)
  method <- get(paste0("tt_", document$verb), asNamespace("thinkthen"))
  selector <- document$question
  question <- switch(selector$kind, file = tt_question(file = selector$path),
    name = tt_question(name = selector$name), reference = tt_question(reference = selector$reference),
    tt_question(selector$value))
  input <- do.call(tt_input, document$input)
  options <- document$options
  if (isTRUE(document$incremental) || isTRUE(document$held_cancel) || isTRUE(document$cancel)) {
    stream <- tt_batch(document$verb, question, input, options)
    if (isTRUE(document$batch_probe)) {cat("ready\n");flush(stdout());readLines(incoming,n=1L,warn=FALSE)}
    if (isTRUE(document$held_cancel)) {
      # Advance once, then let the owned listener confirm request admission.
      stopifnot(is.null(stream$poll()))
      stopifnot(identical(readLines(incoming,n=1L,warn=FALSE), "continue"))
      stream$cancel()
    }
    if (isTRUE(document$cancel)) stream$cancel()
    repeat {
      row<-stream$next_result()
      if(is.null(row)) break
      prefix[[length(prefix)+1L]]<-row
    }
    done<-list(results=prefix,facts=stream$facts())
  } else done <- method(question, input, options)
  stopifnot(inherits(done$facts$call_id,"thinkthen_CallId"))
  plain <- get(".tt_complete_plain",asNamespace("thinkthen"))
  if (!isTRUE(document$incremental) && !isTRUE(document$held_cancel)) stopifnot(inherits(done, "thinkthen_Call"))
  for (result in done$results) {
    stopifnot(inherits(result, "thinkthen_complete"), inherits(result$answer_id, "thinkthen_AnswerId"))
    stopifnot("value" %in% names(plain(result)))
    for (key in c("question_sha256", "questions_sha256", "context_sha256")) {
      digest <- result$meta[[key]]
      if (!inherits(digest, "thinkthen_absent")) stopifnot(inherits(digest, "thinkthen_Digest"),
        identical(capture.output(print(digest)), "<complete identity>"))
    }
    stopifnot(all(vapply(result$meta$requests, inherits, TRUE, "thinkthen_Digest")))
    retained <- result$value
    gc()
    stopifnot(identical(result[["value"]], retained),
              identical(capture.output(print(result)), "<complete carrier: content withheld>"))
  }
  facts <- done$facts
  encoded <- plain(facts)
  absent <- names(facts)[vapply(facts, inherits, TRUE, "thinkthen_absent")]
  stopifnot(!any(absent %in% names(encoded)))
  stopifnot(is.numeric(facts$largest_request_bytes), is.null(facts$largest_request_estimated_input_tokens) || is.numeric(facts$largest_request_estimated_input_tokens), is.character(facts$token_estimate_method))
  for (key in c("largest_request_bytes","largest_request_estimated_input_tokens","token_estimate_method")) stopifnot(key %in% names(encoded), identical(encoded[[key]],facts[[key]]))
  if (!inherits(facts$usage_persistence,"thinkthen_absent")) {
    observation <- facts$usage_persistence
    stopifnot(inherits(observation,"thinkthen_PersistenceObservation"), observation$state %in% c("disabled","pending","written","failed"), observation$observed_at=="facts_snapshot", identical(encoded$usage_persistence,plain(observation)))
  }
  for (result in done$results) {
    if (inherits(result,"thinkthen_RankResult") && !inherits(result$members,"thinkthen_absent")) {
      for (member in result$members) stopifnot(inherits(member,"thinkthen_RankMember"),inherits(member$result,"thinkthen_RankMemberResult"),inherits(member$result$answer_id,"thinkthen_AnswerId"),member$result$value>0,inherits(member$result$question,"thinkthen_DecideQuestion"),inherits(member$result$answer,"thinkthen_YesNo"),inherits(member$result$meta,"thinkthen_Meta"))
    }
  }
  packet <- list(native=TRUE, results = lapply(done$results,plain), facts=plain(done$facts))
  cat(jsonlite::toJSON(packet,auto_unbox=TRUE,null="null",digits=NA))
}, error=function(e) {
  if (is.null(e$kind)) stop(e)
  if (!is.null(e$complete)) {
    stopifnot(inherits(e$complete, "thinkthen_CallError"), inherits(e$complete, "thinkthen_complete"),
      identical(capture.output(print(e$complete)), "<complete carrier: content withheld>"))
    if (!inherits(e$complete$facts, "thinkthen_absent")) {
      stopifnot(inherits(e$complete$facts, "thinkthen_Facts"), inherits(e$complete$facts$call_id, "thinkthen_CallId"))
      gc()
      stopifnot(inherits(e$complete$facts$call_id, "thinkthen_CallId"))
    }
    if (!length(prefix) && !is.null(e$completed)) prefix <- e$completed
    plain<-get(".tt_complete_plain",asNamespace("thinkthen"))
    cat(jsonlite::toJSON(list(completed=if(length(prefix)) list(native=TRUE, results=lapply(prefix,plain),facts=plain(e$complete$facts)) else NULL,error=plain(e$complete),facts=if(inherits(e$complete$facts,"thinkthen_absent")) NULL else plain(e$complete$facts)),auto_unbox=TRUE,null="null",digits=NA));return(invisible(NULL))
  }
  cat(jsonlite::toJSON(list(error=list(kind=e$kind,message=conditionMessage(e),retryable=e$retryable),facts=e$facts),auto_unbox=TRUE,null="null",digits=NA))
})
