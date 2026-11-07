import Foundation
import ThinkThen

// Feature: saved Swift toolboxes preserve typed questions and native rows.
func toolboxJSON(_ json: String) async throws {
  let toolbox = try Toolbox(json: json)
  precondition(toolbox.questions.count == 9 && toolbox["missing"] == nil)
  let topic = try toolbox.choose(named: "topic")
  precondition(topic.choices == ["billing", "shipping"])
  let names = try toolbox.relate(named: "names")
  precondition(names.rules.count == 1)
  let constructed = Toolbox(toolbox.questions)
  let saved = try constructed.jsonString()
  let decoded = try JSONDecoder().decode(Toolbox.self, from: Data(saved.utf8))
  precondition(decoded == toolbox)
  let encoded = try JSONEncoder().encode(constructed)
  let restored = try JSONDecoder().decode(Toolbox.self, from: encoded)
  precondition(restored == toolbox)
  let directory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
  try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: false)
  defer { try? FileManager.default.removeItem(at: directory) }
  let file = directory.appendingPathComponent("questions.json")
  try toolbox.write(to: file)
  let loaded = try Toolbox(contentsOf: file)
  precondition(loaded == toolbox)
  let children = AnnotationQuestions(["topic": .choose(topic)])
  let childFile = directory.appendingPathComponent("annotations.json")
  try JSONEncoder().encode(children).write(to: childFile)
  let loadedChildren = try AnnotationQuestions(contentsOf: childFile)
  let textChildren = try AnnotationQuestions(
    json: String(decoding: JSONEncoder().encode(children), as: UTF8.self))
  precondition(loadedChildren == children && textChildren == children)
  let selectedChildren = try AnnotationQuestions(Toolbox(["topic": .choose(topic)]))
  precondition(selectedChildren == children)
  let client = try Client(.init(backend: .stub))
  let record: Record = .json(.object(["body": .string("Bill")]))
  let actual = try await client.choose(topic, record: record)
  let direct = try await client.choose(
    ChooseQuestion(
      "Which topic?", choices: ["billing", "shipping"],
      options: .init(field: "/body", threshold: 0.7)), record: record)
  precondition(actual == direct)
  print(saved)
}
