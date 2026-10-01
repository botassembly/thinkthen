require "thinkthen"

report = "Steps: click Log in. Nobody gets in."
triage = ThinkThen.annotate("form.json", [report]).value
raise unless triage == [
  { steps: true, area: "login", impact: 1.98 }
]
