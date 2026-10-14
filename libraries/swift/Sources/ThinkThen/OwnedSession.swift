import Foundation
import CThinkThen

public struct SessionBoundaryFailure: Error, Sendable, CustomStringConvertible {
    public let code: Int32
    public let message: String
    public var retryable = false
    public var facts: JSONValue? = nil
    public var description: String { message }
    public var kind: OwnedFailureKind { nativeFailureKind(code) }

}

func sessionCheck(_ code: Int32) throws {
    if code != 0 { throw SessionBoundaryFailure(code: code, message: String(cString: thinkthen_session_error_message()), retryable: thinkthen_session_error_retryable() != 0, facts: try thinkthen_session_error_facts_json().map { try JSONValue.parse(Data(String(cString: $0).utf8)) }) }
}

private final class SessionOwner: @unchecked Sendable {
    let handle: OpaquePointer
    init(_ handle: OpaquePointer) { self.handle = handle }
    deinit { thinkthen_session_free(handle) }
}

/// A bounded native session. One producer and one reader may operate concurrently.
public final class Session: @unchecked Sendable {
    private let lock = NSLock()
    private var owner: SessionOwner?
    private var reading = false
    private var producing = false
    private var cancelled = false
    init(_ handle: OpaquePointer) { owner = SessionOwner(handle) }
    deinit { close() }
    private func lease() throws -> SessionOwner {
        try lock.withLock {
            guard let owner else { throw SessionBoundaryFailure(code: Int32(THINKTHEN_EUSAGE), message: "Closed session") }
            return owner
        }
    }
    private func begin(reader: Bool) throws {
        try lock.withLock {
            guard owner != nil else { throw SessionBoundaryFailure(code: Int32(THINKTHEN_EUSAGE), message: "Closed session") }
            guard !(reader ? reading : producing) else { throw SessionBoundaryFailure(code: Int32(THINKTHEN_EUSAGE), message: reader ? "Session already has a reader" : "Session already has a producer") }
            if reader { reading = true } else { producing = true }
        }
    }
    private func end(reader: Bool) { lock.withLock { if reader { reading = false } else { producing = false } } }
    public func cancel() {
        let current = lock.withLock { cancelled = true; return owner }
        if let current { withExtendedLifetime(current) { thinkthen_session_cancel(current.handle) } }
    }
    /// Detach ownership without waiting for native settlement or a blocked provider.
    public func close() {
        let current = lock.withLock { let current = owner; owner = nil; return current }
        if let current { withExtendedLifetime(current) { thinkthen_session_cancel(current.handle) } }
    }
    public func finish(_ failure: InputRequestReaderFailure? = nil) throws {
        let current = try lease()
        let bytes = try failure?.json.data()
        try withExtendedLifetime(current) {
            if let bytes {
                try bytes.withUnsafeBytes { raw in
                    try sessionCheck(thinkthen_session_finish(current.handle, raw.bindMemory(to: CChar.self).baseAddress, raw.count))
                }
            } else { try sessionCheck(thinkthen_session_finish(current.handle, nil, 0)) }
        }
    }
    public func push(_ descriptor: InputRequestSessionDescriptor) async throws -> Bool {
        try begin(reader: false)
        defer { end(reader: false) }
        let bytes = try descriptor.json.data()
        return try await withTaskCancellationHandler {
            try Task.checkCancellation()
            let current = try lease()
            let observed: Result<Bool, any Error> = await withCheckedContinuation { continuation in
                DispatchQueue.global(qos: .userInitiated).async {
                    do {
                        var status: UInt32 = 0
                        try withExtendedLifetime(current) {
                            try bytes.withUnsafeBytes { raw in
                                try sessionCheck(thinkthen_session_push(current.handle, raw.bindMemory(to: CChar.self).baseAddress, raw.count, &status))
                            }
                        }
                        continuation.resume(returning: .success(status == UInt32(THINKTHEN_SESSION_ACCEPTED_V1)))
                    } catch { continuation.resume(returning: .failure(error)) }
                }
            }
            try Task.checkCancellation()
            return try observed.get()
        } onCancel: { self.cancel() }
    }
    /// Read a packet, including terminal settlement after an explicit cancel.
    public func read() async throws -> OwnedSessionPacket? {
        try begin(reader: true)
        defer { end(reader: true) }
        return try await withTaskCancellationHandler {
            try Task.checkCancellation()
            let current = try lease()
            let interest = lock.withLock { cancelled ? UInt32(THINKTHEN_SESSION_WAIT_SETTLEMENT_V1) : UInt32(THINKTHEN_SESSION_WAIT_RESULT_V1) }
            try Task.checkCancellation()
            let observed: Result<Data?, any Error> = await withCheckedContinuation { continuation in
                DispatchQueue.global(qos: .userInitiated).async {
                    do {
                        let copied: Data? = try withExtendedLifetime(current) {
                            var status: UInt32 = 0
                            var packet: OpaquePointer?
                            try sessionCheck(thinkthen_session_next_packet(current.handle, interest, &status, &packet))
                            guard let packet else {
                                guard status == UInt32(THINKTHEN_SESSION_END_V1) else { throw JSONConversionError("Missing native packet") }
                                return nil
                            }
                            defer { thinkthen_session_result_free(packet) }
                            var pointer: UnsafePointer<CChar>?
                            var length = 0
                            try sessionCheck(thinkthen_session_result_json(packet, &pointer, &length))
                            guard let pointer else { throw JSONConversionError("Missing native packet bytes") }
                            return Data(bytes: pointer, count: length)
                        }
                        continuation.resume(returning: .success(copied))
                    } catch { continuation.resume(returning: .failure(error)) }
                }
            }
            try Task.checkCancellation()
            return try observed.get().map { try OwnedSessionPacket.read(JSONValue.parse($0)) }
        } onCancel: { self.cancel() }
    }

}

