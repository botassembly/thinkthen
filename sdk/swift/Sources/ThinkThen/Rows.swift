import Foundation
import Synchronization

// A single completion races cancellation against an asynchronous source. A late
// source result has no native pointer and can only resolve this empty slot.
private final class Completion<T: Sendable>: Sendable {
  private struct State: Sendable {
    var continuation: CheckedContinuation<T, any Error>?
    var result: Result<T, any Error>?
  }
  private let state = Mutex(State())
  func install(_ value: CheckedContinuation<T, any Error>) {
    let ready = state.withLock { state -> Result<T, any Error>? in
      if let result = state.result { return result }
      state.continuation = value
      return nil
    }
    if let ready { value.resume(with: ready) }
  }
  func finish(_ value: Result<T, any Error>) {
    let waiter = state.withLock { state -> CheckedContinuation<T, any Error>? in
      guard state.result == nil else { return nil }
      state.result = value
      defer { state.continuation = nil }
      return state.continuation
    }
    waiter?.resume(with: value)
  }
}
protocol RecordSource: Sendable {
  var finiteRecords: [Record]? { get }
  func next() async throws -> Record?
}
extension RecordSource { var finiteRecords: [Record]? { nil } }
private actor SequenceSource<S: AsyncSequence & Sendable>: RecordSource where S.Element == Record {
  private let source: S
  var iterator: S.AsyncIterator?
  init(_ source: S) { self.source = source }
  func next() async throws -> Record? {
    var current = iterator ?? source.makeAsyncIterator()
    iterator = nil
    defer { iterator = current }
    return try await current.next(isolation: self)
  }
}
private actor SynchronousSource<S: Sequence & Sendable>: RecordSource where S.Element == Record {
  let source: S
  nonisolated var finiteRecords: [Record]? { source as? [Record] }
  var iterator: S.Iterator?
  init(_ source: S) { self.source = source }
  func next() -> Record? {
    var current = iterator ?? source.makeIterator()
    let value = current.next()
    iterator = current
    return value
  }
}
func source<S: AsyncSequence & Sendable>(_ input: S) -> any RecordSource where S.Element == Record {
  SequenceSource(input)
}
func source<S: Sequence & Sendable>(_ input: S) -> any RecordSource where S.Element == Record {
  SynchronousSource(input)
}

