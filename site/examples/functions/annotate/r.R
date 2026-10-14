library(thinkthen)

form <- tt_question(file = "form.json")
report <- "Steps: click Log in. Nobody gets in."
triage <- tt_annotate(form, report)$value[[1]]
stopifnot(
  identical(triage$steps, TRUE),
  identical(triage$area, "login"),
  identical(triage$impact, 1.98)
)
