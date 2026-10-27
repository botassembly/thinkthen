import Foundation
import Synchronization

private final class WorkerExit: Sendable {
  private struct State: Sendable {
    var finished = false
    var waiters: [CheckedContinuation<Void, Never>] = []
  }
  private let state = Mutex(State())
  func wait() async {
    await withCheckedContinuation { continuation in
      let ready = state.withLock { state in
        if state.finished { return true }
        state.waiters.append(continuation)
        return false
      }
      if ready { continuation.resume() }
    }
  }
  func finish() {
    let waiters = state.withLock { state in
      state.finished = true
      defer { state.waiters.removeAll() }
      return state.waiters
    }
    for waiter in waiters { waiter.resume() }
  }
}
// The dedicated thread owns all driving and frees both native owners after its
// final wait returns. Cancellation and input readiness only wake its event.
struct NativeHandle: @unchecked Sendable { let pointer: OpaquePointer }
private final class CallWorker: Sendable {
  private struct State: Sendable {
    var handle: NativeHandle?
    var events: NativeHandle?
    var operation: (@Sendable (OpaquePointer, OpaquePointer, Native) -> Void)?
    var stopped = false
  }
  private let state: Mutex<State>
  private let native: Native
  private let finished = WorkerExit()
  init(native: Native, handle: NativeHandle) {
    self.native = native
    state = Mutex(
      State(handle: handle, events: native.events(handle.pointer).map { NativeHandle(pointer: $0) })
    )
    Thread { [self] in run() }.start()
  }
  func wake() { state.withLock { if let events = $0.events { native.eventWake(events.pointer) } } }
  func cancel() { state.withLock { if let handle = $0.handle { native.cancel(handle.pointer) } } }
  func stop() {
    state.withLock {
      $0.stopped = true
      if let handle = $0.handle { native.cancel(handle.pointer) }
      if let events = $0.events { native.eventWake(events.pointer) }
    }
  }
  func join() async { await finished.wait() }
  func submit(_ operation: @escaping @Sendable (OpaquePointer, OpaquePointer, Native) -> Void)
    -> Bool
  {
    state.withLock {
      guard !$0.stopped else { return false }
      $0.operation = operation
      if let events = $0.events { native.eventWake(events.pointer) }
      return true
    }
  }
  private func run() {
    Thread.current.name = "ThinkThen.row"
    guard
      let (handle, events) = state.withLock({ state in
        state.handle.flatMap { handle in state.events.map { (handle, $0) } }
      })
    else {
      finished.finish()
      return
    }
    while true {
      let version = native.eventVersion(events.pointer)
      let (operation, stopped) = state.withLock {
        let operation = $0.operation
        $0.operation = nil
        return (operation, $0.stopped)
      }
      if let operation { operation(handle.pointer, events.pointer, native) }
      if stopped { break }
      if operation == nil { native.eventWait(events.pointer, version) }
    }
    state.withLock {
      native.free(handle.pointer)
      native.eventFree(events.pointer)
      $0.handle = nil
      $0.events = nil
    }
    finished.finish()
  }
}
final class NativeCall: Sendable {
  private let worker: CallWorker
  init(client: Client, request: Data, name: String?, records: [Record]?) throws {
    let native = try Native.loaded.get()
    worker = CallWorker(
      native: native, handle: try client.open(request, name: name, records: records))
  }
  func cancel() { worker.cancel() }
  func wake() { worker.wake() }
  func perform<T: Sendable>(
    _ operation: @escaping @Sendable (OpaquePointer, OpaquePointer, Native) throws -> T
  )
    async throws -> T
  {
    try await withCheckedThrowingContinuation { continuation in
      if !worker.submit({ handle, events, native in
        do { continuation.resume(returning: try operation(handle, events, native)) } catch {
          continuation.resume(throwing: error)
        }
      }) {
        continuation.resume(throwing: ThinkThenError.cancelled)
      }
    }
  }
  func close() async {
    worker.stop()
    await worker.join()
  }
  deinit { worker.stop() }
}

// One task owns the caller's iterator for the entire stream. A single demand
// and result slot bound input, including when a caller ignores cancellation.
final class InputPump: Sendable {
  private struct State: Sendable {
    var demand: CheckedContinuation<Bool, Never>?
    var requested = false, stopped = false
    var result: Result<Record?, any Error>?
  }
  private let state = Mutex(State())
  private let task = Mutex<Task<Void, Never>?>(nil)
  init(source: any RecordSource, wake: @escaping @Sendable () -> Void) {
    task.withLock { task in
      task = Task { [self] in
        while await withCheckedContinuation({ continuation in
          let ready = state.withLock { state -> Bool? in
            if state.stopped { return false }
            if state.requested {
              state.requested = false
              return true
            }
            state.demand = continuation
            return nil
          }
          if let ready { continuation.resume(returning: ready) }
        }) {
          let result: Result<Record?, any Error>
          do { result = .success(try await source.next()) } catch {
            result = .failure(
              error is CancellationError
                ? ThinkThenError.cancelled
                : ThinkThenError.recordSource(message: String(describing: error)))
          }
          state.withLock { if !$0.stopped { $0.result = result } }
          wake()
        }
      }
    }
  }
  func request() {
    let demand = state.withLock {
      let demand = $0.demand
      $0.demand = nil
      if demand == nil { $0.requested = true }
      return demand
    }
    demand?.resume(returning: true)
  }
  func take() -> Result<Record?, any Error>? {
    state.withLock {
      let result = $0.result
      $0.result = nil
      return result
    }
  }
  func cancel() {
    let demand = state.withLock { state in
      state.stopped = true
      state.result = nil
      defer { state.demand = nil }
      return state.demand
    }
    demand?.resume(returning: false)
    task.withLock {
      $0?.cancel()
      $0 = nil
    }
  }
}
