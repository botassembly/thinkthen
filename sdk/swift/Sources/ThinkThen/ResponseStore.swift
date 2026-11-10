import Foundation

/// How the explicitly selected local store serves this client.
public enum ResponseMode: String, Codable, Sendable {
  case cache, record, replay
}

/// Explicit private local storage of input/output text. Retention is off unless selected.
public struct ResponseStore: Codable, Sendable, Equatable {
  public let path: String
  public let mode: ResponseMode
  public let namespace: String
  public let modelGeneration: String
  public let maxBytes: UInt64?
  public let maxEntries: UInt64?
  public let maxAge: UInt64?
  public let busyTimeoutMs: UInt64?
  /// Select storage with explicit nonsecret identities and optional shared limits.
  public init(
    path: String, mode: ResponseMode, namespace: String, modelGeneration: String,
    maxBytes: UInt64? = nil, maxEntries: UInt64? = nil,
    maxAge: UInt64? = nil, busyTimeoutMs: UInt64? = nil
  ) {
    self.path = path
    self.mode = mode
    self.namespace = namespace
    self.modelGeneration = modelGeneration
    self.maxBytes = maxBytes
    self.maxEntries = maxEntries
    self.maxAge = maxAge
    self.busyTimeoutMs = busyTimeoutMs
  }
  enum CodingKeys: String, CodingKey {
    case path, mode, namespace
    case modelGeneration = "model_generation"
    case maxBytes = "max_bytes"
    case maxEntries = "max_entries"
    case maxAge = "max_age"
    case busyTimeoutMs = "busy_timeout_ms"
  }
}
