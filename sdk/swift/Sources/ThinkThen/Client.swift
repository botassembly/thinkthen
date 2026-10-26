import Foundation
import Synchronization

public final class Client: Sendable {
  private let native: Native
  private let handle: Mutex<NativeHandle>
  public init(_ settings: Settings = .init()) throws(ThinkThenError) {
    do {
      native = try Native.loaded.get()
      handle = Mutex(NativeHandle(pointer: try native.openClient(settings)))
    } catch { throw typedError(error) }
  }
  func open(_ request: Data, name: String?, records: [Record]?) throws -> NativeHandle {
    try handle.withLock {
      NativeHandle(pointer: try native.openCall($0.pointer, request, name: name, records: records))
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
  func single<E: Sendable>(_ rows: Rows<E>) async throws(ThinkThenError) -> E {
    let iterator = rows.makeAsyncIterator()
    guard let first = try await iterator.next() else {
      throw .contract(message: "Engine returned no single row")
    }
    guard try await iterator.next() == nil else {
      throw .contract(message: "Engine returned multiple single rows")
    }
    return first
  }
}
