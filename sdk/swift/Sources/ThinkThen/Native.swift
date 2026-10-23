import CThinkThen
import Foundation

#if os(Linux)
  import Glibc
#elseif os(Windows)
  import WinSDK
#else
  import Darwin
#endif

// Function signatures use the shipped C header's types and owned output slots.
private typealias OpenClient =
  @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<OpaquePointer?>?,
    UnsafeMutablePointer<thinkthen_buffer>?
  ) -> Int32
private typealias OpenCall =
  @convention(c) (
    OpaquePointer?, UnsafePointer<UInt8>?, Int, UInt32,
    UnsafeMutablePointer<OpaquePointer?>?, UnsafeMutablePointer<thinkthen_buffer>?
  ) -> Int32
private typealias Send =
  @convention(c) (
    OpaquePointer?, UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<thinkthen_buffer>?
  ) -> Int32
private typealias Next =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<thinkthen_row>?, UnsafeMutablePointer<thinkthen_buffer>?
  ) -> Int32
private typealias Columns =
  @convention(c) (
    UnsafePointer<thinkthen_row>?, UnsafeMutablePointer<thinkthen_buffer>?,
    UnsafeMutablePointer<thinkthen_buffer>?
  ) -> Int32
typealias PointerAction = @convention(c) (OpaquePointer?) -> Void
private typealias FreeBuffer = @convention(c) (UnsafeMutablePointer<thinkthen_buffer>?) -> Void
private typealias FreeRow = @convention(c) (UnsafeMutablePointer<thinkthen_row>?) -> Void

