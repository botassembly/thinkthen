import Foundation

enum FunctionKey: String, CodingKey { case function }
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

func checkFunction(_ function: FunctionName, from decoder: any Decoder) throws {
  let c = try decoder.container(keyedBy: FunctionKey.self)
  if let actual = try c.decodeIfPresent(FunctionName.self, forKey: .function), actual != function {
    throw DecodingError.dataCorruptedError(
      forKey: .function, in: c, debugDescription: "Question function does not match its type")
  }
}

/// A label or level and its optional meaning for the backend.
public struct Choice: Sendable, Codable, Equatable, Hashable, ExpressibleByStringLiteral {
  /// The name value.
  public let name: String
  /// The optional explanation of this label or level for the backend.
  public let description: String?
  /// Create this value from its typed fields.
  public init(_ name: String, description: String? = nil) {
    self.name = name
    self.description = description
  }
  /// Create a value from a string literal.
  public init(stringLiteral value: String) { self.init(value) }
  private enum Keys: String, CodingKey { case name, description }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    if let name = try? decoder.singleValueContainer().decode(String.self) {
      self.init(name)
    } else {
      let c = try decoder.container(keyedBy: Keys.self)
      self.init(
        try c.decode(String.self, forKey: .name),
        description: try c.decodeIfPresent(String.self, forKey: .description))
    }
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    if let description {
      var c = encoder.container(keyedBy: Keys.self)
      try c.encode(name, forKey: .name)
      try c.encode(description, forKey: .description)
    } else {
      var c = encoder.singleValueContainer()
      try c.encode(name)
    }
  }
}
