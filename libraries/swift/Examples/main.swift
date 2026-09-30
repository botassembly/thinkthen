import Foundation
import ThinkThen

let engine = try Engine()
let answer = try engine.decide("Is it?", "consumer-swift")
let answerFacts = try JSONSerialization.jsonObject(with: Data(answer.facts.utf8)) as! [String: Any]
precondition(answer.value.outcome == .yes && abs(answer.value.probability - 0.9) < 0.00001 && answerFacts["records"] as? Int == 1 && answerFacts["requests_sent"] as? Int == 1)
let response = try engine.call("{\"choose\":\"Which?\",\"options\":[\"first\",\"second\"],\"evidence\":\"consumer-json\"}")
let envelope = try JSONSerialization.jsonObject(with: Data(response.utf8)) as! [String: Any]
precondition(envelope["value"] as? String == "first")
let facts = envelope["facts"] as! [String: Any]
precondition(facts["records"] as? Int == 1 && facts["requests_sent"] as? Int == 1 && facts["cache_answers"] as? Int == 0)
precondition(facts["input_tokens"] as? Int == 1 && facts["output_tokens"] as? Int == 1 && facts["model"] as? String == "jev-1.13.0")
precondition((facts["seconds"] as? Double ?? -1) >= 0)
print("PACKAGED_ENVELOPE_PASS value=first")
let spent = try CancelToken()
spent.cancel()
spent.cancel()
do {
    _ = try engine.decide("Is it?", "no-arrival-spent-token", token: spent.handle)
    fatalError("spent token unexpectedly sent")
} catch let failure as DoorFailure {
    precondition(failure.code == 5 && !failure.retryable)
}
engine.close()
print("INSTALLED_SWIFT_CONSUMER_PASS outcome=\(answer.value.outcome)")
