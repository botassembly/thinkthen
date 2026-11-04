import Foundation

/// The typed AnnotationResult value exchanged with the engine.
public enum AnnotationResult: Sendable, Codable, Equatable, Hashable {
  /// A yes-or-no question or its typed outcome.
  case decide(Outcome<Bool, NoDetail>)
  /// A one-label choice or its typed outcome.
  case choose(Outcome<String, LabelDetail>)
  /// A set of labels or its typed outcome.
  case tag(Outcome<[String], LabelDetail>)
  /// A score on ordered levels or its typed outcome.
  case score(Outcome<Double, LevelDetail>)
  /// A kept-or-not decision or its typed outcome.
  case filter(Outcome<Bool, NoDetail>)
  /// A matching text span, including an answered absence.
  case find(Outcome<TextSpan?, NoDetail>)
  /// Recognized names and their typed detail.
  case recognize(Outcome<[RecognizedName], RecognizeDetail>)
  /// Relationships between names and their typed detail.
  case relate(Outcome<[Relation], RelationDetail>)
  /// Read this child's answer, throwing when it is unsure, failed or a different type.
  public func get<A: Sendable>(as type: A.Type = A.self) throws -> A {
    let answer: any Sendable
    switch self {
    case .decide(let outcome): answer = try outcome.get()
    case .choose(let outcome): answer = try outcome.get()
    case .tag(let outcome): answer = try outcome.get()
    case .score(let outcome): answer = try outcome.get()
    case .filter(let outcome): answer = try outcome.get()
    case .find(let outcome): answer = try outcome.get()
    case .recognize(let outcome): answer = try outcome.get()
    case .relate(let outcome): answer = try outcome.get()
    }
    guard let typed = answer as? A else {
      throw ThinkThenError.contract(message: "Annotation answer does not match \(type)")
    }
    return typed
  }
  /// The function this question or result belongs to.
  public var function: FunctionName {
    switch self {
    case .decide: .decide
    case .choose: .choose
    case .tag: .tag
    case .score: .score
    case .filter: .filter
    case .find: .find
    case .recognize: .recognize
    case .relate: .relate
    }
  }
  /// Whether the record was answered, was unsure or failed.
  public var status: Status {
    switch self {
    case .decide(let x): x.status
    case .choose(let x): x.status
    case .tag(let x): x.status
    case .score(let x): x.status
    case .filter(let x): x.status
    case .find(let x): x.status
    case .recognize(let x): x.status
    case .relate(let x): x.status
    }
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let tag = try decoder.container(keyedBy: FunctionKey.self).decode(
      FunctionName.self, forKey: .function)
    switch tag {
    case .decide: self = .decide(try Outcome(from: decoder))
    case .choose: self = .choose(try Outcome(from: decoder))
    case .tag: self = .tag(try Outcome(from: decoder))
    case .score: self = .score(try Outcome(from: decoder))
    case .filter: self = .filter(try Outcome(from: decoder))
    case .find: self = .find(try Outcome(from: decoder))
    case .recognize: self = .recognize(try Outcome(from: decoder))
    case .relate: self = .relate(try Outcome(from: decoder))
    default:
      throw DecodingError.dataCorrupted(
        .init(codingPath: decoder.codingPath, debugDescription: "Invalid annotation function"))
    }
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.container(keyedBy: FunctionKey.self)
    try c.encode(function, forKey: .function)
    switch self {
    case .decide(let x): try x.encode(to: encoder)
    case .choose(let x): try x.encode(to: encoder)
    case .tag(let x): try x.encode(to: encoder)
    case .score(let x): try x.encode(to: encoder)
    case .filter(let x): try x.encode(to: encoder)
    case .find(let x): try x.encode(to: encoder)
    case .recognize(let x): try x.encode(to: encoder)
    case .relate(let x): try x.encode(to: encoder)
    }
  }
}
/// The typed Annotations value exchanged with the engine.
public struct Annotations: Sendable, Codable, Equatable, Hashable {
  /// The annotation outcomes indexed by their question names.
  public let entries: [String: AnnotationResult]
  /// Read an entry by its name, or nil if the name is absent.
  public subscript(_ name: String) -> AnnotationResult? { entries[name] }
  /// Return the typed answer, throwing RowError for unsure and failed outcomes.
  public func get<A: Sendable>(_ name: String, as type: A.Type = A.self) throws -> A {
    guard let entry = entries[name] else {
      throw ThinkThenError.contract(message: "No annotation named \(name)")
    }
    return try entry.get(as: type)
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    entries = try decoder.singleValueContainer().decode([String: AnnotationResult].self)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    try c.encode(entries)
  }
}
