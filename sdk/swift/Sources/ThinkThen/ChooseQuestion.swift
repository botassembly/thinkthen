import Foundation

public struct ChooseQuestion<ChoiceValue: Sendable>: SavedQuestion, FunctionShape, Equatable,
  Hashable
{
  public typealias Answer = ChoiceValue
  public typealias Detail = LabelDetail
  public static var function: FunctionName { .choose }
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, choices: ChoiceValue.Type, descriptions: [ChoiceValue: String] = [:],
    threshold: Double? = nil, field: String? = nil
  ) where ChoiceValue: RawRepresentable & CaseIterable & Hashable, ChoiceValue.RawValue == String {
    definition = describedDefinition(
      "choose",
      [
        "question": .string(question), "choices": choicesJSON(ChoiceValue.self, descriptions),
        "threshold": json(threshold), "field": json(field),
      ], names: ChoiceValue.allCases.map(\.rawValue), descriptions: rawDescriptions(descriptions))
  }
  public init(
    _ question: String, choices: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil
  ) where ChoiceValue == String {
    definition = describedDefinition(
      "choose",
      [
        "question": .string(question), "choices": choicesJSON(choices, descriptions),
        "threshold": json(threshold), "field": json(field),
      ], names: choices, descriptions: descriptions)
  }
  init(definition: JSONValue) { self.definition = definition }
  private static func convert(_ raw: String) throws -> ChoiceValue {
    if let string = raw as? ChoiceValue { return string }
    if let type = ChoiceValue.self as? any RawRepresentable.Type,
      let value = rawValue(type, raw) as? ChoiceValue
    {
      return value
    }
    throw InternalError(code: .internalBug, message: "Answer is outside the enum's choices")
  }
  public static func decodeAnswer(_ data: Data, using decoder: JSONDecoder) throws -> Answer {
    return try convert(decoder.decode(String.self, from: data))
  }
}
extension ChooseQuestion: Decodable where ChoiceValue == String {
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
}
public typealias ChooseRow<ChoiceValue: Sendable, Item: Sendable> = Row<
  ChooseQuestion<ChoiceValue>, Item
>
public typealias ChooseRows<ChoiceValue: Sendable, Item: Sendable> = Rows<
  ChooseRow<ChoiceValue, Item>
>
