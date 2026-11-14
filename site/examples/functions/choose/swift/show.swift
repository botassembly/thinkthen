import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let topic = try await client.choose(
  "What is the message about?",
  record: message,
  choices: ["billing", "shipping", "other"]
).answer
print(topic)
