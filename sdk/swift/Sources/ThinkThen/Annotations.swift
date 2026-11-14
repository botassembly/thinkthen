import Foundation

/// A form's entries. Each selector returns an ordinary typed result.
public struct Annotations<Item: Sendable>: Sendable, Codable, Equatable, Hashable {
  private let entries: [String: JSONValue]
  private let definitions: [String: JSONValue]
  private let entryRows: [String: JSONValue]
  private let item: Item
  private let texts: [String: String]
  public init(from decoder: any Decoder) throws {
    entries = try decoder.singleValueContainer().decode([String: JSONValue].self)
    guard let original = decoder.userInfo[.originalItem] as? Item,
      let parent = decoder.userInfo[.parentRow] as? JSONValue
    else {
      throw CallError.contract(message: "Form has no original record or parent metadata")
    }
    item = original
    // Ordinary entries expose inherited position/id/source as their own fields.
    // Freeze that same expanded representation for lookup, equality and hashing.
    entryRows = entries.mapValues { expandedEntry($0, parent: parent) }
    definitions = (decoder.userInfo[.questionDefinitions] as? JSONValue)?.object ?? [:]
    texts = decoder.userInfo[.selectedTexts] as? [String: String] ?? [:]
  }
  private func select<F: FunctionShape>(_ name: String, type: F.Type) throws -> Row<F, Item> {
    guard let entry = exactValue(entryRows, name),
      entry.object["function"]?.string == F.function.rawValue
    else {
      throw UsageError(message: "Form entry \(name) must use \(F.function.rawValue)")
    }
    let decoder = JSONDecoder()
    decoder.userInfo[.originalItem] = item
    decoder.userInfo[.parentRow] = entry
    decoder.userInfo[.sourceText] = exactValue(texts, name) ?? ""
    return try Row(saved: entry, item: item, decoder: decoder)
  }
  public func decide(_ name: String) throws -> DecideRow<Item> {
    try select(name, type: DecideQuestion.self)
  }
  public func find(_ name: String) throws -> FindRow<Item> {
    try select(name, type: FindQuestion.self)
  }
  public func recognize(_ name: String) throws -> RecognizeRow<Item> {
    try select(name, type: RecognizeQuestion.self)
  }
  public func relate(_ name: String) throws -> RelateRow<Item> {
    try select(name, type: RelateQuestion.self)
  }
  public func score(_ name: String) throws -> ScoreRow<Item> {
    try select(name, type: ScoreQuestion.self)
  }
  public func choose(_ name: String) throws -> ChooseRow<String, Item> {
    try select(name, type: ChooseQuestion<String>.self)
  }
  public func choose<E: RawRepresentable & CaseIterable & Hashable & Sendable>(
    _ name: String, choices: E.Type
  ) throws -> ChooseRow<E, Item> where E.RawValue == String {
    guard let definition = exactValue(definitions, name) else {
      throw UsageError(message: "Form entry \(name) has no question definition")
    }
    try checkDomain(definition, key: "choices", type: E.self, name: name)
    return try select(name, type: ChooseQuestion<E>.self)
  }
  public func tag(_ name: String) throws -> TagRow<String, Item> {
    try select(name, type: TagQuestion<String>.self)
  }
  public func tag<E: RawRepresentable & CaseIterable & Hashable & Sendable>(
    _ name: String, labels: E.Type
  ) throws -> TagRow<E, Item> where E.RawValue == String {
    guard let definition = exactValue(definitions, name) else {
      throw UsageError(message: "Form entry \(name) has no question definition")
    }
    try checkDomain(definition, key: "labels", type: E.self, name: name)
    return try select(name, type: TagQuestion<E>.self)
  }
  public func encode(to encoder: any Encoder) throws { try entries.encode(to: encoder) }
  public static func == (lhs: Self, rhs: Self) -> Bool { lhs.entryRows == rhs.entryRows }
  public func hash(into hasher: inout Hasher) { hasher.combine(entryRows) }
}