// Only this private wrapper crosses worker boundaries with an imported handle.
// Its owner serializes driving and frees only after the last queued operation.
struct NativeHandle: @unchecked Sendable { let pointer: OpaquePointer }
// The lock protects pointer lifetime against cancel. All other operations run
// on queue. No caller waits synchronously for that queue or the backend.
final class NativeCall: Sendable {
  private let state: Mutex<NativeHandle?>
  private let queue = DispatchQueue(label: "ThinkThen.call")
  private let native: Native
  init(client: Client, request: Data, name: String?, records: [Record]?) throws {
    native = try Native.loaded.get()
    state = Mutex(try client.open(request, name: name, records: records))
  }
  func cancel() { state.withLock { if let handle = $0 { native.cancel(handle.pointer) } } }
  func perform<T: Sendable>(_ operation: @escaping @Sendable (OpaquePointer, Native) throws -> T)
    async throws -> T
  {
    try await withCheckedThrowingContinuation { continuation in
      queue.async { [self] in
        do {
          guard let handle = state.withLock({ $0 }) else { throw ThinkThenError.cancelled }
          continuation.resume(returning: try operation(handle.pointer, native))
        } catch { continuation.resume(throwing: error) }
      }
    }
  }
  func close() async {
    await withCheckedContinuation { continuation in
      queue.async { [self] in
        state.withLock { handle in
          if let handle { native.free(handle.pointer) }
          handle = nil
        }
        continuation.resume()
      }
    }
  }
  deinit {
    let handle = state.withLock { handle in
      let value = handle
      handle = nil
      return value
    }
    if let handle {
      let native = native
      queue.async { native.free(handle.pointer) }
    }
  }
}
final class Driver<A: Sendable>: Sendable {
  private struct State: Sendable {
    var busy = false, cancelled = false, finished = false
    var call: NativeCall?
    var interruptInput: (@Sendable () -> Void)?
    var items: [UInt64: Record] = [:]
    var accepted: UInt64 = 0
  }
  private let state = Mutex(State())
  private let client: Client, request: Result<Data, any Error>, source: any RecordSource
  private let name: String?
  private let retainItems: Bool
  private let decode: @Sendable (WireRow, Record?) throws -> A
  init(
    client: Client, request: Result<Data, any Error>, name: String?, source: any RecordSource,
    retainItems: Bool,
    decode: @escaping @Sendable (WireRow, Record?) throws -> A
  ) {
    self.name = name
    self.retainItems = retainItems
    self.client = client
    self.request = request
    self.source = source
    self.decode = decode
  }
  func cancel() {
    state.withLock { state in
      state.cancelled = true
      state.call?.cancel()
      state.interruptInput?()
    }
  }
  func close() async {
    cancel()
    let native = state.withLock { state in state.call }
    await native?.close()
  }
  private func input() async throws -> Record? {
    let completion = Completion<Record?>()
    let source = self.source
    let task = Task {
      do { completion.finish(.success(try await source.next())) } catch {
        completion.finish(
          .failure(
            error is CancellationError
              ? ThinkThenError.cancelled
              : ThinkThenError.recordSource(message: String(describing: error))))
      }
    }
    state.withLock { state in
      let interrupt: @Sendable () -> Void = {
        task.cancel()
        completion.finish(.failure(CancellationError()))
      }
      state.interruptInput = interrupt
      if state.cancelled { interrupt() }
    }
    defer { state.withLock { state in state.interruptInput = nil } }
    return try await withTaskCancellationHandler {
      try await withCheckedThrowingContinuation { completion.install($0) }
    } onCancel: {
      task.cancel()
      completion.finish(.failure(CancellationError()))
    }
  }
  func next() async throws -> A? {
    let proceed = try state.withLock { state -> Bool in
      guard !state.busy else { throw ThinkThenError.iteration(.concurrentNext) }
      if state.finished { return false }
      state.busy = true
      return true
    }
    guard proceed else { return nil }
    defer { state.withLock { state in state.busy = false } }
    return try await withTaskCancellationHandler {
      do {
        if state.withLock({ $0.cancelled }) || Task.isCancelled { throw ThinkThenError.cancelled }
        let native: NativeCall
        if let call = state.withLock({ $0.call }) {
          native = call
        } else {
          let bytes: Data
          do { bytes = try request.get() } catch {
            throw ThinkThenError.encoding(message: String(describing: error))
          }
          let records = source.finiteRecords
          if retainItems, let records {
            state.withLock {
              $0.items = Dictionary(
                uniqueKeysWithValues: records.enumerated().map {
                  (UInt64($0.offset + 1), $0.element)
                })
            }
          }
          let opened: NativeCall = try await withCheckedThrowingContinuation { continuation in
            let client = client
            let name = name
            DispatchQueue.global().async {
              do {
                continuation.resume(
                  returning: try NativeCall(
                    client: client, request: bytes, name: name, records: records))
              } catch { continuation.resume(throwing: error) }
            }
          }
          state.withLock {
            $0.call = opened
            if $0.cancelled { opened.cancel() }
          }
          native = opened
        }
        while true {
          try Task.checkCancellation()
          let event = try await native.perform { try $1.next($0) }
          if Task.isCancelled || state.withLock({ $0.cancelled }) { throw ThinkThenError.cancelled }
          switch event {
          case .input:
            if let record = try await input() {
              if Task.isCancelled || state.withLock({ $0.cancelled }) {
                throw ThinkThenError.cancelled
              }
              if retainItems {
                state.withLock {
                  $0.accepted += 1
                  $0.items[$0.accepted] = record
                }
              }
              try await native.perform { try $1.send($0, record) }
            } else {
              try await native.perform { $1.end($0) }
            }
          case .row(let wire):
            let item = state.withLock { $0.items.removeValue(forKey: wire.position) }
            return try decode(wire, item)
          case .end:
            state.withLock { state in
              state.finished = true
              state.items.removeAll()
            }
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
  deinit { state.withLock { $0.call }?.cancel() }
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
    state.withLock { state in
      guard !state.claimed else { return nil }
      state.claimed = true
      let driver = make()
      state.driver = driver
      if state.cancelled { driver.cancel() }
      return driver
    }
  }
  func cancel() {
    state.withLock {
      $0.cancelled = true
      $0.driver?.cancel()
    }
  }
}
public final class Rows<Element: Sendable>: AsyncSequence, Sendable {
  public typealias Failure = ThinkThenError
  private let traversal: Traversal<Element>
  init(make: @escaping @Sendable () -> Driver<Element>) { traversal = Traversal(make) }
  public final class AsyncIterator: AsyncIteratorProtocol, Sendable {
    public typealias Failure = ThinkThenError
    private let driver: Driver<Element>?
    fileprivate init(_ driver: Driver<Element>?) { self.driver = driver }
    public func next() async throws(ThinkThenError) -> Element? {
      guard let driver else { throw .iteration(.alreadyIterated) }
      do { return try await driver.next() } catch { throw typedError(error) }
    }
    public func next(isolation actor: isolated (any Actor)?) async throws(ThinkThenError)
      -> Element?
    { try await next() }
    deinit { driver?.cancel() }
  }
  public func makeAsyncIterator() -> AsyncIterator { AsyncIterator(traversal.claim()) }
  public func cancel() { traversal.cancel() }
  public func collect() async throws(ThinkThenError) -> [Element] {
    let iterator = makeAsyncIterator()
    var output: [Element] = []
    while let element = try await iterator.next() { output.append(element) }
    return output
  }
}
