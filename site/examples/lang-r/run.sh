# Run the unchanged example against the staged R package.
set -eu
root=$(CDPATH= cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../.." && pwd)
packages=${SITE_PACKAGES:-$root/target/site-packages}
R_LIBS="$packages/r" Rscript - <<'RANSWERS'
shown <- new.env()
source("show.R", local = shown, print.eval = TRUE)
case_file <- Sys.getenv("SITE_CASE_FILE")
if (nzchar(case_file)) {
  fn <- basename(dirname(case_file))
  answers <- switch(fn,
    decide = as.list(shown$is_refund),
    score = as.list(shown$urgency),
    choose = as.list(as.character(shown$topic)),
    tag = list(sort(as.character(shown$topics))),
    filter = as.list(shown$refund_requests),
    rank = as.list(shown$urgency_rank),
    find = lapply(shown$action$text, function(text) if (is.na(text) || !nzchar(text)) NULL else text),
    recognize = list(lapply(seq_len(length(shown$mentions)), function(i)
      list(text = shown$mentions$text[[i]], kind = as.character(shown$mentions$kind[[i]])))),
    relate = list(lapply(seq_len(length(shown$employers)), function(i)
      list(source = shown$employers$source$text[[i]], relation = as.character(shown$employers$relation[[i]]),
           target = shown$employers$target$text[[i]]))),
    annotate = lapply(seq_len(nrow(shown$triage)), function(i)
      list(refund = shown$triage$refund[[i]], topic = as.character(shown$triage$topic[[i]]),
           urgency = shown$triage$urgency[[i]])),
    stop("Unknown function example"))
  writeLines(jsonlite::toJSON(answers, auto_unbox = TRUE, digits = NA, na = "null", null = "null"),
             Sys.getenv("SITE_ANSWERS_FILE"))
}
RANSWERS
