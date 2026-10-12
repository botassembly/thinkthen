import ThinkThen

let client = try Client()
let report = "Steps: click Log in. Nobody gets in."
let call = try await client.annotate(
    .file("form.json"),
    input: .text(report)
)
var steps = false
var area = ""
var impact = 0.0
for case .annotateRow(let row) in call.packets {
    let triage = row.value.value
    if case .alternative0(let given)? = triage["steps"] {
        steps = given
    }
    if case .alternative2(let part)? = triage["area"] {
        area = part
    }
    if case .alternative4(let level)? = triage["impact"] {
        impact = level
    }
}
precondition(steps && area == "login" && impact == 1.98)
client.close()
