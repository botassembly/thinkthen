import Foundation

public struct ScoreQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Double
  public typealias Detail = LevelDetail
  public static let function = FunctionName.score
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, levels: [String], descriptions: [String: String] = [:], field: String? = nil
  ) {
    definition = describedDefinition(
      "score",
      [
        "question": .string(question), "levels": choicesJSON(levels, descriptions),
        "field": json(field),
      ], names: levels, descriptions: descriptions)
  }
  public init<E: RawRepresentable & CaseIterable & Hashable>(
    _ question: String, levels: E.Type, descriptions: [E: String] = [:], field: String? = nil
  ) where E.RawValue == String {
    definition = describedDefinition(
      "score",
      [
        "question": .string(question), "levels": choicesJSON(levels, descriptions),
        "field": json(field),
      ], names: E.allCases.map(\.rawValue), descriptions: rawDescriptions(descriptions))
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias ScoreRow<Item: Sendable> = Row<ScoreQuestion, Item>
public typealias ScoreRows<Item: Sendable> = Rows<ScoreRow<Item>>
