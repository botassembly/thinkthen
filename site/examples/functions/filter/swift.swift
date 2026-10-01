import Foundation
import ThinkThen

struct Filter: Encodable {
    let filter: String
    let records: [String]
}

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "Is this a complaint?"
let reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "The strap snapped on day two.",
]
let filter = try JSONEncoder().encode(
    Filter(filter: question, records: reviews)
)
let filtered = try tt.call(
    String(decoding: filter, as: UTF8.self)
)
let complaints = try JSONDecoder().decode(
    Reply<[String]>.self,
    from: Data(filtered.utf8)
).value
precondition(complaints == [reviews[1], reviews[3]])
tt.close()
