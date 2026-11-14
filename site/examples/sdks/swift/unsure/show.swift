import ThinkThen

let client = try Client()
do {
  let refundCheck = try await client.decide(
    "Does the customer request a refund?",
    record: "I was charged twice for one invoice.",
    threshold: .band(low: 0.2, high: 0.8))
  print(try refundCheck.answer)
} catch let unsure as Unsure {
  print("Not sure (\(unsure.probability)): ask a person.")
} catch let failure as Failed {
  print("\(failure.code.rawValue): \(failure.message)")
}
