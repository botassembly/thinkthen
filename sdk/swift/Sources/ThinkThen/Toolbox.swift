import Foundation

/// A typed question that can be saved and reused for this function.
public enum ToolboxQuestion: Sendable, Codable, Equatable, Hashable {
  /// A yes-or-no question or its typed outcome.
  case decide(DecideQuestion)
  /// A one-label choice or its typed outcome.
  case choose(ChooseQuestion)
  /// A set of labels or its typed outcome.
  case tag(TagQuestion)
  /// A score on ordered levels or its typed outcome.
  case score(ScoreQuestion)
  /// A kept-or-not decision or its typed outcome.
  case filter(FilterQuestion)
  /// A ranking question on ordered levels.
  case rank(RankQuestion)
  /// A matching text span, including an answered absence.
  case find(FindQuestion)
  /// Recognized names and their typed detail.
  case recognize(RecognizeQuestion)
  /// Relationships between names and their typed detail.
  case relate(RelateQuestion)
  /// Decode this value from its canonical JSON representation.
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
  /// Encode this value in its canonical JSON representation.
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
/// A typed question that can be saved and reused for this function.
public enum AnnotationQuestion: Sendable, Codable, Equatable, Hashable {
  /// A yes-or-no question or its typed outcome.
  case decide(DecideQuestion)
  /// A one-label choice or its typed outcome.
  case choose(ChooseQuestion)
  /// A set of labels or its typed outcome.
  case tag(TagQuestion)
  /// A score on ordered levels or its typed outcome.
  case score(ScoreQuestion)
  /// A kept-or-not decision or its typed outcome.
  case filter(FilterQuestion)
  /// A matching text span, including an answered absence.
  case find(FindQuestion)
  /// Recognized names and their typed detail.
  case recognize(RecognizeQuestion)
  /// Relationships between names and their typed detail.
  case relate(RelateQuestion)
  /// Decode this value from its canonical JSON representation.
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
  /// Encode this value in its canonical JSON representation.
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
/// The typed AnnotationQuestions value exchanged with the engine.
public struct AnnotationQuestions: Sendable, Codable, Equatable, Hashable {
  /// The saved questions indexed by their entry names.
  public let questions: [String: AnnotationQuestion]
  /// Create this value from its typed fields.
  public init(_ questions: [String: AnnotationQuestion]) { self.questions = questions }
  /// Decode the saved mapping of child question names to definitions.
  public init(json: String) throws {
    self = try JSONDecoder().decode(Self.self, from: Data(json.utf8))
  }
  /// Load the saved JSON value from a UTF-8 file.
  public init(contentsOf url: URL) throws {
    try self.init(json: String(contentsOf: url, encoding: .utf8))
  }
  /// Create this value from its typed fields.
  public init(_ toolbox: Toolbox) throws {
    self = try JSONDecoder().decode(Self.self, from: JSONEncoder().encode(toolbox.questions))
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    questions = try decoder.singleValueContainer().decode([String: AnnotationQuestion].self)
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    var c = encoder.singleValueContainer()
    try c.encode(questions)
  }
}
/// The typed Toolbox value exchanged with the engine.
public struct Toolbox: Sendable, Codable, Equatable, Hashable {
  /// The saved questions indexed by their entry names.
  public let questions: [String: ToolboxQuestion]
  /// Create this value from its typed fields.
  public init(_ questions: [String: ToolboxQuestion]) { self.questions = questions }
  /// Load the saved JSON representation.
  public init(json: String) throws {
    let native = try Native.loaded.get()
    let definitions = try native.toolboxJSON(Data(json.utf8), loading: true)
    self.init(try JSONDecoder().decode([String: ToolboxQuestion].self, from: definitions))
  }
  /// Serialize the saved toolbox in the engine-owned JSON format.
  public func jsonString() throws -> String {
    let native = try Native.loaded.get()
    let saved = try native.toolboxJSON(JSONEncoder().encode(questions), loading: false)
    return String(decoding: saved, as: UTF8.self)
  }
  /// Load the saved JSON value from a UTF-8 file.
  public init(contentsOf url: URL) throws {
    try self.init(json: String(contentsOf: url, encoding: .utf8))
  }
  /// Write the saved toolbox JSON to a file atomically.
  public func write(to url: URL) throws {
    try jsonString().write(to: url, atomically: true, encoding: .utf8)
  }
  /// Decode this value from its canonical JSON representation.
  public init(from decoder: any Decoder) throws {
    let value = try JSONValue(from: decoder)
    self = try Self(json: String(decoding: JSONEncoder().encode(value), as: UTF8.self))
  }
  /// Encode this value in its canonical JSON representation.
  public func encode(to encoder: any Encoder) throws {
    let value = try JSONDecoder().decode(JSONValue.self, from: Data(jsonString().utf8))
    try value.encode(to: encoder)
  }
  /// Read an entry by its name, or nil if the name is absent.
  public subscript(name: String) -> ToolboxQuestion? { questions[name] }
  /// Look up a saved question by its entry name.
  public func question(named name: String) -> ToolboxQuestion? { self[name] }
  /// Select a saved decide question, throwing if the entry has a different function.
  public func decide(named name: String) throws -> DecideQuestion {
    guard case .decide(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .decide)
    }
    return x
  }
  /// Select a saved choose question, throwing if the entry has a different function.
  public func choose(named name: String) throws -> ChooseQuestion {
    guard case .choose(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .choose)
    }
    return x
  }
  /// Select a saved tag question, throwing if the entry has a different function.
  public func tag(named name: String) throws -> TagQuestion {
    guard case .tag(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .tag)
    }
    return x
  }
  /// Select a saved score question, throwing if the entry has a different function.
  public func score(named name: String) throws -> ScoreQuestion {
    guard case .score(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .score)
    }
    return x
  }
  /// Select a saved filter question, throwing if the entry has a different function.
  public func filter(named name: String) throws -> FilterQuestion {
    guard case .filter(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .filter)
    }
    return x
  }
  /// Select a saved rank question, throwing if the entry has a different function.
  public func rank(named name: String) throws -> RankQuestion {
    guard case .rank(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .rank)
    }
    return x
  }
  /// Select a saved find question, throwing if the entry has a different function.
  public func find(named name: String) throws -> FindQuestion {
    guard case .find(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .find)
    }
    return x
  }
  /// Select a saved recognize question, throwing if the entry has a different function.
  public func recognize(named name: String) throws -> RecognizeQuestion {
    guard case .recognize(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .recognize)
    }
    return x
  }
  /// Select a saved relate question, throwing if the entry has a different function.
  public func relate(named name: String) throws -> RelateQuestion {
    guard case .relate(let x) = questions[name] else {
      throw ThinkThenError.toolboxSelection(name: name, expected: .relate)
    }
    return x
  }
}
