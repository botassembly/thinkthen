import Foundation

/// A backend name. New backends do not change the type's public shape.
public struct Backend: RawRepresentable, Sendable, Codable, Hashable {
  /// The stable wire spelling; names from newer engines remain representable.
  public let rawValue: String
  /// Preserve a stable name, including a name added by a newer engine.
  public init(rawValue: String) { self.rawValue = rawValue }
  /// The systemOne value.
  public static let systemOne = Self(rawValue: "systemone")
  /// The openAI value.
  public static let openAI = Self(rawValue: "openai")
  /// The stub value.
  public static let stub = Self(rawValue: "stub")
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    self.init(rawValue: try decoder.singleValueContainer().decode(String.self))
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var container = encoder.singleValueContainer()
    try container.encode(rawValue)
  }
}
/// A function name, including functions added by a newer engine.
public struct FunctionName: RawRepresentable, Sendable, Codable, Hashable {
  /// The stable wire spelling; names from newer engines remain representable.
  public let rawValue: String
  /// Preserve a stable name, including a name added by a newer engine.
  public init(rawValue: String) { self.rawValue = rawValue }
  /// The decide value.
  public static let decide = Self(rawValue: "decide")
  /// The choose value.
  public static let choose = Self(rawValue: "choose")
  /// The tag value.
  public static let tag = Self(rawValue: "tag")
  /// The score value.
  public static let score = Self(rawValue: "score")
  /// The filter value.
  public static let filter = Self(rawValue: "filter")
  /// The rank value.
  public static let rank = Self(rawValue: "rank")
  /// The find value.
  public static let find = Self(rawValue: "find")
  /// The recognize value.
  public static let recognize = Self(rawValue: "recognize")
  /// The relate value.
  public static let relate = Self(rawValue: "relate")
  /// The annotate value.
  public static let annotate = Self(rawValue: "annotate")
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    self.init(rawValue: try decoder.singleValueContainer().decode(String.self))
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var container = encoder.singleValueContainer()
    try container.encode(rawValue)
  }
}
/// The typed JSONValue value exchanged with the engine.
public enum JSONValue: Sendable, Codable, Equatable, Hashable {
  /// A JSON null value.
  case null
  /// A JSON boolean value.
  case bool(Bool)
  /// A string value.
  case string(String)
  /// A floating-point JSON number.
  case number(Double)
  /// A signed integer without conversion through Double.
  case integer(Int64)
  /// An unsigned integer without conversion through Double.
  case unsigned(UInt64)
  /// A JSON array stored as one structured value.
  case array([JSONValue])
  /// A JSON object whose members retain their types.
  case object([String: JSONValue])
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.singleValueContainer()
    if c.decodeNil() {
      self = .null
    } else if let x = try? c.decode(Bool.self) {
      self = .bool(x)
    } else if let x = try? c.decode(String.self) {
      self = .string(x)
    } else if let x = try? c.decode(Int64.self) {
      self = .integer(x)
    } else if let x = try? c.decode(UInt64.self) {
      self = .unsigned(x)
    } else if let x = try? c.decode(Double.self) {
      self = .number(x)
    } else if let x = try? c.decode([JSONValue].self) {
      self = .array(x)
    } else {
      self = .object(try c.decode([String: JSONValue].self))
    }
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    switch self {
    case .null: try c.encodeNil()
    case .bool(let x): try c.encode(x)
    case .string(let x): try c.encode(x)
    case .number(let x): try c.encode(x)
    case .integer(let x): try c.encode(x)
    case .unsigned(let x): try c.encode(x)
    case .array(let x): try c.encode(x)
    case .object(let x): try c.encode(x)
    }
  }
}
/// The typed Record value exchanged with the engine.
public enum Record: Sendable, Codable, Equatable, Hashable, ExpressibleByStringLiteral {
  /// Create a value from a string literal.
  public init(stringLiteral value: String) { self = .text(value) }
  /// An unstructured text record.
  case text(String)
  /// A structured JSON record.
  case json(JSONValue)
  /// A missing record that the engine reports as a failed row.
  case missing
  /// Report a value that could not be encoded.
  public static func encoding<T: Encodable>(_ value: T) throws -> Record {
    do {
      return .json(try JSONDecoder().decode(JSONValue.self, from: JSONEncoder().encode(value)))
    } catch { throw ThinkThenError.encoding(message: String(describing: error)) }
  }
}
/// The typed RecordID value exchanged with the engine.
public enum RecordID: Sendable, Codable, Equatable, Hashable {
  /// A string value.
  case string(String)
  /// A signed integer without conversion through Double.
  case integer(Int64)
  /// An unsigned integer without conversion through Double.
  case unsigned(UInt64)
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.singleValueContainer()
    if let x = try? c.decode(String.self) {
      self = .string(x)
    } else if let x = try? c.decode(Int64.self) {
      self = .integer(x)
    } else {
      self = .unsigned(try c.decode(UInt64.self))
    }
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    switch self {
    case .string(let x): try c.encode(x)
    case .integer(let x): try c.encode(x)
    case .unsigned(let x): try c.encode(x)
    }
  }
}
/// The typed Threshold value exchanged with the engine.
public enum Threshold: Sendable, Codable, Equatable, Hashable,
  ExpressibleByFloatLiteral, ExpressibleByIntegerLiteral
{
  /// A single numeric threshold.
  case value(Double)
  /// Low and high thresholds with an unsure band between them.
  case band(low: Double, high: Double)
  /// Create a numeric threshold from a literal.
  public init(floatLiteral value: Double) { self = .value(value) }
  /// Create a numeric threshold from a literal.
  public init(integerLiteral value: Int) { self = .value(Double(value)) }
  private enum Keys: String, CodingKey { case low, high }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    if let x = try? decoder.singleValueContainer().decode(Double.self) {
      self = .value(x)
    } else {
      let c = try decoder.container(keyedBy: Keys.self)
      self = .band(
        low: try c.decode(Double.self, forKey: .low), high: try c.decode(Double.self, forKey: .high)
      )
    }
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    switch self {
    case .value(let x):
      var c = encoder.singleValueContainer()
      try c.encode(x)
    case .band(let low, let high):
      var c = encoder.container(keyedBy: Keys.self)
      try c.encode(low, forKey: .low)
      try c.encode(high, forKey: .high)
    }
  }
}
/// The typed APIKey value exchanged with the engine.
public struct APIKey: Sendable, Equatable, CustomStringConvertible, CustomDebugStringConvertible,
  CustomReflectable
{
  let value: String
  /// Create this value from its typed fields.
  public init(_ value: String) { self.value = value }
  /// A readable value summary; credentials remain redacted.
  public var description: String { "<redacted>" }
  /// The redacted diagnostic representation.
  public var debugDescription: String { description }
  /// A reflection view that never reveals the credential.
  public var customMirror: Mirror { Mirror(self, children: ["value": "<redacted>"]) }
}
/// Optional retry overrides. Omitted fields use the engine's own defaults.
public struct Retry: Sendable, Codable, Equatable, Hashable {
  /// The retry count after the initial attempt.
  public let maxRetries: Int?, initialBackoff: Duration?, attemptTimeout: Duration?
  /// Create this value from its typed fields.
  public init(
    maxRetries: Int? = nil, initialBackoff: Duration? = nil, attemptTimeout: Duration? = nil
  ) {
    self.maxRetries = maxRetries
    self.initialBackoff = initialBackoff
    self.attemptTimeout = attemptTimeout
  }
  enum CodingKeys: String, CodingKey {
    case maxRetries = "max_retries"
    case initialBackoff = "initial_backoff_ms"
    case attemptTimeout = "attempt_timeout_ms"
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    maxRetries = try c.decodeIfPresent(Int.self, forKey: .maxRetries)
    initialBackoff = try c.decodeIfPresent(Int64.self, forKey: .initialBackoff).map {
      .milliseconds($0)
    }
    attemptTimeout = try c.decodeIfPresent(Int64.self, forKey: .attemptTimeout).map {
      .milliseconds($0)
    }
  }
  func milliseconds(_ value: Duration) throws -> Int64 {
    let x = value.components
    let (seconds, overflow) = x.seconds.multipliedReportingOverflow(by: 1000)
    let (total, sumOverflow) = seconds.addingReportingOverflow(
      x.attoseconds / 1_000_000_000_000_000)
    guard !overflow, !sumOverflow, x.attoseconds % 1_000_000_000_000_000 == 0 else {
      throw ThinkThenError.encoding(
        message: "Duration cannot be represented as integer milliseconds")
    }
    return total
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encodeIfPresent(maxRetries, forKey: .maxRetries)
    try c.encodeIfPresent(initialBackoff.map { try milliseconds($0) }, forKey: .initialBackoff)
    try c.encodeIfPresent(attemptTimeout.map { try milliseconds($0) }, forKey: .attemptTimeout)
  }
}
/// The typed Settings value exchanged with the engine.
public struct Settings: Sendable, Codable, Equatable, CustomStringConvertible,
  CustomDebugStringConvertible, CustomReflectable
{
  /// The selected backend name.
  public let backend: Backend, url: URL?, model: String?, apiKey: APIKey?, batch: Int?,
    retry: Retry?
  /// Create this value from its typed fields.
  public init(
    backend: Backend? = nil, url: URL? = nil, model: String? = nil, apiKey: APIKey? = nil,
    batch: Int? = nil, retry: Retry? = nil
  ) {
    self.backend = backend ?? Self.defaultBackend
    self.url = url
    self.model = model
    self.apiKey = apiKey
    self.batch = batch
    self.retry = retry
  }
  private static var defaultBackend: Backend {
    ProcessInfo.processInfo.environment["THINKTHEN_BACKEND"].map(Backend.init(rawValue:))
      ?? .systemOne
  }
  enum CodingKeys: String, CodingKey {
    case backend, url, model, batch, retry
    case apiKey = "api_key"
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    backend = try c.decodeIfPresent(Backend.self, forKey: .backend) ?? Self.defaultBackend
    url = try c.decodeIfPresent(URL.self, forKey: .url)
    model = try c.decodeIfPresent(String.self, forKey: .model)
    apiKey = try c.decodeIfPresent(String.self, forKey: .apiKey).map(APIKey.init)
    batch = try c.decodeIfPresent(Int.self, forKey: .batch)
    retry = try c.decodeIfPresent(Retry.self, forKey: .retry)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(backend, forKey: .backend)
    try c.encodeIfPresent(url, forKey: .url)
    try c.encodeIfPresent(model, forKey: .model)
    try c.encodeIfPresent(batch, forKey: .batch)
    try c.encodeIfPresent(retry, forKey: .retry)
  }
  /// A readable value summary; credentials remain redacted.
  public var description: String { "Settings(apiKey: <redacted>)" }
  /// The redacted diagnostic representation.
  public var debugDescription: String { description }
  /// A reflection view that never reveals the credential.
  public var customMirror: Mirror { Mirror(self, children: ["apiKey": "<redacted>"]) }
}
/// The typed ABIVersion value exchanged with the engine.
public struct ABIVersion: Sendable, Equatable { public let major: UInt32, minor: UInt32 }
/// The typed IterationIssue value exchanged with the engine.
public enum IterationIssue: Sendable, Equatable { case alreadyIterated, concurrentNext }
/// An error reported while opening or reading a ThinkThen call.
public struct ThinkThenError: LocalizedError, Sendable, Equatable, CustomStringConvertible {
  /// An extensible category for errors originating in this package.
  public struct Kind: RawRepresentable, Sendable, Hashable {
    /// The stable wire spelling; names from newer engines remain representable.
    public let rawValue: String
    /// Preserve a stable name, including a name added by a newer engine.
    public init(rawValue: String) { self.rawValue = rawValue }
    /// The engine value.
    public static let engine = Self(rawValue: "engine")
    /// The engineVersion value.
    public static let engineVersion = Self(rawValue: "engineVersion")
    /// The encoding value.
    public static let encoding = Self(rawValue: "encoding")
    /// The contract value.
    public static let contract = Self(rawValue: "contract")
    /// The recordSource value.
    public static let recordSource = Self(rawValue: "recordSource")
    /// The toolboxSelection value.
    public static let toolboxSelection = Self(rawValue: "toolboxSelection")
    /// The iteration value.
    public static let iteration = Self(rawValue: "iteration")
  }
  /// The recognized kind, or the local error category.
  public let kind: Kind
  /// The engine diagnostic, with credential contents redacted.
  public let message: String
  /// The record failure, or nil when the outcome did not fail.
  public let failure: RowFailure?
  /// The readable diagnostic shown by localizedDescription.
  public var errorDescription: String? { message }
  /// A readable value summary; credentials remain redacted.
  public var description: String { message }
  /// Create this value from its typed fields.
  public init(kind: Kind, message: String, failure: RowFailure? = nil) {
    self.kind = kind
    self.message = message
    self.failure = failure
  }
  /// Wrap a record failure without changing its stable code or message.
  public static func engine(_ failure: RowFailure) -> Self {
    Self(kind: .engine, message: failure.message, failure: failure)
  }
  /// Report a value that could not be encoded.
  public static func encoding(message: String) -> Self { Self(kind: .encoding, message: message) }
  /// Report a malformed native response.
  public static func contract(message: String) -> Self { Self(kind: .contract, message: message) }
  /// Report an error while reading the caller's input source.
  public static func recordSource(message: String) -> Self {
    Self(kind: .recordSource, message: message)
  }
  /// Describe an incompatible native ABI version.
  public static func engineVersion(required: ABIVersion, found: ABIVersion) -> Self {
    Self(
      kind: .engineVersion,
      message:
        "Native ABI \(found.major).\(found.minor) does not support required ABI \(required.major).\(required.minor)"
    )
  }
  /// Report a saved entry that does not have the requested function.
  public static func toolboxSelection(name: String, expected: FunctionName) -> Self {
    Self(kind: .toolboxSelection, message: "Toolbox question \(name) must use \(expected.rawValue)")
  }
  /// Report an invalid attempt to iterate the single-pass stream.
  public static func iteration(_ issue: IterationIssue) -> Self {
    Self(
      kind: .iteration,
      message: issue == .alreadyIterated
        ? "Rows have already been iterated" : "Rows cannot be advanced concurrently")
  }
}
func typedError(_ error: any Error) -> any Error {
  if error is CancellationError || error is ThinkThenError || error is RowFailure
    || error is RowError
  {
    return error
  }
  if error is EncodingError { return ThinkThenError.encoding(message: String(describing: error)) }
  return ThinkThenError.contract(message: String(describing: error))
}
