import Foundation
import ThinkThen

actor Signal {
  private var fired = false
  private var waiters: [CheckedContinuation<Void, Never>] = []
  func wait() async {
    if fired { return }
    await withCheckedContinuation { waiters.append($0) }
  }
  func fire() {
    fired = true
    for waiter in waiters { waiter.resume() }
    waiters.removeAll()
  }
}

// Deliberately lacks Sendable. A Sendable sequence need not have a Sendable iterator.
final class LocalIterator: AsyncIteratorProtocol {
  typealias Element = Record
  private var records: [Record]
  init(_ records: [Record]) { self.records = records }
  func next() async -> Record? { records.isEmpty ? nil : records.removeFirst() }
  func next(isolation actor: isolated (any Actor)?) async -> Record? {
    records.isEmpty ? nil : records.removeFirst()
  }
}
struct ConfinedSource: AsyncSequence, Sendable {
  typealias Element = Record
  let records: [Record]
  func makeAsyncIterator() -> LocalIterator { LocalIterator(records) }
}
struct StubbornSource: AsyncSequence, Sendable {
  typealias Element = Record
  let entered: Signal, released: Signal, returned: Signal
  final class AsyncIterator: AsyncIteratorProtocol {
    let source: StubbornSource
    init(_ source: StubbornSource) { self.source = source }
    func next() async -> Record? {
      await source.entered.fire()
      await source.released.wait()
      await source.returned.fire()
      return .text("late evidence")
    }
    func next(isolation actor: isolated (any Actor)?) async -> Record? {
      await source.entered.fire()
      await source.released.wait()
      await source.returned.fire()
      return .text("late evidence")
    }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator(self) }
}
enum SourceFailure: Error { case expected }
struct FailingSource: AsyncSequence, Sendable {
  typealias Element = Record
  struct AsyncIterator: AsyncIteratorProtocol {
    mutating func next() async throws -> Record? { throw SourceFailure.expected }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator() }
}
actor OtherConsumer {
  func read(_ rows: DecideRows<Record>) async throws -> [DecideRow<Record>] {
    var output: [DecideRow<Record>] = []
    let iterator = rows.makeAsyncIterator()
    while let row = try await iterator.next(isolation: self) { output.append(row) }
    return output
  }
}
func requireSendable<T: Sendable>(_: T) {}

// Keep error matching outside async typed-catch lowering in the pinned compiler.
func requireError(_ actual: any Error, equals expected: IterationIssue) throws {
  guard let typed = actual as? UsageError else { throw actual }
  let message =
    expected == .alreadyIterated
    ? "Rows have already been iterated" : "Rows cannot be advanced concurrently"
  guard typed.code == .usageArgument && typed.message == message else { throw actual }
}
func requireSourceError(_ actual: any Error) throws {
  guard actual is InternalError else { throw actual }
}

// Feature: Swift actor callers consume sources without sending their iterators.
@MainActor func isolatedIteration(_ client: Client) async throws {
  let source = ConfinedSource(records: [.text("one"), .text("two")])
  let rows: DecideRows<Record> = client.decide("Ready?", records: source)
  requireSendable(rows)
  let first = rows.makeAsyncIterator()
  requireSendable(first)
  let rejected = rows.makeAsyncIterator()
  do {
    _ = try await rejected.next(isolation: MainActor.shared)
    preconditionFailure("second iterator was admitted")
  } catch { try requireError(error, equals: .alreadyIterated) }
  var positions: [Int] = []
  while let row = try await first.next(isolation: MainActor.shared) {
    positions.append(row.position)
  }
  precondition(positions == [1, 2])
  let afterEnd = try await first.next()
  precondition(afterEnd == nil)
  let other = try await OtherConsumer().read(client.decide("Ready?", records: source))
  precondition(other.map(\.position) == [1, 2])
}

