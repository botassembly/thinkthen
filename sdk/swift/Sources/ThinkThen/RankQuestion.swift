import Foundation

/// Field selection and options for this function.
public struct RankOptions: Sendable, Codable, Equatable, Hashable {
  /// A JSON pointer selecting the field, or nil to use the whole record.
  public let field: String?
  /// The number of highest-ranked records to retain; nil retains every rank.
  public let top: Int?
  /// Create this value from its typed fields.
  public init(field: String? = nil, top: Int? = nil) {
    self.field = field
    self.top = top
  }
}
/// A typed question that can be saved and reused for this function.
public struct RankQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = Int
  /// The probability detail returned by this function.
  public typealias Detail = LevelDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.rank
  /// The question supplied to the engine.
  public let question: String
  /// The ordered score or rank levels, with optional descriptions.
  public let levels: [Choice]
  /// The field selection and function-specific options for this question.
  public let options: RankOptions
  /// Create this value from its typed fields.
  public init(_ question: String, levels: [Choice], options: RankOptions = .init()) {
    self.question = question
    self.levels = levels
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question, levels }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    levels = try c.decode([Choice].self, forKey: .levels)
    options = try RankOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encode(levels, forKey: .levels)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias RankRow = Row<RankQuestion>
/// The lazy stream of typed results for this function.
public typealias RankRows = Rows<RankResult>
