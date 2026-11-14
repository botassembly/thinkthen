import Foundation

public struct RankQuestion: SavedQuestion, FunctionShape, Codable, Equatable, Hashable {
  public typealias Answer = Int
  public typealias Detail = LevelDetail
  public static let function = FunctionName.rank
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, levels: [String], descriptions: [String: String] = [:], top: Int? = nil,
    field: String? = nil
  ) {
    definition = describedDefinition(
      "rank",
      [
        "question": .string(question), "levels": choicesJSON(levels, descriptions),
        "field": json(field), "top": json(top),
      ], names: levels, descriptions: descriptions)
  }
  public init<E: RawRepresentable & CaseIterable & Hashable>(
    _ question: String, levels: E.Type, descriptions: [E: String] = [:], top: Int? = nil,
    field: String? = nil
  ) where E.RawValue == String {
    definition = describedDefinition(
      "rank",
      [
        "question": .string(question), "levels": choicesJSON(levels, descriptions),
        "field": json(field), "top": json(top),
      ], names: E.allCases.map(\.rawValue), descriptions: rawDescriptions(descriptions))
  }
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
  init(definition: JSONValue) { self.definition = definition }
}
public typealias RankRow<Item: Sendable> = Row<RankQuestion, Item>
public typealias RankRows<Item: Sendable> = Rows<RankRow<Item>>
