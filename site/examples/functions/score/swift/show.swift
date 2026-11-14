import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let urgency = try await client.score(
  "How urgent is the request?",
  record: message,
  levels: ["routine", "urgent"]
).answer
print(urgency)
