import Foundation
import ThinkThen

struct Find: Encodable {
    let find: String
    let units: [String]
}

struct Unit: Decodable {
    let unit: String
}

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "Which line gives the refund deadline?"
let policy = [
    "Returns need the original receipt.",
    "Refunds are issued within 30 days of purchase.",
    "Shipping is free on orders over $50.",
    "Gift cards cannot be exchanged for cash.",
]
let find = try JSONEncoder().encode(
    Find(find: question, units: policy)
)
let located = try tt.call(
    String(decoding: find, as: UTF8.self)
)
let refundDeadline = try JSONDecoder().decode(
    Reply<Unit>.self,
    from: Data(located.utf8)
).value
precondition(refundDeadline.unit == policy[1])
tt.close()
