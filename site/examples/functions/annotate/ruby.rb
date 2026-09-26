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
  "CSV export fails. Steps: click Export.",
  "The login page spins and nobody can sign in.",
  "The Pay button on the billing page is too blue."
]
forms = ThinkThen.annotate(form, reports)
raise unless forms == [
  { steps: true, area: "export", impact: 1.94 },
  { steps: false, area: "login", impact: 2.0 },
  { steps: false, area: "billing", impact: 0.06 }
]
