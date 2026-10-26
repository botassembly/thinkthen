import Foundation

public struct ChooseOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Threshold?
  public init(evidence: String? = nil, threshold: Threshold? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct ChooseQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = String
  public typealias Detail = LabelDetail
  public static let function = FunctionName.choose
  public let question: String
  public let choices: [String]
  public let options: ChooseOptions
  public init(_ question: String, choices: [String], options: ChooseOptions = .init()) {
    self.question = question
    self.choices = choices
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question, choices }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    choices = try c.decode([String].self, forKey: .choices)
    options = try ChooseOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encode(choices, forKey: .choices)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias ChooseRow = Row<ChooseQuestion>
public typealias ChooseRows = Rows<ChooseRow>
