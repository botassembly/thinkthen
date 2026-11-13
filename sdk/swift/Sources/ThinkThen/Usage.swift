import Foundation

/// Provider-reported token sum; nil means overflow, with finished unknown attempts separate.
public struct TokenUsage: Codable, Sendable {
  public let reported: UInt64?
  public let unknownAttempts: UInt64
  enum CodingKeys: String, CodingKey {
    case reported
    case unknownAttempts = "unknown_attempts"
  }
}

/// A copied cumulative client snapshot, valid after the client is destroyed.
public struct Usage: Codable, Sendable {
  public let attempts: UInt64
  public let responses: UInt64
  public let statusRetries: UInt64
  public let connectionRetries: UInt64
  public let requestBodyBytes: UInt64
  public let activeAttempts: UInt64
  public let activeCalls: UInt64
  public let inputTokens: TokenUsage
  public let outputTokens: TokenUsage
  enum CodingKeys: String, CodingKey {
    case attempts, responses
    case statusRetries = "status_retries"
    case connectionRetries = "connection_retries"
    case requestBodyBytes = "request_body_bytes"
    case activeAttempts = "active_attempts"
    case activeCalls = "active_calls"
    case inputTokens = "input_tokens"
    case outputTokens = "output_tokens"
  }
}
