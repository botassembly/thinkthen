import Foundation
import ThinkThen

struct Score: Encodable {
    let score: String
    let levels: [String]
    let evidence: String
}

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "How urgent is this?"
let score = try JSONEncoder().encode(
    Score(
        score: question,
        levels: ["Routine.", "Soon.", "Immediate."],
        evidence: "Our checkout page is down and "
            + "customers cannot pay.\n"
    )
)
let scored = try tt.call(
    String(decoding: score, as: UTF8.self)
)
let urgency = try JSONDecoder().decode(
    Reply<Double>.self,
    from: Data(scored.utf8)
).value
precondition(urgency == 2.0)
tt.close()
