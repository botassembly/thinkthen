import Foundation
import ThinkThen

let client = try Client()
let refundCheck = try await client.decide(
  "Does the customer request a refund?",
  record: "Please refund my order.")
do {
  let isRefund = try refundCheck.answer
  print(
    isRefund
      ? "Send this message to the refunds queue."
      : "Leave this message in the inbox.")
} catch is NoAnswer {
  print("Please review this message.")
}
