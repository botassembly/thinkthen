import Foundation

guard CommandLine.arguments.count == 3 else { fatalError("configured URL and cache required") }
let base = CommandLine.arguments[1]
let cache = CommandLine.arguments[2]
let configured = try Engine(settingsJSON: "{\"base_url\":\"\(base)\",\"cache\":\"\(cache)\"}")
let environment = try Engine()
let empty = try Engine(settingsJSON: "{}")
for invalid in ["{\"nope\":1}", "{\"timeout\":\"30\"}"] {
    do { _ = try Engine(settingsJSON: invalid); fatalError("invalid settings accepted") }
    catch let failure as DoorFailure {
        precondition(failure.kind == .usage && !failure.retryable && !failure.message.isEmpty && failure.factsJSON == nil)
    }
}
let a = try configured.decide("Is it?", "swift-settings")
let b = try environment.decide("Is it?", "swift-env")
let c = try empty.decide("Is it?", "swift-empty")
precondition(a.outcome == .yes && b.outcome == .yes && c.outcome == .yes)
configured.close(); environment.close(); empty.close()
print("SWIFT_SETTINGS_PASS")
