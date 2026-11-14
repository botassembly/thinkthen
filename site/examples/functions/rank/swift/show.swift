import Foundation
import ThinkThen

let messages = try String(contentsOfFile: "messages.txt", encoding: .utf8)
  .split(separator: "\n").map(String.init)
let client = try Client()
// Every message, most urgent first. try answer is its rank, starting at 1.
for try await ranked in client.rank(
  "How urgent is the request?",
  records: messages, levels: ["routine", "urgent"])
{
  print(try ranked.answer, ranked.item)
}
