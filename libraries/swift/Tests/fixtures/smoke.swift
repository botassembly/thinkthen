// The replay smoke (ticket 0335): one decide through Engine(), which reads the
// environment, with the question and text sdlc/scripts/smoke names.
import Foundation

let environment = ProcessInfo.processInfo.environment
let engine = try Engine()
let answer = try engine.decide(environment["THINKTHEN_SMOKE_QUESTION"]!, environment["THINKTHEN_SMOKE_TEXT"]!)
engine.close()
print("smoke: " + [Outcome.yes: "true", .no: "false", .unsure: "null"][answer.value.outcome]!)
