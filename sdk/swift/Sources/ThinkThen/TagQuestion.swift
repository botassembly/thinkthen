import Foundation

public struct TagOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Double?
  public init(evidence: String? = nil, threshold: Double? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct TagQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = [String]
  public typealias Detail = LabelDetail
  public static let function = FunctionName.tag
  public let question: String
  public let labels: [String]
  public let options: TagOptions
  public init(_ question: String, labels: [String], options: TagOptions = .init()) {
    self.question = question
    self.labels = labels
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question, labels }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    labels = try c.decode([String].self, forKey: .labels)
    options = try TagOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encode(labels, forKey: .labels)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias TagRow = Row<TagQuestion>
public typealias TagRows = Rows<TagRow>
