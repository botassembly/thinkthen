import ThinkThen

let client = try Client()
let whichTeam = Question.file("teams.json")
let parcel = "My parcel went to the wrong address."
let judgment = try await client.chooseAnswer(
    whichTeam,
    input: .text(parcel)
)
let team = judgment ?? ""
precondition(team == "shipping")
client.close()
