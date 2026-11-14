import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
// nil when no line answers the question.
let action = try await client.find(
  "Which line states the requested action?", record: message
).answer
print(action?.text ?? "no line answers")
