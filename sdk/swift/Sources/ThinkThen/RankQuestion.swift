import Foundation

public struct RankOptions: Sendable, Codable, Equatable, Hashable {
  public let evidence: String?
  public let top: Int?
  public init(evidence: String? = nil, top: Int? = nil) {
    self.evidence = evidence
    self.top = top
  }
}
public struct RankQuestion: FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Int
  public typealias Detail = LevelDetail
  public static let function = FunctionName.rank
  public let question: String
  public let levels: [String]
  public let options: RankOptions
  public init(_ question: String, levels: [String], options: RankOptions = .init()) {
    self.question = question
    self.levels = levels
    self.options = options
  }
  enum CodingKeys: String, CodingKey { case question, levels }
  public init(from decoder: any Decoder) throws {
    try checkFunction(Self.function, from: decoder)
    let c = try decoder.container(keyedBy: CodingKeys.self)
    question = try c.decode(String.self, forKey: .question)
    levels = try c.decode([String].self, forKey: .levels)
    options = try RankOptions(from: decoder)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: CodingKeys.self)
    try c.encode(question, forKey: .question)
    try c.encode(levels, forKey: .levels)
    try options.encode(to: encoder)
    var tag = encoder.container(keyedBy: FunctionKey.self)
    try tag.encode(Self.function, forKey: .function)
  }
}
public typealias RankRow = Row<RankQuestion>
public typealias RankRows = Rows<RankRow>
