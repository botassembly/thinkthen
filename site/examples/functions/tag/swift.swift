import ThinkThen

let client = try Client()
let question = "Which labels fit this message?"
let labels: [JSONValue] = [
    .string("praise"),
    .string("bug"),
    .string("billing"),
]
let whichLabels = try client.parseQuestion(
    .atomic,
    authored: .orderedObject([
        JSONMember("tag", .string(question)),
        JSONMember("labels", .array(labels)),
    ])
)
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
