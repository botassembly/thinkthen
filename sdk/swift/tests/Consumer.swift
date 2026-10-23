import Foundation
import ThinkThen

func json(_ value: Any) throws -> JSON {
  JSON(
    utf8: try JSONSerialization.data(
      withJSONObject: value, options: [.fragmentsAllowed, .sortedKeys]))
}
func encoded<A: Encodable>(_ value: A) throws -> Data { try JSONEncoder().encode(value) }
func projected<A>(_ row: Row<A>, encode: (A) throws -> Data) throws -> Data {
  var fields = [Data("\"status\":\"\(row.status.rawValue)\"".utf8)]
  if let id = row.id { fields.append(Data("\"id\":".utf8) + id.utf8) }
  if let answer = row.answer {
    fields.append(Data("\"answer\":".utf8) + (try encode(answer)))
  } else if row.error == nil {
    fields.append(Data("\"answer\":null".utf8))
  }
  if let p = row.p { fields.append(Data("\"p\":\(p)".utf8)) }
  if let error = row.error {
    fields.append(Data("\"error\":{\"code\":".utf8) + (try encoded(error.code)) + Data("}".utf8))
  }
  if let detail = row.detail { fields.append(Data("\"detail\":".utf8) + detail.utf8) }
  return Data("{".utf8) + fields.reduce(Data()) { $0 + ($0.isEmpty ? Data() : Data(",".utf8)) + $1 }
    + Data("}".utf8)
}
func collect<A>(_ rows: Rows<A>, cancelled: Bool, encode: @Sendable (A) throws -> Data) async throws
  -> Data
{
  if cancelled { rows.cancel() }
  return try await rows.withRows { sequence in
    var output = [Data]()
    var failure: String?
    do {
      for try await row in sequence { output.append(try projected(row, encode: encode)) }
    } catch is CancellationError where cancelled {
      failure = "local.cancelled"
    } catch let error as ThinkThenError {
      failure = error.code
    }
    let delivered =
      Data("[".utf8)
      + output.reduce(Data()) { $0 + ($0.isEmpty ? Data() : Data(",".utf8)) + $1 } + Data("]".utf8)
    if let failure {
      return Data(#"{"error":"#.utf8) + (try encoded(failure))
        + Data(#", "rows":"#.utf8) + delivered + Data("}".utf8)
    }
    return delivered
  }
}
@MainActor func run(_ test: [String: Any], url: String?) async throws -> Data {
  let settings = test["client"] as? [String: Any] ?? [:]
  let client = try Client(
    settings: .init(
      mode: settings["mode"] as? String,
      adapter: settings["adapter"] as? String, url: url ?? settings["url"] as? String,
      model: settings["model"] as? String, keySource: settings["key_source"] as? String,
      batch: settings["batch"] as? Double))
  let request = test["request"] as! [String: Any]
  let question = request["question"] as! [String: Any]
  let value = try json(question["value"] ?? NSNull())
  let q: Question
  switch question["type"] as? String {
  case "toolbox": q = .toolbox(value, name: question["name"] as? String)
  case "reference": q = .reference(question["name"] as! String)
  default: q = .inline(value)
  }
  let records = try (request["records"] as! [Any]).map { record -> Record in
    if let text = record as? String { return .text(text) }
    return .json(try json(record))
  }
  let options = request["options"] as? [String: Any] ?? [:]
  let cut: Threshold?
  if let number = options["threshold"] as? Double {
    cut = .number(number)
  } else if let band = options["threshold"] as? String {
    cut = .band(band)
  } else {
    cut = nil
  }
  let call = CallOptions(threshold: cut, on: options["on"] as? String)
  let cancelled = test["cancelled"] as? Bool ?? false
  switch request["function"] as! String {
  case "decide":
    return try await collect(
      client.decide(q, records: records, options: call), cancelled: cancelled, encode: encoded)
  case "choose":
    return try await collect(
      client.choose(
        q, records: records, options: options["options"] as? [String], callOptions: call),
      cancelled: cancelled, encode: encoded)
  case "tag":
    return try await collect(
      client.tag(
        q, records: records, labels: options["labels"] as? [String], callOptions: call),
      cancelled: cancelled, encode: encoded)
  case "score":
    return try await collect(
      client.score(
        q, records: records, levels: options["levels"] as? [String],
        callOptions: .init(on: options["on"] as? String)), cancelled: cancelled, encode: encoded)
  case "filter":
    return try await collect(
      client.filter(q, records: records, options: call), cancelled: cancelled, encode: encoded)
  case "rank":
    return try await collect(
      client.rank(
        q, records: records,
        options: .init(
          on: options["on"] as? String, top: options["top"] as? Double,
          levels: options["levels"] as? [String])),
      cancelled: cancelled, encode: encoded)
  case "find":
    return try await collect(
      client.find(q, records: records, options: call), cancelled: cancelled, encode: encoded)
  case "annotate":
    return try await collect(
      client.annotate(value, records: records, options: .init(on: options["on"] as? String)),
      cancelled: cancelled,
      encode: { $0.utf8 })
  case "recognize":
    return try await collect(
      client.recognize(q, records: records, options: call), cancelled: cancelled, encode: encoded)
  default:
    return try await collect(
      client.relate(q, records: records, options: call), cancelled: cancelled, encode: encoded)
  }
}

actor Signal {
  var fired = false
  var waiters: [CheckedContinuation<Void, Never>] = []
  func wait() async {
    if fired { return }
    await withCheckedContinuation { waiters.append($0) }
  }
  func fire() {
    fired = true
    for waiter in waiters { waiter.resume() }
    waiters = []
  }
}
struct StubbornSource: AsyncSequence, Sendable {
  typealias Element = Record
  let entered: Signal, released: Signal
  struct AsyncIterator: AsyncIteratorProtocol {
    let entered: Signal, released: Signal
    mutating func next() async -> Record? {
      await entered.fire()
      await released.wait()
      return nil
    }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator(entered: entered, released: released) }
}
enum SourceFailure: Error { case expected }
struct FailingSource: AsyncSequence, Sendable {
  typealias Element = Record
  struct AsyncIterator: AsyncIteratorProtocol {
    mutating func next() async throws -> Record? { throw SourceFailure.expected }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator() }
}
func surface() async throws {
  let client = try Client(settings: .init(adapter: "stub"))
  let stream = AsyncStream<Record> { continuation in
    continuation.yield(.text("first"))
    continuation.yield(.text("second"))
    continuation.finish()
  }
  try await savedQuestions(client)
  let rows = try client.decide("Ready?", records: stream)
  var first = rows.makeAsyncIterator()
  var rejected = rows.makeAsyncIterator()
  do {
    _ = try await rejected.next()
    preconditionFailure("second traversal was admitted")
  } catch RowsIterationError.alreadyIterated {}
  var count = 0
  while let row = try await first.next() {
    precondition(row.position == count + 1)
    count += 1
  }
  precondition(count == 2)
  let failed = try client.decide("Ready?", records: FailingSource())
  do {
    try await failed.withRows { sequence in
      for try await _ in sequence { preconditionFailure("failed source returned a row") }
    }
    preconditionFailure("source error was discarded")
  } catch SourceFailure.expected {}
  async let left = client.decide("Ready?", record: .json(try json(["id": "left", "body": "a"])))
  async let right = client.decide("Ready?", record: .json(try json(["id": "right", "body": "b"])))
  let (leftRow, rightRow) = try await (left, right)
  let leftID = try leftRow.id!.decode(String.self)
  let rightID = try rightRow.id!.decode(String.self)
  precondition(leftID == "left" && rightID == "right")
  let entered = Signal()
  let released = Signal()
  let stubborn = StubbornSource(entered: entered, released: released)
  let waiting = try client.decide("Ready?", records: stubborn)
  let task = Task {
    try await waiting.withRows { sequence in
      for try await _ in sequence { preconditionFailure("source returned a record") }
    }
  }
  await entered.wait()
  task.cancel()
  do {
    try await task.value
    preconditionFailure("source cancellation did not throw")
  } catch is CancellationError {}
  // Releasing the source only after task completion proves no join occurred.
  await released.fire()
  let scoped = try client.decide("Ready?", records: [.text("a"), .text("b")])
  var retained = scoped.makeAsyncIterator()
  try await scoped.withRows { _ in
    let firstRow = try await retained.next()
    precondition(firstRow != nil)
  }
  do {
    _ = try await retained.next()
    preconditionFailure("scope left its call open")
  } catch is CancellationError {}
  let raw = JSON(
    utf8: Data(
      #"{"body":"evidence","id":{"\u00e9":1,"e\u0301":2,"a\u0000b":18446744073709551615}}"#.utf8))
  let saved = try await client.decide("Ready?", record: .json(raw))
  FileHandle.standardOutput.write(saved.id!.utf8 + Data("\n".utf8))
  let named = JSON(
    utf8: Data(#"{"questions":{"\u00e9":{"decide":"Ready?"},"e\u0301":{"decide":"Ready?"}}}"#.utf8))
  let annotation = try await client.annotate(named, record: .text("evidence"))
  FileHandle.standardOutput.write(annotation.answer!.utf8 + Data("\n".utf8))
  let bad = JSON(utf8: Data("{".utf8))
  do {
    _ = try await client.decide("Ready?", record: .json(bad))
    preconditionFailure("bad JSON was admitted")
  } catch let error as ThinkThenError { precondition(error.code.hasPrefix("usage.")) }
}

@main struct Consumer {
  static func main() async throws {
    let args = CommandLine.arguments
    if args[1] == "surface" {
      try await surface()
      return
    }
    if args[1] == "cancel" {
      let client = try Client(settings: .init(url: args[2]))
      let task = Task { try await client.decide("Ready?", record: .text("evidence")) }
      await withCheckedContinuation { continuation in
        DispatchQueue.global().async {
          _ = FileHandle.standardInput.readData(ofLength: 1)
          continuation.resume()
        }
      }
      task.cancel()
      do {
        _ = try await task.value
        preconditionFailure("held call did not cancel")
      } catch is CancellationError {}
      return
    }
    if args[1] == "backend" {
      do {
        let client = try Client(settings: .init(url: args[2]))
        _ = try await client.decide("Ready?", record: .text("evidence"))
        preconditionFailure("unavailable backend answered")
      } catch let error as ThinkThenError { precondition(error.code.hasPrefix("backend.")) }
      return
    }
    let bytes = try Data(contentsOf: URL(fileURLWithPath: args[1]))
    let value = try JSONSerialization.jsonObject(with: bytes)
    let cases = (value as? [[String: Any]]) ?? [value as! [String: Any]]
    for test in cases {
      do {
        FileHandle.standardOutput.write(try await run(test, url: args.count > 2 ? args[2] : nil))
      } catch let error as ThinkThenError {
        FileHandle.standardOutput.write(
          Data("{\"error\":".utf8) + (try encoded(error.code)) + Data("}".utf8))
      }
      FileHandle.standardOutput.write(Data("\n".utf8))
    }
  }
}
