import Foundation
import Synchronization
import ThinkThen

private struct StressRecords: Sequence, Sendable {
  let count: Int, offset: Int
  let generated: (@Sendable () -> Void)?
  struct Iterator: IteratorProtocol {
    let count: Int, offset: Int
    let generated: (@Sendable () -> Void)?
    var index = 0
    mutating func next() -> Record? {
      guard index < count else { return nil }
      defer { index += 1 }
      let id = offset + index
      generated?()
      return .json(.object(["id": .string("record-\(id)"), "body": .string("record \(id)")]))
    }
  }
  func makeIterator() -> Iterator { Iterator(count: count, offset: offset, generated: generated) }
}

private func stressRecords(
  _ count: Int, offset: Int, generated: (@Sendable () -> Void)? = nil
) -> some Sequence<Record> & Sendable {
  StressRecords(count: count, offset: offset, generated: generated)
}

private final class StressGeneration: Sendable {
  let produced = Mutex(0)
}

// Feature: The stress source generates records only when iteration requests input.
func stressInputStreaming() async throws {
  let generation = StressGeneration()
  let source = stressRecords(1000, offset: 10_000_000) {
    generation.produced.withLock { $0 += 1 }
  }
  precondition(generation.produced.withLock { $0 } == 0)
  let client = try Client(.init(backend: .stub, batch: 1))
  let rows = client.decide("Ready?", records: source, options: .init(evidence: "/body"))
  precondition(generation.produced.withLock { $0 } == 0)
  let first = try await rows.makeAsyncIterator().next()
  precondition(first?.id == .string("record-10000000") && first?.position == 1)
  precondition(generation.produced.withLock { $0 } > 0)
  rows.cancel()
}

// Feature: Swift tasks share a client and retain independent rows under faults.
func stress(_ args: [String]) async throws {
  let client = try Client(
    decoded(Settings.self, JSONSerialization.jsonObject(with: Data(args[2].utf8))))
  let count = Int(args[3])!
  let question = DecideQuestion("Ready?", options: .init(evidence: "/body"))
  let cancel = args[1] == "stress-cancel"
  let workers = cancel ? 1 : Int(args[7])!
  var failures = 0
  var widths: UInt64 = 0
  let rounds = Int(args[8])!
  for round in 0..<rounds {
    if cancel {
      let task = Task {
        for try await _ in client.decide(question, records: stressRecords(count, offset: 0)) {
          preconditionFailure("held stream published a row")
        }
      }
      await withCheckedContinuation { continuation in
        DispatchQueue.global().async {
          precondition(FileHandle.standardInput.readData(ofLength: 1) == Data("c".utf8))
          continuation.resume()
        }
      }
      task.cancel()
      do {
        try await task.value
        preconditionFailure("held stream completed")
      } catch { try requireError(error, equals: .cancelled) }
    } else {
      try await withThrowingTaskGroup(of: (Int, UInt64).self) { group in
        for worker in 0..<workers {
          let offset = Int(args[9])! + (round * workers + worker) * count
          group.addTask {
            var received = 0
            var failed = 0
            var mask: UInt64 = 0
            for try await row in client.decide(
              question, records: stressRecords(count, offset: offset))
            {
              received += 1
              if case .string(let id) = row.id { mask |= UInt64(1) << id.count }
              precondition(row.position == UInt64(received))
              precondition(row.id == .string("record-\(offset + received - 1)"))
              switch row.outcome {
              case .answered(let answer, let probability, _):
                precondition(answer && probability == Double(args[6])!)
              case .failed(let failure):
                failed += 1
                precondition(failure.code.rawValue == args[5])
              case .unsure: preconditionFailure("unexpected unsure row")
              }
            }
            precondition(received == count)
            return (failed, mask)
          }
        }
        for try await (failed, mask) in group {
          failures += failed
          widths |= mask
        }
      }
    }
    if let hook = ProcessInfo.processInfo.environment["THINKTHEN_STRESS_CHECKPOINT_COMMAND"] {
      let checkpoint = Process()
      checkpoint.executableURL = URL(fileURLWithPath: "/bin/sh")
      checkpoint.arguments = ["-c", hook]
      try checkpoint.run()
      checkpoint.waitUntilExit()
      precondition(checkpoint.terminationStatus == 0)
    }
  }
  precondition(cancel || (args[4] == "failed" ? failures > 0 : failures == 0))
  let summary: [String: Any] = [
    "identifier_width_mask": widths, "calls": workers * rounds,
    "rows": cancel ? 0 : workers * count * rounds,
    "failed": failures, "cancelled": cancel,
  ]
  print(String(decoding: try JSONSerialization.data(withJSONObject: summary), as: UTF8.self))
}
