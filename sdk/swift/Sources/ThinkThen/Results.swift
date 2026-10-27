import Foundation

public enum Status: String, Sendable, Codable { case answered, unsure, failed }
public struct RowFailure: Error, Sendable, Codable, Equatable, Hashable {
  public let code: ErrorCode, message: String, httpStatus: UInt16?
  public let rawCode: Int32?
  public let rawName: String?
  public init(
    code: ErrorCode, message: String, httpStatus: UInt16?, rawCode: Int32? = nil,
    rawName: String? = nil
  ) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
    self.rawCode = rawCode
    self.rawName = rawName
  }
  enum CodingKeys: String, CodingKey {
    case code, message
    case rawCode = "raw_code"
    case rawName = "raw_name"
    case httpStatus = "http_status"
  }
  public init(from decoder: any Decoder) throws {
    let fields = try decoder.container(keyedBy: CodingKeys.self)
    let name = try fields.decode(String.self, forKey: .code)
    code = ErrorCode(rawValue: name) ?? .unknown
    message = try fields.decode(String.self, forKey: .message)
    httpStatus = try fields.decodeIfPresent(UInt16.self, forKey: .httpStatus)
    rawCode = try fields.decodeIfPresent(Int32.self, forKey: .rawCode)
    let preservedName = try fields.decodeIfPresent(String.self, forKey: .rawName)
    rawName = preservedName ?? (code == .unknown ? name : nil)
  }
  public func encode(to encoder: any Encoder) throws {
    var fields = encoder.container(keyedBy: CodingKeys.self)
    try fields.encode(rawName ?? code.rawValue, forKey: .code)
    try fields.encode(message, forKey: .message)
    try fields.encodeIfPresent(httpStatus, forKey: .httpStatus)
    try fields.encodeIfPresent(rawCode, forKey: .rawCode)
  }
}
public enum Outcome<A: Sendable, D: Sendable>: Sendable {
  case answered(answer: A, probability: Double?, detail: D?)
  case unsure(probability: Double, detail: D?)
  case failed(RowFailure)
  public var status: Status {
    switch self {
    case .answered: .answered
    case .unsure: .unsure
    case .failed: .failed
    }
  }
}
extension Outcome: Equatable where A: Equatable, D: Equatable {}
extension Outcome: Hashable where A: Hashable, D: Hashable {}
extension Outcome: Codable where A: Codable, D: Codable {
  enum Keys: String, CodingKey { case status, answer, probability, detail, error }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: Keys.self)
    switch try c.decode(Status.self, forKey: .status) {
    case .answered:
      guard !c.contains(.error) else {
        throw DecodingError.dataCorruptedError(
          forKey: .error, in: c, debugDescription: "Answered outcomes cannot contain an error")
      }
      self = .answered(
        answer: try c.decode(A.self, forKey: .answer),
        probability: try c.decodeIfPresent(Double.self, forKey: .probability),
        detail: try c.decodeIfPresent(D.self, forKey: .detail))
    case .unsure:
      guard !c.contains(.answer), !c.contains(.error) else {
        throw DecodingError.dataCorruptedError(
          forKey: .status, in: c,
          debugDescription: "Unsure outcomes cannot contain an answer or error")
      }
      self = .unsure(
        probability: try c.decode(Double.self, forKey: .probability),
        detail: try c.decodeIfPresent(D.self, forKey: .detail))
    case .failed:
      guard !c.contains(.answer), !c.contains(.probability), !c.contains(.detail) else {
        throw DecodingError.dataCorruptedError(
          forKey: .status, in: c, debugDescription: "Failed outcomes cannot contain answer fields")
      }
      self = .failed(try c.decode(RowFailure.self, forKey: .error))
    }
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: Keys.self)
    try c.encode(status, forKey: .status)
    switch self {
    case .answered(let answer, let probability, let detail):
      try c.encode(answer, forKey: .answer)
      try c.encodeIfPresent(probability, forKey: .probability)
      try c.encodeIfPresent(detail, forKey: .detail)
    case .unsure(let probability, let detail):
      try c.encode(probability, forKey: .probability)
      try c.encodeIfPresent(detail, forKey: .detail)
    case .failed(let error): try c.encode(error, forKey: .error)
    }
  }
}
public protocol FunctionShape: Sendable {
  associatedtype Answer: Sendable & Codable & Equatable & Hashable
  associatedtype Detail: Sendable & Codable & Equatable & Hashable
  static var function: FunctionName { get }
}
public struct Source: Sendable, Codable, Equatable, Hashable {
  public let questionIdentity: String, backend: Backend, model: String, name: String?
  enum CodingKeys: String, CodingKey {
    case backend, model, name
    case questionIdentity = "question_identity"
  }
}
public struct Row<F: FunctionShape>: Sendable, Codable, Equatable, Hashable {
  public let position: UInt64, id: RecordID?, source: Source, outcome: Outcome<F.Answer, F.Detail>
  public var status: Status { outcome.status }
  public var function: FunctionName { F.function }
  public var answer: F.Answer? {
    if case .answered(let answer, _, _) = outcome { return .some(answer) }
    return nil
  }
  public func result() -> Result<F.Answer?, RowFailure> {
    switch outcome {
    case .answered(let answer, _, _): .success(.some(answer))
    case .unsure: .success(nil)
    case .failed(let error): .failure(error)
    }
  }
  enum Keys: String, CodingKey { case function, position, id, source }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: Keys.self)
    guard try c.decode(FunctionName.self, forKey: .function) == F.function else {
      throw DecodingError.dataCorruptedError(
        forKey: .function, in: c, debugDescription: "Function discriminator does not match row")
    }
    position = try c.decode(UInt64.self, forKey: .position)
    id = try c.decodeIfPresent(RecordID.self, forKey: .id)
    source = try c.decode(Source.self, forKey: .source)
    outcome = try Outcome(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: Keys.self)
    try c.encode(F.function, forKey: .function)
    try c.encode(position, forKey: .position)
    try c.encodeIfPresent(id, forKey: .id)
    try c.encode(source, forKey: .source)
    try outcome.encode(to: encoder)
  }
}
public enum NoDetail: Sendable, Codable, Equatable, Hashable {
  public init(from decoder: any Decoder) throws {
    throw DecodingError.dataCorrupted(
      .init(codingPath: decoder.codingPath, debugDescription: "No detail is allowed"))
  }
  public func encode(to encoder: any Encoder) throws { switch self {} }
}
public struct FilterResult: Sendable, Equatable, Hashable {
  public let item: Record, row: FilterRow
  public var kept: Bool { row.answer == true }
}
public struct TextSpan: Sendable, Codable, Equatable, Hashable {
  public let text: String
  public let start: Int
  public let end: Int
}
public struct RecognizedName: Sendable, Codable, Equatable, Hashable {
  public let text: String
  public let start: Int
  public let end: Int
  public let kind: String
  public let probability: Double
}
public struct RelationName: Sendable, Codable, Equatable, Hashable {
  public let text: String
  public let start: Int
  public let end: Int
  public let kind: String
}
public struct Relation: Sendable, Codable, Equatable, Hashable {
  public let source: RelationName
  public let target: RelationName
  public let relation: String
  public let probability: Double
  public let either: Bool?
}
public struct LabelDetail: Sendable, Codable, Equatable, Hashable {
  public let labels: [String: Double]
}
public struct LevelDetail: Sendable, Codable, Equatable, Hashable {
  public let levels: [Double]
}
public struct RelationDetail: Sendable, Codable, Equatable, Hashable {
  public let names: [RecognizedName]
  public let pairs: [Relation]
}
public struct PieceDetail: Sendable, Codable, Equatable, Hashable {
  public let text: String
  public let start: Int
  public let end: Int
  public let tags: [BoundaryTag: Double]
  enum Keys: String, CodingKey { case text, start, end, tags }
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: Keys.self)
    text = try c.decode(String.self, forKey: .text)
    start = try c.decode(Int.self, forKey: .start)
    end = try c.decode(Int.self, forKey: .end)
    let raw = try c.decode([String: Double].self, forKey: .tags)
    var typed: [BoundaryTag: Double] = [:]
    for (key, value) in raw {
      guard let tag = BoundaryTag(rawValue: key) else {
        throw DecodingError.dataCorruptedError(
          forKey: .tags, in: c, debugDescription: "Unknown boundary tag")
      }
      typed[tag] = value
    }
    tags = typed
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: Keys.self)
    try c.encode(text, forKey: .text)
    try c.encode(start, forKey: .start)
    try c.encode(end, forKey: .end)
    try c.encode(
      Dictionary(uniqueKeysWithValues: tags.map { ($0.key.rawValue, $0.value) }), forKey: .tags)
  }
}
public struct KindDetail: Sendable, Codable, Equatable, Hashable {
  public let start: Int
  public let end: Int
  public let kinds: [String: Double]
}
public struct RecognizeDetail: Sendable, Codable, Equatable, Hashable {
  public let pieces: [PieceDetail]
  public let names: [KindDetail]
}
public enum BoundaryTag: String, Sendable, Codable {
  case begin = "BEGIN"
  case inside = "INSIDE"
  case end = "END"
  case single = "SINGLE"
  case out = "OUT"
}
