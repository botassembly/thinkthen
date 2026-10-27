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
  func read(_ rows: DecideRows) async throws -> [DecideRow] {
    var output: [DecideRow] = []
    let iterator = rows.makeAsyncIterator()
    while let row = try await iterator.next(isolation: self) { output.append(row) }
    return output
  }
}
func requireSendable<T: Sendable>(_: T) {}

// Keep error matching outside async typed-catch lowering in the pinned compiler.
func requireError(_ actual: any Error, equals expected: ThinkThenError) throws {
  guard let typed = actual as? ThinkThenError, typed == expected else { throw actual }
}
func requireSourceError(_ actual: any Error) throws {
  guard let typed = actual as? ThinkThenError else { throw actual }
  guard case .recordSource = typed else { throw actual }
}

// Feature: Swift actor callers consume sources without sending their iterators.
@MainActor func isolatedIteration(_ client: Client) async throws {
  let source = ConfinedSource(records: [.text("one"), .text("two")])
  let rows: DecideRows = client.decide("Ready?", records: source)
  requireSendable(rows)
  let first = rows.makeAsyncIterator()
  requireSendable(first)
  let rejected = rows.makeAsyncIterator()
  do {
    _ = try await rejected.next(isolation: MainActor.shared)
    preconditionFailure("second iterator was admitted")
  } catch { try requireError(error, equals: .iteration(.alreadyIterated)) }
  var positions: [UInt64] = []
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
  } catch { try requireError(error, equals: .iteration(.concurrentNext)) }
  active.cancel()
  do {
    _ = try await active.value
    preconditionFailure("task cancellation did not throw")
  } catch { try requireError(error, equals: .cancelled) }
  // Completion before release proves cancellation does not join the source.
  await released.fire()
  await returned.wait()
  let terminal = try await iterator.next()
  precondition(terminal == nil, "late source result reached a cancelled call")

  let beforeStart = client.decide("Ready?", records: [.text("never sent")])
  beforeStart.cancel()
  beforeStart.cancel()
  do {
    _ = try await beforeStart.makeAsyncIterator().next()
    preconditionFailure("cancellation before iteration was ignored")
  } catch { try requireError(error, equals: .cancelled) }
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
  } catch { try requireError(error, equals: .cancelled) }
  await released.fire()
  await returned.wait()
}

// Feature: public row values preserve typed detail and full precision through Codable.
func rowRepresentation() throws {
  let wire = Data(
    #"{"function":"choose","position":7,"id":"r7","source":{"question_identity":"q","backend":"openai","model":"fixture"},"status":"answered","answer":"shipping","probability":0.873194276251,"detail":{"labels":{"billing":0.126805723749,"shipping":0.873194276251}}}"#
      .utf8)
  let row = try JSONDecoder().decode(ChooseRow.self, from: wire)
  guard case .answered(let answer, let probability, let detail) = row.outcome else {
    preconditionFailure("answered row lost its case")
  }
  precondition(answer == "shipping" && probability == 0.873194276251)
  precondition(detail?.labels["billing"] == 0.126805723749)
  precondition(row.id == .string("r7") && row.position == 7 && row.source.backend == .openai)
  let copy = try JSONDecoder().decode(ChooseRow.self, from: encoded(row))
  precondition(copy == row)
  let fields = try JSONSerialization.jsonObject(with: encoded(copy)) as! [String: Any]
  precondition(fields["p"] == nil && fields["probability"] as? Double == probability)
  // A matching answer type cannot excuse the wrong function discriminator.
  let mismatched = Data(
    String(decoding: wire, as: UTF8.self).replacingOccurrences(
      of: "\"function\":\"choose\"", with: "\"function\":\"decide\""
    ).utf8)
  do {
    _ = try JSONDecoder().decode(ChooseRow.self, from: mismatched)
    preconditionFailure("wrong function decoded as ChooseRow")
  } catch is DecodingError {}
  let unsure = Data(
    #"{"function":"decide","position":1,"source":{"question_identity":"q","backend":"systemone","model":"fixture"},"status":"unsure","probability":0.51234567891}"#
      .utf8)
  let unsureRow = try JSONDecoder().decode(DecideRow.self, from: unsure)
  precondition(unsureRow.status == .unsure && unsureRow.answer == nil)
  let unsureFields = try JSONSerialization.jsonObject(with: encoded(unsureRow)) as! [String: Any]
  precondition(unsureFields["answer"] == nil)
  let malformed = Data(
    String(decoding: unsure, as: UTF8.self).dropLast().appending(",\"answer\":true}").utf8)
  do {
    _ = try JSONDecoder().decode(DecideRow.self, from: malformed)
    preconditionFailure("contradictory outcome fields were ignored")
  } catch is DecodingError {}
  let absent = Data(
    #"{"function":"find","position":1,"source":{"question_identity":"q","backend":"stub","model":"fixture"},"status":"answered","answer":null,"probability":0.9}"#
      .utf8)
  let find = try JSONDecoder().decode(FindRow.self, from: absent)
  guard case .answered(let span, _, _) = find.outcome else {
    preconditionFailure("find null became unsure")
  }
  precondition(span == nil)
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

// Feature: Swift record conversion preserves supported IDs and yields failures as rows.
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
    let row: DecideRow = try await client.decide("Ready?", record: record)
    guard case .failed(let failure) = row.outcome else {
      preconditionFailure("record problem escaped its failed row")
    }
    let name = try JSONDecoder().decode(String.self, from: encoded(failure.code))
    precondition(name == (record == .missing ? "record.missing" : "record.invalid"))
    let roundTrip = try JSONDecoder().decode(DecideRow.self, from: encoded(row))
    precondition(roundTrip == row)
  }
  let input = [Record.missing, .text("kept or not kept")]
  let filtered = try await client.filter("Ready?", records: input).collect()
  precondition(filtered.count == 2 && filtered.map(\.item) == input)
  precondition(filtered[0].row.position == 1 && !filtered[0].kept)
  for result in filtered {
    if case .answered(let kept, _, _) = result.row.outcome { precondition(result.kept == kept) }
  }
}

// Feature: unrepresentable records report local encoding failures on both input paths.
func recordEncodingFailures(_ client: Client) async throws {
  let invalid = Record.json(.number(.infinity))
  do {
    _ = try await client.decide("Ready?", record: invalid)
    preconditionFailure("finite record encoding unexpectedly succeeded")
  } catch {
    guard case ThinkThenError.encoding = error else { throw error }
  }
  do {
    _ = try await client.decide("Ready?", records: ConfinedSource(records: [invalid])).collect()
    preconditionFailure("stream record encoding unexpectedly succeeded")
  } catch {
    guard case ThinkThenError.encoding = error else { throw error }
  }
}

func surface() async throws {
  try await stressInputStreaming()
  let client = try Client(.init(backend: .stub))
  requireSendable(client)
  // Feature: host error presentation preserves the library's diagnostic message.
  let diagnostic = ThinkThenError.unknownCode(
    rawCode: 9999, message: "A future engine diagnostic", httpStatus: nil, rawName: nil)
  precondition(diagnostic.localizedDescription == "A future engine diagnostic")
  precondition(ThinkThenError.cancelled.localizedDescription == "The call was cancelled")
  try redactedSettings()
  try rowRepresentation()
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
  let retained: DecideRow = try await {
    let temporary = try Client(.init(backend: .stub))
    return try await temporary.decide("Ready?", record: .text("owned output"))
  }()
  let ownedCopy = try JSONDecoder().decode(DecideRow.self, from: encoded(retained))
  precondition(ownedCopy == retained)
}
