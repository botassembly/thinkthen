import Foundation
import ThinkThen

struct Rank: Encodable {
    let rank: String
    let records: [String]
}

struct Ranked: Decodable {
    let index: Int
}

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "Is this urgent?"
let inbox = [
    "Newsletter: our autumn catalog is here. "
        + "No reply needed.",
    "Our checkout page is down and customers cannot pay",
    "Reminder: your invoice is due in 30 days",
    "Please send the signed quote by 5 pm today",
]
let rank = try JSONEncoder().encode(
    Rank(rank: question, records: inbox)
)
let ranking = try tt.call(
    String(decoding: rank, as: UTF8.self)
)
let byUrgency = try JSONDecoder().decode(
    Reply<[Ranked]>.self,
    from: Data(ranking.utf8)
).value
precondition(byUrgency.map(\.index) == [1, 3, 2, 0])
tt.close()
