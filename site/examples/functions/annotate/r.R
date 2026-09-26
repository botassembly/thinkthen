library(thinkthen)

reports <- data.frame(body = c(
  "CSV export fails. Steps: click Export.",
  "The login page spins and nobody can sign in.",
  "The Pay button on the billing page is too blue."
))
forms <- tt_annotate("form.json", reports, on = "body")
stopifnot(identical(forms$steps, c(TRUE, FALSE, FALSE)))
stopifnot(identical(
  forms$area,
  c("export", "login", "billing")
))
stopifnot(identical(forms$impact, c(1.94, 2.0, 0.06)))
