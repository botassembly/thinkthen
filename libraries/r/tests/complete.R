# Pure fixtures do not qualify native complete execution or parity cells.
source("thinkthen/R/complete.R")
source("thinkthen/R/complete_requests.R")
f <- jsonlite::fromJSON("../python/tests/fixtures/complete.json", simplifyVector = FALSE)
same_json <- function(a, b) {
  if (is.list(a) && is.list(b)) {
    if (length(a) != length(b) || is.null(names(a)) != is.null(names(b))) return(FALSE)
    if (!is.null(names(a))) {
      if (!setequal(names(a), names(b))) return(FALSE)
      a <- a[names(b)]
    }
    return(all(mapply(same_json, a, b)))
  }
  identical(a, b)
}
for (row in f$results) {
  result <- .tt_complete_decode(row$type, row$result)
  occurrence <- row$result; occurrence$index <- 0
  stopifnot(.tt_complete_decode(row$type, occurrence)$index == 0)
  occurrence$index <- -1
  stopifnot(inherits(try(.tt_complete_decode(row$type, occurrence),silent=TRUE),"try-error"))
  encoded <- jsonlite::fromJSON(.tt_complete_json_text(result), simplifyVector = FALSE)
  stopifnot(same_json(encoded, row$result), inherits(result$answer_id, "thinkthen_AnswerId"),
            identical(.tt_complete_plain(result$meta$requests), list(paste(rep("e", 64), collapse=""), paste(rep("e", 64), collapse=""))))
}
candidate <- .tt_complete_decode("FindCandidate",list(index=0,input=FALSE,probability=.25,source=list(file="é.txt",first_line=2,last_line=2)))
stopifnot(identical(candidate$input,FALSE),candidate$source$first_line==2,candidate$probability==.25)
decide <- .tt_complete_decode("DecideResult", f$results[[1L]]$result)
stopifnot(identical(decide$value, FALSE), is.null(decide$question$true))
for (value in list("private reading", list(private=list(FALSE,NULL)), list("private reading",2L), NULL)) {
  original <- f$results[[1L]]$result; original["value"] <- list(value)
  original$question <- list(verb="decide",text="Q",false=value)
  result <- .tt_complete_decode("DecideResult",original)
  stopifnot(same_json(jsonlite::fromJSON(.tt_complete_json_text(result),simplifyVector=FALSE),original),
            !any(grepl("private",capture.output(print(result)))))
}
choose <- .tt_complete_decode("ChooseResult", f$results[[2L]]$result)
stopifnot(is.null(choose$value), choose$answer$confidence == 0,
          identical(names(choose$answer$probabilities), c("b", "a")),
          identical(choose$position$images, list("red.png", "blue.png", "red.png")),
          inherits(choose$position$first, "thinkthen_absent"))
annotation <- .tt_complete_decode("AnnotateResult", f$results[[8L]]$result)
stopifnot(is.null(annotation$answers$ok$value), annotation$answers$bad$failure$cause == "missing_answer")
recognition <- .tt_complete_decode("RecognizeResult", f$results[[9L]]$result)
stopifnot(recognition$value$entities[[1L]]$end == 2, is.null(recognition$answer$names[[1L]]$edges))
relation <- .tt_complete_decode("RelateResult", f$results[[10L]]$result)
stopifnot(relation$value[[1L]]$source$file == "é.txt", relation$value[[1L]]$target$file == "é.txt",
          inherits(relation$answer$questions[[2L]], "thinkthen_RelationFailure"))
