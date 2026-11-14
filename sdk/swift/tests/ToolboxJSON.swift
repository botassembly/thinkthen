import Foundation
import ThinkThen

// Feature: saved Swift toolboxes preserve typed questions and native rows.
func toolboxJSON(_ json: String) async throws {
  let toolbox = try Toolbox(json: json)
  let topic = try toolbox.choose("topic")
  let names = try toolbox.relate("names")
  let constructed = Toolbox([
    "ready": try toolbox.filter("ready"), "topic": topic, "names": names,
    "decision": try toolbox.decide("decision"), "tags": try toolbox.tag("tags"),
    "urgency": try toolbox.score("urgency"), "priority": try toolbox.rank("priority"),
    "find": try toolbox.find("find"), "recognized": try toolbox.recognize("recognized"),
  ])
  let saved = try constructed.jsonString()
  let decoded = try Toolbox(json: saved)
  precondition(decoded == toolbox)
  let encoded = try JSONEncoder().encode(constructed)
  let restored = try Toolbox(json: String(decoding: encoded, as: UTF8.self))
  precondition(restored == toolbox)
  let directory = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString)
  try FileManager.default.createDirectory(at: directory, withIntermediateDirectories: false)
  defer { try? FileManager.default.removeItem(at: directory) }
  let file = directory.appendingPathComponent("questions.json")
  try toolbox.write(to: file)
  let loaded = try Toolbox(contentsOf: file)
  precondition(loaded == toolbox)
  let children = AnnotationQuestions(["topic": topic])
  let childFile = directory.appendingPathComponent("annotations.json")
  try JSONEncoder().encode(children).write(to: childFile)
  let loadedChildren = try AnnotationQuestions(contentsOf: childFile)
  let textChildren = try AnnotationQuestions(
    json: String(decoding: JSONEncoder().encode(children), as: UTF8.self))
  precondition(loadedChildren == children && textChildren == children)
  let selectedChildren = AnnotationQuestions(Toolbox(["topic": topic]))
  precondition(selectedChildren == children)
  let client = try Client(.init(backend: .stub))
  let record: Record = .json(.object(["body": .string("Bill")]))
  let actual = try await client.choose(topic, record: record)
  let direct = try await client.choose(
    ChooseQuestion(
      "Which topic?", choices: ["billing", "shipping"],
      threshold: 0.7, field: "/body"), record: record)
  precondition(actual == direct)
  print(saved)
}
