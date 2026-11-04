source(file.path(dirname(sub("^--file=", "", grep("^--file=", commandArgs(), value = TRUE))), "support.R"))
# Feature: descriptions retain values and unknown names receive the engine diagnostic.
client <- tt_client(backend = "openai", url = Sys.getenv("THINKTHEN_TEST_URL"), api_key = "synthetic-local-key", retry = tt_retry(max_retries = 0))
for (verb in c("choose", "tag", "score", "rank")) {
  field <- switch(verb, choose = "choices", tag = "labels", "levels")
  definition <- list(`function` = verb, question = "Which?")
  definition[[field]] <- list(list(name = "a", description = ""), "b")
  definition$descriptions <- list(other = "unknown")
  # Raw engine admission is the reference; the public calls below exercise mapping.
  expected <- .Call("wrap__r_admit", jsonlite::toJSON(list(question = definition), auto_unbox = TRUE), 0L, PACKAGE = "thinkthen")$error
  stopifnot(!is.null(expected))
  actual <- condition(do.call(get(paste0("tt_", verb, "_rows")), list("alpha", "Which?", c("a", "b"), descriptions = c(a = "", other = "unknown"), client = client)), "thinkthen_usage_error")
  stopifnot(identical(actual$code, expected$code), identical(conditionMessage(actual), expected$message))
  constructor <- get(paste0("tt_", verb, "_question"))
  actual <- condition(constructor("Which?", c("a", "b"), descriptions = c(other = "unknown")), "thinkthen_usage_error")
  stopifnot(identical(actual$code, expected$code), identical(conditionMessage(actual), expected$message))
  valid <- constructor("Which?", c("a", "b"), descriptions = c(a = "", b = "Meaning"))
  saved <- jsonlite::fromJSON(tt_toolbox_to_json(tt_toolbox(list(entry = valid))), simplifyVector = FALSE)$entries$entry$question[[field]]
  stopifnot(identical(saved[[1]]$description, ""), identical(saved[[2]]$description, "Meaning"))
}
tt_close(client)
definition <- list(`function` = "decide", question = "Which?", descriptions = list(a = "Meaning"))
expected <- .Call("wrap__r_admit", jsonlite::toJSON(list(question = definition), auto_unbox = TRUE), 0L, PACKAGE = "thinkthen")$error
actual <- condition(tt_decide_question("Which?", descriptions = c(a = "Meaning")), "thinkthen_usage_error")
stopifnot(!is.null(expected), identical(actual$code, expected$code), identical(conditionMessage(actual), expected$message))
cat("R descriptions preserve values and match the engine diagnostic\n")