empty <- .tt_complete_decode("RelateResult", f$empty$result)
stopifnot(is.null(empty$meta$origin), identical(empty$meta$cached, FALSE), inherits(empty$meta$answered_by, "thinkthen_absent"))
facts <- .tt_complete_decode("Facts", f$facts)
stopifnot(inherits(facts$call_id, "thinkthen_CallId"), inherits(facts$command_ms, "thinkthen_absent"))
for (error in f$errors) stopifnot(inherits(.tt_complete_decode("CallError", error)$facts, "thinkthen_absent"))
failed <- .tt_complete_decode("CallError", f$started_error)
stopifnot(failed$attempts[[1L]]$server_ms == 0, inherits(failed$attempts[[1L]]$sdk_request_id, "thinkthen_SdkRequestId"))
refuses <- function(expr) {
  error <- tryCatch({force(expr); NULL}, error = identity)
  stopifnot(inherits(error, "error"), conditionMessage(error) == "invalid complete result")
}
for (id in c(paste(rep("A",64),collapse=""), paste(rep("a",63),collapse=""), "0")) refuses(.tt_complete_decode("AnswerId", id))
for (change in list(list(schema="thinkthen.result/1"), list(value=0), list(answer=list(kind="yes_no",probability=TRUE)), list(proxy=NULL), list(position=list(file="x",first=4)))) {
  row <- f$results[[1L]]$result
  row[names(change)] <- change
  refuses(.tt_complete_decode("DecideResult", row))
}
for (change in list(list(origin="proxy"),list(cached=FALSE),list(answered_by="invented"),list(observations=list()),list(failed_questions=1))) {
  meta <- f$results[[1L]]$result$meta
  meta[names(change)] <- change
  refuses(.tt_complete_decode("Meta", meta))
}
child <- f$results[[1L]]$result[c("schema","answer_id","question","answer","meta")]
child$value <- 3L; child["threshold"] <- list(NULL);child$meta$usage <- list(input_tokens=2L)
parent <- f$results[[6L]]$result
parent$question <- child$question;parent$answer <- child$answer;parent$question_name <- "saved"
parent$members <- list(list(name="saved",result=child))
ranked <- .tt_complete_decode("RankResult",parent)
stopifnot(ranked$members[[1L]]$result$value==3,inherits(ranked$members[[1L]]$result$meta$usage$output_tokens,"thinkthen_absent"))
stopifnot(same_json(jsonlite::fromJSON(.tt_complete_json_text(ranked),simplifyVector=FALSE),parent))
for (change in list(list(value=0),list(input=FALSE),list(members=list()))) {
  invalid <- parent;invalid$members[[1L]]$result[names(change)] <- change
  refuses(.tt_complete_decode("RankResult",invalid))
}
for (members in list(list(),NULL,list(list(name="saved")))) {
  invalid <- parent;invalid["members"] <- list(members)
  refuses(.tt_complete_decode("RankResult",invalid))
}
refuses(.tt_complete_decode("Usage",list()))
record <- .tt_complete_decode("RecordInput", list(records=list(FALSE,NULL,list(id=1),list(id=1)),context=list(context=list())))
specs <- list(
  decide=list(type="DecideSpec",raw=list(decide=list("Q",list(active=FALSE)),false=NULL)),
  choose=list(type="ChooseSpec",raw=list(choose="Q",options=list(b=NULL,a=list(nested=list(FALSE))))),
  tag=list(type="TagSpec",raw=list(tag="Q",labels=list("a"))),
  score=list(type="ScoreSpec",raw=list(score="Q",levels=list("low","high"))),
  filter=list(type="DecideSpec",raw=list(decide="Q")),
  rank=list(type="ScoreSpec",raw=list(score="Q",levels=list("low","high"))),
  find=list(type="FindSpec",raw=list(find="Q",none=TRUE)),
  annotate=list(type="QuestionSet",raw=list(version=1,questions=list(a=list(decide="Q",false=NULL)))),
  recognize=list(type="RecognitionSpec",raw=list(version=1,recognize=list(kinds=list(person=list(nested=list(FALSE)))))),
  relate=list(type="RelationSpec",raw=list(version=1,relate=list(relations=list(list(name="knows",source="*",target="*")))))
)
for (verb in names(specs)) {
  one <- specs[[verb]]
  request <- get(paste0(".tt_complete_",verb))(.tt_complete_decode(one$type,one$raw),record)
  stopifnot(identical(.tt_complete_plain(request$question),one$raw),identical(.tt_complete_plain(request$input),.tt_complete_plain(record)))
}
image <- list(data=as.raw(c(1,2,3)),name="red.png")
images <- .tt_complete_decode("ImageInput",list(images=list(image,list(data=as.raw(4)),image),text=NULL))
request <- .tt_complete_decide(.tt_complete_decode("DecideSpec",list(decide="Q")),images)
images$images[[1L]]$data[1L] <- as.raw(99)
stopifnot(identical(request$input$images[[1L]]$data,as.raw(c(1,2,3))),identical(request$input$images[[3L]]$data,as.raw(c(1,2,3))),is.null(request$input$text))
for (verb in c("tag","filter","rank","find","annotate","recognize","relate")) {
  one <- specs[[verb]]
  error <- tryCatch(get(paste0(".tt_complete_",verb))(.tt_complete_decode(one$type,one$raw),images),error=identity)
  stopifnot(inherits(error,"error"),conditionMessage(error)=="this function is text-only")
}

corpus <- jsonlite::fromJSON("../../specification/fixtures/question-file/corpus.json", simplifyVector = FALSE)
for (row in corpus$cases) {
  if (row$valid) {
    type <- if (row$verb == "relate") "RelationSpec" else paste0(toupper(substr(row$verb,1L,1L)),substring(row$verb,2L),"Spec")
    stopifnot(same_json(jsonlite::fromJSON(.tt_complete_json_text(.tt_complete_decode(type,row$file)),simplifyVector=FALSE),row$file))
  }
}
refuses(.tt_complete_decode("DecideSpec",list(decide=FALSE)))
refuses(.tt_complete_decode("ChooseSpec",list(choose="Q",options=list(a=TRUE))))
refuses(.tt_complete_decode("ChooseSpec",list(choose="Q",options=list("a","b"),threshold=0)))
refuses(.tt_complete_decode("QuestionSet",list(version=1,questions=list(ready=list(decide="Q",profile="other")))))
raw <- list(decide=structure(list(),names=character()),on="/body",false=NULL)
stopifnot(same_json(.tt_complete_plain(.tt_complete_decode("DecideSpec",raw)),raw))

# The existing wrapper also checks that these pure tests sent nothing.
if (nzchar(Sys.getenv("TT_BACKEND_OUT"))) cat("expect count 0\n")

cat("private complete carrier fixtures: pass\n")

for (raw in f$observed_facts) {
  facts <- .tt_complete_decode("Facts", raw)
  stopifnot(identical(facts$largest_request_bytes, raw$largest_request_bytes),
    identical(facts$largest_request_estimated_input_tokens, raw$largest_request_estimated_input_tokens),
    identical(facts$token_estimate_method, raw$token_estimate_method),
    same_json(.tt_complete_plain(facts$usage_persistence), raw$usage_persistence),
    same_json(.tt_complete_plain(facts), raw))
}
