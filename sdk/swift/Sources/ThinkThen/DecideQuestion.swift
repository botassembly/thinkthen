import Foundation

public struct DecideQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Bool
  public typealias Detail = NoDetail
  public static let function = FunctionName.decide
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, yesMeans: String? = nil, noMeans: String? = nil,
    threshold: Threshold? = nil, field: String? = nil
  ) {
    definition = questionDefinition(
      "decide",
      [
        "question": .string(question), "yes_means": json(yesMeans), "no_means": json(noMeans),
        "threshold": json(threshold), "field": json(field),
      ])
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias DecideRow<Item: Sendable> = Row<DecideQuestion, Item>
public typealias DecideRows<Item: Sendable> = Rows<DecideRow<Item>>
