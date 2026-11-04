import Foundation
import ThinkThen

let message = try String(contentsOfFile: "message.txt", encoding: .utf8)
let client = try Client()
// get() throws RowError.unsure when not sure, RowError.failed on failure.
let isRefund = try await client.decide(
    "Does the customer request a refund?", record: .text(message)).get()
print(isRefund)
