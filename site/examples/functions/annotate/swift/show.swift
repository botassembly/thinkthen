import Foundation
import ThinkThen

let messages = try String(contentsOfFile: "messages.txt", encoding: .utf8)
  .split(separator: "\n").map(String.init)
let questions = AnnotationQuestions([
  "refund": DecideQuestion("Does the customer request a refund?"),
  "topic": ChooseQuestion("What is the message about?", choices: ["billing", "shipping", "other"]),
  "urgency": ScoreQuestion("How urgent is the request?", levels: ["routine", "urgent"]),
])
let client = try Client()
for try await annotated in client.annotate(questions, records: messages) {
  if annotated.isFailed {
    print(annotated)
    continue
  }
  let entries = try annotated.answer
  let refund = try entries.decide("refund")
  let topic = try entries.choose("topic")
  let urgency = try entries.score("urgency")
  if refund.isAnswered && topic.isAnswered && urgency.isAnswered {
    print(try refund.answer, try topic.answer, try urgency.answer)
  } else {
    print("A person reads it", annotated.item)
  }
}
