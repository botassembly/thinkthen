import Foundation

/// Field selection and options for this function.
public struct ChooseOptions: Sendable, Codable, Equatable, Hashable {
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
public struct ChooseQuestion: FunctionShape, Codable, Equatable, Hashable {
  /// The plain answer returned by this function.
  public typealias Answer = String
  /// The probability detail returned by this function.
  public typealias Detail = LabelDetail
  /// The function this question or result belongs to.
  public static let function = FunctionName.choose
  /// The question supplied to the engine.
  public let question: String
  /// The allowed answer labels and their optional descriptions.
  public let choices: [Choice]
  /// The field selection and function-specific options for this question.
  public let options: ChooseOptions
  /// Create this value from its typed fields.
  public init(_ question: String, choices: [Choice], options: ChooseOptions = .init()) {
    self.question = question
    self.choices = choices
    self.options = options
  }
  /// Use every case of a String-backed enum as a choice label.
  public init<C: RawRepresentable & CaseIterable>(
    _ question: String, choices: C.Type,
    options: ChooseOptions = .init()
  ) where C.RawValue == String {
    self.init(question, choices: C.allCases.map { Choice($0.rawValue) }, options: options)
  }
  enum CodingKeys: String, CodingKey { case question, choices }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    choices = try c.decode([Choice].self, forKey: .choices)
    options = try ChooseOptions(from: decoder)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encode(choices, forKey: .choices)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
/// The typed result for one record.
public typealias ChooseRow = Row<ChooseQuestion>
/// The lazy stream of typed results for this function.
public typealias ChooseRows = Rows<ChooseRow>
