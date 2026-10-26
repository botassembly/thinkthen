import Foundation

enum FunctionKey: String, CodingKey { case function }
public struct RelationRule: Sendable, Codable, Equatable, Hashable {
  public let source: String, target: String, relation: String, reads: String?, either: Bool?
  public init(
    source: String, target: String, relation: String, reads: String? = nil, either: Bool? = nil
  ) {
    self.source = source
    self.target = target
    self.relation = relation
    self.reads = reads
    self.either = either
  }
}

func checkFunction(_ function: FunctionName, from decoder: any Decoder) throws {
  let c = try decoder.container(keyedBy: FunctionKey.self)
  if let actual = try c.decodeIfPresent(FunctionName.self, forKey: .function), actual != function {
    throw DecodingError.dataCorruptedError(
      forKey: .function, in: c, debugDescription: "Question function does not match its type")
  }
}
