import Foundation
import CThinkThen

func check(_ ok: @autoclosure () -> Bool, _ message: String) { precondition(ok(), message) }
func answer(_ value: Answer, _ code: Int32, _ probability: Double) {
    check(value.outcome.rawValue == code && abs(value.probability - probability) < 0.00001, "answer \(value.outcome) / \(value.probability)")
}
func answer(_ value: thinkthen_answer, _ code: Int32, _ probability: Double) {
    check(value.outcome == code && abs(value.probability - probability) < 0.00001, "C answer \(value.outcome) / \(value.probability)")
}
func error(_ wanted: Int32, _ body: () throws -> Void) {
    do { try body(); fatalError("expected native failure \(wanted)") }
    catch let failure as DoorFailure {
        check(failure.code == wanted && !failure.retryable && !failure.message.isEmpty, "wrong failure \(failure)")
        if wanted == 2 {
            let facts = decoded(failure.factsJSON ?? "null") as? [String: Any]
            check(failure.kind == .backend && (facts?["requests_sent"] as? Int ?? 0) > 0, "started failure facts")
        }
    } catch { fatalError("wrong error \(error)") }
}
func decoded(_ json: String) -> Any {
    try! JSONSerialization.jsonObject(with: Data(json.utf8), options: [.fragmentsAllowed])
}
func callValue(_ json: String) -> Any {
    let envelope = decoded(json) as! [String: Any]
    let facts = envelope["facts"] as! [String: Any]
    check((facts["records"] as? Int ?? -1) >= 0 && (facts["requests_sent"] as? Int ?? -1) >= 0 && (facts["cache_answers"] as? Int ?? -1) >= 0, "C JSON facts counters")
    check((facts["seconds"] as? Double ?? -1) >= 0 && (facts["input_tokens"] as? Int ?? -1) >= 0 && (facts["output_tokens"] as? Int ?? -1) >= 0 && facts["model"] as? String == "jev-1.13.0", "C JSON facts types")
    return envelope["value"]!
}
func waitFor(_ state: String) {
    let path = ProcessInfo.processInfo.environment["TT_BARRIER_DIR"]! + "/arrived-" + state
    let end = Date().addingTimeInterval(6)
    while !FileManager.default.fileExists(atPath: path) && Date() < end { Thread.sleep(forTimeInterval: 0.005) }
    check(FileManager.default.fileExists(atPath: path), "no arrival \(state)")
}
func release(_ state: String) {
    let path = ProcessInfo.processInfo.environment["TT_BARRIER_DIR"]! + "/release-" + state
    check(FileManager.default.createFile(atPath: path, contents: Data()), "release \(state)")
}
// The token is intentionally shared across threads. Join both users before free.
final class ConcurrentResults: @unchecked Sendable {
    private let lock = NSLock()
    private var failures = [String: DoorFailure]()
    private var success = [String: CallResult<Answer>]()
    func record(_ answer: CallResult<Answer>, for state: String = "success") { lock.lock(); defer { lock.unlock() }; success[state] = answer }
    func record(_ failure: DoorFailure, for state: String) { lock.lock(); defer { lock.unlock() }; failures[state] = failure }
    func failure(_ state: String) -> DoorFailure? { lock.lock(); defer { lock.unlock() }; return failures[state] }
    func answer() -> CallResult<Answer>? { lock.lock(); defer { lock.unlock() }; return success["success"] }
    func answer(_ state: String) -> CallResult<Answer>? { lock.lock(); defer { lock.unlock() }; return success[state] }
}

