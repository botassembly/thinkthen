import Foundation

// A single completion races cancellation against an asynchronous source. A late
// source result has no native pointer and can only resolve this empty slot.
private final class Completion<T: Sendable>: @unchecked Sendable {
  private let lock = NSLock()
  private var continuation: CheckedContinuation<T, any Error>?
  private var result: Result<T, any Error>?
  func install(_ value: CheckedContinuation<T, any Error>) {
    let ready = lock.withLock { () -> Result<T, any Error>? in
      if let result { return result }
      continuation = value
      return nil
    }
    if let ready { value.resume(with: ready) }
  }
  func finish(_ value: Result<T, any Error>) {
    let waiter = lock.withLock { () -> CheckedContinuation<T, any Error>? in
      guard result == nil else { return nil }
      result = value
      defer { continuation = nil }
      return continuation
    }
    waiter?.resume(with: value)
  }
}
protocol Source: Sendable { func next() async throws -> Record? }
private actor SequenceSource<S: AsyncSequence & Sendable>: Source where S.Element == Record {
  var iterator: S.AsyncIterator?
  init(_ source: S) { iterator = source.makeAsyncIterator() }
  func next() async throws -> Record? {
    var current = iterator!
    iterator = nil
    defer { iterator = current }
    return try await current.next(isolation: self)
  }
}
private actor ArraySource: Source {
  let records: [Record]
  var index = 0
  init(_ records: [Record]) { self.records = records }
  func next() -> Record? {
    guard index < records.count else { return nil }
    defer { index += 1 }
    return records[index]
  }
}
func source<S: AsyncSequence & Sendable>(_ input: S) -> any Source where S.Element == Record {
  SequenceSource(input)
}
func source(_ input: [Record]) -> any Source { ArraySource(input) }

// The lock protects pointer lifetime against cancel. All other operations run
// on queue. No caller waits synchronously for that queue or the backend.
final class NativeCall: @unchecked Sendable {
  private let lock = NSLock(), queue = DispatchQueue(label: "ThinkThen.call")
  private var pointer: OpaquePointer?
  private let native: Native
  init(client: Client, request: Data) throws {
    native = try Native.loaded.get()
    pointer = try client.open(request)
  }
  func cancel() { lock.withLock { if let pointer { native.cancel(pointer) } } }
  func perform<T: Sendable>(_ operation: @escaping @Sendable (OpaquePointer, Native) throws -> T)
    async throws -> T
  {
    try await withCheckedThrowingContinuation { continuation in
      queue.async { [self] in
        do {
          let pointer = lock.withLock { self.pointer }
          guard let pointer else { throw CancellationError() }
          continuation.resume(returning: try operation(pointer, native))
        } catch { continuation.resume(throwing: error) }
      }
    }
  }
  func close() async {
    await withCheckedContinuation { continuation in
      queue.async { [self] in
        lock.withLock {
          if let pointer {
            native.free(pointer)
            self.pointer = nil
          }
        }
        continuation.resume()
      }
    }
  }
  deinit { if let pointer { native.free(pointer) } }
}
final class Driver<A: Sendable>: @unchecked Sendable {
  private let lock = NSLock()
  private var busy = false, cancelled = false, finished = false
  private var call: NativeCall?
  private var interruptInput: (@Sendable () -> Void)?
  private let client: Client, request: Data, source: any Source
  private let decode: @Sendable (Data) throws -> A
  init(
    client: Client, request: Data, source: any Source,
    decode: @escaping @Sendable (Data) throws -> A
  ) {
    self.client = client
    self.request = request
    self.source = source
    self.decode = decode
  }
  func cancel() {
    lock.withLock {
      cancelled = true
      call?.cancel()
      interruptInput?()
    }
  }
  func close() async {
    cancel()
    let native = lock.withLock { call }
    await native?.close()
  }
  private func input() async throws -> Record? {
    let completion = Completion<Record?>()
    let source = self.source
    let task = Task { completion.finish(await Result { try await source.next() }) }
    lock.withLock {
      let interrupt: @Sendable () -> Void = {
        task.cancel()
        completion.finish(.failure(CancellationError()))
      }
      interruptInput = interrupt
      if cancelled { interrupt() }
    }
    defer { lock.withLock { interruptInput = nil } }
    return try await withTaskCancellationHandler {
      try await withCheckedThrowingContinuation { completion.install($0) }
    } onCancel: {
      task.cancel()
      completion.finish(.failure(CancellationError()))
    }
  }
  func next() async throws -> Row<A>? {
    let proceed = try lock.withLock { () -> Bool in
      guard !busy else { throw RowsIterationError.concurrentNext }
      if finished { return false }
      busy = true
      return true
    }
    guard proceed else { return nil }
    defer { lock.withLock { busy = false } }
    return try await withTaskCancellationHandler {
      do {
        let native = try lock.withLock { () -> NativeCall in
          if cancelled || Task.isCancelled { throw CancellationError() }
          if let call { return call }
          let opened = try NativeCall(client: client, request: request)
          call = opened
          return opened
        }
        while true {
          try Task.checkCancellation()
          let event = try await native.perform { try $1.next($0) }
          switch event {
          case .input:
            if let record = try await input() {
              try await native.perform { try $1.send($0, record) }
            } else {
              try await native.perform { $1.end($0) }
            }
          case .row(let wire): return try wire.decode(decode)
          case .end:
            lock.withLock { finished = true }
            await native.close()
            return nil
          }
        }
      } catch {
        lock.withLock { finished = true }
        await close()
        if Task.isCancelled { throw CancellationError() }
        throw error
      }
    } onCancel: {
      self.cancel()
    }
  }
  deinit { call?.cancel() }
}

// A second iterator does not acquire a call or any ownership of the first.
private final class Traversal<A: Sendable>: @unchecked Sendable {
  let lock = NSLock()
  var claimed = false, cancelled = false
  weak var driver: Driver<A>?
  let make: @Sendable () -> Driver<A>
  init(_ make: @escaping @Sendable () -> Driver<A>) { self.make = make }
  func claim() -> Driver<A>? {
    lock.withLock {
      guard !claimed else { return nil }
      claimed = true
      let value = make()
      driver = value
      if cancelled { value.cancel() }
      return value
    }
  }
  func cancel() {
    lock.withLock {
      cancelled = true
      driver?.cancel()
    }
  }
}
public struct Rows<Answer: Sendable>: AsyncSequence, Sendable {
  public typealias Element = Row<Answer>
  private let traversal: Traversal<Answer>
  init(make: @escaping @Sendable () -> Driver<Answer>) { traversal = Traversal(make) }
  public struct AsyncIterator: AsyncIteratorProtocol {
    let driver: Driver<Answer>?
    public mutating func next() async throws -> Row<Answer>? {
      guard let driver else { throw RowsIterationError.alreadyIterated }
      return try await driver.next()
    }
  }
  public func makeAsyncIterator() -> AsyncIterator { AsyncIterator(driver: traversal.claim()) }
  public func cancel() { traversal.cancel() }
  public func withRows<T>(
    isolation: isolated (any Actor)? = #isolation,
    _ body: (Rows<Answer>) async throws -> T
  ) async throws -> T {
    do {
      let result = try await body(self)
      let driver = traversal.lock.withLock { traversal.driver }
      await driver?.close()
      return result
    } catch {
      let driver = traversal.lock.withLock { traversal.driver }
      await driver?.close()
      throw error
    }
  }
}

extension Result where Failure == any Error {
  fileprivate init(catching body: () async throws -> Success) async {
    do { self = .success(try await body()) } catch { self = .failure(error) }
  }
}
