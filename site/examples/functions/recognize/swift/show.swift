import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let names = try await client.recognize(
  record: message, kinds: ["person", "organization"]
).answer
for name in names {
  print("\(name.kind): \(name.text)")
}
