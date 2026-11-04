import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let urgency = try await client.score(
    "How urgent is the request?",
    levels: ["routine", "urgent"],
    record: .text(message)).get()
print(urgency)
