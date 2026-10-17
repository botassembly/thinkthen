import ThinkThen

let client = try Client()
let whichTeam = Question.file("teams.json")
let parcel = "My parcel went to the wrong address."
let judgment = try await client.choose(
  whichTeam,
  input: InputText(text: parcel)
)
let team = judgment.value ?? ""
precondition(team == "shipping")
client.close()
