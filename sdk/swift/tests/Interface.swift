import Foundation
import Synchronization
import ThinkThen

private enum Decision: FunctionShape {
  typealias Answer = Bool
  typealias Detail = NoDetail
  static let function = FunctionName.decide
}

func checkPublicInterface() throws {
  precondition(
    Settings().backend.rawValue
      == (ProcessInfo.processInfo.environment["THINKTHEN_BACKEND"] ?? "systemone"))
  precondition(Settings(backend: .openAI).backend == .openAI)
  let retry =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(Retry())) as? [String: Any]
  precondition(retry?.isEmpty == true)
  let noRetries =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(Retry(maxRetries: 0)))
    as? [String: Int]
  precondition(noRetries == ["max_retries": 0])
  let source = #""source":{"question_identity":"test","backend":"future-backend","model":"test"}"#
  func decision(_ outcome: String) throws -> Row<Decision> {
    try JSONDecoder().decode(
      Row<Decision>.self,
      from: Data("{\"function\":\"decide\",\"position\":1,\(source),\(outcome)}".utf8))
  }
  let approved = try decision(#""status":"answered","answer":false,"probability":0.1"#)
  let answer = try approved.get()
  precondition(answer == false)
  let resultAnswer = try approved.result().get()
  precondition(resultAnswer == false)
  precondition(approved.probability == 0.1 && !approved.isUnsure && !approved.isFailed)
  precondition(approved.source.backend.rawValue == "future-backend")
  let position: Int = approved.position
  precondition(position == 1)
  let uncertain = try decision(#""status":"unsure","probability":0.5"#)
  do {
    _ = try uncertain.get()
    preconditionFailure("not sure became an answer")
  } catch RowError.unsure(let probability) { precondition(probability == 0.5) }
  let failed = try decision(
    #""status":"failed","error":{"code":"future.failure","message":"future diagnostic","http_status":503}"#
  )
  do {
    _ = try failed.result().get()
    preconditionFailure("failure became an answer")
  } catch RowError.failed(let error) {
    precondition(error.code.rawValue == "future.failure")
    precondition(error.localizedDescription == "future diagnostic")
    let status: Int? = error.httpStatus
    precondition(status == 503)
  }
  let annotations = try JSONDecoder().decode(
    Annotations.self,
    from: Data(
      #"{"ready":{"function":"decide","status":"answered","answer":false},"review":{"function":"decide","status":"unsure","probability":0.5}}"#
        .utf8))
  let ready: Bool = try annotations.get("ready")
  precondition(!ready)
  do {
    let _: Bool = try annotations.get("review")
    preconditionFailure("annotation not sure became an answer")
  } catch RowError.unsure(let probability) { precondition(probability == 0.5) }
  let text = "a👩‍🔬e\u{301}z"
  let span = try JSONDecoder().decode(
    TextSpan.self, from: Data(#"{"text":"👩‍🔬","start":1,"end":4}"#.utf8))
  precondition(span.range(in: text).map { String(text[$0]) } == "👩‍🔬")
  let beyond = try JSONDecoder().decode(
    TextSpan.self, from: Data(#"{"text":"bad","start":-1,"end":99}"#.utf8))
  precondition(beyond.range(in: text) == nil)
  print("Swift public interface passed")
}

// The module must accept ordinary caller code without loading an engine here.
func checkCallSignatures(_ client: Client) async throws {
  let inputs: [Record] = ["message"]
  let kept: Rows<Record> = client.filter("Needs help?", records: inputs)
  let keptTexts: Rows<String> = client.filter("Needs help?", records: ["message"])
  _ = keptTexts
  let full: FilterRows = client.filterResults("Needs help?", records: inputs)
  let failureRows: FilterRows = client.filter("Needs help?", records: inputs, failures: .row)
  let ranked: RankRows = client.rank("How urgent?", levels: ["low", "high"], records: inputs)
  if let result = try await ranked.makeAsyncIterator().next() {
    let original: Record = result.item
    _ = original
    let rank: Int = try result.get()
    _ = rank
  }
  _ = (kept, full, failureRows)
}

// A generated text source must remain untouched until its result is read.
private final class TextReads: Sendable {
  let count = Mutex(0)
}

private struct CountedTexts: Sequence, Sendable {
  let reads: TextReads
  struct Iterator: IteratorProtocol {
    let reads: TextReads
    mutating func next() -> String? {
      reads.count.withLock { count in
        count += 1
        return count <= 2 ? "Support message \(count)" : nil
      }
    }
  }
  func makeIterator() -> Iterator { Iterator(reads: reads) }
}

func checkTextSequenceLaziness(_ client: Client) {
  let reads = TextReads()
  let texts = CountedTexts(reads: reads)
  let questions = AnnotationQuestions(["refund": .decide(DecideQuestion("Refund?"))])
  let calls: [any Sendable] = [
    client.decide("Refund?", records: texts),
    client.choose("Queue?", choices: ["billing", "shipping"], records: texts),
    client.tag("Topics?", labels: ["billing"], records: texts),
    client.score("Urgent?", levels: ["low", "high"], records: texts),
    client.filter("Refund?", records: texts),
    client.filterResults("Refund?", records: texts),
    client.rank("Urgent?", levels: ["low", "high"], records: texts),
    client.find("Refund request", records: texts),
    client.recognize(kinds: ["person"], records: texts),
    client.relate(
      rules: [.init(source: "person", target: "organization", relation: "works_for")],
      records: texts),
    client.annotate(questions, records: texts),
  ]
  withExtendedLifetime(calls) {
    precondition(reads.count.withLock { $0 } == 0, "String sequence was eagerly consumed")
  }
}

func checkRankOriginalKinds(_ client: Client) async throws {
  let records: [Record] = [.json(.null), .missing]
  let results = try await client.rank("Urgent?", levels: ["low", "high"], records: records)
    .collect()
  precondition(results.count == records.count)
  for result in results {
    precondition(
      result.item == records[result.position - 1], "Rank changed its original record kind")
  }
}
