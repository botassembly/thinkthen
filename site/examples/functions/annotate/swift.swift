import ThinkThen

let client = try Client()
let report = "Steps: click Log in. Nobody gets in."
let judgment = try await client.annotate(
  .file("form.json"),
  input: InputText(text: report)
)
let triage = judgment.value
var steps = false
var area = ""
var impact = 0.0
if case .boolean(let given)? = triage["steps"] {
  steps = given
}
if case .string(let part)? = triage["area"] {
  area = part
}
if case .number(let level)? = triage["impact"] {
  impact = level
}
precondition(steps && area == "login" && impact == 1.98)
client.close()
