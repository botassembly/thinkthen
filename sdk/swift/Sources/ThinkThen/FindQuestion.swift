import Foundation

/// Field selection and options for this function.
public struct FindOptions: Sendable, Codable, Equatable, Hashable {
  /// A JSON pointer selecting the field, or nil to use the whole record.
  public let field: String?
  /// The threshold used to decide whether the engine is sure enough.
  public let threshold: Double?
  /// Create this value from its typed fields.
  public init(field: String? = nil, threshold: Double? = nil) {
    self.field = field
    self.threshold = threshold
  }
}
/// A typed question that can be saved and reused for this function.
public struct FindQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = TextSpan?
  /// The probability detail returned by this function.
  public typealias Detail = NoDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.find
  /// The question supplied to the engine.
  public let question: String
  /// The field selection and function-specific options for this question.
  public let options: FindOptions
  /// Create this value from its typed fields.
  public init(_ question: String, options: FindOptions = .init()) {
    self.question = question
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    options = try FindOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias FindRow = Row<FindQuestion>
/// The lazy stream of typed results for this function.
public typealias FindRows = Rows<FindRow>
