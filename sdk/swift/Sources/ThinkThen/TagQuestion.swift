import Foundation

public struct TagQuestion<ChoiceValue: Sendable>: SavedQuestion, FunctionShape, Equatable, Hashable
{
  public typealias Answer = [ChoiceValue]
  public typealias Detail = LabelDetail
  public static var function: FunctionName { .tag }
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
  public let definition: JSONValue
  public init(
    _ question: String, labels: ChoiceValue.Type, descriptions: [ChoiceValue: String] = [:],
    threshold: Double? = nil, field: String? = nil
  ) where ChoiceValue: RawRepresentable & CaseIterable & Hashable, ChoiceValue.RawValue == String {
    definition = describedDefinition(
      "tag",
      [
        "question": .string(question), "labels": choicesJSON(ChoiceValue.self, descriptions),
        "threshold": json(threshold), "field": json(field),
      ], names: ChoiceValue.allCases.map(\.rawValue), descriptions: rawDescriptions(descriptions))
  }
  public init(
    _ question: String, labels: [String], descriptions: [String: String] = [:],
    threshold: Double? = nil, field: String? = nil
  ) where ChoiceValue == String {
    definition = describedDefinition(
      "tag",
      [
        "question": .string(question), "labels": choicesJSON(labels, descriptions),
        "threshold": json(threshold), "field": json(field),
      ], names: labels, descriptions: descriptions)
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
    return try decoder.decode([String].self, from: data).map(convert)
  }
}
extension TagQuestion: Decodable where ChoiceValue == String {
  public init(from decoder: any Decoder) throws { definition = try JSONValue(from: decoder) }
}
public typealias TagRow<ChoiceValue: Sendable, Item: Sendable> = Row<TagQuestion<ChoiceValue>, Item>
public typealias TagRows<ChoiceValue: Sendable, Item: Sendable> = Rows<TagRow<ChoiceValue, Item>>
