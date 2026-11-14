import Foundation

/// A safe diagnostic on a failed result.
public struct RowFailure: Sendable, Codable, Equatable, Hashable, CustomStringConvertible {
  public let code: ErrorCode
  public let message: String
  public let httpStatus: Int?
  public init(code: ErrorCode, message: String, httpStatus: Int? = nil) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
  }
  enum CodingKeys: String, CodingKey {
    case code, message
    case httpStatus = "http_status"
  }
  public var description: String { "\(code): \(message)" }
}

public protocol NoAnswer: Error {}
public protocol ThinkThenError: Error, Sendable {
  var code: ErrorCode { get }
  var message: String { get }
  var httpStatus: Int? { get }
}
public struct UsageError: ThinkThenError, LocalizedError {
  public let code: ErrorCode
  public let message: String
  public let httpStatus: Int?
  public var errorDescription: String? { message }
  public init(code: ErrorCode = .usageQuestion, message: String, httpStatus: Int? = nil) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
  }
}
public struct SetupError: ThinkThenError, LocalizedError {
  public let code: ErrorCode
  public let message: String
  public let httpStatus: Int?
  public var errorDescription: String? { message }
  public init(code: ErrorCode, message: String, httpStatus: Int? = nil) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
  }
}
public struct InternalError: ThinkThenError, LocalizedError {
  public let code: ErrorCode
  public let message: String
  public let httpStatus: Int?
  public var errorDescription: String? { message }
  public init(code: ErrorCode, message: String, httpStatus: Int? = nil) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
  }
}
public struct Unsure: NoAnswer, Sendable {
  public let probability: Double
  public let row: AnyRow
}
public struct Failed: ThinkThenError, NoAnswer, LocalizedError {
  public let code: ErrorCode
  public let message: String
  public let httpStatus: Int?
  public let rows: [AnyRow]
  public var errorDescription: String? { message }
  init(_ failure: RowFailure, rows: [AnyRow]) {
    code = failure.code
    message = failure.message
    httpStatus = failure.httpStatus
    self.rows = rows
  }
}
