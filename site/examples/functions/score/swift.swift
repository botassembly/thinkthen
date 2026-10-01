import Foundation
import ThinkThen

struct Reply<Value: Decodable>: Decodable {
    let value: Value
}

let tt = try Engine()
let question = "How urgent is this?"
let score = """
    {"score": "\(question)",
     "levels": ["Routine.", "Soon.", "Immediate."],
     "evidence": "Our checkout page is down and \
    customers cannot pay.\\n"}
    """
let scored = try tt.call(score)
let urgency = try JSONDecoder().decode(
    Reply<Double>.self,
    from: Data(scored.utf8)
).value
precondition(urgency == 2.0)
tt.close()
