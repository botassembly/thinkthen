import Foundation

/// Field selection and options for this function.
public struct DecideOptions: Sendable, Codable, Equatable, Hashable {
  /// A JSON pointer selecting the field, or nil to use the whole record.
  public let field: String?
  /// The threshold used to decide whether the engine is sure enough.
  public let threshold: Threshold?
  /// Create this value from its typed fields.
  public init(field: String? = nil, threshold: Threshold? = nil) {
    self.field = field
    self.threshold = threshold
  }
}
/// A typed question that can be saved and reused for this function.
public struct DecideQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = Bool
  /// The probability detail returned by this function.
  public typealias Detail = NoDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.decide
  /// The question supplied to the engine.
  public let question: String
  /// The meaning attached to a yes answer, when supplied.
  public let yesMeans: String?
  /// The meaning attached to a no answer, when supplied.
  public let noMeans: String?
  /// The field selection and function-specific options for this question.
  public let options: DecideOptions
  /// Create this value from its typed fields.
  public init(
    _ question: String, yesMeans: String? = nil, noMeans: String? = nil,
    options: DecideOptions = .init()
  ) {
    self.question = question
    self.yesMeans = yesMeans
    self.noMeans = noMeans
    self.options = options
  }
  enum CodingKeys: String, CodingKey {
    case question
    case yesMeans = "true"
    case noMeans = "false"
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    yesMeans = try c.decodeIfPresent(String.self, forKey: .yesMeans)
    noMeans = try c.decodeIfPresent(String.self, forKey: .noMeans)
    options = try DecideOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encodeIfPresent(yesMeans, forKey: .yesMeans)
    try c.encodeIfPresent(noMeans, forKey: .noMeans)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias DecideRow = Row<DecideQuestion>
/// The lazy stream of typed results for this function.
public typealias DecideRows = Rows<DecideRow>
