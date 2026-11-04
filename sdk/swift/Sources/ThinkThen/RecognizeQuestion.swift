import Foundation

/// Field selection and options for this function.
public struct RecognizeOptions: Sendable, Codable, Equatable, Hashable {
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
public struct RecognizeQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = [RecognizedName]
  /// The probability detail returned by this function.
  public typealias Detail = RecognizeDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.recognize
  /// The recognized kinds and their probabilities, or the requested kind names.
  public let kinds: [String]
  /// The field selection and function-specific options for this question.
  public let options: RecognizeOptions
  /// Create this value from its typed fields.
  public init(kinds: [String], options: RecognizeOptions = .init()) {
    self.kinds = kinds
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case kinds }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    kinds = try c.decode([String].self, forKey: .kinds)
    options = try RecognizeOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(kinds, forKey: .kinds)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias RecognizeRow = Row<RecognizeQuestion>
/// The lazy stream of typed results for this function.
public typealias RecognizeRows = Rows<RecognizeRow>
