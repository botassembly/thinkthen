import Foundation
import ThinkThen

let messages = try String(contentsOfFile: "messages.txt", encoding: .utf8)
    .split(separator: "\n").map(String.init)
let questions = AnnotationQuestions([
    "refund": .decide(DecideQuestion("Does the customer request a refund?")),
    "topic": .choose(ChooseQuestion(
        "What is the message about?",
        choices: ["billing", "shipping", "other"])),
    "urgency": .score(ScoreQuestion(
        "How urgent is the request?", levels: ["routine", "urgent"])),
])
let client = try Client()
for try await annotated in client.annotate(
    questions, records: messages.map(Record.text)) {
    let triage = try annotated.get()
    let isRefund = try triage.get("refund", as: Bool.self)
    let topic = try triage.get("topic", as: String.self)
    let urgency = try triage.get("urgency", as: Double.self)
    print(isRefund, topic, urgency)
}
