import ThinkThen

let client = try Client()
let whichLabels = Question.file("labels.json")
let message = """
    Love the new dashboard, but export crashes the app,
    and I was charged twice.

    """
let judgment = try await client.tagAnswer(
    whichLabels,
    input: .text(message)
)
let fittingLabels = judgment
precondition(fittingLabels == ["praise", "bug", "billing"])
client.close()