// Feature: typed cancellation detaches a source that ignores cancellation.
func cancelledSource(_ client: Client) async throws {
  let entered = Signal()
  let released = Signal()
  let returned = Signal()
  let source = StubbornSource(entered: entered, released: released, returned: returned)
  let rows = client.decide("Ready?", records: source)
  let iterator = rows.makeAsyncIterator()
  let active = Task { try await iterator.next() }
  await entered.wait()
  do {
    _ = try await iterator.next()
    preconditionFailure("concurrent next was admitted")
  } catch { try requireError(error, equals: .concurrentNext) }
  active.cancel()
  do {
    _ = try await active.value
    preconditionFailure("task cancellation did not throw")
  } catch { guard error is CancellationError else { throw error } }
  // Completion before release proves cancellation does not join the source.
  await released.fire()
  await returned.wait()
  let terminal = try await iterator.next()
  precondition(terminal == nil, "late source result reached a cancelled call")

  let beforeStart = client.decide("Ready?", records: [Record.text("never sent")])
  beforeStart.cancel()
  beforeStart.cancel()
  do {
    _ = try await beforeStart.makeAsyncIterator().next()
    preconditionFailure("cancellation before iteration was ignored")
  } catch { guard error is CancellationError else { throw error } }
}

// Feature: explicit stream cancel wakes a pending source read.
func explicitlyCancelledSource(_ client: Client) async throws {
  let entered = Signal()
  let released = Signal()
  let returned = Signal()
  let rows = client.decide(
    "Ready?", records: StubbornSource(entered: entered, released: released, returned: returned))
  let active = Task { try await rows.makeAsyncIterator().next() }
  await entered.wait()
  rows.cancel()
  do {
    _ = try await active.value
    preconditionFailure("explicit cancellation did not throw")
  } catch { guard error is CancellationError else { throw error } }
  await released.fire()
  await returned.wait()
}

// Feature: public results preserve engine probability and detail when saved.
// Literal guide scenarios exercise loud reads for all states; shared consumer
// fixtures retain the exact precision and malformed-native rejection checks.
func rowRepresentation(_ client: Client) async throws {
  let row = try await client.choose(
    "Which queue?", record: "evidence", choices: ["billing", "shipping"])
  let answer = try row.answer
  let fields = try JSONSerialization.jsonObject(with: encoded(row)) as! [String: Any]
  precondition(fields["answer"] as? String == answer)
  precondition(fields["probability"] as? Double == row.probability && fields["p"] == nil)
  let detail = fields["detail"] as! [String: Any]
  precondition(detail["labels"] as? [String: Double] == row.detail?.labels)
  precondition(fields["item"] == nil)
}

// Feature: settings never disclose an explicit key through Swift encoding or inspection.
func redactedSettings() throws {
  let secret = "swift-test-secret-never-a-credential"
  let settings = try JSONDecoder().decode(
    Settings.self, from: Data("{\"backend\":\"stub\",\"api_key\":\"\(secret)\"}".utf8))
  let key = settings.apiKey!
  let views = [
    String(describing: settings), String(reflecting: settings), String(describing: key),
    String(reflecting: key),
  ]
  for view in views { precondition(!view.contains(secret)) }
  var inspection = ""
  dump(settings, to: &inspection)
  dump(key, to: &inspection)
  precondition(!inspection.contains(secret))
  struct Envelope: Encodable { let settings: Settings }
  let nested = String(decoding: try encoded(Envelope(settings: settings)), as: UTF8.self)
  precondition(!nested.contains(secret) && !nested.contains("api_key"))
  let cut =
    try JSONSerialization.jsonObject(
      with: encoded(Threshold.value(0.734567891)), options: [.fragmentsAllowed]) as! Double
  let band =
    try JSONSerialization.jsonObject(
      with: encoded(Threshold.band(low: 0.234567891, high: 0.834567891))) as! [String: Double]
  precondition(cut == 0.734567891 && band == ["low": 0.234567891, "high": 0.834567891])
}

