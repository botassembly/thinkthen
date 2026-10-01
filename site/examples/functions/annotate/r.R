library(thinkthen)

reports <- data.frame(body = c(
  "Steps: click Export. It is very slow.",
  "Steps: click Log in. Nobody gets in.",
  "The Pay button on billing is too blue."
))
triage <- tt_annotate(
  "form.json", reports, on = "body"
)$value
stopifnot(identical(triage$steps, c(TRUE, TRUE, FALSE)))
stopifnot(identical(
  triage$area,
  c("export", "login", "billing")
))
stopifnot(identical(triage$impact, c(1.04, 1.98, 0.09)))
