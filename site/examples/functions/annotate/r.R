library(thinkthen)

reports <- data.frame(body = c(
  "Steps: click Export. It is very slow.",
  "Steps: click Log in. Nobody gets in.",
  "The Pay button on billing is too blue."
))
triage <- tt_annotate(
  "form.json", reports, on = "body"
)$value
print(triage[c("steps", "area", "impact")])
