import Foundation

public enum Status: String, Sendable, Codable { case answered, unsure, failed }

public protocol FunctionShape: Sendable {
  associatedtype Answer: Sendable
  associatedtype Detail: Sendable & Codable & Equatable & Hashable
  static var function: FunctionName { get }
  static func decodeAnswer(_ data: Data, using decoder: JSONDecoder) throws -> Answer
}
extension FunctionShape where Answer: Decodable {
  public static func decodeAnswer(_ data: Data, using decoder: JSONDecoder) throws -> Answer {
    try decoder.decode(Answer.self, from: data)
  }
}
public struct Source: Sendable, Codable, Equatable, Hashable {
  public let questionIdentity: String
  public let backend: Backend
  public let model: String?
  public let name: String?
  enum CodingKeys: String, CodingKey {
    case questionIdentity = "question_identity"
    case backend, model, name
  }
}

/// The typed result and its retained caller record. Equality never reads the record.
public struct Row<F: FunctionShape, Item: Sendable>: Sendable, Encodable, Equatable, Hashable,
  CustomStringConvertible, CustomDebugStringConvertible
{
  public let item: Item
  public let position: Int
  public let id: RecordID?
  public let source: Source
  public let status: Status
  public let probability: Double?
  public let error: RowFailure?
  public let detail: F.Detail?
  private let value: F.Answer?
  // The owned nine-field engine value is the equality key. Native span indexes
  // are projection views; neither they nor the caller's item enter this key.
  let saved: JSONValue
  public var function: FunctionName { F.function }
  public var isAnswered: Bool { status == .answered }
  public var isUnsure: Bool { status == .unsure }
  public var isFailed: Bool { status == .failed }
  public var answer: F.Answer {
    get throws {
      switch status {
      case .answered:
        guard let value else {
          throw InternalError(code: .internalBug, message: "Answered result has no projected value")
        }
        return value
      case .unsure:
        guard let probability else {
          throw InternalError(code: .internalBug, message: "Unsure result has no probability")
        }
        throw Unsure(probability: probability, row: erased)
      case .failed:
        guard let error else {
          throw InternalError(code: .internalBug, message: "Failed result has no diagnostic")
        }
        throw Failed(error, rows: [erased])
      }
    }
  }
  @discardableResult public func throwIfFailed() throws -> Self {
    if let failed = firstFailedResult(saved) {
      let failure = try decodeJSON(RowFailure.self, failed.object["error"], JSONDecoder())
      let failedFunction = try decodeJSON(
        FunctionName.self, failed.object["function"], JSONDecoder())
      let row = AnyRow(
        item: item, function: failedFunction, position: position, id: id, source: source,
        status: .failed, probability: nil, error: failure, saved: failed)
      throw Failed(failure, rows: [row])
    }
    return self
  }
  var erased: AnyRow {
    AnyRow(
      item: item, function: function, position: position, id: id, source: source,
      status: status, probability: probability, error: error, saved: saved)
  }
  init(saved: JSONValue, item: Item, decoder: JSONDecoder) throws {
    guard saved.object["function"]?.string == F.function.rawValue else {
      throw CallError.contract(message: "Native row function does not match its Swift result")
    }
    self.saved = saved
    self.item = item
    let fields = saved.object
    position = try decodeJSON(Int.self, fields["position"], decoder)
    id = try fields["id"].map { try decodeJSON(RecordID.self, $0, decoder) }
    source = try decodeJSON(Source.self, fields["source"], decoder)
    status = try decodeJSON(Status.self, fields["status"], decoder)
    probability = try fields["probability"].map { try decodeJSON(Double.self, $0, decoder) }
    error = try fields["error"].map { try decodeJSON(RowFailure.self, $0, decoder) }
    detail = try fields["detail"].map { try decodeJSON(F.Detail.self, $0, decoder) }
    if status == .answered {
      guard let answer = fields["answer"] else {
        throw CallError.contract(message: "Answered result has no answer field")
      }
      value = .some(try F.decodeAnswer(JSONEncoder().encode(answer), using: decoder))
    } else {
      value = nil
    }
  }
  public func encode(to encoder: any Encoder) throws { try saved.encode(to: encoder) }
  public static func == (lhs: Self, rhs: Self) -> Bool { lhs.saved == rhs.saved }
  public func hash(into hasher: inout Hasher) { hasher.combine(saved) }
  public var description: String { printRow(saved) }
  public var debugDescription: String { description }
}

/// A result whose original record type is erased for a shared error catch.
public struct AnyRow: Sendable, Encodable, Equatable, Hashable, CustomStringConvertible,
  CustomDebugStringConvertible
{
  public let item: any Sendable
  public let function: FunctionName
  public let position: Int
  public let id: RecordID?
  public let source: Source
  public let status: Status
  public let probability: Double?
  public let error: RowFailure?
  let saved: JSONValue
  public var isAnswered: Bool { status == .answered }
  public var isUnsure: Bool { status == .unsure }
  public var isFailed: Bool { status == .failed }
  public var description: String { printRow(saved) }
  public var debugDescription: String { description }
  public func encode(to encoder: any Encoder) throws { try saved.encode(to: encoder) }
  public static func == (lhs: Self, rhs: Self) -> Bool { lhs.saved == rhs.saved }
  public func hash(into hasher: inout Hasher) { hasher.combine(saved) }
}
public enum NoDetail: Sendable, Codable, Equatable, Hashable {
  public init(from decoder: any Decoder) throws {
    throw DecodingError.dataCorrupted(
      .init(codingPath: decoder.codingPath, debugDescription: "This function has no detail"))
  }
  public func encode(to encoder: any Encoder) throws { switch self {} }
}
