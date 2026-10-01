library(thinkthen)

reports <- data.frame(
  body = "Steps: click Log in. Nobody gets in."
)
triage <- tt_annotate(
  "form.json", reports, on = "body"
)$value
stopifnot(identical(triage$steps, TRUE))
stopifnot(identical(triage$area, "login"))
stopifnot(identical(triage$impact, 1.98))
