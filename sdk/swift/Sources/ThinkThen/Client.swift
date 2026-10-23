import Foundation

/// An immutable admitted client. Independent calls may run concurrently.
public final class Client: @unchecked Sendable {
  private let lock = NSLock(), native: Native, pointer: OpaquePointer
  public init(settings: ClientSettings = .init()) throws {
    native = try Native.loaded.get()
    pointer = try native.openClient(JSONEncoder().encode(settings))
  }
  func open(_ request: Data) throws -> OpaquePointer {
    try lock.withLock { try native.openCall(pointer, request) }
  }
  deinit { native.clientFree(pointer) }
  func rows<A: Sendable>(
    function: String, question: Question, source: any Source,
    options: Data,
    decode: @escaping @Sendable (Data) throws -> A
  ) -> Rows<A> {
    let request =
      Data(("{\"function\":\"" + function + "\",\"question\":").utf8)
      + question.json + Data(",\"options\":".utf8) + options + Data("}".utf8)
    return Rows { [self] in
      Driver(client: self, request: request, source: source, decode: decode)
    }
  }
  func single<A: Sendable>(_ rows: Rows<A>) async throws -> Row<A> {
    try await rows.withRows { sequence in
      var iterator = sequence.makeAsyncIterator()
      var result: Row<A>?
      while let row = try await iterator.next() { result = row }
      guard let result else {
        throw NativeLoadError(message: "The engine returned no single result")
      }
      return result
    }
  }
}
func decoded<T: Decodable & Sendable>(_ type: T.Type) -> @Sendable (Data) throws -> T {
  { try JSONDecoder().decode(type, from: $0) }
}
