import Foundation
import ThinkThen

let messages = try String(contentsOfFile: "messages.txt", encoding: .utf8)
    .split(separator: "\n").map(String.init)
let client = try Client()
// Only the messages that answer yes.
// A failed message throws after the kept ones.
for try await message in client.filter(
    "Does the customer request a refund?", records: messages) {
    print(message)
}
