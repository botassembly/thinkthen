import Foundation

public enum AnnotationResult: Sendable, Codable, Equatable, Hashable {
  case decide(Outcome<Bool, NoDetail>)
  case choose(Outcome<String, LabelDetail>)
  case tag(Outcome<[String], LabelDetail>)
  case score(Outcome<Double, LevelDetail>)
  case filter(Outcome<Bool, NoDetail>)
  case find(Outcome<TextSpan?, NoDetail>)
  case recognize(Outcome<[RecognizedName], RecognizeDetail>)
  case relate(Outcome<[Relation], RelationDetail>)
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
public struct Annotations: Sendable, Codable, Equatable, Hashable {
  public let entries: [String: AnnotationResult]
  public subscript(_ name: String) -> AnnotationResult? { entries[name] }
  public init(from decoder: any Decoder) throws {
    entries = try decoder.singleValueContainer().decode([String: AnnotationResult].self)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    try c.encode(entries)
  }
}
