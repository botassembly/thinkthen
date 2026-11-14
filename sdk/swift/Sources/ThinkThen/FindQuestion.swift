import Foundation

public struct FindQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = TextSpan?
  public typealias Detail = NoDetail
  public static let function = FunctionName.find
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(_ question: String, threshold: Double? = nil, field: String? = nil) {
    definition = questionDefinition(
      "find", ["question": .string(question), "threshold": json(threshold), "field": json(field)])
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias FindRow<Item: Sendable> = Row<FindQuestion, Item>
public typealias FindRows<Item: Sendable> = Rows<FindRow<Item>>
