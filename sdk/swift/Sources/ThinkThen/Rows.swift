import Foundation
import Synchronization

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

final class Driver<A: Sendable>: Sendable {
  private struct State: Sendable {
    var busy = false, cancelled = false, finished = false
    var call: NativeCall?
    var pump: InputPump?
    var inputPending = false
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
      state.pump?.cancel()
    }
  }
  func close() async {
    cancel()
    let native = state.withLock { state in state.call }
    await native?.close()
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
          let event = try await native.perform { [self] handle, events, library in
            while true {
              let version = library.eventVersion(events)
              let event = try library.next(handle)
              guard case .input = event else { return event }
              let pump = state.withLock { state in
                if let pump = state.pump { return pump }
                let pump = InputPump(source: source) { [weak native] in native?.wake() }
                state.pump = pump
                if state.cancelled { pump.cancel() }
                return pump
              }
              let request = state.withLock { state -> Bool in
                if state.inputPending { return false }
                state.inputPending = true
                return true
              }
              if request { pump.request() }
              if let result = pump.take() {
                state.withLock { $0.inputPending = false }
                if state.withLock({ $0.cancelled }) { throw ThinkThenError.cancelled }
                if let record = try result.get() {
                  if retainItems {
                    state.withLock {
                      $0.accepted += 1
                      $0.items[$0.accepted] = record
                    }
                  }
                  try library.send(handle, record)
                } else {
                  library.end(handle)
                }
              } else {
                library.eventWait(events, version)
              }
            }
          }
          if Task.isCancelled || state.withLock({ $0.cancelled }) { throw ThinkThenError.cancelled }
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
            state.withLock { $0.pump?.cancel() }
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
    state.withLock {
      $0.pump?.cancel()
      $0.call?.cancel()
    }
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
