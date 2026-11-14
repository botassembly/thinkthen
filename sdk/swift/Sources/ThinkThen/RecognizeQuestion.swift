import Foundation

public struct RecognizeQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = [RecognizedName]
  public typealias Detail = RecognizeDetail
  public static let function = FunctionName.recognize
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(kinds: [String], threshold: Double? = nil, field: String? = nil) {
    definition = questionDefinition(
      "recognize",
      [
        "kinds": .array(kinds.map(JSONValue.string)), "threshold": json(threshold),
        "field": json(field),
      ])
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias RecognizeRow<Item: Sendable> = Row<RecognizeQuestion, Item>
public typealias RecognizeRows<Item: Sendable> = Rows<RecognizeRow<Item>>
