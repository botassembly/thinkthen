import Foundation

#if os(macOS)
  import Darwin
#elseif os(Linux)
  import Glibc
#endif

// The external audit identifies Foundation pthread entries and their exits.
// Inherited runtime labels never register a bridge identity with the audit.
enum BridgeAudit {
  private static func symbol(_ name: String) -> UnsafeMutableRawPointer {
    guard let process = dlopen(nil, RTLD_NOW) else {
      preconditionFailure("cannot inspect the consumer process")
    }
    defer { dlclose(process) }
    guard let address = dlsym(process, name) else {
      preconditionFailure("run stream lifetime checks through the pthread audit")
    }
    return address
  }
  static func worker() -> UInt64 {
    unsafeBitCast(symbol("thinkthen_audit_worker"), to: (@convention(c) () -> UInt64).self)()
  }
  static func count() -> UInt64 {
    unsafeBitCast(symbol("thinkthen_audit_count"), to: (@convention(c) () -> UInt64).self)()
  }
  #if os(Linux)
    static func labelCurrent() {
      let name = unsafeBitCast(
        symbol("pthread_setname_np"),
        to: (@convention(c) (pthread_t, UnsafePointer<CChar>) -> Int32).self)
      precondition("ThinkThen.row".withCString { name(pthread_self(), $0) } == 0)
    }
  #endif
  static func join() async {
    let wait = unsafeBitCast(symbol("thinkthen_audit_join"), to: (@convention(c) () -> Void).self)
    await withCheckedContinuation { continuation in
      DispatchQueue.global().async {
        wait()
        continuation.resume()
      }
    }
    precondition(worker() == 0, "a bridge worker survived stream cleanup")
  }
}

#if os(Linux)
  // Linux inherits the creator's OS label. The child never names itself and
  // must not be mistaken for the stream's explicitly named bridge worker.
  final class InheritedLabel: Sendable {
    private final class State: Sendable {
      let entered = DispatchSemaphore(value: 0)
      let released = DispatchSemaphore(value: 0)
      let returned = DispatchSemaphore(value: 0)
    }
    private let state = State()
    init() {
      let state = state
      Thread {
        BridgeAudit.labelCurrent()
        var thread = pthread_t()
        let context = Unmanaged.passRetained(state).toOpaque()
        precondition(
          pthread_create(
            &thread, nil,
            { context in
              let state = Unmanaged<State>.fromOpaque(context!).takeRetainedValue()
              precondition(Thread.current.name == "ThinkThen.row", "OS label was not inherited")
              state.entered.signal()
              state.released.wait()
              state.returned.signal()
              return nil
            }, context) == 0)
        precondition(pthread_detach(thread) == 0)
      }.start()
      state.entered.wait()
    }
    func stop() {
      state.released.signal()
      state.returned.wait()
    }
  }
#endif