func held(_ engine: Engine, _ state: String, _ wanted: Int32, many: Bool = false) throws {
    let token = try CancelToken()
    let semaphore = DispatchSemaphore(value: 0)
    Thread.detachNewThread {
        waitFor(state)
        if wanted == 5 { token.cancel(); token.cancel() }
        Thread.sleep(forTimeInterval: 0.12)
        if many { for i in 1...6 { release("hold-bulk-\(i)") } }
        else { release(state) }
        semaphore.signal()
    }
    if many {
        error(wanted) { _ = try engine.decideMany("Is it?", (1...6).map { "hold-bulk-\($0)" }, token: token.handle) }
    } else {
        error(wanted) { _ = try engine.decide("Is it?", state, deadline: wanted == 3 ? 25 : -1, token: token.handle) }
    }
    check(semaphore.wait(timeout: .now() + 10) == .success, "helper did not finish")
    print("HELD_\(state)_PASS code=\(wanted) untouched-output")
}
func direct() throws {
    guard let engine = thinkthen_engine_new() else { fatalError("direct engine") }
    defer { thinkthen_engine_free(engine) }
    try withInput("Is it?") { q in
        withEvidence("direct") { bytes, size in
            var out = thinkthen_answer(outcome: 123, probability: -1)
            check(thinkthen_decide_opts(engine, q, bytes, size, -1, nil, &out) == 0, "direct decide")
            answer(out, 1, 0.9)
        }
    }
    let token = thinkthen_cancel_token_new()!
    defer { thinkthen_cancel_token_free(token) }
    thinkthen_cancel(token); thinkthen_cancel(token)
    try withInput("Is it?") { q in
        withEvidence("no-arrival-spent-token") { bytes, size in
            var out = thinkthen_answer(outcome: 123, probability: -1)
            let rc = thinkthen_decide_opts(engine, q, bytes, size, -1, token, &out)
            let last = thinkthen_error_code(engine)
            let retry = thinkthen_error_retryable(engine)
            let message = String(cString: thinkthen_error_message(engine)!)
            check(rc == 5 && last == 5 && retry == 0 && !message.isEmpty && out.outcome == 123 && out.probability == -1, "spent direct token")
        }
    }
    // Raw C negative: Swift String cannot encode an invalid UTF-8 question.
    var invalid: [CChar] = [-1, 0]
    invalid.withUnsafeMutableBufferPointer { q in
        withEvidence("no-arrival-invalid-utf8") { bytes, size in
            var out = thinkthen_answer(outcome: 123, probability: -1)
            let rc = thinkthen_decide(engine, q.baseAddress, bytes, size, &out)
            let last = thinkthen_error_code(engine)
            let retry = thinkthen_error_retryable(engine)
            let message = String(cString: thinkthen_error_message(engine)!)
            check(rc == 1 && last == 1 && retry == 0 && !message.isEmpty && out.outcome == 123 && out.probability == -1, "invalid UTF-8 C refusal")
        }
    }
    print("DIRECT_SWIFT_C_PASS")
}
// Facts are JSON text read with Foundation; a reader ignores unknown members.
func facts<T>(_ result: CallResult<T>) -> [String: Any] { decoded(result.facts) as! [String: Any] }
func factsLifetime() throws {
    let engine = try Engine()
    let results = ConcurrentResults()
    let group = DispatchGroup()
    for state in ["hold-facts-one", "hold-facts-no-usage"] {
        group.enter()
        DispatchQueue.global().async {
            defer { group.leave() }
            do { results.record(try engine.decide("Is it?", state), for: state) }
            catch { fatalError("held facts failed: \(error)") }
        }
    }
    waitFor("hold-facts-one")
    waitFor("hold-facts-no-usage")
    release("hold-facts-one")
    release("hold-facts-no-usage")
    check(group.wait(timeout: .now() + 10) == .success, "held facts calls did not finish")
    let first = results.answer("hold-facts-one")!
    let second = results.answer("hold-facts-no-usage")!
    answer(first.value, 1, 0.9); answer(second.value, 1, 0.9)
    check(facts(first)["records"] as? Int == 1 && facts(first)["requests_sent"] as? Int == 1 && facts(first)["input_tokens"] as? Int == 1 && facts(first)["model"] as? String == "jev-1.13.0", "first owned facts")
    check(facts(second)["records"] as? Int == 1 && facts(second)["requests_sent"] as? Int == 1 && facts(second)["input_tokens"] == nil && facts(second)["output_tokens"] == nil && facts(second)["model"] as? String == "jev-1.13.0", "no-usage owned facts")
    var failed: DoorFailure?
    do { _ = try engine.decide("Is it?", "status-401"); fatalError("expected backend failure") }
    catch let failure as DoorFailure { failed = failure }
    let failedFacts = failed!.factsJSON!
    check(failed!.kind == .backend && (decoded(failedFacts) as! [String: Any])["requests_sent"] as? Int == 1, "typed started failure facts")
    let later = try engine.decide("Is it?", "recovery-scalar")
    engine.close()
    answer(later.value, 1, 0.9)
    check(facts(first)["input_tokens"] as? Int == 1 && facts(second)["input_tokens"] == nil && failed!.factsJSON == failedFacts, "owned snapshots after close")
    print("SWIFT_FACTS_LIFETIME_PASS")
}
func matrix() throws {
    let engine = try Engine()
    for (text, code, p) in [("yes", 1, 0.9), ("no", 0, 0.1), ("unsure", 2, 0.5), ("café", 1, 0.9), ("a\0b", 1, 0.9)] {
        let result = try engine.decide(text == "unsure" ? "{\"decide\":\"Is it?\",\"threshold\":\"0.4:0.8\"}" : "Is it?", text)
        answer(result.value, Int32(code), p)
        check(facts(result)["records"] as? Int == 1 && facts(result)["requests_sent"] as? Int == 1 && facts(result)["cache_answers"] as? Int == 0 && (facts(result)["seconds"] as? Double ?? -1) >= 0, "scalar facts")
    }
    let rows = try engine.decideMany("Is it?", ["first", "second", "third"])
    check(facts(rows)["records"] as? Int == 3 && facts(rows)["requests_sent"] as? Int == 1 && facts(rows)["cache_answers"] as? Int == 0, "bulk facts")
    for i in rows.value.indices { answer(rows.value[i], [1, 0, 1][i], [0.9, 0.1, 0.6][i]) }
    let replay = try engine.decideMany("Is it?", ["first", "second", "third"])
    check(replay.value.map(\.probability) == [0.9, 0.1, 0.6] && facts(replay)["records"] as? Int == 3 && facts(replay)["requests_sent"] as? Int == 0 && facts(replay)["cache_answers"] as? Int == 3, "each replayed question is one cache answer")
    let cached = try engine.decideMany("Is it?", ["first", "second", "first", "second"])
    check(cached.value.map(\.probability) == [0.9, 0.1, 0.9, 0.1], "bulk cache/order")
    let empty = try engine.decideMany("Is it?", [])
    check(facts(empty)["records"] as? Int == 0 && facts(empty)["model"] == nil, "empty facts")
    check(empty.value.isEmpty, "zero bulk")
    let requests = [
        "{\"decide\":\"Is it?\",\"evidence\":\"json-decide\",\"details\":true}",
        "{\"choose\":\"Which team?\",\"options\":[\"first\",\"second\"],\"evidence\":\"choose\"}",
        "{\"tag\":\"Which labels?\",\"labels\":[\"first\",\"second\"],\"evidence\":\"tag\"}",
        "{\"score\":\"What level?\",\"levels\":[\"Low.\",\"High.\"],\"evidence\":\"score\"}",
        "{\"filter\":\"Is it?\",\"records\":[\"filter-one\",\"filter-two\"]}",
        "{\"rank\":\"Is it?\",\"records\":[\"rank-one\",\"rank-two\"]}",
        "{\"find\":\"Which line?\",\"units\":[\"find-one\",\"find-two\"]}",
        "{\"annotate\":{\"version\":1,\"questions\":{\"check\":{\"decide\":\"Is it?\"}}},\"records\":[\"annotate-one\"]}",
        "{\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}},\"version\":1,\"evidence\":\"Maria Chen\"}",
        "{\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]},\"version\":1,\"records\":[{\"name\":\"First\",\"kind\":\"alert\"},{\"name\":\"Second\",\"kind\":\"alert\"}]}"
    ]
    for (i, request) in requests.enumerated() {
        let value = callValue(try engine.call(request))
        switch i {
        case 0: let v = value as! [String: Any]; check(Set(v.keys) == ["schema", "value", "question", "answer", "threshold", "meta"] && v["schema"] as? String == "thinkthen.result/1" && v["value"] as? Bool == true && (v["answer"] as? [String: Any])?["kind"] as? String == "yes_no" && (v["answer"] as? [String: Any])?["probability"] as? Double == 0.9, "decide details")
        case 1: check(value as? String == "first", "choose")
        case 2: check((value as! [String]) == ["first", "second"], "tag")
        case 3: check((value as! Double) == 0.1, "score")
        case 4: check((value as! [String]) == ["filter-one", "filter-two"], "filter")
        case 5: check((value as! [String]) == ["rank-one", "rank-two"], "rank")
        case 6: check(value as? String == "find-one", "find")
        case 7: check((value as! [[String: Any]])[0]["check"] as? Bool == true, "annotate")
        case 8: check(((value as! [String: Any])["entities"] as! [Any]).count == 1, "recognize json")
        case 9: check(((value as! [String: Any])["edges"] as! [Any]).count == 2, "relate json")
        default: fatalError("missing verb")
        }
    }
    let spec = "{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}"
    let namedCall = try engine.recognize(spec, "John Smith")
    let named = decoded(namedCall.value) as! [String: Any]
    check(facts(namedCall)["requests_sent"] as? Int == 2 && facts(namedCall)["records"] as? Int ?? 0 > 0, "recognize facts")
    check((named["entities"] as! [Any]).count == 1, "typed recognize")
    let relSpec = "{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}"
    let edgeCall = try engine.relate(relSpec, ["{\"name\":\"Third\",\"kind\":\"alert\"}", "{\"name\":\"Fourth\",\"kind\":\"alert\"}"])
    let edges = decoded(edgeCall.value) as! [String: Any]
    check(facts(edgeCall)["requests_sent"] as? Int == 1 && facts(edgeCall)["records"] as? Int ?? 0 > 0, "relate facts")
    check((edges["edges"] as! [Any]).count == 2, "typed relate")
    let usage = decoded(try engine.call("{\"usage\":true}")) as! [String: Any]
    check(usage["requests_sent"] != nil, "usage")
    error(1) { _ = try engine.decide("Is it?\0suffix", "x") }
    error(1) { _ = try engine.decideMany("Is it?\0suffix", ["x"]) }
    error(1) { _ = try engine.call("{}\0suffix") }
    error(1) { _ = try engine.recognize("{}\0suffix", "x") }
    error(1) { _ = try engine.relate("{}\0suffix", []) }
    error(1) { _ = try engine.decide("{invalid", "text") }
    let other = try Engine(); error(1) { _ = try other.decide("Is it?", "") }; other.close()
    error(2) { _ = try engine.decide("Is it?", "status-401") }
    error(3) { _ = try engine.decide("Is it?", "zero-deadline", deadline: 0) }
    error(1) { _ = try engine.decide("Is it?", "negative-deadline", deadline: -2) }
    error(1) { _ = try engine.decide("Is it?", "oversized-deadline", deadline: 4_294_967_295_001) }
    let spent = try CancelToken()
    spent.cancel()
    error(5) { _ = try engine.decide("Is it?", "no-arrival-pre-cancelled", token: spent.handle) }
    let group = DispatchGroup()
    let results = ConcurrentResults()
    for state in ["failure-one", "failure-two", "success"] {
        group.enter()
        DispatchQueue.global().async {
            defer { group.leave() }
            do { results.record(try engine.decide("Is it?", state)) }
            catch let failure as DoorFailure { results.record(failure, for: state) }
            catch { fatalError("other error: \(error)") }
        }
    }
    check(group.wait(timeout: .now() + 15) == .success, "concurrent calls")
    check(results.failure("failure-one")?.kind == .backend && results.failure("failure-one")!.message.contains("401"), "first thread error")
    check(results.failure("failure-two")?.kind == .backend && results.failure("failure-two")!.message.contains("403"), "second thread error")
    answer(results.answer()!.value, 1, 0.9)
    try held(engine, "hold-deadline", 3)
    try held(engine, "hold-bulk-1", 5, many: true)
    try held(engine, "hold-scalar", 5)
    answer(try engine.decide("Is it?", "recovery-scalar").value, 1, 0.9)
    print("STRICT_CANCELLED_SCALAR_PASS code=5 fresh-token-recovery")
    engine.close()
    error(1) { _ = try engine.call("{\"usage\":true}") }
    print("MATRIX_PASS")
}

do {
    guard CommandLine.arguments.count == 2 else { fatalError("mode direct|matrix") }
    if CommandLine.arguments[1] == "direct" { try direct() }
    else if CommandLine.arguments[1] == "matrix" { try matrix() }
    else if CommandLine.arguments[1] == "facts" { try factsLifetime() }
    else { fatalError("mode direct|matrix") }
} catch { FileHandle.standardError.write(Data("\(error)\n".utf8)); exit(1) }
