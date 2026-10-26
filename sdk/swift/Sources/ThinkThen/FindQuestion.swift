import Foundation

public struct FindOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Double?
  public init(evidence: String? = nil, threshold: Double? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct FindQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = TextSpan?
  public typealias Detail = NoDetail
  public static let function = FunctionName.find
  public let question: String
  public let options: FindOptions
  public init(_ question: String, options: FindOptions = .init()) {
    self.question = question
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    options = try FindOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias FindRow = Row<FindQuestion>
public typealias FindRows = Rows<FindRow>
