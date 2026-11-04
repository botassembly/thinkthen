import ThinkThen

let client = try Client()
let refundCheck = try await client.decide(
    "Does the customer request a refund?",
    record: .text("I was charged twice for one invoice."),
    options: .init(threshold: .band(low: 0.2, high: 0.8)))
do {
    print(try refundCheck.get())
} catch RowError.unsure(let probability) {
    print("Not sure (\(probability)): ask a person.")
} catch RowError.failed(let failure) {
    print("\(failure.code.rawValue): \(failure.message)")
}
