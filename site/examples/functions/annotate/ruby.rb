require "thinkthen"

form = ThinkThen.set(
  steps: {
    decide: "Does the report give steps to reproduce?"
  },
  area: {
    choose: "Which part of the app is this?",
    options: ["export", "login", "billing"]
  },
  impact: {
    score: "How much does this block the user?",
    levels: ["None.", "Slows them.", "Blocks work."]
  }
)
reports = [
  "Steps: click Export. It is very slow.",
  "Steps: click Log in. Nobody gets in.",
  "The Pay button on billing is too blue."
]
triage = ThinkThen.annotate(form, reports).value
raise unless triage == [
  { steps: true, area: "export", impact: 1.04 },
  { steps: true, area: "login", impact: 1.98 },
  { steps: false, area: "billing", impact: 0.09 }
]
