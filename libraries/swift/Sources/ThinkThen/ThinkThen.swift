import Foundation
import CThinkThen

public struct DoorFailure: Error, CustomStringConvertible {
    public let code: Int32
    public let retryable: Bool
    public let message: String
    public let factsJSON: String?
    public var kind: FailureKind { FailureKind(rawValue: code) ?? .defect }
    public init(code: Int32, retryable: Bool, message: String, factsJSON: String? = nil) {
        self.code = code
        self.retryable = retryable
        self.message = message
        self.factsJSON = factsJSON
    }
    public var description: String { "ThinkThen \(code): \(message)" }
}

public enum FailureKind: Int32, Sendable {
    case usage = 1, backend, deadline, local, cancelled, defect
}

// C strings are copied into call-scoped owned buffers, never bridged through a
// Swift temporary. Text evidence is byte-counted and may contain NUL bytes.
func withInput<R>(_ value: String, _ body: (UnsafePointer<CChar>) throws -> R) throws -> R {
    let bytes = Array(value.utf8)
    guard !bytes.contains(0) else { throw DoorFailure(code: 1, retryable: false, message: "interior NUL") }
    guard bytes.count < Int.max else { throw DoorFailure(code: 1, retryable: false, message: "input length exceeds host bounds") }
    let storage = UnsafeMutablePointer<CChar>.allocate(capacity: bytes.count + 1)
    defer { storage.deallocate() }
    for (i, byte) in bytes.enumerated() { storage[i] = CChar(bitPattern: byte) }
    storage[bytes.count] = 0
    return try body(UnsafePointer(storage))
}

func withEvidence<R>(_ value: String, _ body: (UnsafePointer<CChar>?, Int) throws -> R) rethrows -> R {
    let bytes = Array(value.utf8)
    let storage = UnsafeMutablePointer<CChar>.allocate(capacity: max(1, bytes.count))
    defer { storage.deallocate() }
    for (i, byte) in bytes.enumerated() { storage[i] = CChar(bitPattern: byte) }
    return try body(UnsafePointer(storage), bytes.count)
}

// Native outcomes are 0, 1 and 2; reject unknown C results at the boundary.
public enum Outcome: Int32, Sendable { case no = 0, yes = 1, unsure = 2 }
public struct Answer: Sendable {
    public let outcome: Outcome
    public let probability: Double
    init(_ raw: thinkthen_answer) throws {
        guard let value = Outcome(rawValue: raw.outcome) else {
            throw DoorFailure(code: 6, retryable: false, message: "invalid native outcome")
        }
        outcome = value
        probability = raw.probability
    }
}

// Facts, recognize and relate values stay JSON text; decode them with
// Foundation. A reader ignores members it does not know (ADR 0112).
public struct CallResult<Value: Sendable>: Sendable {
    public let value: Value
    public let facts: String
}

private func copyJSON(_ pointer: UnsafeMutablePointer<CChar>?, _ length: Int) throws -> String {
    guard let pointer, length >= 0 else {
        throw DoorFailure(code: 6, retryable: false, message: "missing native JSON")
    }
    return String(decoding: UnsafeRawBufferPointer(start: pointer, count: length), as: UTF8.self)
}

// One annotate answer member, decoded by Foundation: NSNull is unresolved,
// {"failed": {...}} is a failure, and any other value is answered. No answered
// value is an object, so another object is refused.
public enum AnnotatedField {
    case unresolved
    case answered(Any)
    case failed(kind: String, cause: String)
    public static func read(_ member: Any) throws -> AnnotatedField {
        if member is NSNull { return .unresolved }
        guard let object = member as? [String: Any] else { return .answered(member) }
        guard let failed = object["failed"] as? [String: Any], let kind = failed["kind"] as? String,
              let cause = failed["cause"] as? String else {
            throw DoorFailure(code: 6, retryable: false, message: "annotate member is an object but not a failure")
        }
        return .failed(kind: kind, cause: cause)
    }
}

// Join every call using a token before closing it; cancel() may run on another thread.
public final class CancelToken: @unchecked Sendable {
    public let handle: OpaquePointer
    public init() throws {
        guard let value = thinkthen_cancel_token_new() else {
            throw DoorFailure(code: 4, retryable: false, message: "native token allocation failed")
        }
        handle = value
    }
    deinit { thinkthen_cancel_token_free(handle) }
    public func cancel() { thinkthen_cancel(handle) }
}

