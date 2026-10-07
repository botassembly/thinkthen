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
      # R stays on its main thread while the native batch owns the held call.
      until <- Sys.time() + 0.15
      repeat {
        stream$poll()
        if (Sys.time() >= until) break
      }
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
