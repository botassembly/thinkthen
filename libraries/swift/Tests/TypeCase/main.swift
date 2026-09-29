import Foundation
import ThinkThen

do {
    guard CommandLine.arguments.count == 2 else { throw DoorFailure(code: 1, retryable: false, message: "one request required") }
    let engine = try Engine()
    defer { engine.close() }
    print(try engine.call(CommandLine.arguments[1]))
} catch let failure as DoorFailure {
    let value: [String: Any] = ["error": String(describing: failure.kind), "retryable": failure.retryable,
                                "facts": failure.factsJSON.map { $0 as Any } ?? NSNull()]
    let data = try JSONSerialization.data(withJSONObject: value, options: [.sortedKeys])
    print(String(decoding: data, as: UTF8.self))
}
