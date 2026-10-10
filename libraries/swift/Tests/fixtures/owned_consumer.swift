import Foundation
import ThinkThen

@main enum OwnedConsumer {
    static func expect(_ condition: Bool, _ message: String) throws {
        if !condition { throw JSONConversionError(message) }
    }
    static func arrived(_ path: String) async throws {
        let until = ContinuousClock.now + .seconds(10)
        while !FileManager.default.fileExists(atPath: path) {
            if ContinuousClock.now >= until { throw JSONConversionError("Provider did not arrive") }
            try await Task.sleep(for: .milliseconds(10))
        }
    }
    static func main() async {
        do { try await run() }
        catch { FileHandle.standardError.write(Data("Swift consumer failed: \(error)\n".utf8)); exit(1) }
    }
    static func run() async throws {
        let raw = Data(#"{"future":1e400,"large":9007199254740993,"duplicate":1,"duplicate":2,"escaped":"\uD83D\uDE00"}"#.utf8)
        try expect(try JSONValue.parse(raw).data() == raw, "Lossless original JSON")
        let terminalBytes = Data(#"{"kind":"terminal","failure":null,"future":{"large":9007199254740993,"n":1e400}}"#.utf8)
        let terminal = try OwnedSessionPacketTerminal.read(JSONValue.parse(terminalBytes))
        try expect(!terminal.facts.isPresent && terminal.failure.isPresent && terminal.failure.value == nil, "Missing and null presence")
        try expect(try terminal.json.data() == terminalBytes, "Generated unknown-field round trip")
        for bad in [#""\uD800""#, #""\uDC00""#, "[01]", "[1e]", "[1.]", "[true,]"] {
            var refused = false
            do { _ = try JSONValue.parse(Data(bad.utf8)) } catch { refused = true }
            try expect(refused, "Invalid JSON accepted")
        }
        var refusedInfinity = false
        do { _ = try Double.infinity.json.data() } catch { refusedInfinity = true }
        try expect(refusedInfinity, "Infinity encoded")
        let settings = try InputEngineSettings.read(JSONValue.parse(Data(CommandLine.arguments[1].utf8)))
        let barrier = CommandLine.arguments[2]
        let heldClient = try Client(settings: settings)
        let held = Task { try await heldClient.decide(.text("Is it?"), input: .text("hold-owned-swift")) }
        try await arrived(barrier + "/arrived-hold-owned-swift")
        let progress = await Task { "independent" }.value
        try expect(progress == "independent", "Unrelated task stalled")
        held.cancel()
        do { _ = try await held.value; throw JSONConversionError("Cancelled task answered") }
        catch is CancellationError { }
        heldClient.close()
        try expect(!FileManager.default.fileExists(atPath: barrier + "/release-hold-owned-swift"), "Provider released before cancellation and cleanup")
        _ = FileManager.default.createFile(atPath: barrier + "/release-hold-owned-swift", contents: Data())
        let client = try Client(settings: settings)
        let call = try await client.decide(.text("Is it?"), input: .text("owned-swift"))
        let rows: [OwnedAtomicDecideValue] = call.packets.compactMap { if case .decideRow(let row) = $0 { return row.value }; return nil }
        guard let row = rows.first else { throw JSONConversionError("Missing typed decide row") }
        try expect(try row.value.json.data() == Data("true".utf8), "Typed native answer")
        try expect(call.terminal.facts.isPresent, "Native final facts missing")
        let facts = try call.terminal.json.data()
        do { _ = try await client.decide(.text("Is it?"), input: .text("status-401")); throw JSONConversionError("Backend failure answered") }
        catch let failure as SessionFailure {
            guard case .backend = failure.kind else { throw JSONConversionError("Wrong typed failure kind") }
            try expect(failure.failure.facts.isPresent && failure.call.terminal.facts.isPresent, "Failure facts lost")
        }
        client.close()
        try expect(try call.terminal.json.data() == facts, "Result ownership ended with client")
        print("OWNED_SWIFT_PASS typed answer/failure, presence, lossless JSON, independent task, cancellation and cleanup")
    }
}