public struct OwnedCall: Sendable {
    public let packets: [OwnedSessionPacket]
    public let terminal: OwnedSessionPacketTerminal
}

public struct SessionFailure: Error, Sendable, CustomStringConvertible {
    public let failure: OwnedCallError
    public let call: OwnedCall
    public var description: String { failure.error.message }
    public var kind: OwnedFailureKind { failure.error.kind }
    public var retryable: Bool { failure.error.retryable }
}

public struct FeedReadFailure: Error, Sendable {
    public let failure: InputRequestReaderFailure
    public init(_ failure: InputRequestReaderFailure) { self.failure = failure }
}

private final class ProducerFailure: @unchecked Sendable {
    private let lock = NSLock()
    private var failure: (any Error)?
    func record(_ failure: any Error) { lock.withLock { self.failure = failure } }
    func check() throws { if let failure = lock.withLock({ failure }) { throw failure } }
}

private final class ClientOwner: @unchecked Sendable {
    let handle: OpaquePointer
    init(_ handle: OpaquePointer) { self.handle = handle }
    deinit { thinkthen_engine_free(handle) }
}

/// The generated ten-function family. Inputs and outputs are owned Swift values.
public final class Client: @unchecked Sendable {
    private let lock = NSLock()
    private let authoredLock = NSLock()
    private var owner: ClientOwner?
    public init(settings: InputEngineSettings = InputEngineSettings()) throws {
        try requireNativePackage()
        let bytes = try settings.json.data()
        let handle = Array(bytes).map { CChar(bitPattern: $0) } + [0]
        let pointer = handle.withUnsafeBufferPointer { thinkthen_engine_new_with($0.baseAddress) }
        guard let pointer else {
            throw SessionBoundaryFailure(code: thinkthen_error_code(nil), message: thinkthen_error_message(nil).map { String(cString: $0) } ?? "Native constructor failed", retryable: thinkthen_error_retryable(nil) != 0, facts: try thinkthen_error_facts_json(nil).map { try JSONValue.parse(Data(String(cString: $0).utf8)) })
        }
        owner = ClientOwner(pointer)
    }
    deinit { close() }
    public func close() {
        let current = lock.withLock { let current = owner; owner = nil; return current }
        withExtendedLifetime(current) { }
    }
    private func live<T>(_ body: (OpaquePointer) throws -> T) throws -> T {
        let current = try lock.withLock {
            guard let owner else { throw SessionBoundaryFailure(code: Int32(THINKTHEN_EUSAGE), message: "Closed client") }
            return owner
        }
        return try withExtendedLifetime(current) { try body(current.handle) }
    }
    public func start(_ request: InputRequest) throws -> Session {
        let bytes = try request.json.data()
        return try live { handle in
            var session: OpaquePointer?
            let surface = Array("swift".utf8).map { CChar(bitPattern: $0) }
            try bytes.withUnsafeBytes { raw in
                try surface.withUnsafeBufferPointer { token in
                    try sessionCheck(thinkthen_session_new_with_surface(handle, raw.bindMemory(to: CChar.self).baseAddress, raw.count, token.baseAddress, token.count, &session))
                }
            }
            guard let session else { throw JSONConversionError("Missing native session") }
            return Session(session)
        }
    }
    public func plan(_ request: InputRequest) throws -> OwnedPlan {
        let bytes = try request.json.data()
        return try live { handle in
            var pointer: OpaquePointer?
            try bytes.withUnsafeBytes { raw in
                try sessionCheck(thinkthen_request_plan_value(handle, raw.bindMemory(to: CChar.self).baseAddress, raw.count, &pointer))
            }
            guard let pointer else { throw JSONConversionError("Missing native plan") }
            defer { thinkthen_value_free(pointer) }
            return try OwnedPlan.read(nativeValue(pointer))
        }
    }
    public func parseQuestion(_ grammar: QuestionGrammar, authored: JSONValue) throws -> InputRequestQuestion {
        let bytes = try authored.data()
        return try live { handle in
            var question: OpaquePointer?
            try authoredLock.withLock {
                try bytes.withUnsafeBytes { raw in
                    let code = thinkthen_question_parse(handle, grammar.rawValue, thinkthen_string_v1(data: raw.bindMemory(to: CChar.self).baseAddress, len: raw.count), &question)
                    if code != 0 {
                        throw SessionBoundaryFailure(code: code, message: thinkthen_error_message(handle).map { String(cString: $0) } ?? "Native question admission failed", retryable: thinkthen_error_retryable(handle) != 0, facts: try thinkthen_error_facts_json(handle).map { try JSONValue.parse(Data(String(cString: $0).utf8)) })
                    }
                }
            }
            guard let question else { throw JSONConversionError("Missing admitted question") }
            defer { thinkthen_question_free(question) }
            return .definition(InputRequestQuestionDefinition(value: try InputRequestDefinition.read(authored)))
        }
    }
    public func execute(_ request: InputRequest) async throws -> OwnedCall {
        try Task.checkCancellation()
        let session = try start(request)
        defer { session.close() }
        return try await withTaskCancellationHandler {
            try session.finish()
            return try await collect(session)
        } onCancel: { session.cancel() }
    }
    /// Feed and output advance concurrently. The native queues retain their own bounds.
    public func execute(_ request: InputRequest, feed: AsyncThrowingStream<InputRequestSessionDescriptor, any Error>) async throws -> OwnedCall {
        try Task.checkCancellation()
        let session = try start(request)
        let failure = ProducerFailure()
        let producer = Task {
            do {
                for try await descriptor in feed {
                    try Task.checkCancellation()
                    if try await !session.push(descriptor) { return }
                }
                try session.finish()
            } catch let error as FeedReadFailure {
                do { try session.finish(error.failure) } catch { failure.record(error); session.cancel() }
            } catch {
                if !Task.isCancelled { failure.record(error); session.cancel() }
            }
        }
        defer { producer.cancel(); session.close() }
        return try await withTaskCancellationHandler {
            do {
                let call = try await collect(session)
                producer.cancel()
                await producer.value
                try failure.check()
                return call
            } catch {
                producer.cancel()
                session.cancel()
                await producer.value
                try Task.checkCancellation()
                try failure.check()
                throw error
            }
        } onCancel: { producer.cancel(); session.cancel() }
    }
    private func collect(_ session: Session) async throws -> OwnedCall {
        var packets: [OwnedSessionPacket] = []
        var terminal: OwnedSessionPacketTerminal?
        while let packet = try await session.read() {
            packets.append(packet)
            if case .terminal(let value) = packet { terminal = value }
        }
        try Task.checkCancellation()
        guard let terminal else { throw JSONConversionError("Native End has no terminal") }
        let call = OwnedCall(packets: packets, terminal: terminal)
        if case .value(let failure) = terminal.failure { throw SessionFailure(failure: failure, call: call) }
        return call
    }
    func usage(_ operation: (OpaquePointer?, UnsafeMutablePointer<thinkthen_complete_usage_persistence_v1>?, UnsafeMutablePointer<thinkthen_complete_utf8_v1>?) -> Int32) throws -> OwnedUsagePersistenceStatus {
        try live { handle in
            var state = thinkthen_complete_usage_persistence_v1()
            var advice = thinkthen_complete_utf8_v1()
            try sessionCheck(operation(handle, &state, &advice))
            let copied = advice.data.map { String(decoding: UnsafeRawBufferPointer(start: $0, count: advice.len), as: UTF8.self) }
            guard let kind = UsagePersistenceState(rawValue: state.kind) else { throw JSONConversionError("Unknown native persistence state") }
            return OwnedUsagePersistenceStatus(state: kind, advice: copied)
        }
    }
}

public struct OwnedUsagePersistenceStatus: Sendable {
    public let state: UsagePersistenceState
    public let advice: String?
}
