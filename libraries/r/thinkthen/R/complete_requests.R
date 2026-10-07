# Private host request builders. Native code owns parsing, admission and sends.
.tt_complete_request <- function(verb, question, input, controls = list()) {
  expected <- list(decide = c("DecideSpec", "QuestionFile"), choose = c("ChooseSpec", "QuestionFile"),
    tag = c("TagSpec", "QuestionFile"), score = c("ScoreSpec", "QuestionFile"),
    filter = c("DecideSpec", "QuestionFile"), rank = c("DecideSpec", "ScoreSpec", "QuestionSet", "QuestionFile"),
    find = c("FindSpec", "QuestionFile"), annotate = c("QuestionSet", "QuestionFile"),
    recognize = c("RecognitionSpec", "QuestionFile"), relate = c("RelationSpec", "QuestionFile"))
  kind <- function(v) sub("^thinkthen_", "", class(v)[1L])
  fail <- function(message) stop(message, call. = FALSE)
  if (!kind(question) %in% expected[[verb]]) fail("wrong question kind")
  if (!kind(input) %in% c("TextInput", "RecordInput", "CandidateInput", "ImageInput", "Files")) fail("explicit input required")
  # Round-trip pure carriers to take a deep snapshot; the native grammar is not parsed here.
  snapshot <- function(v) {
    type <- kind(v)
    present <- .tt_complete_plain(v)
    .tt_complete_decode(type, present)
  }
  question <- snapshot(question)
  input <- snapshot(input)
  if (inherits(controls, "thinkthen_Controls")) controls <- snapshot(controls) else controls <- .tt_complete_decode("Controls", controls)
  type <- kind(input)
  has <- .tt_complete_has
  images <- type == "ImageInput" || (type == "Files" && identical(input$media, "image"))
  if (images && !verb %in% c("decide", "choose", "score")) fail("this function is text-only")
  if (type == "CandidateInput" && verb != "find") fail("candidates require find")
  if (type == "TextInput" && verb %in% c("filter", "rank", "find", "annotate", "relate")) fail("this function requires a complete record set")
  if (type == "Files") {
    if ((input$unit == "window") != !inherits(input$window, "thinkthen_absent")) fail("window requires window units and a size")
    if (identical(input$media, "image") && input$unit != "file") fail("image sources require file units")
  }
  if (type == "ImageInput" && !length(input$images)) fail("images require attachments")
  if (!inherits(controls$top, "thinkthen_absent") && verb != "rank") fail("top requires rank")
  if (!inherits(controls$none, "thinkthen_absent") && verb != "find") fail("none requires find")
  structure(list(function_name = verb, question = question, input = input, controls = controls), class = "thinkthen_complete")
}
.tt_complete_decide <- function(question, input, controls = list()) .tt_complete_request("decide", question, input, controls)
.tt_complete_choose <- function(question, input, controls = list()) .tt_complete_request("choose", question, input, controls)
.tt_complete_tag <- function(question, input, controls = list()) .tt_complete_request("tag", question, input, controls)
.tt_complete_score <- function(question, input, controls = list()) .tt_complete_request("score", question, input, controls)
.tt_complete_filter <- function(question, input, controls = list()) .tt_complete_request("filter", question, input, controls)
.tt_complete_rank <- function(question, input, controls = list()) .tt_complete_request("rank", question, input, controls)
.tt_complete_find <- function(question, input, controls = list()) .tt_complete_request("find", question, input, controls)
.tt_complete_annotate <- function(question, input, controls = list()) .tt_complete_request("annotate", question, input, controls)
.tt_complete_recognize <- function(question, input, controls = list()) .tt_complete_request("recognize", question, input, controls)
.tt_complete_relate <- function(question, input, controls = list()) .tt_complete_request("relate", question, input, controls)
