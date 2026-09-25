# The R helper's proof, run by test.sh with sentinels in this process.
source(file.path(Sys.getenv("CHILDREN"), "children.R"))
probe <- paste('test -z "${THINKTHEN_SENTINEL+x}" && test -z "${FAKE_SERVICE_API_KEY+x}" &&',
               'test -z "${ABSENT_0127+x}" && test "$KEPT_0127" = kept && test "$SET_0127" = set')
refused <- c("THINKTHEN_BASE_URL", "OPENAI_API_KEY", "GITHUB_TOKEN", "db_password", "AWS_SECRET_ACCESS_KEY")
words <- clean_env(c("KEPT_0127", "ABSENT_0127"), c(SET_0127 = "set"))
bad <- if (system2("env", c(words, "sh", "-c", shQuote(probe))) == 0) character() else "the child"
for (name in refused) {
  said <- tryCatch({ clean_env(name); name }, error = function(e) conditionMessage(e))
  sentence <- sprintf("a test child may not keep %s from the parent: set a THINKTHEN_ value or a fake key explicitly", name)
  if (!identical(said, sentence)) bad <- c(bad, said)
}
cat("children r: ", if (length(bad)) paste("FAIL", paste(bad, collapse = ", ")) else "ok", "\n", sep = "")
quit(status = if (length(bad)) 1L else 0L)
