import Foundation

public struct FilterQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Bool
  public typealias Detail = NoDetail
  public static let function = FunctionName.filter
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, yesMeans: String? = nil, noMeans: String? = nil, threshold: Double? = nil,
    field: String? = nil
  ) {
    definition = questionDefinition(
      "filter",
      [
        "question": .string(question), "yes_means": json(yesMeans), "no_means": json(noMeans),
        "threshold": json(threshold), "field": json(field),
      ])
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
