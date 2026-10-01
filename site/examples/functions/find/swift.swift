import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
struct Unit: Decodable {
    let unit: String
}

let question = "Which line gives the refund deadline?"
let policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
]
let units = policy.map { "\"\($0)\"" }
let find = """
    {"find": "\(question)",
     "units": [\(units.joined(separator: ", "))]}
    """
let located = try tt.call(find)
let refundDeadline = try JSONDecoder().decode(
    Reply<Unit>.self,
    from: Data(located.utf8)
).value
precondition(refundDeadline.unit == policy[1])
tt.close()
