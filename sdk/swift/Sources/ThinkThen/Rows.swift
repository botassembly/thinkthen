import Foundation
import Synchronization

enum InputPayload: Sendable {
  case text(String)
  case json(Data)
  case missing, invalid
}
struct InputRecord: Sendable {
  let payload: InputPayload
  let item: any Sendable
  init<T: Encodable & Sendable>(_ item: T) {
    self.item = item
    if let value = item as? String {
      payload = .text(value)
    } else if let value = item as? Record {
      switch value {
      case .text(let text): payload = .text(text)
      case .missing: payload = .missing
      case .json(let json): payload = Self.jsonPayload(json)
      }
    } else {
      payload = Self.jsonPayload(item)
    }
  }
  private static func jsonPayload<T: Encodable>(_ value: T) -> InputPayload {
    do { return .json(try JSONEncoder().encode(value)) } catch { return .invalid }
  }
}
protocol RecordSource: Sendable {
  func next() async throws -> InputRecord?
}
private actor SequenceSource<S: AsyncSequence & Sendable>: RecordSource
where S.Element: Encodable & Sendable {
  let source: S
  var iterator: S.AsyncIterator?
  init(_ source: S) { self.source = source }
  func next() async throws -> InputRecord? {
    var current = iterator ?? source.makeAsyncIterator()
    iterator = nil
    defer { iterator = current }
    return try await current.next(isolation: self).map { InputRecord($0) }
  }
}
private actor SynchronousSource<S: Sequence & Sendable>: RecordSource
where S.Element: Encodable & Sendable {
  let source: S
  var iterator: S.Iterator?
  init(_ source: S) { self.source = source }
  func next() throws -> InputRecord? {
    var current = iterator ?? source.makeIterator()
    defer { iterator = current }
    return current.next().map { InputRecord($0) }
  }
}
func source<S: Sequence & Sendable>(_ input: S) -> any RecordSource
where S.Element: Encodable & Sendable { SynchronousSource(input) }
func source<S: AsyncSequence & Sendable>(_ input: S) -> any RecordSource
where S.Element: Encodable & Sendable { SequenceSource(input) }

final class Driver<A: Sendable>: Sendable {
  private struct State: Sendable {
    var busy = false, cancelled = false, finished = false
    var call: NativeCall?
    var pump: InputPump?
    var inputPending = false
    var items: [UInt64: any Sendable] = [:]
    var accepted: UInt64 = 0
  }
  private let state = Mutex(State())
  private let client: Client, request: Result<Data, any Error>, source: any RecordSource
  private let name: String?
  private let retainItems: Bool
  private let decode: @Sendable (WireRow, (any Sendable)?) throws -> A
  init(
    client: Client, request: Result<Data, any Error>, name: String?, source: any RecordSource,
    retainItems: Bool,
    decode: @escaping @Sendable (WireRow, (any Sendable)?) throws -> A
  ) {
    self.name = name
    self.retainItems = retainItems
    self.client = client
    self.request = request
    self.source = source
    self.decode = decode
  }
  func cancel() {
    let (call, pump) = state.withLock { state in
      state.cancelled = true
      return (state.call, state.pump)
    }
    call?.cancel()
    pump?.cancel()
  }
  func close() async {
    cancel()
    let native = state.withLock { state in state.call }
    await native?.close()
  }
  func next() async throws -> A? {
    let proceed = try state.withLock { state -> Bool in
      guard !state.busy else { throw CallError.iteration(.concurrentNext) }
      if state.finished { return false }
      state.busy = true
      return true
    }
    guard proceed else { return nil }
    defer { state.withLock { state in state.busy = false } }
    return try await withTaskCancellationHandler {
      do {
        if state.withLock({ $0.cancelled }) || Task.isCancelled { throw CancellationError() }
        let native: NativeCall
        if let call = state.withLock({ $0.call }) {
          native = call
        } else {
          let bytes: Data
          do { bytes = try request.get() } catch {
            if error is any ThinkThenError { throw error }
            throw CallError.encoding(message: String(describing: error))
          }
          let opened: NativeCall = try await withCheckedThrowingContinuation { continuation in
            let client = client
            let name = name
            DispatchQueue.global().async {
              do {
                continuation.resume(
                  returning: try NativeCall(
                    client: client, request: bytes, name: name))
              } catch { continuation.resume(throwing: error) }
            }
          }
          let cancelled = state.withLock {
            $0.call = opened
            return $0.cancelled
          }
          if cancelled { opened.cancel() }
          native = opened
        }
        while true {
          try Task.checkCancellation()
          let event = try await native.perform { [self] handle, events, library in
            while true {
              let version = library.eventVersion(events)
              let event = try library.next(handle)
              guard case .input = event else { return event }
              let pump = state.withLock { state in
                if let pump = state.pump { return pump }
                let pump = InputPump(source: source) { [weak native] in native?.wake() }
                state.pump = pump
                return pump
              }
              if state.withLock({ $0.cancelled }) { pump.cancel() }
              let request = state.withLock { state -> Bool in
                if state.inputPending { return false }
                state.inputPending = true
                return true
              }
              if request { pump.request() }
              if let result = pump.take() {
                state.withLock { $0.inputPending = false }
                if state.withLock({ $0.cancelled }) { throw CancellationError() }
                if let record = try result.get() {
                  if retainItems {
                    state.withLock {
                      $0.accepted += 1
                      $0.items[$0.accepted] = record.item
                    }
                  }
                  try library.send(handle, record.payload)
                } else {
                  library.end(handle)
                }
              } else {
                library.eventWait(events, version)
              }
            }
          }
          if Task.isCancelled || state.withLock({ $0.cancelled }) { throw CancellationError() }
          switch event {
          case .input: continue
          case .row(let wire):
            let item = state.withLock { $0.items.removeValue(forKey: wire.position) }
            return try decode(wire, item)
          case .end:
            state.withLock { state in
              state.finished = true
              state.items.removeAll()
            }
            let pump = state.withLock { $0.pump }
            pump?.cancel()
            await native.close()
            return nil
          }
        }
      } catch {
        state.withLock { state in
          state.finished = true
          state.items.removeAll()
        }
        await close()
        if Task.isCancelled { throw CancellationError() }
        throw error
      }
    } onCancel: {
      self.cancel()
    }
  }
  deinit {
    let (call, pump) = state.withLock { ($0.call, $0.pump) }
    call?.cancel()
    pump?.cancel()
  }
}

