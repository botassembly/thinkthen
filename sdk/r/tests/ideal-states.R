# Prove state and error policy behind the literal form routing output.
scenario <- Sys.getenv("R_GUIDE_SCENARIO")
expected <- if(scenario %in% c("refund-failed","topic-failed","urgency-failed")) sub("-failed$","",scenario) else NULL
stopifnot(all(forms$status=="answered"),identical(forms$item,messages))
if(!is.null(expected)) {
  stopifnot(all(forms[[paste0(expected,"_status")]]=="failed"))
  for(name in setdiff(c("refund","topic","urgency"),expected)) stopifnot(all(forms[[paste0(name,"_status")]]=="answered"))
}
if(scenario %in% c("refund-unsure","topic-unsure")) {
  target <- sub("-unsure$","",scenario)
  stopifnot(all(forms[[paste0(target,"_status")]]=="unsure"),all(is.na(forms[[target]])))
}
plain_error <- tryCatch({tt_annotate(messages,questions);NULL},thinkthen_failed=identity)
if(scenario=="failed" || !is.null(expected)) {
  stopifnot(inherits(plain_error,"thinkthen_failed"),inherits(plain_error,"thinkthen_no_answer"),nrow(plain_error$rows)==3L,nrow(plain_error$outcomes)==3L)
  warning <- NULL
  plain <- withCallingHandlers(tt_annotate(messages,questions,errors="na"),warning=function(w){warning<<-w;invokeRestart("muffleWarning")})
  stopifnot(inherits(warning,"thinkthen_record_warning"),nrow(tt_failures(plain))==3L)
} else {
  stopifnot(is.null(plain_error))
  if(scenario %in% c("unsure","refund-unsure","topic-unsure")) {
    signal <- NULL
    value <- withCallingHandlers(tryCatch(tt_annotate(messages,questions),error=function(e)stop("unsure became an error")),thinkthen_unsure=function(e)signal<<-e)
    stopifnot(inherits(signal,"thinkthen_no_answer"),!inherits(signal,"error"),!inherits(signal,"thinkthen_error"),nrow(signal$rows)==3L,nrow(tt_failures(value))==0L)
  }
}
path <- tempfile()
tt_details_write(forms,path)
wire <- lapply(readLines(path, encoding="UTF-8"),jsonlite::fromJSON,simplifyVector=FALSE)
stopifnot(length(wire)==length(messages),wire[[1]]$status=="answered",is.null(wire[[1]]$item))
for(name in c("refund","topic","urgency")) stopifnot(wire[[1]]$answer[[name]]$status==as.character(forms[[paste0(name,"_status")]][1]))
# A one-row form is still a table: failed entries are reported after collection,
# while details retain them without a warning. The condition rename adds no alias.
one_warning <- NULL
one <- withCallingHandlers(tt_annotate(messages[1],questions,details=TRUE),warning=function(w){one_warning<<-w;invokeRestart("muffleWarning")})
stopifnot(is.null(one_warning),nrow(one)==1L,one$status=="answered")
for(name in c("refund","topic","urgency")) {
  stopifnot(identical(as.character(one[[paste0(name,"_status")]]),as.character(forms[[paste0(name,"_status")]][1])))
  if(scenario=="failed") stopifnot(one[[paste0(name,"_status")]]=="failed")
}
one_error <- tryCatch({tt_annotate(messages[1],questions);NULL},thinkthen_failed=identity)
if(scenario=="failed" || !is.null(expected)) {
  first <- if(is.null(expected)) "refund" else expected
  stopifnot(inherits(one_error,"thinkthen_failed"),inherits(one_error,"thinkthen_no_answer"),inherits(one_error,"thinkthen_error"),
    !inherits(one_error,"thinkthen_record_error"),nrow(one_error$rows)==1L,nrow(one_error$outcomes)==1L,
    identical(one_error$code,one[[paste0(first,"_error_code")]][1]),
    identical(one_error$http_status,if(is.na(one[[paste0(first,"_error_http_status")]][1])) NULL else one[[paste0(first,"_error_http_status")]][1]),
    grepl("failed: ",conditionMessage(one_error),fixed=TRUE))
  one_warning <- NULL
  coerced <- withCallingHandlers(tt_annotate(messages[1],questions,errors="na"),warning=function(w){
    stopifnot(is.null(one_warning));one_warning<<-w;invokeRestart("muffleWarning")
  })
  stopifnot(inherits(one_warning,"thinkthen_record_warning"),!inherits(one_warning,"thinkthen_failed"),nrow(tt_failures(coerced))==1L)
} else stopifnot(is.null(one_error))
