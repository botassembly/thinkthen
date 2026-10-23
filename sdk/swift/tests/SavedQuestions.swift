import Foundation
import ThinkThen

func savedQuestions(_ client: Client) async throws {
  let toolbox = JSON(
    utf8: Data(
      #"{"questions":{"choice":{"choose":"Topic?","options":["left","right"]},"tag":{"tag":"Topics?","labels":["only"]},"score":{"score":"Urgent?","levels":["low","high"]}}}"#
        .utf8)
  )
  let record: Record = .text("evidence")
  func stream() -> AsyncStream<Record> {
    AsyncStream { continuation in
      continuation.yield(record)
      continuation.finish()
    }
  }
  let choice = try await client.choose(.toolbox(toolbox, name: "choice"), record: record)
  let tag = try await client.tag(.toolbox(toolbox, name: "tag"), record: record)
  let score = try await client.score(.toolbox(toolbox, name: "score"), record: record)
  func check<A: Encodable>(_ rows: Rows<A>, _ single: Row<A>) async throws {
    let actual = try await collect(rows, cancelled: false, encode: encoded)
    let expected = Data("[".utf8) + (try projected(single, encode: encoded)) + Data("]".utf8)
    precondition(actual == expected)
  }
  try await check(client.choose(.toolbox(toolbox, name: "choice"), records: [record]), choice)
  try await check(client.choose(.toolbox(toolbox, name: "choice"), records: stream()), choice)
  try await check(client.tag(.toolbox(toolbox, name: "tag"), records: [record]), tag)
  try await check(client.tag(.toolbox(toolbox, name: "tag"), records: stream()), tag)
  try await check(client.score(.toolbox(toolbox, name: "score"), records: [record]), score)
  try await check(client.score(.toolbox(toolbox, name: "score"), records: stream()), score)
}