// Feature: Swift record conversion preserves IDs and retains rows on scalar failures.
func recordResults(_ client: Client) async throws {
  struct Item<ID: Encodable>: Encodable {
    let id: ID
    let body: String
  }
  let ids = [
    try Record.encoding(Item(id: "string-id", body: "a")),
    try Record.encoding(Item(id: Int64.min, body: "b")),
    try Record.encoding(Item(id: UInt64.max, body: "c")),
  ]
  let rows = try await client.decide("Ready?", records: ids).collect()
  precondition(rows.count == 3 && rows.map(\.position) == [1, 2, 3])
  let idValues = try rows.map {
    try JSONSerialization.jsonObject(with: encoded($0.id!), options: [.fragmentsAllowed])
  }
  precondition(idValues[0] as? String == "string-id")
  precondition((idValues[1] as? NSNumber)?.stringValue == String(Int64.min))
  precondition((idValues[2] as? NSNumber)?.stringValue == String(UInt64.max))
  for record in [Record.missing, try Record.encoding(Item(id: ["invalid"], body: "x"))] {
    do {
      _ = try await client.decide("Ready?", record: record)
      preconditionFailure("record problem returned a scalar result")
    } catch let failure as Failed {
      precondition(failure.rows.count == 1)
      let row = failure.rows[0]
      precondition(row.item as? Record == record && row.position == 1)
      let name = try JSONDecoder().decode(String.self, from: encoded(failure.code))
      precondition(name == (record == .missing ? "record.missing" : "record.invalid"))
      let saved = try JSONSerialization.jsonObject(with: encoded(row)) as! [String: Any]
      precondition(saved["status"] as? String == "failed" && saved["answer"] == nil)
    }
  }
  let input = [Record.missing, .text("kept or not kept")]
  var kept: [Record] = []
  do {
    for try await record in client.filter("Ready?", records: input) { kept.append(record) }
    preconditionFailure("filter dropped a failed record")
  } catch let error as Failed {
    precondition(error.rows.count == 1 && error.rows[0].position == 1)
    precondition(error.rows[0].item as? Record == .missing)
  }
  precondition(kept.allSatisfy { $0 == input[1] })

}

// Feature: unrepresentable records stay failed rows without stopping the stream.
func recordEncodingFailures(_ client: Client) async throws {
  let invalid = Record.json(.number(.infinity))
  do {
    _ = try await client.decide("Ready?", record: invalid)
    preconditionFailure("invalid scalar input returned a result")
  } catch let failure as Failed {
    precondition(failure.code == .recordInvalid && failure.rows.count == 1)
    precondition(failure.rows[0].item as? Record == invalid)
  }
  let inputs = [Record.text("first"), invalid, .text("last")]
  let rows = try await client.decide("Ready?", records: ConfinedSource(records: inputs)).collect()
  precondition(rows.count == 3 && rows.map(\.position) == [1, 2, 3])
  precondition(rows[0].isAnswered && rows[2].isAnswered)
  precondition(rows[1].isFailed && rows[1].error?.code == .recordInvalid)
}

func surface() async throws {
  try await stressInputStreaming()
  let client = try Client(.init(backend: .stub))
  requireSendable(client)
  // Feature: host error presentation preserves the library's diagnostic message.
  let diagnostic = UsageError(
    code: ErrorCode(rawValue: "future.code"), message: "A future engine diagnostic")
  precondition(diagnostic.localizedDescription == "A future engine diagnostic")
  try checkPublicInterface()
  try redactedSettings()
  try await rowRepresentation(client)
  try await settledTextCollections(client)
  try await settledFilter(client)
  try await settledRank(client)
  try await settledQuestionMeanings(client)
  try await savedQuestions(client)
  try await isolatedIteration(client)
  try await cancelledSource(client)
  try await explicitlyCancelledSource(client)
  try await recordResults(client)
  try await recordEncodingFailures(client)
  do {
    _ = try await client.decide("Ready?", records: FailingSource()).collect()
    preconditionFailure("source failure was swallowed")
  } catch { try requireSourceError(error) }
  let retained: DecideRow<Record> = try await {
    let temporary = try Client(.init(backend: .stub))
    return try await temporary.decide("Ready?", record: Record.text("owned output"))
  }()
  let savedRetained = try JSONSerialization.jsonObject(with: encoded(retained)) as! [String: Any]
  precondition(
    savedRetained["function"] as? String == "decide" && retained.item == .text("owned output"))
}
