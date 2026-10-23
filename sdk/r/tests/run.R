library(thinkthen)
args <- commandArgs(TRUE)
case <- jsonlite::fromJSON(args[[1]], simplifyVector = FALSE)
request <- case$request
settings <- case$client
if (is.null(settings)) settings <- list()
settings$url <- Sys.getenv("THINKTHEN_TEST_URL")
if (!nzchar(settings$url)) settings$url <- NULL
question <- request$question
if (question$type == "toolbox") {
  question <- tt_question(question$value, question$name)
} else if (question$type == "reference") {
  question <- tt_reference(question$name)
} else question <- question$value
records <- request$records
if (!length(records)) records <- character()
if (length(records) && all(vapply(records, function(x) is.character(x) && length(x) == 1L, logical(1)))) {
  records <- unlist(records, use.names = FALSE)
}
options <- request$options
if (is.null(options)) options <- list()
for (key in intersect(c("options", "labels", "levels"), names(options))) {
  options[[key]] <- unlist(options[[key]], use.names = FALSE)
}
function_name <- request[["function"]]
question_name <- switch(function_name, recognize = "kinds", relate = "rules", annotate = "toolbox", "question")
if (function_name == "recognize" && !inherits(question, "thinkthen_question")) question <- as.character(unlist(question, use.names = FALSE))
call_args <- c(list(x = records), setNames(list(question), question_name), options)
result <- tryCatch({
  client <- do.call(tt_client, settings)
  call_args$client <- client
  output <- do.call(get(paste0("tt_", function_name)), call_args)
  if (function_name == "filter") {
    # JSON roundtrip describes the selected originals, not a second judgement API.
    list(selected = as.list(output))
  } else {
    # Reuse the shared unsure reply to prove the host helpers preserve abstention.
    if (basename(args[[1]]) == "decide-unsure.json") {
      frame <- data.frame(body = records)
      attached <- tt_bind(frame, output, "decision")
      stopifnot(identical(tt_answer(output), NA), identical(attached$decision_answer, NA),
                identical(attached$decision_status, "unsure"))
    }
    detail <- tt_detail(output)
    if (function_name == "find" && nrow(output) == 1L && !is.null(output$answer[[1]])) {
      span <- output$answer[[1]]
      stopifnot(identical(substr(records[[1]], span$start + 1L, span$end), span$text))
    }
    rows <- lapply(seq_len(nrow(output)), function(i) {
      row <- list(id = output$id[[i]], status = output$status[[i]])
      if (is.null(output$error[[i]])) {
        answer <- if (is.list(output$answer)) output$answer[[i]] else output$answer[[i]]
        if (!is.list(answer) && length(answer) == 1L && is.na(answer)) answer <- NULL
        if (function_name == "tag" && !is.null(answer)) answer <- as.list(answer)
        # Keep explicit JSON null when the R list is serialized below.
        row["answer"] <- list(answer)
      } else row$error <- list(code = output$error[[i]]$code)
      if (!is.na(output$p[[i]])) row$p <- output$p[[i]]
      if (!is.null(detail[[i]])) row$detail <- detail[[i]]
      row
    })
    list(rows = rows)
  }
}, thinkthen_error = function(error) list(error = error$code, family = class(error)[[1]]))
cat(jsonlite::toJSON(result, auto_unbox = TRUE, null = "null", na = "null", digits = NA))
