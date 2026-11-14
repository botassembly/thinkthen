import Foundation
import Synchronization

/// The typed Client value exchanged with the engine.
public final class Client: Sendable {
  private let native: Native
  private let handle: Mutex<NativeHandle>
  /// Create this value from its typed fields.
  public init(_ settings: Settings) throws {
    do {
      native = try Native.loaded.get()
      handle = Mutex(NativeHandle(pointer: try native.openClient(settings)))
    } catch { throw typedError(error, setup: true) }
  }
  /// Open a client, reading THINKTHEN_BACKEND when no backend is passed.
  public convenience init(
    backend: Backend? = nil, url: URL? = nil, model: String? = nil,
    apiKey: APIKey? = nil, batch: Int? = nil, maxRequests: UInt64? = nil, retry: RetryPolicy? = nil
  ) throws {
    try self.init(
      Settings(
        backend: backend, url: url, model: model, apiKey: apiKey, batch: batch,
        maxRequests: maxRequests, retry: retry)
    )
  }
  func open(_ request: Data, name: String?) throws -> NativeHandle {
    try handle.withLock {
      NativeHandle(pointer: try native.openCall($0.pointer, request, name: name))
    }
  }
  /// Read the latest cumulative accounting snapshot.
  public var usage: Usage { handle.withLock { native.usage($0.pointer) } }
  deinit { handle.withLock { native.clientFree($0.pointer) } }
  func rows<Q: SavedQuestion, E: Sendable>(
    _ question: Q, source: any RecordSource, name: String?, retainItems: Bool = true,
    decode: @escaping @Sendable (WireRow, (any Sendable)?) throws -> E
  ) -> Rows<E> {
    let encoded = Result {
      if let actual = question.definition.object["function"]?.string, actual != Q.function.rawValue
      {
        throw UsageError(
          code: .usageFunction, message: "Saved question must use \(Q.function.rawValue)")
      }
      return try JSONEncoder().encode(question)
    }
    return Rows { [self] in
      Driver(
        client: self, request: encoded, name: name, source: source, retainItems: retainItems,
        decode: decode)
    }
  }
  func single<F: FunctionShape, Item: Sendable>(_ rows: Rows<Row<F, Item>>) async throws -> Row<
    F, Item
  > {
    let iterator = rows.makeAsyncIterator()
    guard let first = try await iterator.next() else {
      throw CallError.contract(message: "One record produced no row")
    }
    guard try await iterator.next() == nil else {
      throw CallError.contract(message: "One record produced multiple rows")
    }
    if first.isFailed, let failure = first.error {
      throw Failed(failure, rows: [first.erased])
    }
    return first
  }
}