// Native engine is thread-safe. Callers must join all users before close/deinit.
public final class Engine: @unchecked Sendable {
    private var handle: OpaquePointer?
    public init() throws {
        guard let engine = thinkthen_engine_new() else {
            throw DoorFailure(code: thinkthen_error_code(nil), retryable: thinkthen_error_retryable(nil) != 0,
                              message: thinkthen_error_message(nil).map { String(cString: $0) } ?? "native constructor failed")
        }
        handle = engine
    }
    public init(settingsJSON: String) throws {
        let engine = try withInput(settingsJSON) { thinkthen_engine_new_with($0) }
        guard let engine else {
            throw DoorFailure(code: thinkthen_error_code(nil), retryable: thinkthen_error_retryable(nil) != 0,
                              message: thinkthen_error_message(nil).map { String(cString: $0) } ?? "native constructor failed")
        }
        handle = engine
    }
    deinit { close() }
    // Only call close after all users have joined; no in-flight call may use a freed engine.
    public func close() { if let h = handle { thinkthen_engine_free(h); handle = nil } }
    private func open() throws -> OpaquePointer {
        guard let h = handle else { throw DoorFailure(code: 1, retryable: false, message: "closed engine") }
        return h
    }
    private func failure(_ h: OpaquePointer, _ code: Int32) -> DoorFailure {
        // Copy both borrowed strings before the next native call on this thread.
        let last = thinkthen_error_code(h)
        let retryable = thinkthen_error_retryable(h) != 0
        let message = thinkthen_error_message(h).map { String(cString: $0) } ?? "native call failed"
        let facts = thinkthen_error_facts_json(h).map { String(cString: $0) }
        precondition(last == code, "wrong native error slot")
        return DoorFailure(code: last, retryable: retryable, message: message, factsJSON: facts)
    }
    public func decide(_ question: String, _ text: String, deadline: Int64 = -1,
                token: OpaquePointer? = nil) throws -> CallResult<Answer> {
        let h = try open()
        return try withInput(question) { q in
            try withEvidence(text) { bytes, count in
                var answer = thinkthen_answer(outcome: 123, probability: -1)
                var facts: UnsafeMutablePointer<CChar>? = nil
                var factsLen = 0
                defer { thinkthen_free_string(facts) }
                let rc = thinkthen_decide_with_facts_opts(h, q, bytes, count, deadline, token, &answer, &facts, &factsLen)
                if rc != 0 {
                    let error = failure(h, rc)
                    precondition(answer.outcome == 123 && answer.probability == -1 && facts == nil && factsLen == 0, "failure changed output")
                    throw error
                }
                return try CallResult(value: Answer(answer), facts: copyJSON(facts, factsLen))
            }
        }
    }
    public func decideMany(_ question: String, _ texts: [String], deadline: Int64 = -1,
                    token: OpaquePointer? = nil) throws -> CallResult<[Answer]> {
        let h = try open()
        return try withInput(question) { q in
            let contents = texts.map { Array($0.utf8) }
            let allocations = contents.map { bytes -> UnsafeMutablePointer<CChar> in
                let p = UnsafeMutablePointer<CChar>.allocate(capacity: max(1, bytes.count))
                for (i, byte) in bytes.enumerated() { p[i] = CChar(bitPattern: byte) }
                return p
            }
            defer { allocations.forEach { $0.deallocate() } }
            var pointers: [UnsafePointer<CChar>?] = allocations.map { UnsafePointer($0) }
            var lengths = contents.map { $0.count }
            var answers = Array(repeating: thinkthen_answer(outcome: 123, probability: -1), count: texts.count)
            var facts: UnsafeMutablePointer<CChar>? = nil
            var factsLen = 0
            defer { thinkthen_free_string(facts) }
            let rc = pointers.withUnsafeMutableBufferPointer { ps in
                lengths.withUnsafeMutableBufferPointer { ls in
                    answers.withUnsafeMutableBufferPointer { a in
                        thinkthen_decide_many_with_facts_opts(h, q, ps.baseAddress, ls.baseAddress,
                                                   texts.count, deadline, token, a.baseAddress, &facts, &factsLen)
                    }
                }
            }
            if rc != 0 {
                let error = failure(h, rc)
                precondition(answers.allSatisfy { $0.outcome == 123 && $0.probability == -1 } && facts == nil && factsLen == 0, "bulk failure changed output")
                throw error
            }
            return try CallResult(value: answers.map(Answer.init), facts: copyJSON(facts, factsLen))
        }
    }
    public func call(_ request: String, deadline: Int64 = -1, token: OpaquePointer? = nil) throws -> String {
        let h = try open()
        return try withInput(request) { q in
            guard let pointer = thinkthen_call_opts(h, q, deadline, token) else { throw failure(h, thinkthen_error_code(h)) }
            defer { thinkthen_free_string(pointer) }
            return String(cString: pointer)
        }
    }
    public func recognize(_ spec: String, _ text: String, deadline: Int64 = -1,
                          token: OpaquePointer? = nil) throws -> CallResult<String> {
        let h = try open()
        return try withInput(spec) { q in
            try withEvidence(text) { bytes, count in
                var result: UnsafeMutablePointer<CChar>? = nil
                var length: Int = 991
                var facts: UnsafeMutablePointer<CChar>? = nil
                var factsLen = 0
                defer { thinkthen_free_string(result); thinkthen_free_string(facts) }
                let rc = thinkthen_recognize_with_facts_opts(h, q, bytes, count, deadline, token, &result, &length, &facts, &factsLen)
                if rc != 0 {
                    let error = failure(h, rc)
                    precondition(result == nil && length == 991 && facts == nil && factsLen == 0, "recognize failure changed output")
                    throw error
                }
                return try CallResult(value: copyJSON(result, length), facts: copyJSON(facts, factsLen))
            }
        }
    }
    public func relate(_ spec: String, _ records: [String], deadline: Int64 = -1,
                       token: OpaquePointer? = nil) throws -> CallResult<String> {
        let h = try open()
        return try withInput(spec) { q in
            let contents = records.map { Array($0.utf8) }
            let allocations = contents.map { bytes -> UnsafeMutablePointer<CChar> in
                let p = UnsafeMutablePointer<CChar>.allocate(capacity: max(1, bytes.count))
                for (i, byte) in bytes.enumerated() { p[i] = CChar(bitPattern: byte) }
                return p
            }
            defer { allocations.forEach { $0.deallocate() } }
            var pointers: [UnsafePointer<CChar>?] = allocations.map { UnsafePointer($0) }
            var lengths = contents.map { $0.count }
            var result: UnsafeMutablePointer<CChar>? = nil
            var length: Int = 991
            var facts: UnsafeMutablePointer<CChar>? = nil
            var factsLen = 0
            defer { thinkthen_free_string(result); thinkthen_free_string(facts) }
            let rc = pointers.withUnsafeMutableBufferPointer { ps in
                lengths.withUnsafeMutableBufferPointer { ls in
                    thinkthen_relate_with_facts_opts(h, q, ps.baseAddress, ls.baseAddress, records.count, deadline, token, &result, &length, &facts, &factsLen)
                }
            }
            if rc != 0 {
                let error = failure(h, rc)
                precondition(result == nil && length == 991 && facts == nil && factsLen == 0, "relate failure changed output")
                throw error
            }
            return try CallResult(value: copyJSON(result, length), facts: copyJSON(facts, factsLen))
        }
    }
    // Preview a decide, choose, score or tag call without sending it. The
    // question is bare text, or one question object when it starts with "{",
    // as decide reads it. settings is nil or a thinkthen.settings/1 object.
    // Returns the result schema's plan object as JSON text. The preview
    // needs no key, reads no cache and sends nothing.
    public func plan(_ verb: String, _ question: String, _ input: [String], settings: String? = nil) throws -> String {
        let h = try open()
        func object(_ text: String) throws -> String {
            let trimmed = text.trimmingCharacters(in: .whitespacesAndNewlines)
            guard (try? JSONSerialization.jsonObject(with: Data(trimmed.utf8))) is [String: Any] else {
                throw DoorFailure(code: 1, retryable: false, message: "plan question object or settings is not a JSON object")
            }
            return trimmed
        }
        let encoder = JSONEncoder()
        func text<T: Encodable>(_ value: T) -> String { String(decoding: try! encoder.encode(value), as: UTF8.self) }
        let asked = question.trimmingCharacters(in: .whitespacesAndNewlines).hasPrefix("{") ? try object(question) : text(question)
        let request = "{\"verb\":\(text(verb)),\"question\":\(asked),\"input\":\(text(input))"
            + (try settings.map { ",\"settings\":" + (try object($0)) } ?? "") + "}"
        return try withInput(request) { q in
            var out: UnsafeMutablePointer<CChar>? = nil
            var length = 0
            let rc = thinkthen_plan_json(h, q, &out, &length)
            if rc != 0 { throw failure(h, rc) }
            defer { thinkthen_free_string(out) }
            return try copyJSON(out, length)
        }
    }
}
