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
    } catch { throw typedError(error) }
  }
  /// Open a client, reading THINKTHEN_BACKEND when no backend is passed.
  public convenience init(
    backend: Backend? = nil, url: URL? = nil, model: String? = nil,
    apiKey: APIKey? = nil, batch: Int? = nil, maxRequests: UInt64? = nil,
    responseStore: ResponseStore? = nil, retry: Retry? = nil
  ) throws {
    try self.init(
      Settings(
        backend: backend, url: url, model: model, apiKey: apiKey, batch: batch,
        maxRequests: maxRequests, responseStore: responseStore, retry: retry)
    )
  }
  func open(_ request: Data, name: String?, records: [Record]?) throws -> NativeHandle {
    try handle.withLock {
      NativeHandle(pointer: try native.openCall($0.pointer, request, name: name, records: records))
    }
  }
  /// Return a copied cumulative snapshot. Active calls may still produce more usage.
  public func usage() throws -> Usage {
    do { return try handle.withLock { try native.usage($0.pointer) } } catch {
      throw typedError(error)
    }
  }
  deinit { handle.withLock { native.clientFree($0.pointer) } }
  func rows<Q: FunctionShape & Encodable, E: Sendable>(
    _ question: Q, source: any RecordSource, name: String?, retainItems: Bool = false,
    decode: @escaping @Sendable (WireRow, Record?) throws -> E
  ) -> Rows<E> {
    let encoded = Result { try JSONEncoder().encode(question) }
    return Rows { [self] in
      Driver(
        client: self, request: encoded, name: name, source: source, retainItems: retainItems,
        decode: decode)
    }
  }
  func single<Q: FunctionShape>(_ rows: Rows<Row<Q>>) async throws -> Row<Q> {
    let iterator = rows.makeAsyncIterator()
    guard let first = try await iterator.next() else {
      throw ThinkThenError.contract(message: "Engine returned no single row")
    }
    guard try await iterator.next() == nil else {
      throw ThinkThenError.contract(message: "Engine returned multiple single rows")
    }
    if let failure = first.failure, failure.code == .recordEmpty {
      throw ThinkThenError.engine(failure)
    }
    return first
  }
}
