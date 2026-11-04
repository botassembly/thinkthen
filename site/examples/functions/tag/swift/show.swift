import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let topics = try await client.tag(
    "Which topics appear?",
    labels: ["billing", "shipping"],
    record: .text(message)).get()
print(topics)
