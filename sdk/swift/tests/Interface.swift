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
  let cap = Settings(backend: .stub, maxRequests: UInt64.max)
  let capBytes = try JSONEncoder().encode(cap)
  precondition(
    String(decoding: capBytes, as: UTF8.self).contains("\"max_requests\":18446744073709551615"))
  let decodedCap = try JSONDecoder().decode(Settings.self, from: capBytes)
  precondition(decodedCap.maxRequests == UInt64.max)
  let retry =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(RetryPolicy())) as? [String: Any]
  precondition(retry?.isEmpty == true)
  let noRetries =
    try JSONSerialization.jsonObject(with: JSONEncoder().encode(RetryPolicy(maxRetries: 0)))
    as? [String: Int]
  precondition(noRetries == ["max_retries": 0])
  let future = try JSONDecoder().decode(Backend.self, from: Data("\"future-backend\"".utf8))
  precondition(future.rawValue == "future-backend")
  print("Swift public interface passed")
}

// The module must accept ordinary caller code without loading an engine here.
func checkCallSignatures(_ client: Client) async throws {
  let inputs: [Record] = ["message"]
  let kept: Rows<Record> = client.filter("Needs help?", records: inputs)
  let keptTexts: Rows<String> = client.filter("Needs help?", records: ["message"])
  _ = keptTexts
  let ranked: RankRows<Record> = client.rank(
    "How urgent?", records: inputs, levels: ["low", "high"])
  if let result = try await ranked.makeAsyncIterator().next() {
    let original: Record = result.item
    let rank: Int = try result.answer
    _ = (original, rank)
  }
  _ = kept

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
  let questions = AnnotationQuestions(["refund": DecideQuestion("Refund?")])
  let calls: [any Sendable] = [
    client.decide("Refund?", records: texts),
    client.choose("Queue?", records: texts, choices: ["billing", "shipping"]),
    client.tag("Topics?", records: texts, labels: ["billing"]),
    client.score("Urgent?", records: texts, levels: ["low", "high"]),
    client.filter("Refund?", records: texts),
    client.rank("Urgent?", records: texts, levels: ["low", "high"]),
    client.find("Refund request", records: texts),
    client.recognize(records: texts, kinds: ["person"]),
    client.relate(
      records: texts,
      rules: [.init(source: "person", target: "organization", relation: "works_for")]),
    client.annotate(questions, records: texts),
  ]
  withExtendedLifetime(calls) {
    precondition(reads.count.withLock { $0 } == 0, "String sequence was eagerly consumed")
  }
}

func checkRankOriginalKinds(_ client: Client) async throws {
  let usage: Usage = try {
    let temporary = try Client(backend: .stub)
    return temporary.usage
  }()
  precondition(usage.attempts == 0 && usage.activeCalls == 0)
  precondition(usage.inputTokens.reported == 0 && usage.outputTokens.unknownAttempts == 0)
  let records: [Record] = [.json(.null), .missing]
  let results = try await client.rank("Urgent?", records: records, levels: ["low", "high"])
    .collect()
  precondition(results.count == records.count)
  for result in results {
    precondition(
      result.item == records[result.position - 1], "Rank changed its original record kind")
  }
}
