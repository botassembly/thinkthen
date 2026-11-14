import Foundation

public struct RelateQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = [Relation]
  public typealias Detail = RelationDetail
  public static let function = FunctionName.relate
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(rules: [RelationRule], threshold: Double? = nil, field: String? = nil) {
    definition = questionDefinition(
      "relate",
      [
        "rules": .array(rules.map(\.definition)), "threshold": json(threshold),
        "field": json(field),
      ])
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias RelateRow<Item: Sendable> = Row<RelateQuestion, Item>
public typealias RelateRows<Item: Sendable> = Rows<RelateRow<Item>>
