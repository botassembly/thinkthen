import Foundation
import ThinkThen

// Feature: saved Swift toolboxes preserve typed questions and native rows.
func toolboxJSON(_ json: String) async throws {
  let toolbox = try Toolbox.fromJSON(json)
  precondition(toolbox.questions.count == 9 && toolbox["missing"] == nil)
  let topic = try toolbox.choose(named: "topic")
  precondition(topic.choices == ["billing", "shipping"])
  let names = try toolbox.relate(named: "names")
  precondition(names.rules.count == 1)
  let constructed = Toolbox(toolbox.questions)
  let saved = try constructed.toJSON()
  let decoded = try JSONDecoder().decode(Toolbox.self, from: Data(saved.utf8))
  precondition(decoded == toolbox)
  let encoded = try JSONEncoder().encode(constructed)
  let restored = try JSONDecoder().decode(Toolbox.self, from: encoded)
  precondition(restored == toolbox)
  let client = try Client(.init(backend: .stub))
  let record: Record = .json(.object(["body": .string("Bill")]))
  let actual = try await client.choose(topic, record: record)
  let direct = try await client.choose(
    ChooseQuestion(
      "Which topic?", choices: ["billing", "shipping"],
      options: .init(evidence: "/body", threshold: .value(0.7))), record: record)
  precondition(actual == direct)
  print(saved)
}
