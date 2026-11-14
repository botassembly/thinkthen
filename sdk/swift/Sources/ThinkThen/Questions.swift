import Foundation

/// The typed RelationRule value exchanged with the engine.
public struct RelationRule: Sendable, Codable, Equatable, Hashable {
  /// The source kind, target kind, relationship name, and optional direction description.
  public let source: String, target: String, relation: String, reads: String?, either: Bool?
  /// Create this value from its typed fields.
  public init(
    source: String, target: String, relation: String, reads: String? = nil, either: Bool? = nil
  ) {
    self.source = source
    self.target = target
    self.relation = relation
    self.reads = reads
    self.either = either
  }
}

/// A reusable question definition. The engine validates its serialized options.
public protocol SavedQuestion: Sendable, Encodable {
  static var function: FunctionName { get }
  var definition: JSONValue { get }
}
extension SavedQuestion {
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
}
func questionDefinition(_ function: String, _ fields: [String: JSONValue?]) -> JSONValue {
  var result = fields.compactMapValues { $0 }
  result["function"] = .string(function)
  return .object(result)
}
func json(_ value: String?) -> JSONValue? { value.map(JSONValue.string) }
func json(_ value: Double?) -> JSONValue? { value.map(JSONValue.number) }
func json(_ value: Int?) -> JSONValue? { value.map { .integer(Int64($0)) } }
func json(_ value: Threshold?) -> JSONValue? {
  guard let value else { return nil }
  switch value {
  case .value(let n): return .number(n)
  case .band(let low, let high): return .object(["low": .number(low), "high": .number(high)])
  }
}
func choicesJSON(_ names: [String], _ descriptions: [String: String]) -> JSONValue {
  .array(
    names.map { name in
      if let description = descriptions.first(where: { $0.key.utf8.elementsEqual(name.utf8) })?
        .value
      {
        return .object(["name": .string(name), "description": .string(description)])
      }
      return .string(name)
    })
}
func choicesJSON<E: RawRepresentable & CaseIterable & Hashable>(
  _ type: E.Type, _ descriptions: [E: String]
) -> JSONValue where E.RawValue == String {
  choicesJSON(
    E.allCases.map(\.rawValue),
    rawDescriptions(descriptions))
}

func rawDescriptions<E: RawRepresentable & Hashable>(_ descriptions: [E: String]) -> [String:
  String]
where E.RawValue == String {
  descriptions.reduce(into: [:]) { $0[$1.key.rawValue] = $1.value }
}
func describedDefinition(
  _ function: String, _ fields: [String: JSONValue?], names: [String],
  descriptions: [String: String]
) -> JSONValue {
  var fields = fields
  let unmatched = descriptions.filter { entry in
    !names.contains { $0.utf8.elementsEqual(entry.key.utf8) }
  }
  if !unmatched.isEmpty {
    // Preserve invalid caller options for the engine's shared admission rules.
    fields["descriptions"] = .object(unmatched.mapValues(JSONValue.string))
  }
  return questionDefinition(function, fields)
}

extension RelationRule {
  var definition: JSONValue {
    .object(
      [
        "source": .string(source), "target": .string(target), "relation": .string(relation),
        "reads": json(reads), "either": either.map(JSONValue.bool),
      ].compactMapValues { $0 })
  }
}

private func constructRaw<R: RawRepresentable>(_ type: R.Type, _ raw: String) -> R? {
  guard let value = raw as? R.RawValue else { return nil }
  return R(rawValue: value)
}
func rawValue(_ type: any RawRepresentable.Type, _ raw: String) -> Any? { constructRaw(type, raw) }

// Named schemas use the engine's exact UTF8 spelling, not String's canonical
// equivalence. Values inside ordinary caller records retain Swift behavior.
func exactValue<T>(_ values: [String: T], _ name: String) -> T? {
  values.first { $0.key.utf8.elementsEqual(name.utf8) }?.value
}
