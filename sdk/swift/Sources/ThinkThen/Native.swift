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
private typealias Send =
  @convention(c) (
    OpaquePointer?, UnsafePointer<UInt8>?, Int, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias Next =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<thinkthen_row>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
typealias EventVersion = @convention(c) (OpaquePointer?) -> UInt64
typealias EventWait = @convention(c) (OpaquePointer?, UInt64) -> Void
typealias RetainEvents = @convention(c) (OpaquePointer?) -> OpaquePointer?
private typealias Version = @convention(c) () -> UInt32
private typealias CodeName = @convention(c) (Int32) -> UnsafePointer<CChar>?
typealias PointerAction = @convention(c) (OpaquePointer?) -> Void
private typealias ConversionErrorCall =
  @convention(c) (OpaquePointer?, UnsafeMutablePointer<thinkthen_error>?) -> Int32
private typealias FreeError = @convention(c) (UnsafeMutablePointer<thinkthen_error>?) -> Void
private typealias FreeRow = @convention(c) (UnsafeMutablePointer<thinkthen_row>?) -> Void
private typealias UsageValues =
  @convention(c) (
    OpaquePointer?, UnsafeMutablePointer<thinkthen_usage>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias SaveRow =
  @convention(c) (
    UnsafePointer<thinkthen_row>?, UnsafeMutablePointer<thinkthen_buffer>?,
    UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32
private typealias SelectedText =
  @convention(c) (
    UnsafePointer<thinkthen_row>?, UnsafePointer<CChar>?, Int,
    UnsafeMutablePointer<thinkthen_text>?, UnsafeMutablePointer<thinkthen_error>?
  ) -> Int32

// Immutable process-owned function table. Workers may outlive a cancelled call.
final class Native: Sendable {
  static let loaded = Result { try Native() }
  private let clientOpen: OpenClient, callOpen: OpenCall, sendText: Send, sendJSON: Send
  private let nextRow: Next, errorFree: FreeError, rowFree: FreeRow, codeName: CodeName
  private let selectedText: SelectedText
  private let saveRow: SaveRow
  private let admitSettings: Admit, admitQuestion: Admit
  private let settingsFree: PointerAction, questionFree: PointerAction
  private let admitToolbox: Admit, admitDefinitions: Admit
  private let exportToolbox: ExportJSON, exportDefinitions: ExportJSON, exportNames: ExportJSON
  private let clientUsage: UsageValues
  private let toolboxFree: PointerAction, bufferFree: FreeBuffer
  private let sendMissing: ConversionErrorCall, sendInvalid: ConversionErrorCall
  let events: RetainEvents, eventVersion: EventVersion, eventWait: EventWait
  let eventWake: PointerAction, eventFree: PointerAction
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
      throw engineLoadFailure("No packaged engine for this host")
    #endif
    let chosen = ProcessInfo.processInfo.environment[THINKTHEN_LIBRARY_VARIABLE] ?? ""
    guard chosen.isEmpty || isFullPath(chosen) else {
      throw engineLoadFailure(THINKTHEN_LIBRARY_PATH_MESSAGE)
    }
    let folders = [Bundle.main.bundleURL, Bundle.main.executableURL?.deletingLastPathComponent()]
    let bundles = folders.compactMap { $0 }.flatMap { base in
      ["ThinkThen_ThinkThen.bundle", "ThinkThen_ThinkThen.resources"].map {
        base.appendingPathComponent($0).appendingPathComponent("Native/" + resource).path
      }
    }
    let packaged = bundles.first(where: { FileManager.default.fileExists(atPath: $0) })
    guard let path = chosen.isEmpty ? packaged : chosen else {
      throw engineLoadFailure(
        "cannot load native engine; searched \(bundles.joined(separator: ", ")); set \(THINKTHEN_LIBRARY_VARIABLE) to the native library path"
      )
    }
    #if os(Windows)
      // The search flags need a full path with backslashes, and they keep the
      // engine's dependencies out of the working folder.
      let file = URL(fileURLWithPath: path).withUnsafeFileSystemRepresentation {
        $0.map { String(cString: $0) } ?? path
      }
      let flags = DWORD(LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR | LOAD_LIBRARY_SEARCH_SYSTEM32)
      guard
        let handle = file.withCString(encodedAs: UTF16.self, { LoadLibraryExW($0, nil, flags) })
      else {
        throw engineLoadFailure("cannot load native engine \(path): system error \(GetLastError())")
      }
    #else
      guard let handle = dlopen(path, RTLD_NOW | RTLD_LOCAL) else {
        throw engineLoadFailure(
          "cannot load native engine \(path): "
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
        throw rowABIFailure(THINKTHEN_ABI_ERROR_MESSAGE + ": missing native symbol " + name)
      }
      return unsafeBitCast(address, to: T.self)
    }
    // No layout-dependent symbol or operation is used before this handshake.
    let version = try symbol("thinkthen_abi_version", Version.self)()
    let required = UInt32(THINKTHEN_ABI_VERSION)
    guard version >> 16 == required >> 16, version & 0xffff >= required & 0xffff else {
      throw CallError.engineVersion(
        required: ABIVersion(major: required >> 16, minor: required & 0xffff),
        found: ABIVersion(major: version >> 16, minor: version & 0xffff))
    }
    admitToolbox = try symbol("thinkthen_toolbox_from_json", Admit.self)
    admitDefinitions = try symbol("thinkthen_toolbox_from_definitions_json", Admit.self)
    exportToolbox = try symbol("thinkthen_toolbox_to_json", ExportJSON.self)
    exportDefinitions = try symbol("thinkthen_toolbox_definitions_json", ExportJSON.self)
    exportNames = try symbol("thinkthen_toolbox_names_json", ExportJSON.self)
    toolboxFree = try symbol("thinkthen_toolbox_free", PointerAction.self)
    bufferFree = try symbol("thinkthen_buffer_free", FreeBuffer.self)
    admitSettings = try symbol("thinkthen_settings_from_json", Admit.self)
    admitQuestion = try symbol("thinkthen_question_from_json", Admit.self)
    settingsFree = try symbol("thinkthen_settings_free", PointerAction.self)
    questionFree = try symbol("thinkthen_question_free", PointerAction.self)
    sendMissing = try symbol("thinkthen_call_send_missing", ConversionErrorCall.self)
    sendInvalid = try symbol("thinkthen_call_send_invalid", ConversionErrorCall.self)
    clientOpen = try symbol("thinkthen_client_open", OpenClient.self)
    callOpen = try symbol("thinkthen_call_open_question", OpenCall.self)
    saveRow = try symbol("thinkthen_row_to_json", SaveRow.self)
    selectedText = try symbol("thinkthen_row_selected_text", SelectedText.self)
    sendText = try symbol("thinkthen_call_send_text", Send.self)
    sendJSON = try symbol("thinkthen_call_send_json", Send.self)
    events = try symbol("thinkthen_call_events", RetainEvents.self)
    eventVersion = try symbol("thinkthen_events_version", EventVersion.self)
    eventWait = try symbol("thinkthen_events_wait", EventWait.self)
    eventWake = try symbol("thinkthen_events_wake", PointerAction.self)
    eventFree = try symbol("thinkthen_events_free", PointerAction.self)
    nextRow = try symbol("thinkthen_call_next", Next.self)
    clientUsage = try symbol("thinkthen_client_usage_values", UsageValues.self)
    clientFree = try symbol("thinkthen_client_free", PointerAction.self)
    cancel = try symbol("thinkthen_call_cancel", PointerAction.self)
    free = try symbol("thinkthen_call_free", PointerAction.self)
    end = try symbol("thinkthen_call_end_input", PointerAction.self)
    errorFree = try symbol("thinkthen_error_free", FreeError.self)
    rowFree = try symbol("thinkthen_row_free", FreeRow.self)
    codeName = try symbol("thinkthen_code_name", CodeName.self)
    for code in ErrorCode.allCases {
      guard let value = code.nativeValue else { continue }
      guard let name = codeName(value), String(cString: name) == code.rawValue else {
        throw CallError.contract(message: "Native error registry does not match this package")
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
    if status < 0 {
      let message = String(decoding: copy(error.message), as: UTF8.self)
      let name = codeName(error.code).map { String(cString: $0) }
      if name == "local.cancelled" { throw CancellationError() }
      throw CallError.engine(
        RowFailure(
          code: ErrorCode(rawValue: name ?? "native.\(error.code)"), message: message,
          httpStatus: error.http_status == 0 ? nil : Int(error.http_status)))
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
      throw CallError.contract(message: "Native admission returned no owner")
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
      throw CallError.contract(message: "Native client returned no owner")
    }
    return pointer
  }
  func usage(_ client: OpaquePointer) -> Usage {
    var value = thinkthen_usage()
    value.struct_size = UInt32(MemoryLayout<thinkthen_usage>.size)
    // The typed ABI only refuses a NULL live owner or an invalid output size.
    // Both are construction invariants here, independent of any caller input.
    let status = clientUsage(client, &value, nil)
    precondition(status == THINKTHEN_OK, "Native usage violated the negotiated ABI")
    return Usage(
      attempts: value.attempts, responses: value.responses,
      statusRetries: value.status_retries, connectionRetries: value.connection_retries,
      requestBodyBytes: value.request_body_bytes, activeAttempts: value.active_attempts,
      activeCalls: value.active_calls,
      inputTokens: TokenUsage(
        reported: value.input_tokens.reported_known ? value.input_tokens.reported : nil,
        unknownAttempts: value.input_tokens.unknown_attempts),
      outputTokens: TokenUsage(
        reported: value.output_tokens.reported_known ? value.output_tokens.reported : nil,
        unknownAttempts: value.output_tokens.unknown_attempts))
  }
  private func toolboxValue(_ bytes: Data, admission: Admit, export: ExportJSON) throws -> Data {
    let toolbox = try admit(bytes, using: admission)
    defer { toolboxFree(toolbox) }
    var output = thinkthen_buffer()
    defer { bufferFree(&output) }
    var error = emptyError()
    // The same admitted native owner supplies names as values before a host
    // decoder can merge canonically equivalent dictionary keys on Linux or Mac.
    try check(exportNames(toolbox, &output, &error), &error)
    let names = try JSONDecoder().decode([String].self, from: copy(output))
    try checkSavedNames(names)
    bufferFree(&output)
    try check(export(toolbox, &output, &error), &error)
    return copy(output)
  }
  func toolboxJSON(_ bytes: Data, loading: Bool) throws -> Data {
    try toolboxValue(
      bytes, admission: loading ? admitToolbox : admitDefinitions,
      export: loading ? exportDefinitions : exportToolbox)
  }
  func annotationJSON(_ bytes: Data) throws -> Data {
    try toolboxValue(bytes, admission: admitDefinitions, export: exportDefinitions)
  }
  func openCall(_ client: OpaquePointer, _ bytes: Data, name: String?) throws -> OpaquePointer {
    let question = try admit(bytes, using: admitQuestion)
    defer { questionFree(question) }
    var pointer: OpaquePointer?
    var error = emptyError()
    let nameBytes = name.map { Data($0.utf8) } ?? Data()
    let status = nameBytes.withUnsafeBytes { buffer in
      var view = thinkthen_text(
        data: buffer.bindMemory(to: CChar.self).baseAddress, len: buffer.count)
      return withUnsafePointer(to: &view) { nameView in
        callOpen(
          client, question, name == nil ? nil : nameView,
          UInt32(THINKTHEN_OFFSET_CODE_POINTS.rawValue), &pointer, &error)
      }
    }
    try check(status, &error)
    guard let pointer else { throw CallError.contract(message: "Native call returned no owner") }
    return pointer
  }
  func send(_ call: OpaquePointer, _ record: InputPayload) throws {
    let bytes: Data
    let operation: Send
    switch record {
    case .text(let text):
      bytes = Data(text.utf8)
      operation = sendText
    case .missing:
      var error = emptyError()
      let status = sendMissing(call, &error)
      try check(status, &error)
      return
    case .invalid:
      var error = emptyError()
      try check(sendInvalid(call, &error), &error)
      return
    case .json(let value):
      bytes = value
      operation = sendJSON
    }
    var error = emptyError()
    let status = bytes.withUnsafeBytes {
      operation(call, $0.bindMemory(to: UInt8.self).baseAddress, bytes.count, &error)
    }
    try check(status, &error)
  }
  private func canonicalJSON(_ row: inout thinkthen_row) throws -> Data {
    var buffer = thinkthen_buffer()
    var error = emptyError()
    defer { bufferFree(&buffer) }
    try check(saveRow(&row, &buffer, &error), &error)
    return copy(buffer)
  }
  private func selectedSource(_ row: inout thinkthen_row, entry: String?) throws -> String? {
    var text = thinkthen_text()
    var error = emptyError()
    let status: Int32
    if let entry {
      let bytes = Array(entry.utf8)
      status = bytes.withUnsafeBytes {
        selectedText(&row, $0.bindMemory(to: CChar.self).baseAddress, bytes.count, &text, &error)
      }
    } else {
      status = selectedText(&row, nil, 0, &text, &error)
    }
    try check(status, &error)
    guard let data = text.data else { return nil }
    return String(decoding: UnsafeRawBufferPointer(start: data, count: text.len), as: UTF8.self)
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
      throw rowABIFailure("Unknown native operation status")
    }
    switch row.status {
    case THINKTHEN_STATUS_ANSWERED, THINKTHEN_STATUS_UNSURE, THINKTHEN_STATUS_FAILED: break
    default: throw rowABIFailure("Unknown native row status")
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
    default: throw rowABIFailure("Unknown native function")
    }
    let canonical = try canonicalJSON(&row)
    var texts: [String: String] = [:]
    if ["find", "recognize", "relate"].contains(function) {
      texts[""] = try selectedSource(&row, entry: nil)
    } else if function == "annotate" {
      let saved = try JSONDecoder().decode(JSONValue.self, from: canonical)
      for (name, entry) in saved.object["answer"]?.object ?? [:] {
        if ["find", "recognize", "relate"].contains(entry.object["function"]?.string ?? "") {
          texts[name] = try selectedSource(&row, entry: name)
        }
      }
    }
    return .row(WireRow(canonical: canonical, selectedTexts: texts, position: row.position))

  }
}
struct WireRow: Sendable {
  let canonical: Data
  let selectedTexts: [String: String]
  let position: UInt64
  func decode<F: FunctionShape, T: Sendable>(_ type: F.Type, item: T, questions: JSONValue = .null)
    throws -> Row<F, T>
  {
    let value = try JSONDecoder().decode(JSONValue.self, from: canonical)
    let decoder = JSONDecoder()
    decoder.userInfo[.originalItem] = item
    decoder.userInfo[.parentRow] = value
    decoder.userInfo[.sourceText] = selectedTexts[""] ?? ""
    decoder.userInfo[.selectedTexts] = selectedTexts
    decoder.userInfo[.questionDefinitions] = questions.object["questions"] ?? .null
    return try Row(saved: value, item: item, decoder: decoder)
  }
}
extension CodingUserInfoKey {
  static let originalItem = Self(rawValue: "ThinkThen.originalItem")!
  static let parentRow = Self(rawValue: "ThinkThen.parentRow")!
  static let questionDefinitions = Self(rawValue: "ThinkThen.questionDefinitions")!
  static let selectedTexts = Self(rawValue: "ThinkThen.selectedTexts")!
  static let sourceText = Self(rawValue: "ThinkThen.sourceText")!
}

/// True for a path the system loader opens without searching.
private func isFullPath(_ path: String) -> Bool {
  #if os(Windows)
    let bytes = Array(path.utf8)
    let separators = [UInt8(ascii: "\\"), UInt8(ascii: "/")]
    return path.hasPrefix("\\\\")
      || (bytes.count > 2 && bytes[1] == UInt8(ascii: ":") && separators.contains(bytes[2]))
  #else
    return path.hasPrefix("/")
  #endif
}

func engineLoadFailure(_ message: String) -> CallError {
  .engine(RowFailure(code: .localIO, message: message, httpStatus: nil))
}

func rowABIFailure(_ message: String) -> CallError {
  CallError(
    kind: .contract, message: message, failure: RowFailure(code: .usageABI, message: message))
}
