import Foundation

public enum Backend: String, Sendable, Codable, CaseIterable { case systemone, openai, stub }
public enum FunctionName: String, Sendable, Codable, CaseIterable {
  case decide, choose, tag, score, filter, rank, find, recognize, relate, annotate
}
public enum JSONValue: Sendable, Codable, Equatable, Hashable {
  case null
  case bool(Bool)
  case string(String)
  case number(Double)
  case integer(Int64)
  case unsigned(UInt64)
  case array([JSONValue])
  case object([String: JSONValue])
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
public enum Record: Sendable, Codable, Equatable, Hashable {
  case text(String)
  case json(JSONValue)
  case missing
  public static func encoding<T: Encodable>(_ value: T) throws(ThinkThenError) -> Record {
    do {
      return .json(try JSONDecoder().decode(JSONValue.self, from: JSONEncoder().encode(value)))
    } catch { throw .encoding(message: String(describing: error)) }
  }
}
public enum RecordID: Sendable, Codable, Equatable, Hashable {
  case string(String)
  case integer(Int64)
  case unsigned(UInt64)
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
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    switch self {
    case .string(let x): try c.encode(x)
    case .integer(let x): try c.encode(x)
    case .unsigned(let x): try c.encode(x)
    }
  }
}
public enum Threshold: Sendable, Codable, Equatable, Hashable,
  ExpressibleByFloatLiteral, ExpressibleByIntegerLiteral
{
  case value(Double)
  case band(low: Double, high: Double)
  public init(floatLiteral value: Double) { self = .value(value) }
  public init(integerLiteral value: Int) { self = .value(Double(value)) }
  private enum Keys: String, CodingKey { case low, high }
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
public struct APIKey: Sendable, Equatable, CustomStringConvertible, CustomDebugStringConvertible,
  CustomReflectable
{
  let value: String
  public init(_ value: String) { self.value = value }
  public var description: String { "<redacted>" }
  public var debugDescription: String { description }
  public var customMirror: Mirror { Mirror(self, children: ["value": "<redacted>"]) }
}
public struct Retry: Sendable, Codable, Equatable, Hashable {
  public let maxRetries: Int, initialBackoff: Duration, attemptTimeout: Duration
  public init(maxRetries: Int, initialBackoff: Duration, attemptTimeout: Duration) {
    self.maxRetries = maxRetries
    self.initialBackoff = initialBackoff
    self.attemptTimeout = attemptTimeout
  }
  enum CodingKeys: String, CodingKey {
    case maxRetries = "max_retries"
    case initialBackoff = "initial_backoff_ms"
    case attemptTimeout = "attempt_timeout_ms"
  }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    maxRetries = try c.decode(Int.self, forKey: .maxRetries)
    initialBackoff = .milliseconds(try c.decode(Int64.self, forKey: .initialBackoff))
    attemptTimeout = .milliseconds(try c.decode(Int64.self, forKey: .attemptTimeout))
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
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(maxRetries, forKey: .maxRetries)
    try c.encode(milliseconds(initialBackoff), forKey: .initialBackoff)
    try c.encode(milliseconds(attemptTimeout), forKey: .attemptTimeout)
  }
}
public struct Settings: Sendable, Codable, Equatable, CustomStringConvertible,
  CustomDebugStringConvertible, CustomReflectable
{
  public let backend: Backend, url: URL?, model: String?, apiKey: APIKey?, batch: Int?,
    retry: Retry?
  public init(
    backend: Backend = .systemone, url: URL? = nil, model: String? = nil, apiKey: APIKey? = nil,
    batch: Int? = nil, retry: Retry? = nil
  ) {
    self.backend = backend
    self.url = url
    self.model = model
    self.apiKey = apiKey
    self.batch = batch
    self.retry = retry
  }
  enum CodingKeys: String, CodingKey {
    case backend, url, model, batch, retry
    case apiKey = "api_key"
  }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    backend = try c.decodeIfPresent(Backend.self, forKey: .backend) ?? .systemone
    url = try c.decodeIfPresent(URL.self, forKey: .url)
    model = try c.decodeIfPresent(String.self, forKey: .model)
    apiKey = try c.decodeIfPresent(String.self, forKey: .apiKey).map(APIKey.init)
    batch = try c.decodeIfPresent(Int.self, forKey: .batch)
    retry = try c.decodeIfPresent(Retry.self, forKey: .retry)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(backend, forKey: .backend)
    try c.encodeIfPresent(url, forKey: .url)
    try c.encodeIfPresent(model, forKey: .model)
    try c.encodeIfPresent(batch, forKey: .batch)
    try c.encodeIfPresent(retry, forKey: .retry)
  }
  public var description: String { "Settings(apiKey: <redacted>)" }
  public var debugDescription: String { description }
  public var customMirror: Mirror { Mirror(self, children: ["apiKey": "<redacted>"]) }
}
public struct ABIVersion: Sendable, Equatable { public let major: UInt32, minor: UInt32 }
public enum IterationIssue: Sendable, Equatable { case alreadyIterated, concurrentNext }
public enum ThinkThenError: LocalizedError, Sendable, Equatable {
  case engine(RowFailure)
  case unknownCode(rawCode: Int32, message: String, httpStatus: UInt16?, rawName: String?)
  case engineUnavailable(message: String)
  case engineVersion(required: ABIVersion, found: ABIVersion)
  case encoding(message: String)
  case contract(message: String)
  case recordSource(message: String)
  case toolboxSelection(name: String, expected: FunctionName)
  case iteration(IterationIssue)
  case cancelled

  public var errorDescription: String? {
    switch self {
    case .engine(let failure): return failure.message
    case .unknownCode(_, let message, _, _), .engineUnavailable(let message),
      .encoding(let message), .contract(let message), .recordSource(let message):
      return message
    case .engineVersion(let required, let found):
      return "Native ABI \(found.major).\(found.minor) does not support required ABI "
        + "\(required.major).\(required.minor)"
    case .toolboxSelection(let name, let expected):
      return "Toolbox question \(name) must use \(expected)"
    case .iteration(.alreadyIterated): return "Rows have already been iterated"
    case .iteration(.concurrentNext): return "Rows cannot be advanced concurrently"
    case .cancelled: return "The call was cancelled"
    }
  }
}
func typedError(_ error: any Error) -> ThinkThenError {
  if let typed = error as? ThinkThenError { return typed }
  if error is CancellationError { return .cancelled }
  if error is EncodingError { return .encoding(message: String(describing: error)) }
  return .contract(message: String(describing: error))
}