// The stream holds the active driver weakly, so breaking iteration releases it.
private final class Traversal<A: Sendable>: Sendable {
  private struct State: Sendable {
    var claimed = false, cancelled = false
    weak var driver: Driver<A>?
  }
  private let state = Mutex(State())
  let make: @Sendable () -> Driver<A>
  init(_ make: @escaping @Sendable () -> Driver<A>) { self.make = make }
  func claim() -> Driver<A>? {
    let (driver, cancelled) = state.withLock { state -> (Driver<A>?, Bool) in
      guard !state.claimed else { return (nil, false) }
      state.claimed = true
      let driver = make()
      state.driver = driver
      return (driver, state.cancelled)
    }
    if cancelled { driver?.cancel() }
    return driver
  }
  func cancel() {
    let driver = state.withLock {
      $0.cancelled = true
      return $0.driver
    }
    driver?.cancel()
  }
}
/// A lazy, single-pass stream. Releasing its iterator stops native work.
public final class Rows<Element: Sendable>: AsyncSequence, Sendable {
  /// The stream may throw a native error or CancellationError.
  public typealias Failure = any Error
  private let iterator: @Sendable () -> AsyncIterator
  private let cancellation: @Sendable () -> Void
  init(make: @escaping @Sendable () -> Driver<Element>) {
    let traversal = Traversal(make)
    iterator = { AsyncIterator(traversal.claim()) }
    cancellation = { traversal.cancel() }
  }
  init(iterator: @escaping @Sendable () -> AsyncIterator, cancel: @escaping @Sendable () -> Void) {
    self.iterator = iterator
    cancellation = cancel
  }
  /// The typed AsyncIterator value exchanged with the engine.
  public final class AsyncIterator: AsyncIteratorProtocol, Sendable {
    /// The stream may throw a native error or CancellationError.
    public typealias Failure = any Error
    private let advance: @Sendable () async throws -> Element?
    private let cancellation: @Sendable () -> Void
    fileprivate init(_ driver: Driver<Element>?) {
      advance = {
        guard let driver else { throw CallError.iteration(.alreadyIterated) }
        return try await driver.next()
      }
      cancellation = { driver?.cancel() }
    }
    init(
      next: @escaping @Sendable () async throws -> Element?, cancel: @escaping @Sendable () -> Void
    ) {
      advance = next
      cancellation = cancel
    }
    /// Read the next element, return nil at the end, or throw the stream error.
    public func next() async throws -> Element? {
      do { return try await advance() } catch { throw typedError(error) }
    }
    /// Read the next element, return nil at the end, or throw the stream error.
    public func next(isolation actor: isolated (any Actor)?) async throws -> Element? {
      try await next()
    }
    deinit { cancellation() }
  }
  /// Claim the single traversal; a second traversal throws an iteration error.
  public func makeAsyncIterator() -> AsyncIterator { iterator() }
  /// Stop pending input and native work; the next read throws CancellationError.
  public func cancel() { cancellation() }
  /// Read the remaining stream into an array, preserving any iteration error.
  public func collect() async throws -> [Element] {
    let iterator = makeAsyncIterator()
    var output: [Element] = []
    while let element = try await iterator.next() { output.append(element) }
    return output
  }
}
