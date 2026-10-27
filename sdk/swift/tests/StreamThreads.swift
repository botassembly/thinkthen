import Foundation
import ThinkThen

#if os(macOS)
  import Darwin
#endif

private actor StreamBarrier {
  private var entered = 0
  private var waiters: [CheckedContinuation<Void, Never>] = []
  func arrive() async {
    entered += 1
    if entered == 100 {
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

#if os(Linux)
  private func streamThreads() throws -> Set<String> {
    let root = URL(fileURLWithPath: "/proc/self/task")
    return Set(
      try FileManager.default.contentsOfDirectory(atPath: root.path).filter { id in
        let name = try? String(
          contentsOf: root.appendingPathComponent(id + "/comm"), encoding: .utf8)
        return name?.trimmingCharacters(in: .whitespacesAndNewlines) == "ThinkThen.row"
      })
  }
#endif

#if os(macOS)
  private func streamThreads() throws -> Set<String> {
    var threads: thread_act_array_t?
    var count: mach_msg_type_number_t = 0
    precondition(task_threads(mach_task_self_, &threads, &count) == KERN_SUCCESS)
    guard let threads else { return [] }
    defer {
      vm_deallocate(
        mach_task_self_, vm_address_t(UInt(bitPattern: threads)),
        vm_size_t(Int(count) * MemoryLayout<thread_t>.stride))
    }
    var result: Set<String> = []
    for thread in UnsafeBufferPointer(start: threads, count: Int(count)) {
      defer { mach_port_deallocate(mach_task_self_, thread) }
      guard let pthread = pthread_from_mach_thread_np(thread) else { continue }
      var name = [CChar](repeating: 0, count: 64)
      guard pthread_getname_np(pthread, &name, name.count) == 0 else { continue }
      let text = String(
        decoding: name.prefix { $0 != 0 }.map { UInt8(bitPattern: $0) }, as: UTF8.self)
      if text == "ThinkThen.row" {
        var id: UInt64 = 0
        precondition(pthread_threadid_np(pthread, &id) == 0)
        result.insert(String(id))
      }
    }
    return result
  }
#endif

// Feature: A Swift stream drives every row on one persistent native thread.
func streamThreadLifetime() async throws {
  let client = try Client(.init(backend: .stub, batch: 1))
  let records = ConfinedSource(records: (0..<1000).map { .text("record \($0)") })
  var received = 0
  #if os(Linux) || os(macOS)
    var threads: Set<String> = []
  #endif
  for try await row in client.decide("Ready?", records: records) {
    received += 1
    precondition(row.position == UInt64(received))
    #if os(Linux) || os(macOS)
      let active = try streamThreads()
      precondition(active.count == 1, "stream has no dedicated thread")
      threads.formUnion(active)
    #endif
  }
  precondition(received == 1000)
  #if os(Linux) || os(macOS)
    precondition(threads.count == 1, "stream replaced its dedicated thread")
  #endif
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
