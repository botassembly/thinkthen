import ThinkThen

let client = try Client()
let whichLabels = Question.file("labels.json")
let message = """
  Love the new dashboard, but export crashes the app,
  and I was charged twice.

  """
let judgment = try await client.tag(
  whichLabels,
  input: InputText(text: message)
)
let fittingLabels = judgment.value
precondition(fittingLabels == ["praise", "bug", "billing"])
client.close()
