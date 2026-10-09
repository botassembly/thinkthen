library(thinkthen)
incoming <- file("stdin")
document <- jsonlite::fromJSON(readLines(incoming,n=1L,warn=FALSE),simplifyVector=FALSE)
prefix <- list()
plain <- get(".tt_complete_plain",asNamespace("thinkthen"))
tryCatch({
  do.call(tt_engine, document$settings)
  method <- get(paste0("tt_", document$verb, "_complete"), asNamespace("thinkthen"))
  if (isTRUE(document$incremental) || isTRUE(document$held_cancel)) {
    stream<-get(paste0("tt_",document$verb,"_batch"),asNamespace("thinkthen"))(document$question,document$input,attempts=TRUE,cancel=isTRUE(document$cancel),deadline_ms=document$deadline_ms,context=document$shared_context)
    if (isTRUE(document$batch_probe)) {cat("ready\n");flush(stdout());readLines(incoming,n=1L,warn=FALSE)}
    if (isTRUE(document$held_cancel)) {
      # Advance once, then let the owned listener confirm request admission.
      stopifnot(is.null(stream$poll()))
      stopifnot(identical(readLines(incoming,n=1L,warn=FALSE), "continue"))
      stream$cancel()
    }
    repeat {
      row<-stream$next_row()
      if(is.null(row)) break
      prefix[[length(prefix)+1L]]<-row
    }
    done<-list(results=lapply(prefix,`[[`,"result"),facts=stream$facts(),ordinals=lapply(prefix,`[[`,"ordinal"),inputs=lapply(prefix,`[[`,"input"))
  } else done <- method(document$question, document$input, attempts = TRUE, deadline_ms = document$deadline_ms,cancel=isTRUE(document$cancel),context=document$shared_context)
  stopifnot(inherits(done$facts$call_id,"thinkthen_CallId"))
  plain <- get(".tt_complete_plain",asNamespace("thinkthen"))
  facts <- done$facts
  encoded <- plain(facts)
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
  packet <- list(results = lapply(done$results,plain), facts=plain(done$facts), ordinals=done$ordinals, inputs=lapply(done$inputs,plain))
  cat(jsonlite::toJSON(packet,auto_unbox=TRUE,null="null",digits=NA))
}, error=function(e) {
  if (is.null(e$kind)) stop(e)
  if (!is.null(e$complete)) {
    plain<-get(".tt_complete_plain",asNamespace("thinkthen"))
    cat(jsonlite::toJSON(list(completed=if(length(prefix)) list(results=lapply(prefix,function(r) plain(r$result)),facts=plain(e$complete$facts),ordinals=lapply(prefix,`[[`,"ordinal"),inputs=lapply(prefix,function(r) plain(r$input))) else NULL,error=plain(e$complete),facts=if(inherits(e$complete$facts,"thinkthen_absent")) NULL else plain(e$complete$facts)),auto_unbox=TRUE,null="null",digits=NA));return(invisible(NULL))
  }
  cat(jsonlite::toJSON(list(error=list(kind=e$kind,message=conditionMessage(e),retryable=e$retryable),facts=e$facts),auto_unbox=TRUE,null="null",digits=NA))
})
