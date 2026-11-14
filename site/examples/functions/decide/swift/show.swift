import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
// A failed call throws Failed; an unsure answer read throws Unsure.
let isRefund = try await client.decide(
  "Does the customer request a refund?", record: message
).answer
print(isRefund)
