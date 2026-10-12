import ThinkThen

let client = try Client()
let question = "Which team owns this?"
let teams = [
    ("billing", "Invoices, fees, and refunds."),
    ("shipping", "Parcels and delivery."),
    ("account", "Logins and passwords."),
]
let options = teams.map { JSONMember($0, .string($1)) }
let whichTeam = try client.parseQuestion(
    .atomic,
    authored: .orderedObject([
        JSONMember("choose", .string(question)),
        JSONMember("options", .orderedObject(options)),
    ])
)
let parcel = "My parcel went to the wrong address."
let call = try await client.choose(
    whichTeam,
    input: .text(parcel)
)
var team = ""
for case .chooseRow(let row) in call.packets {
    if case .alternative0(let name) = row.value.value {
        team = name
    }
}
precondition(team == "shipping")
client.close()
