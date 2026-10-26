import Foundation

public struct FilterOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Threshold?
  public init(evidence: String? = nil, threshold: Threshold? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct FilterQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Bool
  public typealias Detail = NoDetail
  public static let function = FunctionName.filter
  public let question: String
  public let options: FilterOptions
  public init(_ question: String, options: FilterOptions = .init()) {
    self.question = question
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    options = try FilterOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias FilterRow = Row<FilterQuestion>
public typealias FilterRows = Rows<FilterResult>
