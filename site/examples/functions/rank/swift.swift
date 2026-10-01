import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
struct Ranked: Decodable {
    let index: Int
}

let question = "Is this urgent?"
let rank = """
    {"rank": "\(question)", "records": [
     "Newsletter: our autumn catalog is here. \
    No reply needed.",
     "Our checkout page is down and customers cannot pay",
     "Reminder: your invoice is due in 30 days",
     "Please send the signed quote by 5 pm today"]}
    """
let ranking = try tt.call(rank)
let byUrgency = try JSONDecoder().decode(
    Reply<[Ranked]>.self,
    from: Data(ranking.utf8)
).value
precondition(byUrgency.map(\.index) == [1, 3, 2, 0])
tt.close()
