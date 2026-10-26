import CThinkThen
import Foundation

#if os(Linux)
  import Glibc
#elseif os(Windows)
  import WinSDK
#else
  import Darwin
#endif

private typealias Admit =
  @convention(c) (
    UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<OpaquePointer?>?,
    UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias ExportJSON =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<thinkthen_buffer>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias FreeBuffer = @convention(c) (UnsafeMutablePointer<thinkthen_buffer>?) -> Void
private typealias OpenClient =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<OpaquePointer?>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias OpenCall =
  @convention(c) (
    OpaquePointer?, OpaquePointer?, UnsafePointer<thinkthen_text>?, UInt32,
    UnsafeMutablePointer<OpaquePointer?>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias OpenRecords =
  @convention(c) (
    OpaquePointer?, OpaquePointer?, UnsafePointer<thinkthen_text>?,
    UnsafePointer<thinkthen_record_input>?, Int, Int, UInt32, UnsafeMutablePointer<OpaquePointer?>?,
    UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias Send =
  @convention(c) (
    OpaquePointer?, UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias Next =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<thinkthen_row>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias Version = @convention(c) () -> UInt32
private typealias CodeName = @convention(c) (Int32) -> UnsafePointer<CChar>?
typealias PointerAction = @convention(c) (OpaquePointer?) -> Void
private typealias ConversionErrorCall =
  @convention(c) (OpaquePointer?, UnsafeMutablePointer<thinkthen_error>?) -> Int32
private typealias FreeError = @convention(c) (UnsafeMutablePointer<thinkthen_error>?) -> Void
private typealias FreeRow = @convention(c) (UnsafeMutablePointer<thinkthen_row>?) -> Void

// Immutable process-owned function table. Workers may outlive a cancelled call.
final class Native: Sendable {
  static let loaded = Result { try Native() }
  private let clientOpen: OpenClient, callOpen: OpenCall, sendText: Send, sendJSON: Send
  private let nextRow: Next, errorFree: FreeError, rowFree: FreeRow, codeName: CodeName
  private let recordsOpen: OpenRecords
  private let admitSettings: Admit, admitQuestion: Admit
  private let settingsFree: PointerAction, questionFree: PointerAction
  private let admitToolbox: Admit, admitDefinitions: Admit
  private let exportToolbox: ExportJSON, exportDefinitions: ExportJSON
  private let toolboxFree: PointerAction, bufferFree: FreeBuffer
  private let sendMissing: ConversionErrorCall
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
      throw ThinkThenError.engineUnavailable(message: "No packaged engine for this host")
    #endif
    let path = Bundle.module.bundleURL.appendingPathComponent("Native/" + resource).path
    #if os(Windows)
      guard let handle = path.withCString(encodedAs: UTF16.self, { LoadLibraryW($0) }) else {
        throw ThinkThenError.engineUnavailable(
          message: "Cannot load packaged engine \(path): \(GetLastError())")
      }
    #else
      guard let handle = dlopen(path, RTLD_NOW | RTLD_LOCAL) else {
        throw ThinkThenError.engineUnavailable(
          message: "Cannot load packaged engine \(path): "
            + (dlerror().map { String(cString: $0) } ?? "unknown loader error"))
      }
    #endif
    func symbol<T>(_ name: String, _: T.Type) throws -> T {
      #if os(Windows)
        let address = GetProcAddress(handle, name)
      #else
        let address = dlsym(handle, name)
      #endif
      guard let address else {
        throw ThinkThenError.engineUnavailable(
          message: "Cannot load packaged engine \(path): missing native symbol " + name)
      }
      return unsafeBitCast(address, to: T.self)
    }
    // No layout-dependent symbol or operation is used before this handshake.
    let version = try symbol("thinkthen_abi_version", Version.self)()
    let required = UInt32(THINKTHEN_ABI_VERSION)
    guard version >> 16 == required >> 16, version & 0xffff >= required & 0xffff else {
      throw ThinkThenError.engineVersion(
        required: ABIVersion(major: required >> 16, minor: required & 0xffff),
        found: ABIVersion(major: version >> 16, minor: version & 0xffff))
    }
    admitToolbox = try symbol("thinkthen_toolbox_from_json", Admit.self)
    admitDefinitions = try symbol("thinkthen_toolbox_from_definitions_json", Admit.self)
    exportToolbox = try symbol("thinkthen_toolbox_to_json", ExportJSON.self)
    exportDefinitions = try symbol("thinkthen_toolbox_definitions_json", ExportJSON.self)
    toolboxFree = try symbol("thinkthen_toolbox_free", PointerAction.self)
    bufferFree = try symbol("thinkthen_buffer_free", FreeBuffer.self)
    admitSettings = try symbol("thinkthen_settings_from_json", Admit.self)
    admitQuestion = try symbol("thinkthen_question_from_json", Admit.self)
    settingsFree = try symbol("thinkthen_settings_free", PointerAction.self)
    questionFree = try symbol("thinkthen_question_free", PointerAction.self)
    sendMissing = try symbol("thinkthen_call_send_missing", ConversionErrorCall.self)
    clientOpen = try symbol("thinkthen_client_open", OpenClient.self)
    callOpen = try symbol("thinkthen_call_open_question", OpenCall.self)
    recordsOpen = try symbol("thinkthen_call_open_records_question", OpenRecords.self)
    sendText = try symbol("thinkthen_call_send_text", Send.self)
    sendJSON = try symbol("thinkthen_call_send_json", Send.self)
    nextRow = try symbol("thinkthen_call_next", Next.self)
    clientFree = try symbol("thinkthen_client_free", PointerAction.self)
    cancel = try symbol("thinkthen_call_cancel", PointerAction.self)
    free = try symbol("thinkthen_call_free", PointerAction.self)
    end = try symbol("thinkthen_call_end_input", PointerAction.self)
    errorFree = try symbol("thinkthen_error_free", FreeError.self)
    rowFree = try symbol("thinkthen_row_free", FreeRow.self)
    codeName = try symbol("thinkthen_code_name", CodeName.self)
    for code in ErrorCode.allCases {
      guard let name = codeName(code.nativeValue), String(cString: name) == code.rawValue else {
        throw ThinkThenError.contract(message: "Native error registry does not match this package")
      }
    }
  }
  private func copy(_ buffer: thinkthen_buffer) -> Data {
    guard let data = buffer.data else { return Data() }
    return Data(bytes: data, count: buffer.len)
  }
  private func emptyError() -> thinkthen_error {
    var error = thinkthen_error()
    error.struct_size = UInt32(MemoryLayout<thinkthen_error>.size)
    return error
  }
  private func check(_ status: Int32, _ error: inout thinkthen_error) throws {
    defer { errorFree(&error) }
    if status == THINKTHEN_ERROR {
      guard let code = codeName(error.code) else {
        throw ThinkThenError.contract(message: "Unknown native error code")
      }
      let name = String(cString: code)
      if name == "local.cancelled" { throw ThinkThenError.cancelled }
      guard let typed = ErrorCode(rawValue: name) else {
        throw ThinkThenError.contract(message: "Unknown native error code")
      }
      throw ThinkThenError.engine(
        RowFailure(
          code: typed, message: String(decoding: copy(error.message), as: UTF8.self),
          httpStatus: error.http_status == 0 ? nil : error.http_status))
    }
  }
  private func admit(_ bytes: Data, using operation: Admit) throws -> OpaquePointer {
    var pointer: OpaquePointer?
    var error = emptyError()
    let status = bytes.withUnsafeBytes {
      operation($0.bindMemory(to: UInt8.self).baseAddress, bytes.count, &pointer, &error)
    }
    try check(status, &error)
    guard let pointer else {
      throw ThinkThenError.contract(message: "Native admission returned no owner")
    }
    return pointer
  }
  func openClient(_ settings: Settings) throws -> OpaquePointer {
    var object =
      try JSONSerialization.jsonObject(with: JSONEncoder().encode(settings)) as! [String: Any]
    if let key = settings.apiKey { object["api_key"] = key.value }
    let admitted = try admit(JSONSerialization.data(withJSONObject: object), using: admitSettings)
    defer { settingsFree(admitted) }
    var pointer: OpaquePointer?
    var error = emptyError()
    let status = clientOpen(admitted, &pointer, &error)
    try check(status, &error)
    guard let pointer else {
      throw ThinkThenError.contract(message: "Native client returned no owner")
    }
    return pointer
  }
  func toolboxJSON(_ bytes: Data, loading: Bool) throws -> Data {
    let toolbox = try admit(bytes, using: loading ? admitToolbox : admitDefinitions)
    defer { toolboxFree(toolbox) }
    var output = thinkthen_buffer()
    defer { bufferFree(&output) }
    var error = emptyError()
    let operation = loading ? exportDefinitions : exportToolbox
    let status = operation(toolbox, &output, &error)
    try check(status, &error)
    return copy(output)
  }
  func openCall(_ client: OpaquePointer, _ bytes: Data, name: String?, records: [Record]?) throws
    -> OpaquePointer
  {
    let question = try admit(bytes, using: admitQuestion)
    defer { questionFree(question) }
    var pointer: OpaquePointer?
    var error = emptyError()
    let nameBytes = name.map { Data($0.utf8) }
    let status = try (nameBytes ?? Data()).withUnsafeBytes { buffer -> Int32 in
      var view = thinkthen_text(
        data: buffer.bindMemory(to: UInt8.self).baseAddress, len: buffer.count)
      return try withUnsafePointer(to: &view) { nameView in
        let named = name == nil ? nil : nameView
        guard let records else {
          return callOpen(client, question, named, THINKTHEN_OFFSET_CODE_POINTS, &pointer, &error)
        }
        var payload = Data()
        var inputs: [thinkthen_record_input] = []
        var offsets: [Int] = []
        for record in records {
          var input = thinkthen_record_input()
          input.struct_size = UInt32(MemoryLayout<thinkthen_record_input>.size)
          let value: Data
          switch record {
          case .text(let text):
            input.kind = THINKTHEN_RECORD_TEXT
            value = Data(text.utf8)
          case .missing, .json(.null):
            input.kind = THINKTHEN_RECORD_MISSING
            value = Data()
          case .json(let json):
            input.kind = THINKTHEN_RECORD_JSON
            value = try JSONEncoder().encode(json)
          }
          offsets.append(payload.count)
          input.len = value.count
          payload.append(value)
          inputs.append(input)
        }
        return payload.withUnsafeBytes { payloadBuffer in
          for index in inputs.indices {
            inputs[index].data = payloadBuffer.bindMemory(to: UInt8.self).baseAddress?.advanced(
              by: offsets[index])
          }
          return inputs.withUnsafeBufferPointer {
            recordsOpen(
              client, question, named, $0.baseAddress, inputs.count,
              MemoryLayout<thinkthen_record_input>.stride, THINKTHEN_OFFSET_CODE_POINTS, &pointer,
              &error)
          }
        }
      }
    }
    try check(status, &error)
    guard let pointer else {
      throw ThinkThenError.contract(message: "Native call returned no owner")
    }
    return pointer
  }
  func send(_ call: OpaquePointer, _ record: Record) throws {
    let bytes: Data
    let operation: Send
    switch record {
    case .text(let text):
      bytes = Data(text.utf8)
      operation = sendText
    case .missing, .json(.null):
      var error = emptyError()
      let status = sendMissing(call, &error)
      try check(status, &error)
      return
    case .json(let value):
      bytes = try JSONEncoder().encode(value)
      operation = sendJSON
    }
    var error = emptyError()
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
    row.struct_size = UInt32(MemoryLayout<thinkthen_row>.size)
    var error = emptyError()
    defer { rowFree(&row) }
    let status = nextRow(call, &row, &error)
    try check(status, &error)
    if status == THINKTHEN_NEEDS_INPUT { return .input }
    if status == THINKTHEN_END { return .end }
    guard status == THINKTHEN_OK else {
      throw ThinkThenError.contract(message: "Unknown native operation status")
    }
    let rowStatus: Status
    switch row.status {
    case THINKTHEN_STATUS_ANSWERED: rowStatus = .answered
    case THINKTHEN_STATUS_UNSURE: rowStatus = .unsure
    case THINKTHEN_STATUS_FAILED: rowStatus = .failed
    default: throw ThinkThenError.contract(message: "Unknown native row status")
    }
    let function: String
    switch row.function {
    case THINKTHEN_FUNCTION_DECIDE: function = "decide"
    case THINKTHEN_FUNCTION_CHOOSE: function = "choose"
    case THINKTHEN_FUNCTION_TAG: function = "tag"
    case THINKTHEN_FUNCTION_SCORE: function = "score"
    case THINKTHEN_FUNCTION_FILTER: function = "filter"
    case THINKTHEN_FUNCTION_RANK: function = "rank"
    case THINKTHEN_FUNCTION_FIND: function = "find"
    case THINKTHEN_FUNCTION_RECOGNIZE: function = "recognize"
    case THINKTHEN_FUNCTION_RELATE: function = "relate"
    case THINKTHEN_FUNCTION_ANNOTATE: function = "annotate"
    default: throw ThinkThenError.contract(message: "Unknown native function")
    }
    return .row(
      WireRow(
        function: function, id: row.id.data == nil ? nil : copy(row.id), status: rowStatus,
        answer: row.answer.data == nil ? nil : copy(row.answer),
        probability: row.probability.isNaN ? nil : row.probability,
        error: row.error.data == nil ? nil : copy(row.error),
        detail: row.detail.data == nil ? nil : copy(row.detail), source: copy(row.source),
        position: row.position,
        kept: row.status == THINKTHEN_STATUS_ANSWERED && row.answer_bool != 0))
  }
}
struct WireRow: Sendable {
  let function: String
  let id: Data?, status: Status, answer: Data?, probability: Double?, error: Data?
  let detail: Data?, source: Data, position: UInt64, kept: Bool
  func decode<F: FunctionShape>(_ type: F.Type) throws -> Row<F> {
    var fields: [String: Any] = [
      "function": function, "position": position, "status": status.rawValue,
    ]
    fields["source"] = try JSONSerialization.jsonObject(with: source)
    for (key, data) in [("id", id), ("answer", answer), ("detail", detail), ("error", error)] {
      if let data {
        fields[key] = try JSONSerialization.jsonObject(with: data, options: [.fragmentsAllowed])
      }
    }
    fields["probability"] = probability
    do {
      return try JSONDecoder().decode(
        Row<F>.self, from: JSONSerialization.data(withJSONObject: fields))
    } catch { throw ThinkThenError.contract(message: String(describing: error)) }
  }
}
