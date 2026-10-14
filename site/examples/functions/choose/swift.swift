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
let judgment = try await client.chooseAnswer(
    whichTeam,
    input: .text(parcel)
)
let team = judgment ?? ""
precondition(team == "shipping")
client.close()
