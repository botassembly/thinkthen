import Foundation

/// Named question definitions, supplied directly without function wrappers.
public struct AnnotationQuestions: Sendable, Encodable, Equatable, Hashable {
  public let definition: JSONValue
  public init(_ questions: [String: any SavedQuestion]) {
    definition = .object(questions.mapValues(\.definition))
  }
  public init(json: String) throws {
    do {
      let admitted = try Native.loaded.get().annotationJSON(Data(json.utf8))
      definition = try JSONDecoder().decode(JSONValue.self, from: admitted)
    } catch { throw typedError(error) }
  }
  public init(contentsOf url: URL) throws {
    try self.init(json: readSavedJSON(url))
  }
  public init(_ toolbox: Toolbox) { definition = toolbox.definitions }
  public func encode(to encoder: any Encoder) throws { try definition.encode(to: encoder) }
}

public struct Toolbox: Sendable, Encodable, Equatable, Hashable {
  let definitions: JSONValue
  public init(_ questions: [String: any SavedQuestion]) {
    definitions = .object(questions.mapValues(\.definition))
  }
  public init(json: String) throws {
    do {
      let admitted = try Native.loaded.get().toolboxJSON(Data(json.utf8), loading: true)
      definitions = try JSONDecoder().decode(JSONValue.self, from: admitted)
    } catch { throw typedError(error) }
  }
  public init(contentsOf url: URL) throws {
    try self.init(json: readSavedJSON(url))
  }
  public func jsonString() throws -> String {
    do {
      return String(
        decoding: try Native.loaded.get().toolboxJSON(
          JSONEncoder().encode(definitions), loading: false), as: UTF8.self)
    } catch { throw typedError(error) }
  }
  public func write(to url: URL) throws {
    let json = try jsonString()
    do { try json.write(to: url, atomically: true, encoding: .utf8) } catch {
      throw InternalError(code: .localIO, message: error.localizedDescription)
    }
  }
  public func encode(to encoder: any Encoder) throws {
    try JSONDecoder().decode(JSONValue.self, from: Data(jsonString().utf8)).encode(to: encoder)
  }
  private func select(_ name: String, function: String) throws -> JSONValue {
    guard let question = exactValue(definitions.object, name),
      question.object["function"]?.string == function
    else {
      throw UsageError(message: "Toolbox question \(name) must use \(function)")
    }
    return question
  }
  public func decide(_ name: String) throws -> DecideQuestion {
    DecideQuestion(definition: try select(name, function: "decide"))
  }
  public func find(_ name: String) throws -> FindQuestion {
    FindQuestion(definition: try select(name, function: "find"))
  }
  public func recognize(_ name: String) throws -> RecognizeQuestion {
    RecognizeQuestion(definition: try select(name, function: "recognize"))
  }
  public func relate(_ name: String) throws -> RelateQuestion {
    RelateQuestion(definition: try select(name, function: "relate"))
  }
  public func filter(_ name: String) throws -> FilterQuestion {
    FilterQuestion(definition: try select(name, function: "filter"))
  }
  public func score(_ name: String) throws -> ScoreQuestion {
    ScoreQuestion(definition: try select(name, function: "score"))
  }
  public func rank(_ name: String) throws -> RankQuestion {
    RankQuestion(definition: try select(name, function: "rank"))
  }
  public func choose(_ name: String) throws -> ChooseQuestion<String> {
    ChooseQuestion(definition: try select(name, function: "choose"))
  }
  public func choose<E: RawRepresentable & CaseIterable & Hashable & Sendable>(
    _ name: String, choices: E.Type
  ) throws -> ChooseQuestion<E> where E.RawValue == String {
    let value = try select(name, function: "choose")
    try checkDomain(value, key: "choices", type: E.self, name: name)
    return ChooseQuestion(definition: value)
  }
  public func tag(_ name: String) throws -> TagQuestion<String> {
    TagQuestion(definition: try select(name, function: "tag"))
  }
  public func tag<E: RawRepresentable & CaseIterable & Hashable & Sendable>(
    _ name: String, labels: E.Type
  ) throws -> TagQuestion<E> where E.RawValue == String {
    let value = try select(name, function: "tag")
    try checkDomain(value, key: "labels", type: E.self, name: name)
    return TagQuestion(definition: value)
  }
}
func checkDomain<E: RawRepresentable & CaseIterable>(
  _ definition: JSONValue, key: String, type: E.Type, name: String
) throws where E.RawValue == String {
  let actual =
    definition.object[key]?.array.map { $0.string ?? $0.object["name"]?.string ?? "" } ?? []
  let expected = E.allCases.map(\.rawValue)
  guard actual.count == expected.count,
    Set(actual.map { Data($0.utf8) }) == Set(expected.map { Data($0.utf8) })
  else {
    throw UsageError(message: "Question \(name) has a different enum domain")
  }
}

private func readSavedJSON(_ url: URL) throws -> String {
  do { return try String(contentsOf: url, encoding: .utf8) } catch {
    throw InternalError(code: .localIO, message: error.localizedDescription)
  }
}

// Names arrive as values in a native-owned JSON array, before host object decoding.
func checkSavedNames(_ inventory: [String]) throws {
  var names: [String: String] = [:]
  for name in inventory {
    if let previous = names[name], !previous.utf8.elementsEqual(name.utf8) {
      func identify(_ name: String) -> String {
        let scalars = name.unicodeScalars.map {
          "U+" + String($0.value, radix: 16, uppercase: true)
        }
        return String(reflecting: name) + " (" + scalars.joined(separator: " ") + ")"
      }
      throw UsageError(
        code: .usageArgument,
        message: "Saved question names collide in Swift: " + identify(previous) + " and "
          + identify(name))
    }
    names[name] = name
  }
}
