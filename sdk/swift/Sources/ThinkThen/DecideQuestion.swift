import Foundation

public struct DecideOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Threshold?
  public init(evidence: String? = nil, threshold: Threshold? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct DecideQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Bool
  public typealias Detail = NoDetail
  public static let function = FunctionName.decide
  public let question: String
  public let options: DecideOptions
  public init(_ question: String, options: DecideOptions = .init()) {
    self.question = question
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    options = try DecideOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias DecideRow = Row<DecideQuestion>
public typealias DecideRows = Rows<DecideRow>
