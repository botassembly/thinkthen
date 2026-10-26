import Foundation

public enum ToolboxQuestion: Sendable, Codable, Equatable, Hashable {
  case decide(DecideQuestion)
  case choose(ChooseQuestion)
  case tag(TagQuestion)
  case score(ScoreQuestion)
  case filter(FilterQuestion)
  case rank(RankQuestion)
  case find(FindQuestion)
  case recognize(RecognizeQuestion)
  case relate(RelateQuestion)
  public init(from decoder: any Decoder) throws {
    let tag = try decoder.container(keyedBy: FunctionKey.self).decode(
      FunctionName.self, forKey: .function)
    switch tag {
    case .decide: self = .decide(try DecideQuestion(from: decoder))
    case .choose: self = .choose(try ChooseQuestion(from: decoder))
    case .tag: self = .tag(try TagQuestion(from: decoder))
    case .score: self = .score(try ScoreQuestion(from: decoder))
    case .filter: self = .filter(try FilterQuestion(from: decoder))
    case .rank: self = .rank(try RankQuestion(from: decoder))
    case .find: self = .find(try FindQuestion(from: decoder))
    case .recognize: self = .recognize(try RecognizeQuestion(from: decoder))
    case .relate: self = .relate(try RelateQuestion(from: decoder))
    default:
      throw DecodingError.dataCorrupted(
        .init(codingPath: decoder.codingPath, debugDescription: "Ineligible toolbox function"))
    }
  }
  public func encode(to encoder: any Encoder) throws {
    switch self {
    case .decide(let x): try x.encode(to: encoder)
    case .choose(let x): try x.encode(to: encoder)
    case .tag(let x): try x.encode(to: encoder)
    case .score(let x): try x.encode(to: encoder)
    case .filter(let x): try x.encode(to: encoder)
    case .rank(let x): try x.encode(to: encoder)
    case .find(let x): try x.encode(to: encoder)
    case .recognize(let x): try x.encode(to: encoder)
    case .relate(let x): try x.encode(to: encoder)
    }
  }
}
public enum AnnotationQuestion: Sendable, Codable, Equatable, Hashable {
  case decide(DecideQuestion)
  case choose(ChooseQuestion)
  case tag(TagQuestion)
  case score(ScoreQuestion)
  case filter(FilterQuestion)
  case find(FindQuestion)
  case recognize(RecognizeQuestion)
  case relate(RelateQuestion)
  public init(from decoder: any Decoder) throws {
    let tag = try decoder.container(keyedBy: FunctionKey.self).decode(
      FunctionName.self, forKey: .function)
    switch tag {
    case .decide: self = .decide(try DecideQuestion(from: decoder))
    case .choose: self = .choose(try ChooseQuestion(from: decoder))
    case .tag: self = .tag(try TagQuestion(from: decoder))
    case .score: self = .score(try ScoreQuestion(from: decoder))
    case .filter: self = .filter(try FilterQuestion(from: decoder))
    case .find: self = .find(try FindQuestion(from: decoder))
    case .recognize: self = .recognize(try RecognizeQuestion(from: decoder))
    case .relate: self = .relate(try RelateQuestion(from: decoder))
    default:
      throw DecodingError.dataCorrupted(
        .init(codingPath: decoder.codingPath, debugDescription: "Ineligible toolbox function"))
    }
  }
  public func encode(to encoder: any Encoder) throws {
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
public struct AnnotationQuestions: Sendable, Codable, Equatable, Hashable {
  public let questions: [String: AnnotationQuestion]
  public init(_ questions: [String: AnnotationQuestion]) { self.questions = questions }
  public init(from decoder: any Decoder) throws {
    questions = try decoder.singleValueContainer().decode([String: AnnotationQuestion].self)
  }
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    try c.encode(questions)
  }
}
public struct Toolbox: Sendable, Codable, Equatable, Hashable {
  public let questions: [String: ToolboxQuestion]
  public init(_ questions: [String: ToolboxQuestion]) { self.questions = questions }
  public static func fromJSON(_ json: String) throws -> Toolbox {
    let native = try Native.loaded.get()
    let definitions = try native.toolboxJSON(Data(json.utf8), loading: true)
    return Toolbox(try JSONDecoder().decode([String: ToolboxQuestion].self, from: definitions))
  }
  public func toJSON() throws -> String {
    let native = try Native.loaded.get()
    let saved = try native.toolboxJSON(JSONEncoder().encode(questions), loading: false)
    return String(decoding: saved, as: UTF8.self)
  }
  public init(from decoder: any Decoder) throws {
    let value = try JSONValue(from: decoder)
    self = try Self.fromJSON(String(decoding: JSONEncoder().encode(value), as: UTF8.self))
  }
  public func encode(to encoder: any Encoder) throws {
    let value = try JSONDecoder().decode(JSONValue.self, from: Data(toJSON().utf8))
    try value.encode(to: encoder)
  }
  public subscript(name: String) -> ToolboxQuestion? { questions[name] }
  public func question(named name: String) -> ToolboxQuestion? { self[name] }
  public func decide(named name: String) throws(ThinkThenError) -> DecideQuestion {
    guard case .decide(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .decide)
    }
    return x
  }
  public func choose(named name: String) throws(ThinkThenError) -> ChooseQuestion {
    guard case .choose(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .choose)
    }
    return x
  }
  public func tag(named name: String) throws(ThinkThenError) -> TagQuestion {
    guard case .tag(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .tag)
    }
    return x
  }
  public func score(named name: String) throws(ThinkThenError) -> ScoreQuestion {
    guard case .score(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .score)
    }
    return x
  }
  public func filter(named name: String) throws(ThinkThenError) -> FilterQuestion {
    guard case .filter(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .filter)
    }
    return x
  }
  public func rank(named name: String) throws(ThinkThenError) -> RankQuestion {
    guard case .rank(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .rank)
    }
    return x
  }
  public func find(named name: String) throws(ThinkThenError) -> FindQuestion {
    guard case .find(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .find)
    }
    return x
  }
  public func recognize(named name: String) throws(ThinkThenError) -> RecognizeQuestion {
    guard case .recognize(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .recognize)
    }
    return x
  }
  public func relate(named name: String) throws(ThinkThenError) -> RelateQuestion {
    guard case .relate(let x) = questions[name] else {
      throw .toolboxSelection(name: name, expected: .relate)
    }
    return x
  }
}
