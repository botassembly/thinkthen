import ThinkThen

let client = try Client()
let howUrgent = Question.file("levels.json")
let outage =
  "Our checkout page is down and customers cannot pay.\n"
let judgment = try await client.score(
  howUrgent,
  input: InputText(text: outage)
)
let urgency = judgment.value
precondition(urgency == 2.0)
client.close()
