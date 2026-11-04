import Foundation

/// Field selection and options for this function.
public struct AnnotateOptions: Sendable, Codable, Equatable, Hashable {
  /// A JSON pointer selecting the field, or nil to use the whole record.
  public let field: String?
  /// Create this value from its typed fields.
  public init(field: String? = nil) {
    self.field = field
  }
}
/// A typed question that can be saved and reused for this function.
public struct AnnotateQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = Annotations
  /// The probability detail returned by this function.
  public typealias Detail = NoDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.annotate
  /// The saved questions indexed by their entry names.
  public let questions: AnnotationQuestions
  /// The field selection and function-specific options for this question.
  public let options: AnnotateOptions
  /// Create this value from its typed fields.
  public init(_ questions: AnnotationQuestions, options: AnnotateOptions = .init()) {
    self.questions = questions
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case questions }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    questions = try c.decode(AnnotationQuestions.self, forKey: .questions)
    options = try AnnotateOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(questions, forKey: .questions)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias AnnotateRow = Row<AnnotateQuestion>
/// The lazy stream of typed results for this function.
public typealias AnnotateRows = Rows<AnnotateRow>
