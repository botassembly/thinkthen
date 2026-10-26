import Foundation

public struct RelateOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let threshold: Double?
  public init(evidence: String? = nil, threshold: Double? = nil) {
    self.evidence = evidence
    self.threshold = threshold
  }
}
public struct RelateQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = [Relation]
  public typealias Detail = RelationDetail
  public static let function = FunctionName.relate
  public let rules: [RelationRule]
  public let options: RelateOptions
  public init(rules: [RelationRule], options: RelateOptions = .init()) {
    self.rules = rules
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case rules }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    rules = try c.decode([RelationRule].self, forKey: .rules)
    options = try RelateOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(rules, forKey: .rules)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias RelateRow = Row<RelateQuestion>
public typealias RelateRows = Rows<RelateRow>
