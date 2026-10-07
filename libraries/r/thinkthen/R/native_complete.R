# Complete envelopes keep native coordinates; ordinary frame compatibility is separate.
.tt_complete_call <- function(verb, question, input, attempts = FALSE, deadline_ms = NULL, cancel = FALSE, context = NULL) {
  request <- jsonlite::toJSON(list(verb = verb, question = question, input = input, attempts = attempts,cancel=cancel,context=context), auto_unbox = TRUE, null = "null", digits = NA)
  held <- .tt_call(tt_complete_native(request, deadline_ms, NULL))
  payload <- jsonlite::fromJSON(held$value, simplifyVector = FALSE)
  rows <- payload$results
  if (verb %in% c("find", "relate")) rows <- list(rows)
  kind <- paste0(toupper(substr(verb, 1L, 1L)), substr(verb, 2L, nchar(verb)), "Result")
  structure(list(results = lapply(rows, function(row) .tt_complete_decode(kind, row)),
                 facts = .tt_complete_decode("Facts", payload$facts), ordinals = payload$ordinals, inputs = lapply(payload$inputs, function(v) .tt_complete_decode("NativeInput", v))), class = "thinkthen_complete_call")
}

tt_decide_complete <- function(question, input, ...) .tt_complete_call("decide", question, input, ...)
tt_choose_complete <- function(question, input, ...) .tt_complete_call("choose", question, input, ...)
tt_tag_complete <- function(question, input, ...) .tt_complete_call("tag", question, input, ...)
tt_score_complete <- function(question, input, ...) .tt_complete_call("score", question, input, ...)
tt_filter_complete <- function(question, input, ...) .tt_complete_call("filter", question, input, ...)
tt_rank_complete <- function(question, input, ...) .tt_complete_call("rank", question, input, ...)
tt_find_complete <- function(question, input, ...) .tt_complete_call("find", question, input, ...)
tt_annotate_complete <- function(question, input, ...) .tt_complete_call("annotate", question, input, ...)
tt_recognize_complete <- function(question, input, ...) .tt_complete_call("recognize", question, input, ...)
tt_relate_complete <- function(question, input, ...) .tt_complete_call("relate", question, input, ...)
print.thinkthen_complete_call <- function(x, ...) { cat('<CompleteCall: content withheld>\n'); invisible(x) }

.tt_complete_batch <- function(verb,question,input,attempts=FALSE,deadline_ms=NULL,cancel=FALSE,context=NULL) {
  request <- .tt_json(list(verb=verb,question=question,input=input,attempts=attempts,cancel=cancel,context=context))
  native <- .tt_call(tt_complete_batch_start(request,deadline_ms))
  state <- new.env(parent=emptyenv());state$facts<-NULL;state$ended<-FALSE;state$pending<-FALSE
  kind<-paste0(toupper(substr(verb,1L,1L)),substr(verb,2L,nchar(verb)),"Result")
  close<-function() {state$ended<-TRUE;.tt_call(tt_complete_batch_close(native));invisible(NULL)}
  poll<-function() {
    if(state$ended) return(NULL)
    event <- .tt_call(tt_complete_batch_poll(native, !state$pending))
    state$pending <- TRUE
    if (is.null(event)) return(NULL)
    state$pending <- FALSE
    event <- jsonlite::fromJSON(event,simplifyVector=FALSE)
    if(!is.null(event$row)) return(structure(list(result=.tt_complete_decode(kind,event$row),ordinal=event$ordinal,input=.tt_complete_decode("NativeInput",event$input)),class="thinkthen_complete_row"))
    close()
    if(!is.null(event$error)) {
      complete<-.tt_complete_decode("CallError",event$error)
      state$facts<-complete$facts
      stop(structure(list(message=event$error$message,kind=event$error$kind,retryable=event$error$retryable,complete=complete,call=NULL),class=c(paste0("thinkthen_",event$error$kind),"thinkthen_error","error","condition")))
    }
    state$facts<-.tt_complete_decode("Facts",event$facts);NULL
  }
  pull <- function() {
    repeat {
      row <- poll()
      if (!is.null(row) || state$ended) return(row)
    }
  }
  structure(list(next_row=pull,poll=poll,close=close,cancel=function() .tt_call(tt_complete_batch_cancel(native)),facts=function() state$facts),class="thinkthen_complete_batch")
}
tt_decide_batch <- function(question,input,...) .tt_complete_batch("decide",question,input,...)
tt_choose_batch <- function(question,input,...) .tt_complete_batch("choose",question,input,...)
tt_tag_batch <- function(question,input,...) .tt_complete_batch("tag",question,input,...)
tt_score_batch <- function(question,input,...) .tt_complete_batch("score",question,input,...)
tt_filter_batch <- function(question,input,...) .tt_complete_batch("filter",question,input,...)
tt_annotate_batch <- function(question,input,...) .tt_complete_batch("annotate",question,input,...)
print.thinkthen_complete_batch <- function(x,...) {cat('<CompleteBatch>\n');invisible(x)}
print.thinkthen_complete_row <- function(x,...) {cat('<BatchRow: content withheld>\n');invisible(x)}
