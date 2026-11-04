import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
let names = try await client.recognize(
    kinds: ["person", "organization"], record: .text(message)).get()
for name in names {
    print("\(name.kind): \(name.text)")
}
