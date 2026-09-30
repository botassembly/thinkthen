import Foundation
import ThinkThen

// With one argument, it is one JSON-door request and stdout its reply.
// "fields REQUEST" reads each annotate row member through AnnotatedField.read.
// "plan INPUT" reads one thinkthen.plan-input/1 object and prints Engine.plan's object.
// "limits" checks ticket 0291's zero budgets and zero cap; "helper" checks
// AnnotatedField.read's edge table. Each prints one JSON line.
func json(_ text: String) -> Any { try! JSONSerialization.jsonObject(with: Data(text.utf8), options: [.fragmentsAllowed]) }
func line(_ value: Any) -> String {
    String(decoding: try! JSONSerialization.data(withJSONObject: value, options: [.sortedKeys]), as: UTF8.self)
}
func text(_ value: Any) -> String {
    String(decoding: try! JSONSerialization.data(withJSONObject: value, options: [.fragmentsAllowed]), as: UTF8.self)
}
func state(_ field: AnnotatedField) -> String {
    switch field {
    case .unresolved: return "unresolved"
    case .answered: return "answered"
    case .failed(let kind, let cause): return "failed \(kind) \(cause)"
    }
}

// ADR 0112 section 4: null is unresolved, {"failed": ...} is a failure whose
// unknown extra member reads without error, and any other value is answered.
func helper() throws -> String {
    let unresolved = try AnnotatedField.read(NSNull())
    let failed = try AnnotatedField.read(json("{\"failed\":{\"kind\":\"backend\",\"cause\":\"missing_probability\",\"later\":1}}"))
    precondition(state(unresolved) == "unresolved" && state(failed) == "failed backend missing_probability")
    for member in ["true", "\"billing\"", "[\"billing\",\"urgent\"]", "1.2"] {
        guard case .answered(let value) = try AnnotatedField.read(json(member)) else { fatalError("helper \(member)") }
        precondition(text(value) == member, "helper \(member)")
    }
    for member in ["{\"failed\":null}", "{\"team\":\"billing\"}"] {
        precondition((try? AnnotatedField.read(json(member))) == nil, "helper read an object as an answer: \(member)")
    }
    return line(["helper": "pass"])
}

// Ticket 0291: a zero cap and a zero budget each refuse before sending. Relate
// gets two entities, since one entity has no pair to ask.
func limits(_ engine: Engine) throws -> String {
    func refused(_ call: () throws -> Void) -> DoorFailure {
        do { try call() } catch let failure as DoorFailure { return failure } catch { fatalError("\(error)") }
        fatalError("a limited call succeeded")
    }
    let capped = try Engine(settingsJSON: "{\"max_requests_total\":0,\"cache\":false}")
    let cap = refused { _ = try capped.decide("Is it?", "capped") }
    precondition(cap.kind == .usage && cap.code == 1 && cap.message.contains("process send budget"), "cap: \(cap.message)")
    capped.close()
    let calls: [() throws -> Void] = [
        { _ = try engine.call("{\"decide\":\"Is it?\",\"evidence\":\"zero-call\"}", deadline: 0) },
        { _ = try engine.recognize("{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A person's name.\"}}}", "zero-recognize", deadline: 0) },
        { _ = try engine.relate("{\"version\":1,\"relate\":{\"relations\":[{\"name\":\"caused_by\",\"source\":\"alert\",\"target\":\"alert\"}]}}",
                                ["{\"name\":\"A\",\"kind\":\"alert\"}", "{\"name\":\"B\",\"kind\":\"alert\"}"], deadline: 0) },
    ]
    for call in calls {
        let zero = refused(call)
        precondition(zero.kind == .deadline && zero.code == 3, "zero budget: \(zero.message)")
    }
    return line(["limits": "pass"])
}

do {
    let args = Array(CommandLine.arguments.dropFirst())
    guard args.count == 1 || args.count == 2 else { throw DoorFailure(code: 1, retryable: false, message: "one request required") }
    let engine = try Engine()
    defer { engine.close() }
    switch (args.count, args[0]) {
    case (1, "helper"): print(try helper())
    case (1, "limits"): print(try limits(engine))
    case (2, "plan"):
        let input = json(args[1]) as! [String: Any]
        let question = input["question"]!
        print(try engine.plan(input["verb"] as! String, question as? String ?? text(question), input["input"] as! [String],
                              settings: input["settings"].map(text)))
    case (2, "fields"):
        let envelope = json(try engine.call(args[1])) as! [String: Any]
        let rows = (envelope["value"] as! [[String: Any]]).map { row in try! row.mapValues { state(try AnnotatedField.read($0)) } }
        print(line(rows))
    default: print(try engine.call(args[0]))
    }
} catch let failure as DoorFailure {
    let value: [String: Any] = ["failed": ["kind": String(describing: failure.kind), "code": Int(failure.code)], "retryable": failure.retryable,
                                "facts": failure.factsJSON.map { $0 as Any } ?? NSNull()]
    print(line(value))
}