// Immutable process-owned function table; dlclose is intentionally never called.
final class Native: @unchecked Sendable {
  static let loaded = Result { try Native() }
  private let clientOpen: OpenClient, callOpen: OpenCall, sendText: Send, sendJSON: Send
  private let nextRow: Next, columns: Columns, bufferFree: FreeBuffer, rowFree: FreeRow
  let clientFree: PointerAction, cancel: PointerAction, free: PointerAction, end: PointerAction
  private init() throws {
    #if os(Linux) && arch(x86_64)
      let resource = "linux-x64/libthinkthen_ffi.so"
    #elseif os(Linux) && arch(arm64)
      let resource = "linux-arm64/libthinkthen_ffi.so"
    #elseif os(Windows) && arch(x86_64)
      let resource = "win-x64/thinkthen_ffi.dll"
    #elseif os(macOS) && arch(arm64)
      let resource = "osx-arm64/libthinkthen_ffi.dylib"
    #else
      throw NativeLoadError(message: "No packaged engine for this host")
    #endif
    let path = Bundle.module.bundleURL.appendingPathComponent("Native/" + resource).path
    #if os(Windows)
      guard let handle = path.withCString(encodedAs: UTF16.self, { LoadLibraryW($0) }) else {
        throw NativeLoadError(message: "Cannot load packaged engine: \(GetLastError())")
      }
    #else
      guard let handle = dlopen(path, RTLD_NOW | RTLD_LOCAL) else {
        throw NativeLoadError(
          message: dlerror().map { String(cString: $0) } ?? "Cannot load packaged engine")
      }
    #endif
    func symbol<T>(_ name: String, _: T.Type) throws -> T {
      #if os(Windows)
        let address = GetProcAddress(handle, name)
      #else
        let address = dlsym(handle, name)
      #endif
      guard let address else {
        throw NativeLoadError(message: "Missing native symbol: " + name)
      }
      return unsafeBitCast(address, to: T.self)
    }
    clientOpen = try symbol("thinkthen_client_open", OpenClient.self)
    callOpen = try symbol("thinkthen_call_open", OpenCall.self)
    sendText = try symbol("thinkthen_send_text", Send.self)
    sendJSON = try symbol("thinkthen_send_json", Send.self)
    nextRow = try symbol("thinkthen_next", Next.self)
    columns = try symbol("thinkthen_row_columns", Columns.self)
    clientFree = try symbol("thinkthen_client_free", PointerAction.self)
    cancel = try symbol("thinkthen_cancel", PointerAction.self)
    free = try symbol("thinkthen_free", PointerAction.self)
    end = try symbol("thinkthen_end_input", PointerAction.self)
    bufferFree = try symbol("thinkthen_buffer_free", FreeBuffer.self)
    rowFree = try symbol("thinkthen_row_free", FreeRow.self)
  }
  private func copy(_ buffer: thinkthen_buffer) -> Data {
    guard let data = buffer.data else { return Data() }
    return Data(bytes: data, count: buffer.len)
  }
  private func check(_ status: Int32, _ error: inout thinkthen_buffer) throws {
    defer { bufferFree(&error) }
    if status == TTS_ERROR {
      throw try JSONDecoder().decode(ThinkThenError.self, from: copy(error))
    }
  }
  func openClient(_ bytes: Data) throws -> OpaquePointer {
    var pointer: OpaquePointer?
    var error = thinkthen_buffer()
    let status = bytes.withUnsafeBytes {
      clientOpen($0.bindMemory(to: UInt8.self).baseAddress, bytes.count, &pointer, &error)
    }
    try check(status, &error)
    return pointer!
  }
  func openCall(_ client: OpaquePointer, _ bytes: Data) throws -> OpaquePointer {
    var pointer: OpaquePointer?
    var error = thinkthen_buffer()
    let status = bytes.withUnsafeBytes {
      callOpen(
        client, $0.bindMemory(to: UInt8.self).baseAddress, bytes.count,
        TTS_CODE_POINTS, &pointer, &error)
    }
    try check(status, &error)
    return pointer!
  }
  func send(_ call: OpaquePointer, _ record: Record) throws {
    let bytes: Data
    let operation: Send
    switch record {
    case .text(let text):
      bytes = Data(text.utf8)
      operation = sendText
    case .json(let value):
      bytes = value.utf8
      operation = sendJSON
    }
    var error = thinkthen_buffer()
    let status = bytes.withUnsafeBytes {
      operation(call, $0.bindMemory(to: UInt8.self).baseAddress, bytes.count, &error)
    }
    try check(status, &error)
  }
  enum Event: Sendable {
    case input, end
    case row(WireRow)
  }
  func next(_ call: OpaquePointer) throws -> Event {
    var row = thinkthen_row()
    var error = thinkthen_buffer()
    let status = nextRow(call, &row, &error)
    try check(status, &error)
    if status == TTS_NEEDS_INPUT { return .input }
    if status == TTS_END { return .end }
    defer { rowFree(&row) }
    var projected = thinkthen_buffer()
    let projectedStatus = columns(&row, &projected, &error)
    defer { bufferFree(&projected) }
    try check(projectedStatus, &error)
    return .row(
      WireRow(
        columns: copy(projected), detail: row.detail.data == nil ? nil : copy(row.detail),
        position: row.position, kept: row.kept != 0))
  }
}
struct WireRow: Sendable {
  let columns: Data, detail: Data?, position: Int, kept: Bool
  private struct Envelope: Decodable {
    let status: RowStatus, p: Double?, id: String?, answer: String?, error: String?
  }
  func decode<A: Sendable>(_ answer: @Sendable (Data) throws -> A) throws -> Row<A> {
    let envelope = try JSONDecoder().decode(Envelope.self, from: columns)
    let value = try envelope.answer.flatMap { text -> A? in
      let data = Data(text.utf8)
      return data == Data("null".utf8) ? nil : try answer(data)
    }
    let failure = try envelope.error.map {
      try JSONDecoder().decode(ThinkThenError.self, from: Data($0.utf8))
    }
    return Row(
      id: envelope.id.map { JSON(utf8: Data($0.utf8)) }, status: envelope.status,
      answer: value, p: envelope.p, error: failure, position: position, kept: kept,
      detail: detail.map { JSON(utf8: $0) })
  }
}
