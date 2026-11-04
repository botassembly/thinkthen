import Foundation

/// The typed Status value exchanged with the engine.
public enum Status: String, Sendable, Codable {
  /// The engine returned an answer.
  case answered
  /// The engine could not choose an answer at the requested threshold.
  case unsure
  /// The record failed and carries a stable diagnostic.
  case failed
}
/// A failed record with the engine's stable code and readable diagnostic.
public struct RowFailure: LocalizedError, Sendable, Codable, Equatable, Hashable,
  CustomStringConvertible
{
  /// The stable engine diagnostic name.
  public let code: ErrorCode, message: String, httpStatus: Int?
  /// Every failed filter row, in input order, after the kept records have been delivered.
  public let failedRows: [FilterRow]
  /// Create this value from its typed fields.
  public init(code: ErrorCode, message: String, httpStatus: Int?, failedRows: [FilterRow] = []) {
    self.code = code
    self.message = message
    self.httpStatus = httpStatus
    self.failedRows = failedRows
  }
  /// The readable diagnostic shown by localizedDescription.
  public var errorDescription: String? { message }
  /// A readable value summary; credentials remain redacted.
  public var description: String { "\(code.rawValue): \(message)" }
  enum CodingKeys: String, CodingKey {
    case code, message
    case httpStatus = "http_status"
    case failedRows = "failed_rows"
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: CodingKeys.self)
    self.init(
      code: try c.decode(ErrorCode.self, forKey: .code),
      message: try c.decode(String.self, forKey: .message),
      httpStatus: try c.decodeIfPresent(Int.self, forKey: .httpStatus),
      failedRows: try c.decodeIfPresent([FilterRow].self, forKey: .failedRows) ?? [])
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(code, forKey: .code)
    try c.encode(message, forKey: .message)
    try c.encodeIfPresent(httpStatus, forKey: .httpStatus)
    if !failedRows.isEmpty { try c.encode(failedRows, forKey: .failedRows) }
  }

}
/// An answer could not be read because the engine was unsure or the record failed.
public enum RowError: LocalizedError, Sendable, Equatable {
  /// The engine is not sure; the associated probability remains available.
  case unsure(probability: Double)
  /// The record failed and carries the engine diagnostic.
  case failed(RowFailure)
  /// The readable diagnostic shown by localizedDescription.
  public var errorDescription: String? {
    switch self {
    case .unsure: "The answer is not sure"
    case .failed(let failure): failure.message
    }
  }
}
/// The typed Outcome value exchanged with the engine.
public enum Outcome<A: Sendable, D: Sendable>: Sendable {
  /// An answer with its optional probability and detail.
  case answered(answer: A, probability: Double?, detail: D?)
  /// The engine is not sure; the associated probability remains available.
  case unsure(probability: Double, detail: D?)
  /// The record failed and carries the engine diagnostic.
  case failed(RowFailure)
  /// Return the typed answer, throwing RowError for unsure and failed outcomes.
  public func get() throws -> A { try result().get() }
  /// Return the answer or a RowError, preserving unsure as an error.
  public func result() -> Result<A, RowError> {
    switch self {
    case .answered(let answer, _, _): .success(answer)
    case .unsure(let probability, _): .failure(.unsure(probability: probability))
    case .failed(let failure): .failure(.failed(failure))
    }
  }
  /// The answer probability; nil when the function does not report one or the record failed.
  public var probability: Double? {
    switch self {
    case .answered(_, let probability, _): probability
    case .unsure(let probability, _): probability
    case .failed: nil
    }
  }
  /// The function-specific detail, when the engine supplies it.
  public var detail: D? {
    switch self {
    case .answered(_, _, let detail), .unsure(_, let detail): detail
    case .failed: nil
    }
  }
  /// The record failure, or nil when the outcome did not fail.
  public var failure: RowFailure? {
    if case .failed(let failure) = self { return failure }
    return nil
  }
  /// Whether the record was answered, was unsure or failed.
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
  /// Decode this value from its canonical JSON representation.
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
  /// Encode this value in its canonical JSON representation.
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
/// The typed FunctionShape value exchanged with the engine.
public protocol FunctionShape: Sendable {
  /// The plain answer returned by get().
  associatedtype Answer: Sendable & Codable & Equatable & Hashable
  /// The function-specific probability detail.
  associatedtype Detail: Sendable & Codable & Equatable & Hashable
  /// The engine function associated with this shape.
  static var function: FunctionName { get }
}
/// The typed Source value exchanged with the engine.
public struct Source: Sendable, Codable, Equatable, Hashable {
  /// The engine identity for this question and its supplied name.
  public let questionIdentity: String, backend: Backend, model: String, name: String?
  enum CodingKeys: String, CodingKey {
    case backend, model, name
    case questionIdentity = "question_identity"
  }
}
/// The typed Row value exchanged with the engine.
public struct Row<F: FunctionShape>: Sendable, Codable, Equatable, Hashable {
  /// The one-based position in the input, preserved when rank reorders records.
  public let position: Int, id: RecordID?, source: Source, outcome: Outcome<F.Answer, F.Detail>
  /// Whether the record was answered, was unsure or failed.
  public var status: Status { outcome.status }
  /// The function this question or result belongs to.
  public var function: FunctionName { F.function }
  /// The answered value, or nil for unsure and failed outcomes. Use get() to distinguish them.
  public var answer: F.Answer? {
    if case .answered(let answer, _, _) = outcome { return .some(answer) }
    return nil
  }
  /// Return the answer, throwing for not sure and failed records.
  public func get() throws -> F.Answer { try result().get() }
  /// Return the answer or a RowError, preserving unsure as an error.
  public func result() -> Result<F.Answer, RowError> { outcome.result() }
  /// The answer probability; nil when the function does not report one or the record failed.
  public var probability: Double? { outcome.probability }
  /// The function-specific detail, when the engine supplies it.
  public var detail: F.Detail? { outcome.detail }
  /// The record failure, or nil when the outcome did not fail.
  public var failure: RowFailure? { outcome.failure }
  /// Whether the engine could not choose an answer at the requested threshold.
  public var isUnsure: Bool { status == .unsure }
  /// Whether the record failed to produce an answer.
  public var isFailed: Bool { status == .failed }
  enum Keys: String, CodingKey { case function, position, id, source }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let c = try decoder.container(keyedBy: Keys.self)
    guard try c.decode(FunctionName.self, forKey: .function) == F.function else {
      throw DecodingError.dataCorruptedError(
        forKey: .function, in: c, debugDescription: "Function discriminator does not match row")
    }
    position = try c.decode(Int.self, forKey: .position)
    id = try c.decodeIfPresent(RecordID.self, forKey: .id)
    source = try c.decode(Source.self, forKey: .source)
    outcome = try Outcome(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: Keys.self)
    try c.encode(F.function, forKey: .function)
    try c.encode(position, forKey: .position)
    try c.encodeIfPresent(id, forKey: .id)
    try c.encode(source, forKey: .source)
    try outcome.encode(to: encoder)
  }
}
/// The typed NoDetail value exchanged with the engine.
public enum NoDetail: Sendable, Codable, Equatable, Hashable {
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    throw DecodingError.dataCorrupted(
      .init(codingPath: decoder.codingPath, debugDescription: "No detail is allowed"))
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws { switch self {} }
}
/// A result retaining the input value; rank owns an equal snapshot after reordering.
public struct RecordResult<F: FunctionShape>: Sendable, Codable, Equatable, Hashable {
  /// The original input value. Rank uses an equal row-owned copy, not caller object identity.
  public let item: Record
  /// The typed engine result for the original record.
  public let row: Row<F>
  /// The one-based position in the input, preserved when rank reorders records.
  public var position: Int { row.position }
  /// The identifier supplied with the record, when present.
  public var id: RecordID? { row.id }
  /// The question identity and backend that produced this result.
  public var source: Source { row.source }
  /// The answered, unsure or failed outcome, with its function-specific detail.
  public var outcome: Outcome<F.Answer, F.Detail> { row.outcome }
  /// Whether the record was answered, was unsure or failed.
  public var status: Status { row.status }
  /// The function this question or result belongs to.
  public var function: FunctionName { row.function }
  /// The answered value, or nil for unsure and failed outcomes. Use get() to distinguish them.
  public var answer: F.Answer? { row.answer }
  /// The answer probability; nil when the function does not report one or the record failed.
  public var probability: Double? { row.probability }
  /// The function-specific detail, when the engine supplies it.
  public var detail: F.Detail? { row.detail }
  /// The record failure, or nil when the outcome did not fail.
  public var failure: RowFailure? { row.failure }
  /// Whether the engine could not choose an answer at the requested threshold.
  public var isUnsure: Bool { row.isUnsure }
  /// Whether the record failed to produce an answer.
  public var isFailed: Bool { row.isFailed }
  /// Return the typed answer, throwing RowError for unsure and failed outcomes.
  public func get() throws -> F.Answer { try row.get() }
  /// Return the answer or a RowError, preserving unsure as an error.
  public func result() -> Result<F.Answer, RowError> { row.result() }
}
/// The FilterResult representation.
public typealias FilterResult = RecordResult<FilterQuestion>
/// The RankResult representation.
public typealias RankResult = RecordResult<RankQuestion>
extension RecordResult where F == FilterQuestion {
  /// Whether this answered filter row matched the question.
  public var kept: Bool { row.answer == true }
}
/// The typed TextSpan value exchanged with the engine.
public struct TextSpan: Sendable, Codable, Equatable, Hashable {
  /// The matched text for these code-point offsets.
  public let text: String
  /// The inclusive start offset, counted in Unicode code points.
  public let start: Int
  /// The exclusive end offset, counted in Unicode code points.
  public let end: Int
}
/// The typed RecognizedName value exchanged with the engine.
public struct RecognizedName: Sendable, Codable, Equatable, Hashable {
  /// The matched text for these code-point offsets.
  public let text: String
  /// The inclusive start offset, counted in Unicode code points.
  public let start: Int
  /// The exclusive end offset, counted in Unicode code points.
  public let end: Int
  /// The recognized kind, or the local error category.
  public let kind: String
  /// The answer probability; nil when the function does not report one or the record failed.
  public let probability: Double
}
/// The typed RelationName value exchanged with the engine.
public struct RelationName: Sendable, Codable, Equatable, Hashable {
  /// The matched text for these code-point offsets.
  public let text: String
  /// The inclusive start offset, counted in Unicode code points.
  public let start: Int
  /// The exclusive end offset, counted in Unicode code points.
  public let end: Int
  /// The recognized kind, or the local error category.
  public let kind: String
}
/// The typed Relation value exchanged with the engine.
public struct Relation: Sendable, Codable, Equatable, Hashable {
  /// The source name in this relationship.
  public let source: RelationName
  /// The target value.
  public let target: RelationName
  /// The relationship label.
  public let relation: String
  /// The answer probability; nil when the function does not report one or the record failed.
  public let probability: Double
  /// Whether the relation may be read in either direction.
  public let either: Bool?
}
/// The typed LabelDetail value exchanged with the engine.
public struct LabelDetail: Sendable, Codable, Equatable, Hashable {
  /// Each allowed label mapped to its probability.
  public let labels: [String: Double]
}
/// The typed LevelDetail value exchanged with the engine.
public struct LevelDetail: Sendable, Codable, Equatable, Hashable {
  /// The level probabilities in the question's level order.
  public let levels: [Double]
}
/// The typed RelationDetail value exchanged with the engine.
public struct RelationDetail: Sendable, Codable, Equatable, Hashable {
  /// The recognized name spans and their classification detail.
  public let names: [RecognizedName]
  /// The relations between the recognized names.
  public let pairs: [Relation]
}
/// The typed PieceDetail value exchanged with the engine.
public struct PieceDetail: Sendable, Codable, Equatable, Hashable {
  /// The matched text for these code-point offsets.
  public let text: String
  /// The inclusive start offset, counted in Unicode code points.
  public let start: Int
  /// The exclusive end offset, counted in Unicode code points.
  public let end: Int
  /// The tags value.
  public let tags: [BoundaryTag: Double]
  enum Keys: String, CodingKey { case text, start, end, tags }
  /// Decode this value from its canonical JSON representation.
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
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: Keys.self)
    try c.encode(text, forKey: .text)
    try c.encode(start, forKey: .start)
    try c.encode(end, forKey: .end)
    try c.encode(
      Dictionary(uniqueKeysWithValues: tags.map { ($0.key.rawValue, $0.value) }), forKey: .tags)
  }
}
/// The typed KindDetail value exchanged with the engine.
public struct KindDetail: Sendable, Codable, Equatable, Hashable {
  /// The matched text for these code-point offsets.
  public let text: String
  /// The inclusive start offset, counted in Unicode code points.
  public let start: Int
  /// The exclusive end offset, counted in Unicode code points.
  public let end: Int
  /// The recognized kinds and their probabilities, or the requested kind names.
  public let kinds: [String: Double]
}
/// The typed RecognizeDetail value exchanged with the engine.
public struct RecognizeDetail: Sendable, Codable, Equatable, Hashable {
  /// The token spans and their boundary probabilities.
  public let pieces: [PieceDetail]
  /// The recognized name spans and their classification detail.
  public let names: [KindDetail]
}
/// The typed BoundaryTag value exchanged with the engine.
public enum BoundaryTag: String, Sendable, Codable {
  /// The first token in a multi-token name.
  case begin = "BEGIN"
  /// An interior token in a multi-token name.
  case inside = "INSIDE"
  /// The last token in a multi-token name.
  case end = "END"
  /// A name consisting of one token.
  case single = "SINGLE"
  /// A token outside a recognized name.
  case out = "OUT"
}

/// Convert code-point offsets into indexes in the supplied source string.
extension TextSpan {
  /// Map Unicode code-point offsets to String.Index values, or nil if out of bounds.
  public func range(in text: String) -> Range<String.Index>? {
    guard start >= 0, end >= start,
      let lower = text.unicodeScalars.index(
        text.unicodeScalars.startIndex, offsetBy: start, limitedBy: text.unicodeScalars.endIndex),
      let upper = text.unicodeScalars.index(
        lower, offsetBy: end - start, limitedBy: text.unicodeScalars.endIndex)
    else { return nil }
    return lower..<upper
  }
}
extension Row: CustomStringConvertible {
  /// A readable value summary; credentials remain redacted.
  public var description: String {
    switch outcome {
    case .answered(let answer, let probability, _):
      "\(F.function.rawValue)(\(String(describing: answer)), probability: \(probability.map { String($0) } ?? "nil"))"
    case .unsure(let probability, _): "\(F.function.rawValue)(unsure, probability: \(probability))"
    case .failed(let failure): "\(F.function.rawValue)(\(failure))"
    }
  }
}
