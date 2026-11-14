# Public promises that are prose in the ideal guide: a valid control precedes
# each refusal so a broken install cannot pass by rejecting everything.
refused <- function(expr, code = NULL, mention = NULL, class = "thinkthen_usage_error") {
  error <- tryCatch({force(expr); NULL}, error=identity)
  stopifnot(inherits(error, class))
  if (!is.null(code)) stopifnot(identical(error$code, code))
  if (!is.null(mention)) stopifnot(grepl(mention, conditionMessage(error), fixed=TRUE))
  invisible(error)
}
client <- tt_client(retry=tt_retry(max_retries=0))
stopifnot(identical(tt_decide("text", question="Ready?", client=client), TRUE))
for (details in c(FALSE, TRUE)) {
  for (name in c("position", "id", "item", "status", "probability", "source", "error_code", "error_message", "error_http_status", "detail")) {
    questions <- tt_annotation_questions(setNames(list(tt_decide_question("Ready?")), name))
    before <- tt_usage(client)$attempts
    if(!details && name=="position" && Sys.getenv("R_GUIDE_NEGATIVE")=="refusal") questions <- tt_annotation_questions(list(valid=tt_decide_question("Ready?")))
    refused(tt_annotate("text", questions, details=details, client=client), "usage.column", name)
    stopifnot(tt_usage(client)$attempts == before)
  }
  for (suffix in c("_status", "_probability", "_error_code", "_error_message", "_error_http_status", "_detail")) {
    questions <- tt_annotation_questions(setNames(rep(list(tt_decide_question("Ready?")),2), c("refund",paste0("refund",suffix))))
    before <- tt_usage(client)$attempts
    refused(tt_annotate("text", questions, details=details, client=client), "usage.column", paste0("refund",suffix))
    stopifnot(tt_usage(client)$attempts == before)
  }
}
# Actual collisions, rather than a blanket suffix reservation; admission occurs
# even without input or a configured client.
q <- tt_annotation_questions(list(answer=tt_decide_question("Ready?"), lone_status=tt_decide_question("Ready?")))
stopifnot(nrow(tt_annotate("text", q, client=client)) == 1L)
withr::with_envvar(c(THINKTHEN_API_KEY=NA, OPENAI_API_KEY=NA), {
  q <- tt_annotation_questions(list(status=tt_decide_question("Ready?")))
  refused(tt_annotate(character(), q), "usage.column", "status")
  refused(tt_client(), "usage.key-missing", class="thinkthen_setup_error")
})
refused(tt_filter("text", question="Ready?", threshold=c(.2,.8),client=client), "usage.threshold", "tt_decide")
refused(tt_filter_question("Ready?",threshold=c(.2,.8)), "usage.threshold", "tt_decide")
refused(tt_toolbox_from_json('{"entries":{"kept":{"question":{"function":"filter","question":"Ready?","threshold":{"low":0.2,"high":0.8}}}}}'), "usage.threshold")
refused(tt_filter("text",question="Ready?",errors="na",client=client))
refused(tt_filter("text",question="Ready?",details=TRUE,client=client))
refused(tt_decide("text",question="Ready?",threshold=2,client=client), "usage.threshold")
refused(tt_question(tt_toolbox(list(ready=tt_decide_question("Ready?"))), "absent"), class="thinkthen_usage_error")
refused(tt_choose("text", tt_decide_question("Ready?"), client=client))
refused(tt_annotation_questions(list(kept=tt_filter_question("Ready?"))))
for (name in c(paste0("tt_",c("decide","choose","tag","score","filter","rank","find","annotate","recognize","relate"),"_rows"), "tt_keep")) stopifnot(!name %in% getNamespaceExports("thinkthen"))
stopifnot(!any(c("errors","details") %in% names(formals(tt_filter))))
# Native enum levels/order and ordinary equality stay R's own.
topic <- tt_choose("text", question="Topic?",choices=c("shipping","billing","other"),client=client)
stopifnot(is.factor(topic), identical(levels(topic),c("shipping","billing","other")), identical(topic,topic), topic == "billing")
tagged <- tt_tag("text", question="Topics?",labels=c("shipping","billing"),client=client)
stopifnot(is.factor(tagged[[1]]),identical(levels(tagged[[1]]), c("shipping","billing")))
# Metadata remains inspectable and printable even when input admission failed.
input <- data.frame(id=c("same","same","same"), text=c("text",NA,""), day=as.Date("2026-01-01")+1:3, row.names=c("one","two","three"))
attr(input$text,"origin") <- "column"
details <- tt_decide(input,question="Ready?",field="/text",details=TRUE,client=client)
stopifnot(inherits(details,"tt_details"),is.character(details$error_code),identical(details$item,input), identical(details$position,c(1,2,3)), identical(levels(details$status),c("answered","unsure","failed")))
invisible(capture.output(print(details)))
error <- refused(tt_decide(input,question="Ready?",field="/text",client=client), class="thinkthen_failed")
expected_items <- input[2:3,,drop=FALSE]
attr(expected_items$text,"origin") <- attr(input$text,"origin")
stopifnot(inherits(error,"thinkthen_no_answer"), nrow(error$rows)==2L, nrow(error$outcomes)==3L, identical(error$rows$item,expected_items),length(strsplit(conditionMessage(error),"\n",fixed=TRUE)[[1]])==2L)
warning <- NULL
answers <- withCallingHandlers(tt_decide(input,question="Ready?",field="/text",client=client,errors="na"),warning=function(w){warning<<-w;invokeRestart("muffleWarning")})
stopifnot(inherits(warning,"thinkthen_record_warning"),identical(as.vector(answers),c(TRUE,NA,NA)),nrow(tt_failures(answers))==2L, nrow(tt_failures(answers[2:3]))==0L)
kept <- refused(tt_filter(input,question="Ready?",field="/text",client=client),class="thinkthen_failed")
expected_items <- input[1,,drop=FALSE]
attr(expected_items$text,"origin") <- attr(input$text,"origin")
stopifnot(identical(kept$kept,expected_items),nrow(kept$rows)==2L)
# The writer preserves exact integer IDs and caller edits; it never saves item.
ids <- list("text-id",tt_id_integer("-9223372036854775808"),tt_id_integer("18446744073709551615"))
records <- lapply(ids,function(id)list(id=id,text="text"))
saved <- tt_decide(records,question="Ready?",details=TRUE,client=client)
saved$answer[1] <- FALSE
path <- tempfile()
tt_details_write(saved,path)
lines <- readLines(path, encoding="UTF-8")
stopifnot(grepl('"answer":false',lines[1],fixed=TRUE),grepl('"id":-9223372036854775808',lines[2],fixed=TRUE),grepl('"id":18446744073709551615',lines[3],fixed=TRUE),!any(grepl('"item"',lines,fixed=TRUE)))
tt_details_write(details,path)
wire <- lapply(readLines(path, encoding="UTF-8"),jsonlite::fromJSON,simplifyVector=FALSE,bigint_as_char=TRUE)
stopifnot(is.null(wire[[2]]$answer),is.null(wire[[2]]$probability),wire[[2]]$error$code=="record.missing")
# Invalid ordinary IDs fail their record, retain item, and admit no identifier.
invalid <- list(list(id=TRUE,text="text"),list(id=1.5,text="text"))
invalid_details <- tt_decide(invalid,question="Ready?",details=TRUE,client=client)
stopifnot(all(invalid_details$status=="failed"),identical(invalid_details$item,invalid),all(vapply(invalid_details$id,is.null,logical(1))))
# Rank details are ordered; the bare column remains input-aligned, including top.
ranked <- tt_rank(c("first","second","third"),question="Urgent?",levels=urgency_levels,top=2,details=TRUE,client=client)
ranks <- tt_rank(c("first","second","third"),question="Urgent?",levels=urgency_levels,top=2,client=client)
stopifnot(identical(ranked$answer,c(1,2)),identical(ranked$item,c("first","second")),identical(ranks,c(1,2,NA)))
# Failed parent and independent failed children have different retained states.
questions <- tt_annotation_questions(list(refund=tt_decide_question("Ready?",field="/missing"),topic=tt_choose_question("Topic?",choices=topic_choices,field="/text")))
forms <- tt_annotate(list(list(text="text"),NULL),questions,details=TRUE,client=client)
stopifnot(forms$status[1]=="answered",forms$refund_status[1]=="failed",forms$topic_status[1]=="answered",forms$status[2]=="failed",is.na(forms$topic_status[2]))
refused(tt_annotate(list(list(text="text")),questions,client=client),class="thinkthen_failed")
tt_details_write(forms,path)
wire <- lapply(readLines(path, encoding="UTF-8"),jsonlite::fromJSON,simplifyVector=FALSE)
stopifnot(wire[[1]]$status=="answered",wire[[1]]$answer$refund$status=="failed",wire[[2]]$status=="failed",is.null(wire[[2]]$answer))
# Host and saved offsets agree on non-ASCII text and omit an answered empty span.
text <- "é🙂\n 答🙂 "
span <- tt_find(text,question="Which line?",details=TRUE,client=client)
stopifnot(substr(text,span$answer$start,span$answer$end)==span$answer$text)
tt_details_write(span,path)
wire <- jsonlite::fromJSON(readLines(path, encoding="UTF-8"),simplifyVector=FALSE)
stopifnot(wire$answer$start==span$answer$start-1,wire$answer$end==span$answer$end,wire$answer$text==" 答🙂 ")
box <- tt_toolbox(setNames(list(tt_choose_question("答🙂?",choices=c("答🙂","other"))),"題"))
tt_toolbox_write(box,path)
stopifnot(identical(tt_toolbox_to_json(tt_toolbox_read(path)),tt_toolbox_to_json(box)))
# A whole failed parent keeps the same table failure and payload, even with one
# input. No transport is needed; details neither raise nor warn.
before <- tt_usage(client)$attempts
parent_warning <- NULL
parent <- withCallingHandlers(tt_annotate(NA_character_,questions,details=TRUE,client=client),warning=function(w){parent_warning<<-w;invokeRestart("muffleWarning")})
stopifnot(is.null(parent_warning),nrow(parent)==1L,parent$status=="failed",parent$error_code=="record.missing")
parent_error <- refused(tt_annotate(NA_character_,questions,client=client),class="thinkthen_failed")
stopifnot(inherits(parent_error,"thinkthen_no_answer"),inherits(parent_error,"thinkthen_error"),!inherits(parent_error,"thinkthen_record_error"),
 identical(parent_error$code,"record.missing"),is.null(parent_error$http_status),nrow(parent_error$rows)==1L,nrow(parent_error$outcomes)==1L,
 parent_error$rows$status=="failed",tt_usage(client)$attempts==before)
tt_close(client)
cat("R ideal refusals, native vectors, packed records and canonical saving passed\n")
