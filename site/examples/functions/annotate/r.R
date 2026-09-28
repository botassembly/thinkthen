library(thinkthen)

reports <- data.frame(body = c(
  "CSV export fails. Steps: click Export.",
  "The login page spins and nobody can sign in.",
  "The Pay button on the billing page is too blue."
))
triage <- tt_annotate(
  "form.json", reports, on = "body"
)$value
stopifnot(identical(triage$steps, c(TRUE, FALSE, FALSE)))
stopifnot(identical(
  triage$area,
  c("export", "login", "billing")
))
stopifnot(identical(triage$impact, c(1.94, 2.0, 0.06)))
