import Foundation
import ThinkThen

private actor StreamBarrier {
  private var entered = 0
  private var waiters: [CheckedContinuation<Void, Never>] = []
  func arrive() async {
    entered += 1
    if entered == 100 {
      precondition(BridgeAudit.count() == 100, "concurrent streams lost a bridge worker")
      for waiter in waiters { waiter.resume() }
      waiters.removeAll()
    } else {
      await withCheckedContinuation { waiters.append($0) }
    }
  }
}

private struct ConcurrentRecords: AsyncSequence, Sendable {
  typealias Element = Record
  let barrier: StreamBarrier
  struct AsyncIterator: AsyncIteratorProtocol {
    let barrier: StreamBarrier
    var index = 0
    mutating func next() async -> Record? {
      if index == 0 { await barrier.arrive() }
      guard index < 10 else { return nil }
      defer { index += 1 }
      return .text("record \(index)")
    }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator(barrier: barrier) }
}

// Feature: A Swift stream drives every row on one persistent native thread.
func streamThreadLifetime() async throws {
  #if os(Linux)
    let inherited = InheritedLabel()
    defer { inherited.stop() }
    await BridgeAudit.join()
  #endif
  let client = try Client(.init(backend: .stub, batch: 1))
  let records = ConfinedSource(records: (0..<1000).map { .text("record \($0)") })
  var received = 0
  var worker: UInt64?
  for try await row in client.decide("Ready?", records: records) {
    received += 1
    precondition(row.position == UInt64(received))
    let active = BridgeAudit.worker()
    precondition(active != 0, "stream does not have exactly one dedicated bridge thread")
    if let worker {
      precondition(active == worker, "stream replaced its dedicated bridge thread")
    } else {
      worker = active
    }
  }
  precondition(received == 1000)
  await BridgeAudit.join()
}

// Feature: cancellation and iterator release retire the stream's bridge worker.
func streamThreadCleanup() async throws {
  let client = try Client(.init(backend: .stub, batch: 1))
  let entered = Signal()
  let released = Signal()
  let returned = Signal()
  let rows = client.decide(
    "Ready?", records: StubbornSource(entered: entered, released: released, returned: returned))
  let pending = Task { try await rows.makeAsyncIterator().next() }
  await entered.wait()
  precondition(BridgeAudit.worker() != 0, "waiting input lost its bridge worker")
  rows.cancel()
  do {
    _ = try await pending.value
    preconditionFailure("cancelled input returned a row")
  } catch { try requireError(error, equals: .cancelled) }
  await BridgeAudit.join()
  await released.fire()
  await returned.wait()

  let early = client.decide(
    "Ready?", records: ConfinedSource(records: (0..<1000).map { .text("record \($0)") }))
  var iterator: DecideRows.AsyncIterator? = early.makeAsyncIterator()
  let first = try await iterator!.next()
  precondition(first?.position == 1 && BridgeAudit.worker() != 0)
  iterator = nil
  await BridgeAudit.join()

  let unopened = client.decide("Ready?", records: [.text("never sent")])
  unopened.cancel()
  do {
    _ = try await unopened.makeAsyncIterator().next()
    preconditionFailure("cancellation before iteration was ignored")
  } catch { try requireError(error, equals: .cancelled) }
  precondition(BridgeAudit.count() == 0, "an unopened stream started a bridge worker")
}

// Feature: Swift streams do not hold a shared worker pool slot while awaiting input.
func concurrentStreamThreads() async throws {
  let client = try Client(.init(backend: .stub, batch: 1))
  let barrier = StreamBarrier()
  try await withThrowingTaskGroup(of: Int.self) { group in
    for _ in 0..<100 {
      group.addTask {
        let rows = try await client.decide(
          "Ready?", records: ConcurrentRecords(barrier: barrier)
        ).collect()
        precondition(rows.map(\.position) == Array(UInt64(1)...10))
        return rows.count
      }
    }
    var received = 0
    for try await count in group { received += count }
    precondition(received == 1000)
  }
  await BridgeAudit.join()
}

private struct PausedTail: AsyncSequence, Sendable {
  typealias Element = Record
  let entered: Signal, released: Signal, returned: Signal
  struct AsyncIterator: AsyncIteratorProtocol {
    let source: PausedTail
    var first = true
    mutating func next() async -> Record? {
      if first {
        first = false
        return .text("ready evidence")
      }
      FileHandle.standardOutput.write(Data("input\n".utf8))
      await source.entered.fire()
      await source.released.wait()
      await source.returned.fire()
      return nil
    }
  }
  func makeAsyncIterator() -> AsyncIterator { AsyncIterator(source: self) }
}

// Feature: Output remains responsive while the caller's next input is suspended.
func outputWhileInputWaits(_ url: String) async throws {
  let client = try Client(.init(url: URL(string: url)!, batch: 1))
  let entered = Signal()
  let released = Signal()
  let returned = Signal()
  let source = PausedTail(entered: entered, released: released, returned: returned)
  let rows = client.decide("Ready?", records: source)
  let iterator = rows.makeAsyncIterator()
  let first = Task { try await iterator.next() }
  await entered.wait()
  let row = try await first.value
  precondition(row?.position == 1)
  rows.cancel()
  do {
    _ = try await iterator.next()
    preconditionFailure("cancelled stream accepted another next")
  } catch { try requireError(error, equals: .cancelled) }
  await released.fire()
  await returned.wait()
}

// Feature: The installed Swift stream starts one bridge thread for 1,000 rows.
// The parent audits pthread_create externally after warming the same process.
func auditStreamThreadStarts(_ records: Int) async throws {
  let client = try Client(.init(backend: .stub, batch: 1))
  let input = ConfinedSource(records: (0..<1000).map { .text("record \($0)") })
  for _ in 0..<8 {
    let rows = try await client.decide("Ready?", records: input).collect()
    precondition(rows.count == 1000)
  }
  FileHandle.standardOutput.write(Data("ready\n".utf8))
  precondition(FileHandle.standardInput.readData(ofLength: 1) == Data("c".utf8))
  var received = 0
  if records > 0 {
    for try await row in client.decide("Ready?", records: input) {
      received += 1
      precondition(row.position == UInt64(received))
    }
  }
  precondition(received == records)
  FileHandle.standardOutput.write(Data("rows \(received)\n".utf8))
  precondition(FileHandle.standardInput.readData(ofLength: 1) == Data("x".utf8))
}
